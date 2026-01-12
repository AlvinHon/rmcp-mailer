pub mod config;
pub mod database;
pub mod error;
pub mod mailer;
pub mod model;
pub mod request;
pub mod service;

use config::Config;
use rmcp::transport::{
    StreamableHttpServerConfig, StreamableHttpService,
    streamable_http_server::session::local::LocalSessionManager,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::read_from_file();
    let bind_address = config.server_host.clone();
    let service = StreamableHttpService::new(
        move || Ok(service::MailerService::new(config.clone())),
        LocalSessionManager::default().into(),
        StreamableHttpServerConfig::default(),
    );

    let router = axum::Router::new().nest_service("/mcp", service);
    let ct = tokio_util::sync::CancellationToken::new();

    let tcp_listener = tokio::net::TcpListener::bind(bind_address).await?;
    let _ = axum::serve(tcp_listener, router)
        .with_graceful_shutdown(async move {
            tokio::signal::ctrl_c().await.unwrap();
            ct.cancel();
        })
        .await;
    Ok(())
}
