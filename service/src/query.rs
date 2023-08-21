use nnc_entity::{
    user::{self, Entity as User},
    wallet::{self, Entity as Wallet},
};
use sea_orm::*;

use crate::compound::UserWithWallets;

pub struct Query;

impl Query {
    pub async fn list_users(db: &DbConn) -> Result<Vec<user::Model>, DbErr> {
        User::find().all(db).await
    }

    pub async fn list_users_with_wallets(db: &DbConn) -> Result<Vec<UserWithWallets>, DbErr> {
        Ok(User::find()
            .find_with_related(Wallet)
            .all(db)
            .await?
            .into_iter()
            .map(|(user, wallets)| UserWithWallets { user, wallets })
            .collect())
    }

    pub async fn get_user_by_id(
        db: &DbConn,
        id: impl ToString,
    ) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id.to_string()).one(db).await
    }

    pub async fn get_user_by_id_with_wallets(
        db: &DbConn,
        id: impl ToString,
    ) -> Result<Option<UserWithWallets>, DbErr> {
        Ok(User::find_by_id(id.to_string())
            .find_with_related(Wallet)
            .all(db)
            .await?
            .into_iter()
            .map(|(user, wallets)| UserWithWallets { user, wallets })
            .nth(0))
    }

    pub async fn list_wallets(db: &DbConn) -> Result<Vec<wallet::Model>, DbErr> {
        Wallet::find().all(db).await
    }
}
