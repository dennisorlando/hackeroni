pub mod osmr_route;

use actix_web::web::{Data, Form};
use serde::{Deserialize, Serialize};

use crate::config::AppConfig;

use self::osmr_route::{OSRMRouteResult, RouteResult};

use super::{hardcoded::ResponseStatus, request::PathRequest, OSRMError, PathResult};


pub async fn get_routes(
    paths: &[PathResult],
    source: (f64, f64),
    destination: (f64, f64),
    config: Data<AppConfig>,
) -> actix_web::Result<Vec<RouteResult>> {
    let mut results = vec![];
    let driving_uri = format!(
        "{}/route/v1/driving/{},{};",
        config.osrm_url, source.0, source.1
    );

    for path in paths {
        let full_url = driving_uri.clone()
            + &format!(
                "{},{}",
                path.station.coordinate_lat, path.station.coordinate_long
            )
            + "?overview=full&geometries=geojson";
        let content = reqwest::get(full_url)
            .await
            .map_err(OSRMError::from)?
            .text()
            .await
            .map_err(OSRMError::from)?;

        let osrm_driving_route_result: OSRMRouteResult = serde_json::from_str(&content)?;

        if osrm_driving_route_result.routes.is_none() {
            continue;
        }
        let driving_nodes = osrm_driving_route_result.routes.as_ref().unwrap()[0]
            .geometry
            .coordinates
            .clone();
        let driving_duration = osrm_driving_route_result.routes.as_ref().unwrap()[0].duration;

        let walking_uri = format!(
            "{}/route/v1/foot/{},{};",
            config.osrm_url, path.station.coordinate_lat, path.station.coordinate_long
        );
        let full_url = walking_uri
            + &format!("{},{}", destination.0, destination.1)
            + "?overview=full&geometries=geojson";
        let content = reqwest::get(full_url)
            .await
            .map_err(OSRMError::from)?
            .text()
            .await
            .map_err(OSRMError::from)?;

        let osrm_foot_route_result: OSRMRouteResult = serde_json::from_str(&content)?;
        if osrm_foot_route_result.routes.is_none() {
            continue;
        }
        let walking_nodes = osrm_foot_route_result.routes.as_ref().unwrap()[0]
            .geometry
            .coordinates
            .clone();
        let walking_duration = osrm_foot_route_result.routes.as_ref().unwrap()[0].duration;
        results.push(RouteResult {
            walking_duration,
            driving_duration,
            walking_nodes,
            driving_nodes,
        });
    }

    Ok(results)
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Routes {
    pub least_walking_time: RouteResult,
    pub least_driving_time: RouteResult,
    pub balanced: RouteResult,
    pub least_cost: RouteResult,
}

pub struct RoutesBuilder {
    paths: Vec<PathResult>,
    source: (f64, f64),
    destination: (f64, f64),
    req: Form<PathRequest>,
    config: Data<AppConfig>,
    walking_uri: String,
    driving_uri: String,
    query_uri: String,
}

impl RoutesBuilder {
    pub fn new(
        paths: Vec<PathResult>,
        source: (f64, f64),
        destination: (f64, f64),
        req: Form<PathRequest>,
        config: Data<AppConfig>,
    ) -> Self {
        let walking_uri = format!("{}/route/v1/foot/", config.osrm_url);
        let driving_uri = format!("{}/route/v1/driving/", config.osrm_url);
        let query_uri = "?overview=full&geometries=geojson".to_string();
        RoutesBuilder {
            paths,
            source,
            destination,
            req,
            config,
            walking_uri,
            driving_uri,
            query_uri,
        }
    }

    pub async fn calculate_routes(
        paths: &[PathResult],
        source: (f64, f64),
        destination: (f64, f64),
        req: Form<PathRequest>,
        config: Data<AppConfig>,
    ) -> actix_web::Result<Routes> {
        let mut routes = Routes::default();


        Ok(routes)
    }

    pub async fn lwt(&self) -> Option<RouteResult> {
        let min_wt_path_res = self.paths.iter().min_by(|a, b| {
            a.duration
                .partial_cmp(&b.duration)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if min_wt_path_res.is_none() {
            return None;
        }

        let min_wt_path = min_wt_path_res.unwrap();


        todo!()
    }

    pub async fn ldt(&self) -> RouteResult {
        todo!()
    }

    pub async fn balanced(&self) -> RouteResult {
        todo!()
    }

    pub async fn least_cost(&self) -> RouteResult {
        todo!()
    }
}
