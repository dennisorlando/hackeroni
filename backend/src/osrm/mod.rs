use crate::{
    config::AppConfig,
    db::{
        stations::{get_all_stations, StationInfo},
        DbPool,
    },
    odh::{get_near_stations, ODHError},
};
use actix_web::{
    get, post,
    web::{self, Data, Form, Json},
    Responder,
};
use log::error;
use request::{OSRMRequest, OSRMResponse};
use serde::Serialize;
use std::f64;
use thiserror::Error;

use self::request::PathRequest;

pub mod hardcoded;
pub mod request;
pub mod routes;

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
pub async fn fuck_all_stations(pool: Data<DbPool>) -> actix_web::Result<impl Responder> {
    let stations = web::block(move || {
        let mut conn = pool.get().unwrap();
        get_all_stations(&mut conn)
    })
    .await?
    .unwrap();
    let stations = serde_json::to_string(&stations)?;
    Ok(stations)
}

#[derive(Serialize, Debug, Clone)]
pub struct PathResult {
    pub duration: f64,
    pub distance: f64,
    pub station: StationInfo,
}

#[post("/get_routes")]
pub async fn get_route(
    req: Form<PathRequest>,
    config: Data<AppConfig>,
    pool: Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let destination = (req.destination_long, req.destination_lat);
    let config2 = config.clone();
    let pool_clone = pool.clone();
    let stations_orig = web::block(move || {
        let mut conn = pool_clone.get()?;
        let stations_orig = get_near_stations(&mut conn, destination, config2.max_walking_meters)?;
        Ok::<Vec<StationInfo>, ODHError>(stations_orig)
    })
    .await??;

    let stations = stations_orig
        .clone()
        .into_iter()
        .map(|x| (x.coordinate_lat, x.coordinate_long))
        .collect();
    let query = OSRMRequest::new(
        stations,
        destination,
        req.max_walking_time.unwrap_or(10.0),
    )
    .build()?;

    let url = config.osrm_url.clone() + "/" + &query;
    let content =  reqwest::ClientBuilder::new().use_rustls_tls().danger_accept_invalid_certs(true).build().unwrap().get(url).send()
        .await
        .map_err(OSRMError::from)?
        .text()
        .await
        .map_err(OSRMError::from)?;
    let response: OSRMResponse = serde_json::from_str(&content)?;

    let mut result: Vec<PathResult> = response
        .durations
        .into_iter()
        .zip(response.distances)
        .zip(stations_orig)
        .map(|((durations, distances), station)| PathResult {
            duration: durations[0],
            distance: distances[0],
            station,
        })
        .collect();

    let result_builder = routes::RoutesBuilder::new(result, req, config.osrm_url.clone());
    let result = result_builder.calculate_routes(pool.clone()).await?;

    Ok(Json(result))
}

pub fn init_osrm(cfg: &mut web::ServiceConfig) {
    cfg.service(get_route);
    cfg.service(fuck_all_stations);
}
