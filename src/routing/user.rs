use axum::{
    extract::{Path, State},
    Json,
};
use serde::Serialize;

use super::AppError;
use crate::{
    database::Db,
    model::{account::AccountWithoutOwner, user::UserPublic},
};

pub async fn user(
    Path(name): Path<String>,
    State(db): State<Db>,
) -> Result<Json<UserResponse>, AppError> {
    Err(AppError::NotFound)
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    #[serde(flatten)]
    user: UserPublic,
    accounts: Vec<AccountWithoutOwner>,
}
