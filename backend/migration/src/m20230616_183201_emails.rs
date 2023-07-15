use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{EnumIter, Iterable};
use sea_orm_migration::sea_query::extension::postgres::Type;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_type(
                Type::create()
                    .as_enum(EmailState::Table)
                    .values(EmailState::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Emails::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Emails::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Emails::Owner).big_integer().not_null())
                    .col(ColumnDef::new(Emails::Subject).string().not_null())
                    .col(ColumnDef::new(Emails::IsHtml).boolean().not_null())
                    .col(ColumnDef::new(Emails::Body).string().not_null())
                    .col(ColumnDef::new(Emails::SendDate).date().not_null())
                    .col(
                        ColumnDef::new(Emails::State)
                            .enumeration(EmailState::Table, EmailState::iter().skip(1))
                            .not_null()
                            .default("default"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Emails::Table).to_owned())
            .await
            .unwrap();

        manager
            .drop_type(Type::drop().if_exists().name(EmailState::Table).to_owned())
            .await
            .unwrap();

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Emails {
    Table,
    Id,
    Owner,
    Subject,
    IsHtml,
    State,
    Body,
    SendDate,
}

#[derive(Iden, EnumIter, Default)]
enum EmailState {
    Table,
    #[iden = "default"]
    #[default]
    Default,
    #[iden = "hidden"]
    Hidden,
    #[iden = "sent"]
    Sent,
}
