pub mod models;
pub mod routes;
pub mod utils;

use axum::{
    extract::Extension,
    routing::{get, patch, post},
    Router,
};
use models::config::Config;
use sea_orm::Database;
use std::{error::Error, net::SocketAddr, result::Result};
use tower_http::trace::TraceLayer;
use utils::migration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize tracing (to stdout)
    tracing_subscriber::fmt::init();

    // Load config from env and connect to database
    let config = envy::from_env::<Config>()?;
    let db = Database::connect(config.database_url.to_owned()).await?;

    // Run migrations
    migration::migrate_all(db.clone()).await;

    // Setup an app
    let users_router = Router::new()
        .route("/", get(routes::users::get_all_users))
        .route("/:id", get(routes::users::get_user_by_id))
        .route("/find", get(routes::users::find_user))
        .route("/@me", get(routes::users::get_self));

    let auth_router = Router::new()
        .route("/", post(routes::auth::login))
        .route("/password", patch(routes::auth::change_password));

    let router = Router::new()
        .nest("/", users_router)
        .nest("/auth", auth_router);

    let app = router
        .layer(TraceLayer::new_for_http())
        .layer(Extension(db))
        .layer(Extension(config));

    // Serve the app
    axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], 8000)))
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
