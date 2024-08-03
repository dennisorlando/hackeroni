use serde::{Deserialize, Serialize};

use crate::osrm::hardcoded::ResponseStatus;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RouteResult {
    pub walking_duration: f64,
    pub driving_duration: f64,
    pub final_charge: f64,
    // pub cost: f64,
    pub walking_nodes: Vec<(f64, f64)>,
    pub driving_nodes: Vec<(f64, f64)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OSRMRouteResult {
    pub code: ResponseStatus,
    pub routes: Option<Vec<OSRMRoute>>,
    pub waypoints: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSRMRoute {
    pub geometry: Geometry,
    pub legs: serde_json::Value,
    pub weight_name: String,
    pub weight: f64,
    pub duration: f64,
    pub distance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Geometry {
    pub coordinates: Vec<(f64, f64)>,

    #[serde(rename = "type")]
    pub type_geometry: String,
}
