pub mod error;
pub mod mailer;
pub mod request;
pub mod service;

use rmcp::transport::sse_server::SseServer;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ct = SseServer::serve("127.0.0.1:3000".parse()?)
        .await?
        .with_service(service::MailerService::new);

    tokio::signal::ctrl_c().await?;
    ct.cancel();
    Ok(())
}
