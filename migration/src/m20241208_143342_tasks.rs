use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Tasks::Table)
                    .col(pk_auto(Tasks::Id))
                    .col(string(Tasks::Title))
                    .col(text_null(Tasks::Description))
                    .col(integer(Tasks::Month))
                    .col(boolean(Tasks::Completed))
                    .col(string_null(Tasks::Category))
                    .col(integer_null(Tasks::Time))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tasks::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Tasks {
    Table,
    Id,
    Title,
    Description,
    Month,
    Completed,
    Category,
    Time,
    
}

