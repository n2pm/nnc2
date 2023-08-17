use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    pub id: u32,
    pub balance: u32,
    pub owner: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountWithoutOwner {
    pub id: u32,
    pub balance: u32,
}
