use actix_web::{post, web::{self, Form}, Responder};

use self::request::PathRequest;

pub mod hardcoded;
pub mod request;

#[post("/stocazzo")]
pub async fn get_route(req: Form<PathRequest>) -> impl Responder {
    format!("{:?}", req)
}

pub fn init_osrm(cfg: &mut web::ServiceConfig) {
    cfg.service(get_route);
    //cfg.service(sd_routes);
}
