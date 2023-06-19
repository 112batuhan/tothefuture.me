pub mod emails;
pub mod sessions;
pub mod users;

use redis::aio::ConnectionManager;
use redis::Client;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use thiserror::Error;
use tracing::log;

const UNIQUE_KEY_VIOLATION_CODE: &str = "23505";

#[derive(Clone)]
pub struct Db {
    pg_con: DatabaseConnection,
    redis_con: ConnectionManager,
}

impl Db {
    pub async fn new() -> Result<Self, DbError> {
        let pg_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");
        let mut opt = ConnectOptions::new(pg_url);
        opt.max_connections(100)
            .min_connections(1)
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Debug);

        let pg_con = Database::connect(opt).await?;

        let redis_url = std::env::var("REDIS_URL").expect("Unable to read DATABASE_URL env var");
        let redis_client = Client::open(redis_url).expect("Can't create Redis client");
        let redis_con = redis_client
            .get_tokio_connection_manager()
            .await
            .expect("Can't create Redis connection manager");

        Ok(Self { pg_con, redis_con })
    }
}

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Tried to insert a value into database that is supposed to be unique")]
    UniqueConstraintViolation,
    #[error("Unhandled database error: {0}")]
    Database(#[from] sea_orm::error::DbErr),
    #[error("Empty query.")]
    EmptyQuery,
    #[error("Missing session token.")]
    MissingSessionToken,
}
