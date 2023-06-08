pub mod authentication;

use std::sync::Arc;

use thiserror::Error;

use super::database::Db;
use crate::database;

pub struct SharedState {
    database: Db,
}

impl SharedState {
    pub async fn new() -> Result<Arc<Self>, ApiError> {
        let database = Db::new().await?;
        Ok(Arc::new(SharedState { database }))
    }
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("placeholder")]
    General,
    // Can't use #[from] here because of poor error implementation in hash library
    #[error("Error during hashing")]
    Hash,
    #[error("Database error {0}")]
    Database(#[from] database::DbError),
}
