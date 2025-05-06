pub mod config;
pub mod error;
pub mod mailer;
pub mod request;
pub mod service;

use config::Config;
use rmcp::transport::sse_server::SseServer;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::read_from_file();

    let ct = SseServer::serve(config.sse_server_host.parse()?)
        .await?
        .with_service(move || service::MailerService::new(config.clone()));

    tokio::signal::ctrl_c().await?;
    ct.cancel();
    Ok(())
}
