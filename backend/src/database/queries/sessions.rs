use chrono::{DateTime, Duration, FixedOffset, Utc};
use sea_orm::error::RuntimeErr;
use sea_orm::ActiveValue::Set;
use sea_orm::{DbErr, EntityTrait};

use crate::database::entities::prelude::*;
use crate::database::entities::*;
use crate::database::{Db, DbError, UNIQUE_KEY_VIOLATION_CODE};

impl Db {
    pub async fn create_session(&self, user_id: i32, token: &str) -> Result<(), DbError> {
        let expiration: DateTime<Utc> = Utc::now() + Duration::hours(1);
        let expiration: DateTime<FixedOffset> = expiration.into();

        let session_to_insert = sessions::ActiveModel {
            id: Set(user_id),
            token: Set(token.to_string()),
            expires: Set(expiration),
        };
        let insert_result = Sessions::insert(session_to_insert).exec(&self.db_con).await;
        match insert_result {
            Ok(_) => Ok(()),
            Err(orm_error) => match orm_error {
                DbErr::Query(RuntimeErr::SqlxError(ref sqlx_error)) => match sqlx_error {
                    sqlx::error::Error::Database(error) => {
                        if error.code().unwrap() == UNIQUE_KEY_VIOLATION_CODE {
                            Err(DbError::UniqueConstraintViolation)
                        } else {
                            Err(DbError::from(orm_error))
                        }
                    }
                    _ => Err(DbError::from(orm_error)),
                },
                other_err => Err(DbError::from(other_err)),
            },
        }
    }
}
