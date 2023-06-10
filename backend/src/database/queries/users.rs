use sea_orm::ActiveValue::Set;
use sea_orm::{DbErr, EntityTrait};

use crate::database::entities::prelude::*;
use crate::database::entities::*;
use crate::database::Db;

impl Db {
    pub async fn create_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<(), DbErr> {
        let user_to_insert = users::ActiveModel {
            username: Set(username.to_string()),
            email: Set(email.to_string()),
            password: Set(password.to_string()),
            ..Default::default()
        };
        Users::insert(user_to_insert).exec(&self.db_con).await?;
        Ok(())
    }
}
