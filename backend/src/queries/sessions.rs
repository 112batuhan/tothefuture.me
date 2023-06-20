use super::{Db, DbError};

impl Db {
    pub async fn upsert_session(&self, user_id: i64, token: String) -> Result<(), DbError> {
        redis::cmd("SET")
            .arg(token)
            .arg(user_id)
            .arg("EX")
            .arg(3600) // 1 hour
            .query_async(&mut self.redis_con.clone())
            .await?;
        Ok(())
    }

    pub async fn get_session(&self, token: &str) -> Result<i64, DbError> {
        let session: Option<i64> = redis::cmd("GET")
            .arg(token)
            .query_async(&mut self.redis_con.clone())
            .await?;

        session.ok_or(DbError::MissingSessionToken)
    }

    pub async fn delete_session(&self, token: &str) -> Result<(), DbError> {
        redis::cmd("DEL")
            .arg(token)
            .query_async(&mut self.redis_con.clone())
            .await?;
        Ok(())
    }
}
