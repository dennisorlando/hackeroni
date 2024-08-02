use actix_web::{get, post, web::{self, Data, Form}, Responder};
use log::error;
use request::OSRMRequest;
use serde::Serialize;
use thiserror::Error;

use crate::{config::AppConfig, odh::{get_near_stations, EChargingStation, ODHBuilder}};

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
#[derive(Serialize, Debug)]
pub struct StationInfo{
    pub coordinate: (f64, f64),
    pub id: String,
    pub name: String,
}

#[get("/get_all_stations")]
pub async fn get_all_stations(config: Data<AppConfig>) -> actix_web::Result<impl Responder> {
    let result: Vec<EChargingStation> = ODHBuilder::default().run().await?;
    let stations: Vec<StationInfo> = result.into_iter().map(|x|
        StationInfo{
        coordinate: (x.scoordinate.x, x.scoordinate.y), id: x.scode, name: x.sname }

    ).collect();
    let stations = serde_json::to_string(&stations)?;
    Ok(stations)
}



#[post("/stocazzo")]
pub async fn get_route(req: Form<PathRequest>, config: Data<AppConfig>) -> actix_web::Result<impl Responder> {
    let destination = (req.destination_long, req.destination_lat);
    let stations = get_near_stations(destination, config.max_walking_meters).await?;
    println!("{:?}", stations);
    let stations = stations.into_iter().map(|x| x.coordinate).collect();
    let query = OSRMRequest::new(stations, destination, req.preferences.max_walking_time.unwrap_or(10)).build()?;


    let url = config.osrm_url.clone() +"/"+ &query;
    println!("{}", url);
    let content = reqwest::get(url).await
        .map_err(|x| OSRMError::from(x))?
        .text()
        .await
        .map_err(|x| OSRMError::from(x))?;
    println!("{}", content);
    Ok(content)
}

pub fn init_osrm(cfg: &mut web::ServiceConfig) {
    cfg.service(get_route);
    cfg.service(get_all_stations);

}
