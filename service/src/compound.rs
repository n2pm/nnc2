use nnc_entity::{user, wallet};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct UserWithWallets {
    #[serde(flatten)]
    pub user: user::Model,
    pub wallets: Vec<wallet::Model>,
}
