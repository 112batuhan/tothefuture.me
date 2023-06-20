pub use sea_orm_migration::prelude::*;

mod m20230610_075132_initial_migration;
mod m20230616_183201_emails;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230610_075132_initial_migration::Migration),
            Box::new(m20230616_183201_emails::Migration),
        ]
    }
}
