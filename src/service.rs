use rmcp::{
    ServerHandler,
    model::{CallToolResult, Content, ServerCapabilities, ServerInfo},
    tool,
};

use crate::{mailer::Mailer, request::SendEmailRequest};

#[derive(Debug, Clone)]
pub struct MailerService;

#[tool(tool_box)]
impl MailerService {
    pub fn new() -> Self {
        Self
    }

    #[tool(description = "Send an simple plain text email")]
    async fn send_email(
        &self,
        #[tool(aggr)] email_request: SendEmailRequest,
    ) -> Result<CallToolResult, rmcp::Error> {
        Mailer::send(&email_request).await?;

        Ok(CallToolResult::success(vec![Content::text(
            "Email sent successfully!",
        )]))
    }
}

#[tool(tool_box)]
impl ServerHandler for MailerService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("send email".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

impl Default for MailerService {
    fn default() -> Self {
        Self::new()
    }
}
