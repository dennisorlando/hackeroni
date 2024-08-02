use serde::Deserialize;

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub enum Profile {
    Car,
    Bike,
    Foot
}

#[derive(Debug, Deserialize)]
pub struct Request {
    service: Service,
    coordinates: Vec<(f64, f64)>,
    profile: Profile,
}

impl Request {
    pub fn new(service: Service, coordinates: Vec<(f64, f64)>, profile: Profile) -> Self {
        Request {
            service,
            coordinates,
            profile,
        }
    }
    pub fn to_osrm_string(&self) -> String {
        let mut request = String::new();
        match &self.service {
            Service::Route { .. } => request.push_str("route/v1/"),
            Service::Table { .. } => request.push_str("table/v1/"),
        }

        match &self.profile {
            Profile::Car => request.push_str("car/"),
            Profile::Bike => request.push_str("bike/"),
            Profile::Foot => request.push_str("foot/"),
        }

        for (i, (lat, lon)) in self.coordinates.iter().enumerate() {
            request.push_str(&format!("{},{}", lon, lat));
            if i < self.coordinates.len() - 1 {
                request.push(';');
            }
        }

        request.push('?');

        let mut query_string = String::new();
        match &self.service {
            Service::Route { alternatives, steps } => {
                if let Some(alternatives) = alternatives {
                    query_string.push_str(&format!("&alternatives={}", alternatives));
                }
                if *steps {
                    query_string.push_str("&steps=true");
                }
            }
            Service::Table { sources, destinations, fallback_speed } => {
                if let Some(sources) = sources {
                    query_string.push_str("&sources=");
                    for source in sources {
                        query_string.push_str(&format!("{};", source));
                    }
                }

                if let Some(destinations) = destinations {
                    query_string.push_str("&destinations=");
                    for destination in destinations {
                        query_string.push_str(&format!("{};", destination));
                    }
                }

                if let Some(fallback_speed) = fallback_speed {
                    query_string.push_str(&format!("&fallback_speed={}", fallback_speed));
                }
            }
        }
        query_string.remove(0);
    
        request.push_str(&query_string);
        request
    }
}
