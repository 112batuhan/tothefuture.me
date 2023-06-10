pub mod entities;
pub mod queries;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use thiserror::Error;
use tracing::log;

const UNIQUE_KEY_VIOLATION_CODE: &str = "23505";

#[derive(Clone)]
pub struct Db {
    db_con: DatabaseConnection,
}

impl Db {
    pub async fn new() -> Result<Self, DbError> {
        let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

        let mut opt = ConnectOptions::new(db_url);
        opt.max_connections(100)
            .min_connections(1)
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Debug);

        let db_con = Database::connect(opt).await?;
        Ok(Self { db_con })
    }
}

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Tried to insert a value into database that is supposed to be unique")]
    UniqueConstraintViolation,
    #[error("Unhandled database error: {0}")]
    Database(#[from] sea_orm::error::DbErr),
    #[error("Empty Query")]
    EmptyQuery,
}
