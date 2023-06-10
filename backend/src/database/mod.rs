pub mod user;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use thiserror::Error;

const UNIQUE_CONSTRAINT_VIOLATION: &str = "23505";

#[derive(Clone)]
pub struct Db {
    pool: PgPool,
}

impl Db {
    pub async fn new() -> Result<Self, DbError> {
        dotenv::dotenv().expect("Unable to load environment variables from .env file");

        let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

        let pool = PgPoolOptions::new()
            .max_connections(100)
            .connect(&db_url)
            .await?;
        Ok(Self { pool })
    }
}

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Tried to insert a value into database that is supposed to be unique: {0}")]
    UniqueConstraintViolation(String),
    #[error("Unhandled database error.")]
    Database(#[from] sqlx::Error),
}
