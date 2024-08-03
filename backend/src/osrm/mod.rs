use crate::{
    config::AppConfig,
    db::{
        stations::{get_all_stations, StationInfo},
        DbPool,
    },
    odh::get_near_stations,
};
use actix_web::{
    get, post,
    web::{self, Data, Form, Json},
    Responder,
};
use log::error;
use request::{OSRMRequest, OSRMResponse};
use serde::{Deserialize, Serialize};
use std::f64;
use thiserror::Error;

use self::{hardcoded::ResponseStatus, request::PathRequest};

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
) -> actix_web::Result<impl Responder> {
    let destination = (req.destination_long, req.destination_lat);
    let stations_orig =
        get_near_stations(&config.odh_hub_url, destination, config.max_walking_meters).await?;
    let stations = stations_orig
        .clone()
        .into_iter()
        .map(|x| (x.coordinate_lat, x.coordinate_long))
        .collect();
    let query = OSRMRequest::new(
        stations,
        destination,
        req.preferences.max_walking_time.unwrap_or(10),
    )
    .build()?;

    let url = config.osrm_url.clone() + "/" + &query;
    let content = reqwest::get(url)
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
    result.sort_by(|x, y| x.duration.partial_cmp(&y.duration).unwrap());

    let result = result
        .iter()
        .filter(|&x| x.duration < req.preferences.max_walking_time.unwrap_or(600) as f64)
        .take(4)
        .cloned()
        .collect::<Vec<_>>();

    let result = routes::get_routes(
        &result,
        (req.source_long, req.source_lat),
        destination,
        config,
    )
    .await?;

    Ok(Json(result))
}

pub fn init_osrm(cfg: &mut web::ServiceConfig) {
    cfg.service(get_route);
    cfg.service(fuck_all_stations);
}
