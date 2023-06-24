use super::{Db, DbError, RedisType};

impl Db {
    // The reason that we insert the token both ways is to check the token with id as key to confirm
    // if the user alrady has a login in the server. Without this, one user can create infinite
    // sessions with login endpoint. This is solved by checking tokens with id as key, deleting the
    // key if it exist, and creating a new one.
    // Check login handler in api module for the implementation.
    pub async fn insert_session(&self, user_id: i64, token: &str) -> Result<(), DbError> {
        redis::pipe()
            .cmd("SET")
            .arg(RedisType::SessionToken(token).to_string())
            .arg(user_id)
            .arg("EX")
            .arg(3600) // 1 hour
            .ignore()
            .cmd("SET")
            .arg(RedisType::SessionID(user_id).to_string())
            .arg(token)
            .arg("EX")
            .arg(3600) // 1 hour
            .ignore()
            .query_async(&mut self.redis_con.clone())
            .await?;
        Ok(())
    }

    pub async fn get_session_with_token(&self, token: &str) -> Result<i64, DbError> {
        let session: Option<i64> = redis::cmd("GET")
            .arg(RedisType::SessionToken(token).to_string())
            .query_async(&mut self.redis_con.clone())
            .await?;

        session.ok_or(DbError::MissingSessionTokenInDatabase)
    }

    pub async fn get_session_with_id(&self, id: i64) -> Result<String, DbError> {
        let session: Option<String> = redis::cmd("GET")
            .arg(RedisType::SessionID(id).to_string())
            .query_async(&mut self.redis_con.clone())
            .await?;

        session.ok_or(DbError::MissingSessionTokenInDatabase)
    }

    // This function may cause inconsistent data as I wanted to have token keys and id keys to be
    // in sync. But async transactions and using connection manager in transactions is not possible
    // with the library. But in the end, the entries have a limited life so they should expire
    // on their own to fix this issue.
    pub async fn delete_session(&self, token: &str) -> Result<(), DbError> {
        let id: Option<i64> = redis::cmd("GETDEL")
            .arg(RedisType::SessionToken(token).to_string())
            .query_async(&mut self.redis_con.clone())
            .await?;

        let id = id.ok_or_else(|| DbError::MissingSessionTokenInDatabase)?;

        let delete_count: u8 = redis::cmd("DEL")
            .arg(RedisType::SessionID(id).to_string())
            .query_async(&mut self.redis_con.clone())
            .await?;

        if delete_count == 0 {
            Err(DbError::MissingSessionTokenInDatabase)
        } else {
            Ok(())
        }
    }
}
