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
    pub max_walking_distance: Option<u32>,
    // other future preferences
}

impl Preferences {
    pub fn new(charge_left: Option<u32>, charge_requested: Option<u32>, max_walking_distance: Option<u32>) -> Self {
        Preferences {
            charge_left,
            charge_requested,
            max_walking_distance,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PathRequest {
    source: (f64, f64),
    destination: (f64, f64),
    preferences: Preferences,
}

impl PathRequest {
    pub fn new(source: (f64, f64), destination: (f64, f64), preferences: Preferences) -> Self {
        PathRequest {
            source,
            destination,
            preferences,
        }
    }

    pub fn to_osrm_string(&self) -> String {
        let mut request = String::new();
        // match &self.service {
        //     Service::Route { .. } => request.push_str("route/v1/"),
        //     Service::Table { .. } => request.push_str("table/v1/"),
        // }

        request.push_str("driving/");

        for (i, (lat, lon)) in self.coordinates.iter().enumerate() {
            request.push_str(&format!("{},{}", lon, lat));
            if i < self.coordinates.len() - 1 {
                request.push(';');
            }
        }

        request.push('?');

        let query_string = String::new();
        // match &self.service {
        //     Service::Route { alternatives, steps } => {
        //         if let Some(alternatives) = alternatives {
        //             query_string.push_str(&format!("&alternatives={}", alternatives));
        //         }
        //         if *steps {
        //             query_string.push_str("&steps=true");
        //         }
        //     }
        //     Service::Table { sources, destinations, fallback_speed } => {
        //         if let Some(sources) = sources {
        //             query_string.push_str("&sources=");
        //             for source in sources {
        //                 query_string.push_str(&format!("{};", source));
        //             }
        //         }
        //
        //         if let Some(destinations) = destinations {
        //             query_string.push_str("&destinations=");
        //             for destination in destinations {
        //                 query_string.push_str(&format!("{};", destination));
        //             }
        //         }
        //
        //         if let Some(fallback_speed) = fallback_speed {
        //             query_string.push_str(&format!("&fallback_speed={}", fallback_speed));
        //         }
        //     }
        // }
        // query_string.remove(0);
    
        request.push_str(&query_string);
        request
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OSRMRequest {
    pub stations: Vec<(f64, f64)>,
    pub destination: Vec<(f64, f64)>,
    pub max_time: u32,
}


impl OSRMRequest {
    pub fn new(stations: Vec<(f64, f64)>, destination: Vec<(f64, f64)>, max_time: u32) -> Self {
        OSRMRequest {
            stations,
            destination,
            max_time,
        }
    }
}

