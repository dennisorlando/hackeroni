use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug)]
pub enum ResponseStatus {
    ///Request could be processed as expected.
    Ok,
    ///URL string is invalid.
    InvalidUrl,
    ///Service name is invalid.
    InvalidService,
    ///Version is not found.
    InvalidVersion,
    ///Options are invalid.
    InvalidOptions,
    ///The query string is synctactically malformed.
    InvalidQuery,
    ///The successfully parsed query parameters are invalid.
    InvalidValue,
    ///One of the supplied input coordinates could not snap to street segment.
    NoSegment,
    ///The request size violates one of the service specific request size restrictions.
    TooBig,
}
#[derive(Deserialize, Serialize, Debug)]
struct Geometry {
    #[serde(rename = "type")]
    type_geometry: String,
    coordinates: Vec<(f32, f32)>,
}
#[derive(Deserialize, Serialize, Debug)]
struct Route {
    distance: f32,
    duration: f32,
    geometry: Geometry,
    weight: f32,
    weight_name: String,
    legs: Value,
}

#[derive(Deserialize, Serialize, Debug)]
struct Response {
    code: ResponseStatus,
    routes: Vec<Route>,
}

//https://router.project-osrm.org/route/v1/driving/13.414167165756226,52.52167215019524;13.4197763,52.5003103?geometries=geojson&alternatives=true&steps=true&generate_hints=false
pub async fn get_info() -> Result<(), Box<dyn Error>> {
    let query = "https://router.project-osrm.org/route/v1/driving/13.414167165756226,52.52167215019524;13.4197763,52.5003103?geometries=geojson&alternatives=true&steps=false&generate_hints=false";
    let content = reqwest::get(query).await?.text().await?;
    let t: Response = serde_json::from_str(&content).unwrap();
    println!("{:?}", t);
    todo!()
}
