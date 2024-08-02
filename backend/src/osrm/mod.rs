use actix_web::{post, web::{self, Data, Form}, Responder};

use crate::{config::AppConfig, odh::get_near_stations};

use self::request::PathRequest;

pub mod hardcoded;
pub mod request;

struct StationInfo{
    coordinate: (f32, f32),
    id: String,
    
}

#[post("/stocazzo")]
pub async fn get_route(req: Form<PathRequest>, config: Data<AppConfig>) -> actix_web::Result<impl Responder> {
    let destination = (req.destination_long, req.destination_lat);
    let stations = get_near_stations(destination, config.max_walking_meters).await.unwrap();
    println!("{:?}", stations);
    let r = OSRMRequest::new(stations, destination, req.preferences.max_walking_time.unwrap_or(10));
    let r = sd_routes(r).await;

    //todo take from config
    let url = config.osrm_url.clone() +"/"+ &r;
    println!("{}", url);
    let content = reqwest::get(url).await.unwrap().text().await.unwrap();
    println!("{}", content);
    Ok(content)
}

pub fn init_osrm(cfg: &mut web::ServiceConfig) {
    cfg.service(get_route);
    //cfg.service(sd_routes);
}
