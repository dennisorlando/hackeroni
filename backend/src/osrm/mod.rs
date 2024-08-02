use actix_web::{post, web::{self, Data, Form}, Responder};
use log::error;
use request::OSRMRequest;
use thiserror::Error;

use crate::{config::AppConfig, odh::get_near_stations};

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
#[post("/stocazzo")]
pub async fn get_route(req: Form<PathRequest>, config: Data<AppConfig>) -> actix_web::Result<impl Responder> {
    let destination = (req.destination_long, req.destination_lat);
    let stations = get_near_stations(destination, config.max_walking_meters).await?;
    println!("{:?}", stations);
    let query = OSRMRequest::new(stations, destination, req.preferences.max_walking_time.unwrap_or(10)).build()?;


    let url = config.osrm_url.clone() +"/"+ &query;
    let content = reqwest::get(url).await
        .map_err(|x| OSRMError::from(x))?
        .text()
        .await
        .map_err(|x| OSRMError::from(x))?;
    Ok(content)
}

pub fn init_osrm(cfg: &mut web::ServiceConfig) {
    cfg.service(get_route);
    //cfg.service(sd_routes);
}
