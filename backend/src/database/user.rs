use super::{Db, DbError};

pub struct User {
    id: i64,
    name: String,
}

impl Db {
    pub async fn create_user(&self, username: &str) -> Result<User, DbError> {
        let inserted_user = sqlx::query_as!(
            User,
            "insert into users(username, email, password) values ($1) returning id, username",
            username,
            email,
            password
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(inserted_user)
    }
}
