use std::error::Error;

use actix_identity::{Identity, IdentityExt, IdentityMiddleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{get, middleware, web, App, HttpRequest, HttpServer, Responder};
use auth::{init_auth, Admin, Authenticated, User};
use db::{initialize_db_pool, run_migrations,  DbPool};
use dotenvy::dotenv;
use log::*;
use osrm::hardcoded::get_info;
pub mod db;
pub mod log;
pub mod auth;
pub mod config;
pub mod stable_diffusion;
pub mod osrm;

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/add_name/{name}")]
async fn hello(
    _pool: web::Data<DbPool>,
    _name: web::Path<String>,
    user: User<Authenticated>
) -> actix_web::Result<impl Responder> {

    Ok(format!("Hello {}!", user.user.name))
}



#[get("/whoami")]
async fn whoami(request: HttpRequest) -> actix_web::Result<impl Responder> {
    let x: Identity = request.get_identity().map_err(actix_web::error::ErrorUnauthorized)?;
    Ok(format!{"you are \"{}\", motherfucker", x.id()?})
}
#[get("/onlyadmin")]
async fn onlyadmin(_: User<Admin>) -> impl Responder {
    "you reaaaaly have power"
}


#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    get_info().await;
    dotenv().ok();
    init_log();
    let conf = config::load_config();
    let pool = initialize_db_pool(conf.database_url.clone());
    run_migrations(&mut pool.get().unwrap())?;


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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}
