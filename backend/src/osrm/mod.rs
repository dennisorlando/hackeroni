use actix_web::{get, post, web::{self, Data, Form, Json}, Responder};
use log::error;
use request::{OSRMRequest, OSRMResponse};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::{config::AppConfig, db::{stations::{get_all_stations, StationInfo}, DbPool}, odh::{get_near_stations, EChargingStation, ODHBuilder}};
use std::{cmp::Ordering, f64};

use self::{hardcoded::ResponseStatus, request::PathRequest};

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

#[derive(Serialize, Debug, Clone)]
struct PathResult{
    pub duration: f64,
    pub distance: f64,
    pub station: StationInfo,
}


#[post("/get_routes")]
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
    let response: OSRMResponse = serde_json::from_str(&content)?;
    
    let mut result: Vec<PathResult> = response.durations.into_iter().zip(response.distances).zip(stations_orig)
        .map(|((durations, distances), station)|{
            PathResult{ duration: durations[0], distance: distances[0], station }
        }).collect();
    result.sort_by(|x, y |{
        x.duration.partial_cmp(&y.duration).unwrap()
    });

    let result = result.iter()
        .filter(|&x| x.duration < req.preferences.max_walking_time.unwrap_or(600) as f64)
        .take(4).cloned().collect::<Vec<_>>();

    let result = get_routes(&result, (req.source_long, req.source_lat), destination, config).await?;

    Ok(Json(result))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteResult {
    pub walking_duration: f64,
    pub driving_duration: f64,
    pub walking_nodes: Vec<(f64, f64)>,
    pub driving_nodes: Vec<(f64, f64)>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OSRMRouteResult {
    code: ResponseStatus,
    routes: Option<Vec<OSRMRoute>>,
    waypoints: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSRMRoute {
    geometry: Geometry,
    legs: serde_json::Value,
    weight_name: String,
    weight: f64,
    duration: f64,
    distance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Geometry {
    pub coordinates: Vec<(f64, f64)>,

    #[serde(rename = "type")]
    pub type_geometry: String,
}

async fn get_routes(paths: &[PathResult], source: (f64, f64), destination: (f64, f64), config: Data<AppConfig>) -> actix_web::Result<Vec<RouteResult>> {
    let mut results = vec![];
    let driving_uri = format!("{}/route/v1/driving/{},{};", config.osrm_url, source.0, source.1);

    for path in paths {
        let full_url = driving_uri.clone() + &format!("{},{}", path.station.coordinate_lat, path.station.coordinate_long) + "?overview=full&geometries=geojson";
        let content = reqwest::get(full_url).await.map_err(|x| OSRMError::from(x))?
        .text()
        .await
        .map_err(|x| OSRMError::from(x))?;
        
        let osrm_driving_route_result: OSRMRouteResult = serde_json::from_str(&content)?;

        if osrm_driving_route_result.routes.is_none() {
            continue;
        }
        let driving_nodes = osrm_driving_route_result.routes.as_ref().unwrap()[0].geometry.coordinates.clone();
        let driving_duration = osrm_driving_route_result.routes.as_ref().unwrap()[0].duration;
        

        let walking_uri = format!("{}/route/v1/foot/{},{};", config.osrm_url, path.station.coordinate_lat, path.station.coordinate_long);
        let full_url = walking_uri + &format!("{},{}", destination.0, destination.1) + "?overview=full&geometries=geojson";
        let content = reqwest::get(full_url).await.map_err(|x| OSRMError::from(x))?
        .text()
        .await
        .map_err(|x| OSRMError::from(x))?;
        
        let osrm_foot_route_result: OSRMRouteResult = serde_json::from_str(&content)?;
        if osrm_foot_route_result.routes.is_none() {
            continue;
        }
        let walking_nodes = osrm_foot_route_result.routes.as_ref().unwrap()[0].geometry.coordinates.clone();
        let walking_duration = osrm_foot_route_result.routes.as_ref().unwrap()[0].duration;
        results.push(RouteResult {
            walking_duration,
            driving_duration,
            walking_nodes,
            driving_nodes
        });
    }

    Ok(results)
}


pub fn init_osrm(cfg: &mut web::ServiceConfig) {
    cfg.service(get_route);
    cfg.service(fuck_all_stations);

}
