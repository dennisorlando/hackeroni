use std::error::Error;

use actix_identity::{Identity, IdentityExt, IdentityMiddleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{get, middleware, web, App, HttpRequest, HttpServer, Responder};
use auth::{init_auth, Admin, Authenticated, User};
use db::{
    initialize_db_pool, run_migrations,
    stations::{update_station_plugs, update_stations, PlugsInfo, StationInfo},
    DbPool,
};
use dotenvy::dotenv;
use futures::future::join;
use log::*;
use odh::{EChargingPlug, EChargingStation, ODHBuilder};
use osrm::init_osrm;
use tokio::task::JoinHandle;
pub mod auth;
pub mod config;
pub mod db;
pub mod log;
pub mod odh;
pub mod osrm;

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/add_name/{name}")]
async fn hello(
    _pool: web::Data<DbPool>,
    _name: web::Path<String>,
    user: User<Authenticated>,
) -> actix_web::Result<impl Responder> {
    Ok(format!("Hello {}!", user.user.name))
}

#[get("/whoami")]
async fn whoami(request: HttpRequest) -> actix_web::Result<impl Responder> {
    let x: Identity = request
        .get_identity()
        .map_err(actix_web::error::ErrorUnauthorized)?;
    Ok(format! {"you are \"{}\", motherfucker", x.id()?})
}
#[get("/onlyadmin")]
async fn onlyadmin(_: User<Admin>) -> impl Responder {
    "you reaaaaly have power"
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //get_info().await;
    dotenv().ok();
    init_log();
    let conf = config::load_config();
    let pool = initialize_db_pool(conf.database_url.clone());
    {
        
        let conn = &mut pool.get().unwrap();
        run_migrations(conn)?;
    }
    

    let pool1=pool.clone();
    let url = conf.odh_hub_url.clone();
    let f1: JoinHandle<Result<(), Box<dyn Error + Send + Sync>>> = tokio::spawn(async move{
        info!("add charging stations");
        let result: Vec<EChargingStation> = ODHBuilder::new(url).run().await?;

        web::block(move ||{
            let mut conn = pool1.get()?;
            let stations: Vec<StationInfo> = result
            .into_iter()
            .map(|x| StationInfo {
                coordinate_lat: x.scoordinate.x,
                coordinate_long: x.scoordinate.y,
                id: x.scode,
                name: x.sname,
            })
            .collect();

            update_stations(&mut conn, stations)?;
            Ok::<(), Box<dyn Error + Send + Sync>>(())
        }).await??;
        Ok(())
    });
    let pool1=pool.clone();
    let url = conf.odh_hub_url.clone();
    let f2: JoinHandle<Result<(), Box<dyn Error + Send + Sync>>> = tokio::spawn(async move{
        info!("adding plugs");
        // add plugs
        let result: Vec<EChargingPlug> = ODHBuilder::new(url).run().await?;
        web::block(move ||{
            let conn = &mut pool1.get().unwrap();
            let plugs = result
                .into_iter()
                .map(|x| PlugsInfo {
                    id: x.pcode,
                    station_id: x.scode,
                    name: x.pname,
                    max_power: x.smetadata.outlets.clone().and_then(|x| x[0].maxPower),
                    max_current: x.smetadata.outlets.clone().and_then(|x| x[0].maxCurrent),
                    min_current: x.smetadata.outlets.clone().and_then(|x| x[0].minCurrent),
                    has_fixed_cable: x.smetadata.outlets.clone().and_then(|x| x[0].hasFixedCable),
                    outlet_type_code: x
                        .smetadata
                        .outlets
                        .clone()
                        .and_then(|x| x[0].outletTypeCode.clone()),
                })
                .collect();
            update_station_plugs(conn, plugs)?;
            Ok::<(), Box<dyn Error + Send + Sync>>(())
        }).await??;
        
        Ok(())
    });
    let (f1, f2)= join(f1, f2).await;
    f1?.unwrap();
    f2?.unwrap();
    
    info!("updated db");
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(hello)
            .service(whoami)
            .service(onlyadmin)
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                conf.cookie_secret.clone(),
            ))
            .app_data(web::Data::new(conf.clone()))
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .configure(init_auth)
            .configure(init_osrm)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;
    Ok(())
}
