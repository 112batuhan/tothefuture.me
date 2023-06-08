use super::{Db, DbError};

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
        let inserted_user = sqlx::query_as!(
            User,
            "insert into users( username, email, password) values ($1,$2,$3) returning id, \
             username",
            username,
            email,
            password
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(inserted_user)
    }
}
