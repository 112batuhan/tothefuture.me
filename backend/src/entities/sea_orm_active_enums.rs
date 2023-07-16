//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "email_state")]
pub enum EmailState {
    #[sea_orm(string_value = "default")]
    Default,
    #[sea_orm(string_value = "hidden")]
    Hidden,
    #[sea_orm(string_value = "sent")]
    Sent,
}