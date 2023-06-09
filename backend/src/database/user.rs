use super::{Db, DbError, UNIQUE_CONSTRAINT_VIOLATION};

pub struct User {
    pub id: i64,
    pub username: String,
}

impl Db {
    pub async fn create_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<User, DbError> {
        let inserted_use_result = sqlx::query_as!(
            User,
            "insert into users( username, email, password) values ($1,$2,$3) returning id, \
             username",
            username,
            email,
            password
        )
        .fetch_one(&self.pool)
        .await;

        match inserted_use_result {
            Ok(user) => Ok(user),
            Err(err) => {
                if let Some(db_error) = err.as_database_error() {
                    let db_error_code = db_error.code().unwrap();
                    if db_error_code == UNIQUE_CONSTRAINT_VIOLATION {
                        return Err(DbError::UniqueConstraintViolation(
                            db_error.message().to_string(),
                        ));
                    }
                }

                Err(DbError::from(err))
            }
        }
    }
}
