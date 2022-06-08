mod claims;
pub use claims::Claims;

mod config;
pub use config::Config;

pub mod models;
pub mod proto;
pub mod service;
pub mod utils;

use proto::users::users_server::UsersServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = envy::from_env::<Config>().unwrap();

    let pool = sqlx::PgPool::connect(&config.database_url.to_owned())
        .await
        .unwrap();

    sqlx::migrate!().run(&pool.clone()).await.unwrap();

    let addr = "0.0.0.0:8000".parse().unwrap();
    let users_service = service::UsersService { pool, config };

    tracing::info!(message = "Starting server.", %addr);

    Server::builder()
        .trace_fn(|_| tracing::info_span!("users_service"))
        .add_service(UsersServer::new(users_service))
        .serve(addr)
        .await?;

    Ok(())
}
