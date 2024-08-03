use actix_web::{get, post, web::{self, Data, Form}, Responder};
use log::error;
use request::{OSRMRequest, OSRMResponse};
use serde::Serialize;
use thiserror::Error;
use crate::{config::AppConfig, db::{stations::{get_all_stations, StationInfo}, DbPool}, odh::{get_near_stations, EChargingStation, ODHBuilder}};

use self::request::PathRequest;

pub mod hardcoded;
pub mod request;

#[derive(Error, Debug)]
pub enum OSRMError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    /*#[error("Build failed. Field {0} not provided")]
    Build(&'static str ),*/
}

impl From<OSRMError> for actix_web::Error {
    fn from(value: OSRMError) -> Self {
        error!("{value}");
        actix_web::error::ErrorInternalServerError("Internal server error")
    }
}


#[get("/get_all_stations")]
pub async fn fuck_all_stations(config: Data<AppConfig>,  pool: Data<DbPool>) -> actix_web::Result<impl Responder> {
    let stations = web::block(move || {
        let mut conn = pool.get().unwrap();
        get_all_stations(&mut conn)
    }).await?.unwrap();
    let stations = serde_json::to_string(&stations)?;
    Ok(stations)
}

#[derive(Serialize, Debug)]
struct PathResult{
    duration: f64,
    distance: f64,
    station: StationInfo,
}


#[post("/stocazzo")]
pub async fn get_route(req: Form<PathRequest>, config: Data<AppConfig>, pool: Data<DbPool>) -> actix_web::Result<impl Responder> {
    
    let destination = (req.destination_long, req.destination_lat);
    let stations_orig = get_near_stations(&config.odh_hub_url, destination, config.max_walking_meters).await?;
    let stations = stations_orig.clone().into_iter().map(|x| (x.coordinate_lat as f64, x.coordinate_long as f64)).collect();
    let query = OSRMRequest::new(stations, destination, req.preferences.max_walking_time.unwrap_or(10)).build()?;


    let url = config.osrm_url.clone() +"/"+ &query;
    let content = reqwest::get(url).await
        .map_err(|x| OSRMError::from(x))?
        .text()
        .await
        .map_err(|x| OSRMError::from(x))?;
    println!("{:?}", content);
    let response: OSRMResponse = serde_json::from_str(&content)?;
    
    let mut result: Vec<PathResult> = response.distances.into_iter().zip(response.durations).zip(stations_orig)
        .map(|((a, b), c)|{
            PathResult{
                duration: b[0], distance: a[0], station: c 

            }
        }).collect();
    result.sort_by(|x, y |{
        x.duration.partial_cmp(&y.duration).unwrap()
    });
    Ok(content)
}

pub fn init_osrm(cfg: &mut web::ServiceConfig) {
    cfg.service(get_route);
    cfg.service(fuck_all_stations);

}
