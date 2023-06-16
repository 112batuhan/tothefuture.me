use chrono::{DateTime, Duration, FixedOffset, Utc};
use sea_orm::sea_query::OnConflict;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};

use super::{Db, DbError};
use crate::entities::prelude::*;
use crate::entities::*;

impl Db {
    pub async fn upsert_session(&self, user_id: i64, token: String) -> Result<(), DbError> {
        let expiration: DateTime<Utc> = Utc::now() + Duration::hours(1);
        let expiration: DateTime<FixedOffset> = expiration.into();

        let session_to_insert = sessions::ActiveModel {
            id: Set(user_id),
            token: Set(token),
            expires: Set(expiration),
        };

        Sessions::insert(session_to_insert)
            .on_conflict(
                OnConflict::column(sessions::Column::Id)
                    .update_column(sessions::Column::Token)
                    .update_column(sessions::Column::Expires)
                    .to_owned(),
            )
            .exec(&self.db_con)
            .await?;

        Ok(())
    }

    pub async fn get_session(&self, token: &str) -> Result<sessions::Model, DbError> {
        let session: Option<sessions::Model> = sessions::Entity::find()
            .filter(sessions::Column::Token.eq(token))
            .one(&self.db_con)
            .await?;

        match session {
            Some(user_session) => Ok(user_session),
            None => Err(DbError::MissingSessionToken),
        }
    }

    pub async fn delete_session(&self, session_id: i64) -> Result<(), DbError> {
        let session: Option<sessions::Model> = sessions::Entity::find_by_id(session_id)
            .one(&self.db_con)
            .await?;

        let session = match session {
            Some(user_session) => user_session,
            None => return Err(DbError::MissingSessionToken),
        };

        session.delete(&self.db_con).await?;
        Ok(())
    }
}
