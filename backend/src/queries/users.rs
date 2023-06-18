use sea_orm::error::RuntimeErr;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter};

use super::{Db, DbError, UNIQUE_KEY_VIOLATION_CODE};
use crate::entities::prelude::*;
use crate::entities::*;

impl Db {
    pub async fn create_user(&self, email: String, password: String) -> Result<(), DbError> {
        let user_to_insert = users::ActiveModel {
            email: Set(email),
            password: Set(password),
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

    pub async fn find_user_by_email(&self, email: &str) -> Result<users::Model, DbError> {
        let user: Option<users::Model> = users::Entity::find()
            .filter(users::Column::Email.eq(email))
            .one(&self.db_con)
            .await?;

        match user {
            Some(user) => Ok(user),
            None => Err(DbError::EmptyQuery),
        }
    }

    pub async fn find_user_by_id(&self, user_id: i64) -> Result<users::Model, DbError> {
        let user: Option<users::Model> =
            users::Entity::find_by_id(user_id).one(&self.db_con).await?;

        match user {
            Some(user) => Ok(user),
            None => Err(DbError::EmptyQuery),
        }
    }
}
