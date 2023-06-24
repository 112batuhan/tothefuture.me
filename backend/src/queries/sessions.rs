use super::{Db, DbError, RedisType};

impl Db {
    pub async fn insert_session(&self, user_id: i64, token: &str) -> Result<(), DbError> {
        redis::pipe()
            .cmd("SET")
            .arg(RedisType::SessionToken(token).to_string())
            .arg(user_id)
            .arg("EX")
            .arg(3600) // 1 hour
            .query_async(&mut self.redis_con.clone())
            .await?;
        Ok(())
    }

    pub async fn get_session(&self, token: &str) -> Result<i64, DbError> {
        let session: Option<i64> = redis::cmd("GET")
            .arg(RedisType::SessionToken(token).to_string())
            .query_async(&mut self.redis_con.clone())
            .await?;

        session.ok_or(DbError::MissingSessionTokenInDatabase)
    }

    pub async fn delete_session(&self, token: &str) -> Result<(), DbError> {
        let id: Option<i64> = redis::cmd("GETDEL")
            .arg(RedisType::SessionToken(token).to_string())
            .query_async(&mut self.redis_con.clone())
            .await?;

        id.ok_or_else(|| DbError::MissingSessionTokenInDatabase)?;
        Ok(())
    }
}
