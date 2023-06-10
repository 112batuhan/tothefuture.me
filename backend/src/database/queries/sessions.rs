use chrono::{DateTime, Duration, FixedOffset, Utc};
use sea_orm::sea_query::OnConflict;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};

use crate::database::entities::prelude::*;
use crate::database::entities::*;
use crate::database::{Db, DbError};

impl Db {
    pub async fn upsert_session(&self, user_id: i32, token: &str) -> Result<(), DbError> {
        let expiration: DateTime<Utc> = Utc::now() + Duration::hours(1);
        let expiration: DateTime<FixedOffset> = expiration.into();

        let session_to_insert = sessions::ActiveModel {
            id: Set(user_id),
            token: Set(token.to_string()),
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
            Some(user) => Ok(user),
            None => Err(DbError::MissingSessionToken),
        }
    }

    pub async fn delete_session(&self, token: &str) -> Result<(), DbError> {
        let session = self.get_session(token).await?;
        session.delete(&self.db_con).await?;
        Ok(())
    }
}
