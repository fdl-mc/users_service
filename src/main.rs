pub mod models;
pub mod routes;
pub mod utils;

use axum::{extract::Extension, routing::get, Router};
use sea_orm::Database;
use std::{error::Error, net::SocketAddr, result::Result};
use tower_http::trace::TraceLayer;

use self::utils::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let config = envy::from_env::<Config>()?;
    let db = Database::connect(config.postgres_url.to_owned()).await?;
    let app = Router::new()
        .route("/", get(routes::identity::get_all_identities))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(db))
        .layer(Extension(config));

    axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], 3000)))
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
