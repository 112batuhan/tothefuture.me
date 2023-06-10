pub mod entities;
pub mod queries;

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use tracing::log;

#[derive(Clone)]
pub struct Db {
    db_con: DatabaseConnection,
}

impl Db {
    pub async fn new() -> Result<Self, DbErr> {
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
