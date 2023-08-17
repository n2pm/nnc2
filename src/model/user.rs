use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub account_limit: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserPublic {
    pub id: u32,
    pub name: String,
}
