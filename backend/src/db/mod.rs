use diesel::{pg::Pg, r2d2, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::info;
use thiserror::Error;

pub mod schema;
pub mod stations;
pub mod user;

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<r2d2::ConnectionManager<PgConnection>>;

#[derive(Error, Debug)]
pub enum DBError {
    #[error("Cannot apply all migrations: {0}")]
    MigrationError(String),
    #[error("While interacting with db: {0}")]
    DieselError(#[from] diesel::result::Error),
}

pub fn initialize_db_pool(url: String) -> DbPool {
    let manager = r2d2::ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be a valid url to an PGsql instance")
}

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_migrations(connection: &mut impl MigrationHarness<Pg>) -> Result<(), DBError> {
    if let Ok(x) = connection.pending_migrations(MIGRATIONS) {
        if !x.is_empty() {
            let s = x
                .into_iter()
                .map(|x| x.name().to_string())
                .fold(String::new(), |x, y| x + "\n\t" + &y);
            info!("Running migrations: {}", s)
        }
    }
    connection.run_pending_migrations(MIGRATIONS).map_err(|e| DBError::MigrationError(e.to_string()))?;
    Ok(())
}
