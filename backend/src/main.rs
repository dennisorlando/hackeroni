use actix_web::{error, get, middleware, web, App, HttpServer, Responder};
use db::{initialize_db_pool, user::insert_new_user, DbPool};
use dotenvy::dotenv;
pub mod db;


#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/add_name/{name}")]
async fn hello(
    pool: web::Data<DbPool>,
    name: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    // use web::block to offload blocking Diesel queries without blocking server thread
    let user = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        insert_new_user(&mut conn, name.to_string())
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(format!("Hello {}!", user.name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let pool = initialize_db_pool();

    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(hello)
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
