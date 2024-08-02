use actix_web::{post, web::{self, Data, Form}, Responder};

use crate::{config::AppConfig, odh::get_near_stations};

use self::request::{OSRMRequest, PathRequest};

pub mod hardcoded;
pub mod request;

struct StationInfo{
    coordinate: (f32, f32),
    id: String,
    
}

async fn sd_routes(osrm_req: OSRMRequest) -> String {
    let mut req = String::from("table/v1/foot/");

    for (i, (lat, lon)) in osrm_req.stations.iter().enumerate() {
        req.push_str(&format!("{},{}", lon, lat));
        if i < osrm_req.stations.len() - 1 {
            req.push(';');
        }
    }

    req.push_str("?source=");
    for i in 0..osrm_req.stations.len() - 1 {
        req.push_str(&format!("{}", i));
        if i < osrm_req.stations.len() - 2 {
            req.push(';');
        }
    }

    req.push_str(&format!("&destination={}", osrm_req.stations.len() - 1));

    req
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