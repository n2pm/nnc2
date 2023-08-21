use sea_orm_migration::prelude::*;

use crate::create_index;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(User::Name).string().not_null().unique_key())
                    .col(
                        ColumnDef::new(User::WalletLimitOverride)
                            .integer()
                            .check(Expr::col((User::Table, User::WalletLimitOverride)).gte(0)),
                    )
                    .take(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Wallet::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Wallet::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Wallet::Name).string().not_null())
                    .col(
                        ColumnDef::new(Wallet::Balance)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Wallet::OwnerId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Wallet::Table, Wallet::OwnerId)
                            .to(User::Table, User::Id),
                    )
                    .take(),
            )
            .await?;

        manager
            .create_index(create_index(Wallet::Table, Wallet::OwnerId))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Wallet::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Name,
    WalletLimitOverride,
}

#[derive(DeriveIden)]
enum Wallet {
    Table,
    Id,
    Name,
    Balance,
    OwnerId,
}
