use actix_web::{post, web::{self, Form}, Responder};

use self::request::{OSRMRequest, PathRequest};

pub mod hardcoded;
pub mod request;

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
pub async fn get_route(req: Form<PathRequest>) -> impl Responder {
    format!("{:?}", req)
}

pub fn init_osrm(cfg: &mut web::ServiceConfig) {
    cfg.service(get_route);
    //cfg.service(sd_routes);
}