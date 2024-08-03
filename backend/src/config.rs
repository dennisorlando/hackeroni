use std::env;

use actix_web::cookie::Key;
use base64::{prelude::BASE64_STANDARD, Engine};
#[derive(Clone)]
pub struct AppConfig {
    pub cookie_secret: Key,
    pub database_url: String,
    pub pepper: String,
    pub max_walking_meters: f64,
    pub osrm_url: String,
    pub odh_hub_url: String,
}
fn mandatory<T: Sized, F: Fn(String) -> T>(name: &str, mapf: F) -> T {
    let tmp = env::var(name)
        .unwrap_or_else(|e| panic!("Got error\"{e}\" while reading mandatory config \"{name}\""));
    mapf(tmp)
}
pub fn load_config() -> AppConfig {
    AppConfig {
        cookie_secret: mandatory("COOKIE_SECRET", |key| {
            let key = BASE64_STANDARD.decode(key.as_bytes()).unwrap();
            let key = Key::try_from(key.as_slice()).unwrap();
            key
        }),
        pepper: mandatory("PEPPER", |x| x),
        database_url: mandatory("DATABASE_URL", |x| x),
        osrm_url: mandatory("OSRM_URL", |x| x),
        max_walking_meters: mandatory("MAX_WALKING_METERS", |x| x.parse::<f64>().unwrap()),
        odh_hub_url: mandatory("ODH_HUB_URL", |x| x),
    }
}
