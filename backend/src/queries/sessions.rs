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

        session.ok_or(DbError::MissingSessionTokenInDatabase)
    }

    pub async fn delete_session(&self, token: &str) -> Result<(), DbError> {
        let deletion_result: u8 = redis::cmd("DEL")
            .arg(token)
            .query_async(&mut self.redis_con.clone())
            .await?;
        dbg!(&deletion_result);
        if deletion_result == 0 {
            Err(DbError::MissingSessionTokenInDatabase)
        } else {
            Ok(())
        }
    }
}
