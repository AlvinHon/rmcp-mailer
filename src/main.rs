pub mod config;
pub mod database;
pub mod error;
pub mod logging;
pub mod mailer;
pub mod model;
pub mod request;
pub mod service;

use axum::{extract::Request, middleware::Next, response::Response};
use config::Config;
use log::info;
use rmcp::transport::{
    StreamableHttpServerConfig, StreamableHttpService,
    streamable_http_server::session::local::LocalSessionManager,
};
use std::error::Error;

use crate::logging::init_logging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Read the configuration
    let config = Config::read_from_file();

    // Initialize logging
    init_logging(&config.logger_config);

    // Start the server
    let bind_address = config.server_host.clone();
    let service = StreamableHttpService::new(
        move || Ok(service::MailerService::new(config.clone())),
        LocalSessionManager::default().into(),
        StreamableHttpServerConfig::default(),
    );

    let router = axum::Router::new()
        .nest_service("/mcp", service)
        .layer(axum::middleware::from_fn(log_request));
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

async fn log_request(request: Request, next: Next) -> Response {
    use http_body_util::BodyExt;

    let (parts, body) = request.into_parts();
    info!("Received request: {} {}", parts.method, parts.uri);

    // axum doesn't implement Clone for the request body,
    // so we need to recreate it after reading for logging purposes
    let recreate_body;
    // get the json body if it's a POST request
    if parts.method == axum::http::Method::POST {
        let body_bytes = body
            .collect()
            .await
            .map(|b| b.to_bytes())
            .unwrap_or_else(|_| axum::body::Bytes::new());
        info!("Request body: {}", String::from_utf8_lossy(&body_bytes));

        recreate_body = axum::body::Body::from(body_bytes);
    } else {
        recreate_body = body;
    }

    let request = Request::from_parts(parts, recreate_body);
    let response = next.run(request).await;

    let (response_parts, response_body) = response.into_parts();
    info!("Response status: {}", response_parts.status);

    // Log the response body in string
    let response_body_bytes = response_body
        .collect()
        .await
        .map(|b| b.to_bytes())
        .unwrap_or_else(|_| axum::body::Bytes::new());
    info!(
        "Response body: {}",
        String::from_utf8_lossy(&response_body_bytes)
    );
    let recreate_body = axum::body::Body::from(response_body_bytes);

    Response::from_parts(response_parts, recreate_body)
}
