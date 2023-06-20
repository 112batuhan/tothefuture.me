use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use super::{Db, DbError};
use crate::entities::prelude::*;
use crate::entities::{self, *};

impl Db {
    pub async fn create_email(
        &self,
        owner: i64,
        subject: String,
        is_html: bool,
        body: String,
        send_date: chrono::NaiveDate,
    ) -> Result<(), DbError> {
        let email_to_insert = emails::ActiveModel {
            owner: Set(owner),
            subject: Set(subject),
            is_html: Set(is_html),
            body: Set(body),
            send_date: Set(send_date),
            ..Default::default()
        };
        // No explicit error handling needed for this operation as there can't be any unique key
        // violations. Maybe in the future we can set a limit per person.
        Emails::insert(email_to_insert).exec(&self.pg_con).await?;
        Ok(())
    }

    pub async fn get_emails_by_user(
        &self,
        user_id: i64,
    ) -> Result<Vec<entities::emails::Model>, DbError> {
        let email_vec = emails::Entity::find()
            .filter(emails::Column::Owner.eq(user_id))
            .all(&self.pg_con)
            .await?;

        if email_vec.is_empty() {
            Err(DbError::EmptyQuery)
        } else {
            Ok(email_vec)
        }
    }

    pub async fn get_emails_by_id(
        &self,
        email_id: i64,
    ) -> Result<entities::emails::Model, DbError> {
        let email = emails::Entity::find_by_id(email_id)
            .one(&self.pg_con)
            .await?;
        email.ok_or(DbError::EmptyQuery)
    }
}
