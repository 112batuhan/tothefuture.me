pub mod entities;
pub mod queries;

use thiserror::Error;

const UNIQUE_CONSTRAINT_VIOLATION: &str = "23505";

#[derive(Clone)]
pub struct Db {
    db_con: PgPool,
}

impl Db {
    pub async fn new() -> Result<Self, DbError> {
        
        let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

        let mut opt = ConnectOptions::new(db_url);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info)
            .set_schema_search_path("my_schema".into()); // Setting default PostgreSQL schema

        let db_con = Database::connect(opt).await?;
        Ok(Self{ db_con   })
        
    }
}

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Tried to insert a value into database that is supposed to be unique: {0}")]
    UniqueConstraintViolation(String),
    #[error("Unhandled database error.")]
    Database(#[from] sea_orm::error::DbErr),
}
