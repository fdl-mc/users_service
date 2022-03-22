pub mod routes;

use axum::{routing::get, Router};
use std::{error::Error, net::SocketAddr, result::Result};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Initialize app
    let app = make_app();
    axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], 3000)))
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

fn make_app() -> Router {
    Router::new()
        .route("/", get(routes::root))
        .layer(TraceLayer::new_for_http())
}
