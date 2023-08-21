use nanoid::nanoid;
use nnc_entity::{user, wallet};
use sea_orm::*;

use crate::compound::UserWithWallets;

pub struct Mutation;

impl Mutation {
    fn generate_id() -> String {
        nanoid!(10) // this still provides 64^10 possible IDs
    }

    pub async fn create_user<C: ConnectionTrait>(
        db: &C,
        name: impl ToString,
    ) -> Result<user::Model, DbErr> {
        user::ActiveModel {
            id: Set(Self::generate_id()),
            name: Set(name.to_string()),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn create_wallet<C: ConnectionTrait>(
        db: &C,
        name: impl ToString,
        owner_id: impl ToString,
    ) -> Result<wallet::Model, DbErr> {
        wallet::ActiveModel {
            id: Set(Self::generate_id()),
            name: Set(name.to_string()),
            owner_id: Set(owner_id.to_string()),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn create_user_with_default_wallet<C: ConnectionTrait + TransactionTrait>(
        db: &C,
        name: impl ToString,
    ) -> Result<UserWithWallets, TransactionError<DbErr>> {
        let name = name.to_string();
        let (user, wallet) = db
            .transaction(|txn| {
                Box::pin(async move {
                    let user = Self::create_user(txn, name).await?;
                    let wallet = Self::create_wallet(txn, "Default", &user.id).await?;
                    Ok((user, wallet))
                })
            })
            .await?;
        Ok(UserWithWallets {
            user,
            wallets: vec![wallet],
        })
    }
}
