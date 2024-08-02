use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[allow(unused)]
pub enum Service {
    /// Finds the fastest route between coordinates in the supplied order.
    Route {
        alternatives: Option<u32>,
        steps: bool,
    },

    /// Computes the duration of the fastest route between all pairs of supplied coordinates.
    /// Returns the durations or distances or both between the coordinate pairs. 
    /// Note that the distances are not the shortest distance between two coordinates, but rather the distances of the fastest routes.
    /// Duration is in seconds and distances is in meters. 
    Table {
        sources: Option<Vec<u32>>, 
        destinations: Option<Vec<u32>>,
        fallback_speed: Option<f64>,
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Profile {
    Car,
    Bike,
    Foot
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Preferences {
    pub charge_left: Option<u32>,
    pub charge_requested: Option<u32>,
    pub max_walking_time: Option<u32>,
    // other future preferences
}

impl Preferences {
    pub fn new(charge_left: Option<u32>, charge_requested: Option<u32>, max_walking_distance: Option<u32>) -> Self {
        Preferences {
            charge_left,
            charge_requested,
            max_walking_time: max_walking_distance,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PathRequest {
    pub source_lat: f64,
    pub source_long: f64,
    pub destination_lat: f64,
    pub destination_long: f64,
    pub duration: u32,
    #[serde(flatten)]
    pub preferences: Preferences,
}

impl PathRequest {
    pub fn new(source: (f64, f64), destination: (f64, f64), duration: u32, preferences: Preferences) -> Self {
        PathRequest {
            source_lat: source.0,
            source_long: source.1,
            destination_lat: destination.0,
            destination_long: destination.1,
            duration,
            preferences,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OSRMRequest {
    pub stations: Vec<(f64, f64)>,
    pub destination: (f64, f64),
    pub max_walking_time: u32,
}


impl OSRMRequest {
    pub fn new(stations: Vec<(f64, f64)>, destination: (f64, f64), max_time: u32) -> Self {
        OSRMRequest {
            stations,
            destination,
            max_walking_time: max_time,
        }
    }
}

