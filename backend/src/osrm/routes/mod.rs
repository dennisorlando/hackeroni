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
            final_charge: -1.0,
        });
    }

    Ok(results)
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Routes {
    pub least_walking_time: Option<RouteResult>,
    pub least_driving_time: Option<RouteResult>,
    pub balanced: Option<RouteResult>,
    pub least_cost: Option<RouteResult>,
}

pub struct RoutesBuilder {
    paths: Vec<PathResult>,
    req: Form<PathRequest>,
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
        osrm_url: String,
    ) -> Self {
        let walking_uri = format!("{}/route/v1/foot/", osrm_url);
        let driving_uri = format!("{}/route/v1/driving/", osrm_url);
        let query_uri = "?overview=full&geometries=geojson".to_string();
        RoutesBuilder {
            paths,
            req,
            walking_uri,
            driving_uri,
            query_uri,
        }
    }

    pub async fn calculate_routes(&self) -> actix_web::Result<Routes> {
        let mut results: Vec<RouteResult> = vec![];
        for path in self.paths.iter() {
            let route = self.get_routes_for_path(&path).await?;
            results.push(route);
        }

        let max_walking_time = self.req.max_walking_time.unwrap_or(600.0) as f64;
        let results = results
            .iter()
            .filter(|x| x.walking_duration < max_walking_time)
            .collect::<Vec<&RouteResult>>();

        let mut scores = vec![];

        for result in results.iter() {
            let walking_score = 1.0 - result.walking_duration as f64 / max_walking_time;
            let chargin_score = result.final_charge as f64
                / self.req.charge_requested.unwrap_or(90.0) as f64;

            let total_score = walking_score;
            scores.push(total_score);
        }

        let mut routes = Routes::default();

        Ok(routes)
    }

    async fn get_routes_for_path(&self, path: &PathResult) -> actix_web::Result<RouteResult> {
        let driving_uri = format!(
            "{}/{},{};",
            self.driving_uri, self.req.source_lat, self.req.source_long
        );

        let full_url = driving_uri.clone()
            + &format!(
                "{},{}",
                path.station.coordinate_lat, path.station.coordinate_long
            )
            + self.query_uri.as_str();
        let content = reqwest::get(full_url)
            .await
            .map_err(OSRMError::from)?
            .text()
            .await
            .map_err(OSRMError::from)?;

        let osrm_driving_route_result: OSRMRouteResult = serde_json::from_str(&content)?;

        if osrm_driving_route_result.routes.is_none() {
            return Ok(RouteResult::default());
        }
        let driving_nodes = osrm_driving_route_result.routes.as_ref().unwrap()[0]
            .geometry
            .coordinates
            .clone();
        let driving_duration = osrm_driving_route_result.routes.as_ref().unwrap()[0].duration;

        let walking_uri = format!(
            "{}/{},{};",
            self.walking_uri, path.station.coordinate_lat, path.station.coordinate_long
        );
        let full_url = walking_uri
            + &format!("{},{}", self.req.destination_lat, self.req.destination_long)
            + self.query_uri.as_str();
        let content = reqwest::get(full_url)
            .await
            .map_err(OSRMError::from)?
            .text()
            .await
            .map_err(OSRMError::from)?;

        let osrm_foot_route_result: OSRMRouteResult = serde_json::from_str(&content)?;
        if osrm_foot_route_result.routes.is_none() {
            return Ok(RouteResult::default());
        }
        let walking_nodes = osrm_foot_route_result.routes.as_ref().unwrap()[0]
            .geometry
            .coordinates
            .clone();
        let walking_duration = osrm_foot_route_result.routes.as_ref().unwrap()[0].duration;

        // TODO: Calculate final charge
        let final_charge = -1.0;

        Ok(RouteResult {
            walking_duration,
            driving_duration,
            final_charge,
            walking_nodes,
            driving_nodes,
        })
    }
}
