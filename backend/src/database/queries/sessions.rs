use sea_orm::error::RuntimeErr;
use sea_orm::ActiveValue::Set;
use sea_orm::{DbErr, EntityTrait};

use crate::database::entities::prelude::*;
use crate::database::entities::*;
use crate::database::{Db, DbError, UNIQUE_KEY_VIOLATION_CODE};

impl Db {
    pub async fn create_user(
        &self,
        email: &str,
        password: &str,
    ) -> Result<(), DbError> {
        let user_to_insert = users::ActiveModel {
            username: Set(username.to_string()),
            email: Set(email.to_string()),
            password: Set(password.to_string()),
            ..Default::default()
        };
        let insert_result = Users::insert(user_to_insert).exec(&self.db_con).await;
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
