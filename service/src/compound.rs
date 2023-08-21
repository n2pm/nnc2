use nnc_entity::{user, wallet};
use sea_orm::DbErr;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct UserWithWallets {
    #[serde(flatten)]
    pub user: user::Model,
    pub wallets: Vec<wallet::Model>,
}

impl From<(user::Model, Vec<wallet::Model>)> for UserWithWallets {
    fn from((user, wallets): (user::Model, Vec<wallet::Model>)) -> Self {
        UserWithWallets { user, wallets }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct WalletWithOwner {
    #[serde(flatten)]
    pub wallet: wallet::Model,
    pub owner: user::Model,
}

impl TryFrom<(wallet::Model, Option<user::Model>)> for WalletWithOwner {
    type Error = DbErr;

    fn try_from(
        (wallet, owner): (wallet::Model, Option<user::Model>),
    ) -> Result<Self, Self::Error> {
        Ok(WalletWithOwner {
            owner: owner.ok_or(DbErr::RecordNotFound(format!(
                "Couldn't find wallet owner (what?!) (wallet ID {})",
                wallet.id
            )))?,
            wallet,
        })
    }
}
