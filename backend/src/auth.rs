use std::{error::Error, fmt::Display, future::Future, marker::PhantomData, pin::Pin};

use actix_identity::{error::GetIdentityError, Identity};
use actix_web::{
    error::{BlockingError, ErrorInternalServerError},
    post,
    web::{self, Data, Form},
    FromRequest, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use argon2::PasswordHasher;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use log::{error, info};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::db::{
    user::{get_user_by_name, insert_new_user, NewUser, UserMem},
    DbPool,
};
pub trait UserState {}
pub struct Admin;
impl UserState for Admin {}
pub struct Authenticated;
impl UserState for Authenticated {}
#[derive(Debug)]
pub struct Argon2ErrorWrapper(argon2::password_hash::Error);
impl Display for Argon2ErrorWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl From<argon2::password_hash::Error> for Argon2ErrorWrapper {
    fn from(value: argon2::password_hash::Error) -> Self {
        Self(value)
    }
}
impl Error for Argon2ErrorWrapper {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
#[derive(Error, Debug)]
pub enum PasswordError {
    #[error("Error while hashing password: {0}")]
    EncriptionError(#[from] Argon2ErrorWrapper),
    #[error("Wrong password")]
    WrongPassword,
    #[error("R2D2 Error: {0}")]
    InternalError(#[from] r2d2::Error),
    #[error("Diesel Error: {0}")]
    DieselError(#[from] diesel::result::Error),
    #[error("Not logged in: {0}")]
    IdentityError(#[from] GetIdentityError),
    #[error("You are not allowed to see this page")]
    NotAdmin,
    #[error("Blocking error: {0}")]
    BlockingError(#[from] BlockingError),
    #[error("Error with actix {0}")]
    ActixError(String),
}
impl From<actix_web::Error> for PasswordError {
    fn from(value: actix_web::Error) -> Self {
        Self::ActixError(value.to_string())
    }
}
impl From<PasswordError> for actix_web::Error {
    fn from(value: PasswordError) -> Self {
        match value {
            PasswordError::WrongPassword => {
                actix_web::error::ErrorUnauthorized("incorrect password")
            }

            PasswordError::IdentityError(e) => {
                info!("{e}");
                actix_web::error::ErrorUnauthorized("You have not logged in")
            }
            PasswordError::NotAdmin => {
                actix_web::error::ErrorUnauthorized("You are not allowed to see this page")
            }
            e => {
                error!("{e}");
                actix_web::error::ErrorInternalServerError("Internal server error")
            }
        }
    }
}

pub fn hash_password(password: &str, pepper: &str) -> Result<String, PasswordError> {
    let password = password.to_string() + pepper;
    let salt = SaltString::generate(&mut OsRng);
    let hasher = Argon2::default();
    let password_hash = hasher
        .hash_password(password.as_bytes(), &salt)
        .map_err(Argon2ErrorWrapper::from)?
        .to_string();
    Ok(password_hash)
}

pub fn verify_password(
    hashed_password: &str,
    password: &str,
    pepper: &str,
) -> Result<(), PasswordError> {
    let password = password.to_string() + pepper;
    let hashed_password =
        PasswordHash::try_from(hashed_password).map_err(Argon2ErrorWrapper::from)?;
    let hasher = Argon2::default();
    hasher
        .verify_password(password.as_bytes(), &hashed_password)
        .map_err(|_| PasswordError::WrongPassword)?;
    Ok(())
}

pub struct User<S: UserState> {
    ph: PhantomData<S>,
    pub user: UserMem,
}

impl FromRequest for User<Authenticated> {
    type Error = PasswordError;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let data = Data::<DbPool>::extract(&req).await?;
            let identity = Identity::extract(&req).await?.id()?;
            let user = web::block(move || -> Result<UserMem, PasswordError> {
                let mut conn = data.get()?;
                let user = get_user_by_name(&mut conn, &identity)?;
                Ok(user)
            })
            .await??;
            Ok(Self {
                ph: PhantomData,
                user,
            })
        })
    }
}
impl FromRequest for User<Admin> {
    type Error = PasswordError;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            let t = User::<Authenticated>::extract(&req).await?;
            if !t.user.is_admin {
                return Err(PasswordError::NotAdmin);
            }
            Ok(Self {
                ph: PhantomData,
                user: t.user,
            })
        })
    }
}

#[derive(Serialize, Deserialize)]
struct Login {
    name: String,
    password: String,
}

#[post("/login")]
async fn login(
    request: HttpRequest,
    login: Form<Login>,
    pool: Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool
        .get()
        .map_err(|_| ErrorInternalServerError("can't get pool"))?;
    let user = web::block(move || {
        let user = get_user_by_name(&mut conn, &login.name)?;
        verify_password(&user.password_hash, &login.password, "figo")?;
        Ok::<UserMem, Box<dyn Error + Send + Sync>>(user)
    })
    .await?
    .map_err(|_| ErrorInternalServerError("can't get pool"))?;
    //TODO check for correct error type

    //if all ok
    // attach a verified user identity to the active session
    Identity::login(&request.extensions(), user.name.clone())?;

    Ok(HttpResponse::Ok())
}
#[post("/register")]
async fn register(register: Form<Login>, pool: Data<DbPool>) -> actix_web::Result<impl Responder> {
    //TODO check for password rightness
    let mut conn = pool
        .get()
        .map_err(|_| ErrorInternalServerError("can't get pool"))?;
    web::block(move || {
        let password_hash = hash_password(&register.password, "figo")?;
        insert_new_user(
            &mut conn,
            NewUser {
                name: register.name.clone(),
                password_hash,
            },
        )?;

        Ok::<(), Box<dyn Error + Send + Sync>>(())
    })
    .await?
    .map_err(|_| ErrorInternalServerError("can't get pool"))?;

    Ok(HttpResponse::Ok())
}

#[post("/logout")]
async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Ok()
}

pub fn init_auth(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(register);
}
