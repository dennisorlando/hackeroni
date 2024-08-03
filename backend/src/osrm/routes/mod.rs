pub mod osmr_route;

use actix_web::{error::ErrorInternalServerError, web::{Data, Form}};
use diesel::PgConnection;
use serde::{Deserialize, Serialize};

use crate::{config::AppConfig, db::{stations::PlugsInfo, DbConnection, DbPool}};

use self::osmr_route::{OSRMRouteResult, RouteResult};

use super::{request::PathRequest, OSRMError, PathResult};

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
        req: Form<PathRequest>,
        osrm_url: String,
    ) -> Self {
        let walking_uri = format!("{}/route/v1/foot", osrm_url);
        let driving_uri = format!("{}/route/v1/driving", osrm_url);
        let query_uri = "?overview=full&geometries=geojson".to_string();
        RoutesBuilder {
            paths,
            req,
            walking_uri,
            driving_uri,
            query_uri,
        }
    }

    pub async fn calculate_routes(&self, pool: Data<DbPool>) -> actix_web::Result<Routes> {
        let mut conn = pool
            .get()
            .map_err(|_| ErrorInternalServerError("can't get pool"))?;

        let mut results: Vec<RouteResult> = vec![];
        for path in self.paths.iter() {
            let route = self.get_routes_for_path(path, &mut conn).await?;
            results.push(route);
        }

        let max_walking_time = self.req.max_walking_time.unwrap_or(600.0);
        let results = results
            .iter()
            .filter(|x| x.walking_duration <= max_walking_time)
            .collect::<Vec<&RouteResult>>();

        let mut scores: Vec<(f64, f64, &RouteResult)> = vec![];
        for result in results.iter() {
            let walking_score = 1.0 - result.walking_duration / max_walking_time;
            let chargin_score = result.final_charge
                / self.req.charge_requested.unwrap_or(90.0);

            scores.push((walking_score, chargin_score, result));
        }

        let mut routes = Routes::default();
        routes.least_walking_time = scores
            .iter()
            .max_by(|x, y| x.0.partial_cmp(&y.0).unwrap())
            .map(|x| x.2.clone());

        routes.least_driving_time = scores.iter().min_by(|x, y| x.2.driving_duration.partial_cmp(&y.2.driving_duration).unwrap()).map(|x| x.2.clone());

        routes.balanced = scores
            .iter()
            .max_by(|x, y| {
                let x_score = x.0 * 0.5 + x.1 * 0.5;
                let y_score = y.0 * 0.5 + y.1 * 0.5;
                x_score.partial_cmp(&y_score).unwrap()
            })
            .map(|x| x.2.clone());

        Ok(routes)
    }

    async fn get_routes_for_path(&self, path: &PathResult, conn: &mut DbConnection) -> actix_web::Result<RouteResult> {
        let plugs: Vec<PlugsInfo> = crate::db::stations::get_plugs(conn, &path.station).map_err(|_| actix_web::error::ErrorInternalServerError("plugs not found"))?;

        let driving_uri = format!(
            "{}/{},{};",
            self.driving_uri, self.req.source_lat, self.req.source_long
        );
        println!("driving_uri {driving_uri}");

        let full_url = driving_uri.clone()
            + &format!(
                "{},{}",
                path.station.coordinate_lat, path.station.coordinate_long
            )
            + self.query_uri.as_str();
        println!("full_url {full_url}");
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
            + &format!("{},{}", self.req.destination_long, self.req.destination_lat)
            + self.query_uri.as_str();
        println!("full_url2 {full_url}");
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


        let max_power = plugs.iter().filter_map(|x| x.max_power).reduce(f64::max).unwrap_or(22.0);
        let capacity = self.req.0.capacity;
        let current_charge = self.req.0.charge_left.unwrap_or(0.0);
        let full_charge_time = capacity / max_power * 60.0;
        let charge_time = 2.0 * walking_duration + self.req.0.duration as f64;

        let cost = capacity * (charge_time / full_charge_time) * 0.82;
        let final_charge = current_charge + (charge_time / full_charge_time * 100.0);
        //println!("{} {} {} {} {} {} {}", max_power, capacity, current_charge, full_charge_time, charge_time, final_charge, walking_duration);

        Ok(RouteResult {
            walking_duration,
            driving_duration,
            final_charge,
            cost,
            walking_nodes,
            driving_nodes,
        })
    }
}
