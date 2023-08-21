mod user;
mod wallet;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use sea_orm::{DbConn, DbErr, TransactionError};
use thiserror::Error;
use tracing::error;

pub fn make_router(db: DbConn) -> Router {
    Router::new()
        .route("/users", get(user::list_users))
        .route("/users", post(user::create_user))
        .route("/users/:id", get(user::get_user_by_id))
        .route("/wallets", get(wallet::list_wallets))
        .route("/wallets/:id", get(wallet::get_wallet_by_id))
        // .route("/users/:name/accounts", get(user_accounts))
        .with_state(db)
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found")]
    NotFound,
    #[error(transparent)]
    Database(#[from] DbErr),
    #[error(transparent)]
    Transaction(#[from] TransactionError<DbErr>),
    #[error(transparent)]
    Eyre(#[from] eyre::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        error!("{self}");
        match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "Not found.".to_string()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong. Please consult the logs.".to_string(),
            ),
        }
        .into_response()
    }
}
