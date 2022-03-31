pub mod models;
pub mod routes;
pub mod utils;

use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use sea_orm::Database;
use std::{error::Error, net::SocketAddr, result::Result};
use tower_http::trace::TraceLayer;

use self::utils::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let config = envy::from_env::<Config>()?;
    let db = Database::connect(config.database_url.to_owned()).await?;
    let app = Router::new()
        .route("/", get(routes::users::get_all_users))
        .route("/:id", get(routes::users::get_user_by_id))
        .route("/@me", get(routes::users::get_self))
        .route("/login", post(routes::users::login))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(db))
        .layer(Extension(config));

    axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], 8000)))
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
