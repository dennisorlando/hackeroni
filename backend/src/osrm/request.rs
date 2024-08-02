use std::error::Error;

use serde::{Deserialize, Serialize};

use super::hardcoded::ResponseStatus;

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

    pub fn build(&self) -> Result<String, Box<dyn Error>> {    
        if self.stations.is_empty() {
            return Err("No stations provided".into());
        }

        let mut req = String::from("table/v1/foot/");

        for (lat, lon) in self.stations.iter() {
            req.push_str(&format!("{},{};", lat, lon));
        }
        req.push_str(&format!("{},{}", self.destination.0, self.destination.1));

        req.push_str("?sources=");
        for i in 0..self.stations.len() {
            req.push_str(&format!("{}", i));
            if i < self.stations.len() - 1 {
                req.push(';');
            }
        }

        req.push_str(&format!("&destinations={}", self.stations.len()));

        Ok(req)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OSRMResponse {
    pub sources: Vec<serde_json::Value>,
    pub durations: Vec<Vec<f64>>,
    pub destinations: Vec<serde_json::Value>,
    pub code: ResponseStatus,  
    pub distances: Vec<Vec<f64>>,
}

#[test]
fn osrm_response() {
    let res_str = r#"{
              "sources": [
                {
                  "location": [
                    13.3888,
                    52.517033
                  ],
                  "hint": "PAMAgEVJAoAUAAAAIAAAAAcAAAAAAAAArss0Qa7LNEHiVIRA4lSEQAoAAAAQAAAABAAAAAAAAADMAAAAAEzMAKlYIQM8TMwArVghAwEA3wps52D3",
                  "name": "Friedrichstraße"
                },
                {
                  "location": [
                    13.397631,
                    52.529432
                  ],
                  "hint": "WIQBgL6mAoAEAAAABgAAAAAAAAA7AAAAhU6PQHvHj0IAAAAAQbyYQgQAAAAGAAAAAAAAADsAAADMAAAAf27MABiJIQOCbswA_4ghAwAAXwVs52D3",
                  "name": "Torstraße"
                },
                {
                  "location": [
                    13.428554,
                    52.523239
                  ],
                  "hint": "7UcAgP___38fAAAAUQAAACYAAABTAAAAhSQKQrXq5kKRbiZCWJo_Qx8AAABRAAAAJgAAAFMAAADMAAAASufMAOdwIQNL58wA03AhAwMAvxBs52D3",
                  "name": "Platz der Vereinten Nationen"
                }
              ],
              "durations": [
                [
                  0,
                  192.6,
                  382.8
                ],
                [
                  199,
                  0,
                  283.9
                ],
                [
                  344.7,
                  222.3,
                  0
                ]
              ],
              "destinations": [
                {
                  "location": [
                    13.3888,
                    52.517033
                  ],
                  "hint": "PAMAgEVJAoAUAAAAIAAAAAcAAAAAAAAArss0Qa7LNEHiVIRA4lSEQAoAAAAQAAAABAAAAAAAAADMAAAAAEzMAKlYIQM8TMwArVghAwEA3wps52D3",
                  "name": "Friedrichstraße"
                },
                {
                  "location": [
                    13.397631,
                    52.529432
                  ],
                  "hint": "WIQBgL6mAoAEAAAABgAAAAAAAAA7AAAAhU6PQHvHj0IAAAAAQbyYQgQAAAAGAAAAAAAAADsAAADMAAAAf27MABiJIQOCbswA_4ghAwAAXwVs52D3",
                  "name": "Torstraße"
                },
                {
                  "location": [
                    13.428554,
                    52.523239
                  ],
                  "hint": "7UcAgP___38fAAAAUQAAACYAAABTAAAAhSQKQrXq5kKRbiZCWJo_Qx8AAABRAAAAJgAAAFMAAADMAAAASufMAOdwIQNL58wA03AhAwMAvxBs52D3",
                  "name": "Platz der Vereinten Nationen"
                }
              ],
              "code": "Ok",
              "distances": [
                [
                  0,
                  1886.89,
                  3791.3
                ],
                [
                  1824,
                  0,
                  2838.09
                ],
                [
                  3275.36,
                  2361.73,
                  0
                ]
              ],
              "fallback_speed_cells": [
                [ 0, 1 ],
                [ 1, 0 ]
              ]
            }"#;

    let res: OSRMResponse = serde_json::from_str(res_str).unwrap();

    for s in res.sources.iter() {
        let s = s.as_object().unwrap()["location"].as_array().unwrap();
        println!("{:?}", s);
    }
}

