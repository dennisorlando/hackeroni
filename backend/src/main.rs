use std::{env, error::Error};

use actix_identity::{Identity, IdentityExt, IdentityMiddleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, get, middleware, post, web::{self, Form}, App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder};
use auth::{init_auth, Authenticated, User};
use base64::{prelude::BASE64_STANDARD, Engine};
use db::{initialize_db_pool, run_migrations, user::{insert_new_user,}, DbPool};
use dotenvy::dotenv;
use log::*;
pub mod db;
pub mod log;
pub mod auth;
#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/add_name/{name}")]
async fn hello(
    pool: web::Data<DbPool>,
    name: web::Path<String>,
    user: User<Authenticated>
) -> actix_web::Result<impl Responder> {

    Ok(format!("Hello {}!", user.user.name))
}



#[get("/whoami")]
async fn whoami(request: HttpRequest) -> actix_web::Result<impl Responder> {
    let x: Identity = request.get_identity().map_err(actix_web::error::ErrorUnauthorized)?;
    Ok(x.id()?)
}

fn get_secret_key() -> Key {
    let key = (|| {
        let key = env::var("SECRET")?;
        let key = BASE64_STANDARD.decode(key.as_bytes())?;
        let key = Key::try_from(key.as_slice())?;
        Ok(key)
    })();
    #[cfg(debug_assertions)]
    let ret = key
        .map_err(|e: Box<dyn Error>| {
            error!(
                "Impossible to obtain secret key, on release this will became an hard error: {e}"
            );
        })
        .unwrap_or(Key::generate());
    #[cfg(not(debug_assertions))]
    let ret = key
        .unwrap_or_else(|e: Box<dyn Error>| panic!("Impossible to obtain secret key. Reason: {e}"));
    ret
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    init_log();
    let pool = initialize_db_pool();
    run_migrations(&mut pool.get().unwrap())?;

    let secret_key = get_secret_key();

    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(hello)
            .service(whoami)
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .configure(init_auth)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}
