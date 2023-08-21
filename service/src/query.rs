use nnc_entity::{
    user::{self, Entity as User},
    wallet::{self, Entity as Wallet},
};
use sea_orm::*;

use crate::compound::{UserWithWallets, WalletWithOwner};

pub struct Query;

impl Query {
    pub async fn list_users<C: ConnectionTrait>(db: &C) -> Result<Vec<user::Model>, DbErr> {
        User::find().all(db).await
    }

    pub async fn list_users_with_wallets<C: ConnectionTrait>(
        db: &C,
    ) -> Result<Vec<UserWithWallets>, DbErr> {
        Ok(User::find()
            .find_with_related(Wallet)
            .all(db)
            .await?
            .into_iter()
            .map(UserWithWallets::from)
            .collect())
    }

    pub async fn get_user_by_id<C: ConnectionTrait>(
        db: &C,
        id: impl ToString,
    ) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id.to_string()).one(db).await
    }

    pub async fn get_user_by_id_with_wallets<C: ConnectionTrait>(
        db: &C,
        id: impl ToString,
    ) -> Result<Option<UserWithWallets>, DbErr> {
        Ok(User::find_by_id(id.to_string())
            .find_with_related(Wallet)
            .all(db)
            .await?
            .into_iter()
            .map(UserWithWallets::from)
            .nth(0))
    }

    pub async fn list_wallets<C: ConnectionTrait>(db: &C) -> Result<Vec<wallet::Model>, DbErr> {
        Wallet::find().all(db).await
    }

    pub async fn list_wallets_with_owners<C: ConnectionTrait>(
        db: &C,
    ) -> Result<Vec<WalletWithOwner>, DbErr> {
        Wallet::find()
            .find_also_related(User)
            .all(db)
            .await?
            .into_iter()
            .map(WalletWithOwner::try_from)
            .collect()
    }

    pub async fn get_wallet_by_id<C: ConnectionTrait>(
        db: &C,
        id: impl ToString,
    ) -> Result<Option<wallet::Model>, DbErr> {
        Wallet::find_by_id(id.to_string()).one(db).await
    }

    pub async fn get_wallet_by_id_with_owner<C: ConnectionTrait>(
        db: &C,
        id: impl ToString,
    ) -> Result<Option<WalletWithOwner>, DbErr> {
        Ok(
            match Wallet::find_by_id(id.to_string())
                .find_also_related(User)
                .one(db)
                .await?
            {
                Some(result) => Some(result.try_into()?),
                None => None,
            },
        )
    }
}
