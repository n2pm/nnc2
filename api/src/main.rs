mod config;
mod routing;
mod tracing;

use ::tracing::{info, debug};
use axum::Server;
use eyre::Context;
use nnc_migration::{Migrator, MigratorTrait};
use sea_orm::Database;

use crate::{config::Config, routing::make_router, tracing::set_up_tracing};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    set_up_tracing("nnc_api").context("Failed to set up tracing")?;

    debug!("Loading config");
    let Config {
        listen_addr,
        db_addr,
    } = Config::load().context("Failed to load config")?;

    debug!("Connecting to database");
    let db = Database::connect(db_addr)
        .await
        .context("Failed to connect to database")?;

    debug!("Migrating database");
    Migrator::up(&db, None)
        .await
        .context("Failed to migrate database")?;

    debug!("Creating router");
    let app = make_router(db);

    info!("Starting service on {listen_addr}");
    Server::bind(&listen_addr)
        .serve(app.into_make_service())
        .await
        .context("Failed to serve app")?;

    Ok(())
}
