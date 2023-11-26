use sea_orm_migration::prelude::*;

use super::m20231125_000001_create_user_table::User;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20231125_000002_create_apikey_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ApiKey::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ApiKey::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ApiKey::Key).string().not_null())
                    .col(ColumnDef::new(ApiKey::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-apikey-user_id")
                            .from(ApiKey::Table, ApiKey::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ApiKey::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum ApiKey {
    Table,
    Id,
    Key,
    UserId,
}
