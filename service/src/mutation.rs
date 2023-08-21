use nanoid::nanoid;
use nnc_entity::{
    user::{self, Entity as User},
    wallet::{self, Entity as Wallet},
};
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

    pub async fn create_default_wallet<C: ConnectionTrait>(
        db: &C,
        owner_id: impl ToString,
    ) -> Result<wallet::Model, DbErr> {
        wallet::ActiveModel {
            id: Set(Self::generate_id()),
            name: Set("Default".to_string()),
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
                    let wallet = Self::create_default_wallet(txn, &user.id).await?;
                    Ok((user, wallet))
                })
            })
            .await?;
        Ok(UserWithWallets {
            user,
            wallets: vec![wallet],
        })
    }

    pub async fn delete_user<C: ConnectionTrait + TransactionTrait>(
        db: &C,
        id: impl ToString,
    ) -> Result<(), TransactionError<DbErr>> {
        let id = id.to_string();
        db.transaction(|txn| {
            Box::pin(async move {
                Wallet::delete_many()
                    .filter(wallet::Column::OwnerId.eq(&id))
                    .exec(txn)
                    .await?;
                User::delete_by_id(&id).exec(txn).await?;
                Ok(())
            })
        })
        .await
    }
}
