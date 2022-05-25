use tonic::transport::Server;
use users_proto::users_server::UsersServer;

pub mod users_proto {
    tonic::include_proto!("fdl.api.users");
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("users_descriptor");
}

mod claims;
pub use claims::Claims;

mod config;
pub use config::Config;

pub mod models;
pub mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = envy::from_env::<Config>().unwrap();

    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(users_proto::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let pool = sqlx::PgPool::connect(&config.database_url.to_owned())
        .await
        .unwrap();

    let addr = "[::1]:50051".parse().unwrap();
    let users_service = service::UsersService { pool };

    tracing::info!(message = "Starting server.", %addr);

    Server::builder()
        .trace_fn(|_| tracing::info_span!("users_service"))
        .add_service(UsersServer::new(users_service))
        .add_service(reflection)
        .serve(addr)
        .await?;

    Ok(())
}
