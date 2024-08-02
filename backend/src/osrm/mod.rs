use actix_web::{dev::Response, get, web, Responder};

pub mod hardcoded;

#[get("/get-path")]
async fn get_path() -> actix_web::Result<impl Responder> {
    Ok(Response::ok())
}

pub fn init_osrm(cfg: &mut web::ServiceConfig) {
    cfg.service(get_path);
}