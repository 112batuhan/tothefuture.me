use crate::database_sea::{Db, DbError, UNIQUE_CONSTRAINT_VIOLATION};
use crate::database_sea::queries::users;

impl Db {
    pub async fn create_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<(), DbError> {
        ()
    }
}
