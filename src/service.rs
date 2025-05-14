use std::sync::Arc;

use rmcp::{
    ServerHandler,
    model::{CallToolResult, Content, ServerCapabilities, ServerInfo},
    schemars, tool,
};
use tokio::sync::Mutex;

use crate::{
    config::Config,
    database::Database,
    error::new_rmcp_error,
    mailer::Mailer,
    request::{SendEmailRequest, SendGroupEmailRequest},
};

#[derive(Debug, Clone)]
pub struct MailerService {
    mailer: Mailer,
    db: Arc<Mutex<Database>>,
}

#[tool(tool_box)]
impl MailerService {
    pub fn new(config: Config) -> Self {
        Self {
            mailer: Mailer::new(config.mailer_config),
            db: Arc::new(Mutex::new(Database::new(config.db_config))),
        }
    }

    #[tool(description = "Send an simple plain text email")]
    async fn send_email(
        &self,
        #[tool(aggr)] email_request: SendEmailRequest,
    ) -> Result<CallToolResult, rmcp::Error> {
        self.mailer.send(&email_request).await?;

        Ok(CallToolResult::success(vec![Content::text(
            "Email sent successfully!",
        )]))
    }

    #[tool(description = "Send an email to a group")]
    async fn send_email_to_group(
        &self,
        #[tool(aggr)] email_request: SendGroupEmailRequest,
    ) -> Result<CallToolResult, rmcp::Error> {
        let mut db = self.db.lock().await;
        let group = db
            .find_group_by_name(email_request.group_name.clone())
            .map_err(|_| new_rmcp_error("Group not found"))?;

        let recipients = db.find_recipients_by_group_id(group.id)?;
        let to = recipients.into_iter().map(|r| r.email).collect::<Vec<_>>();

        let request = SendEmailRequest {
            to,
            reply_to: email_request.reply_to,
            subject: email_request.subject,
            body: email_request.body,
        };

        self.mailer.send(&request).await?;

        Ok(CallToolResult::success(vec![Content::text(
            "Email sent to group successfully!",
        )]))
    }

    #[tool(description = "Get information from phone book")]
    async fn get_phone_book(&self) -> Result<CallToolResult, rmcp::Error> {
        let (recipients, groups) = {
            let mut db = self.db.lock().await;
            let recipients = db.list_recipients()?;
            let groups = db.list_groups()?;

            (recipients, groups)
        };

        let mut result = vec![];
        for recipient in recipients {
            result.push(Content::text(format!("Recipient: {}", recipient.email)));
        }
        for group in groups {
            result.push(Content::text(format!("Group: {}", group.name)));
        }

        Ok(CallToolResult::success(result))
    }

    #[tool(description = "Add a new mail group")]
    async fn add_mail_group(
        &self,
        #[tool(param)]
        #[schemars(description = "mail group name")]
        group_name: String,
    ) -> Result<CallToolResult, rmcp::Error> {
        self.db.lock().await.new_group(group_name)?;

        Ok(CallToolResult::success(vec![Content::text(
            "Recipient group created successfully!",
        )]))
    }

    #[tool(description = "Add a new recipient")]
    async fn add_recipient(
        &self,
        #[tool(param)]
        #[schemars(description = "recipient email")]
        email: String,
    ) -> Result<CallToolResult, rmcp::Error> {
        self.db.lock().await.new_recipient(email)?;

        Ok(CallToolResult::success(vec![Content::text(
            "Recipient added successfully!",
        )]))
    }

    #[tool(description = "Add a recipient to the mail group")]
    async fn add_recipient_to_group(
        &self,
        #[tool(param)]
        #[schemars(description = "Group name")]
        group_name: String,
        #[tool(param)]
        #[schemars(description = "recipient email")]
        email: String,
    ) -> Result<CallToolResult, rmcp::Error> {
        let mut db = self.db.lock().await;
        let group = db
            .find_group_by_name(group_name.clone())
            .map_err(|_| new_rmcp_error("Group not found"))?;
        let recipient = db
            .find_recipient_by_email(email.clone())
            .map_err(|_| new_rmcp_error("Recipient not found"))?;

        db.add_recipient_to_group(group.id, recipient.id)?;

        Ok(CallToolResult::success(vec![Content::text(
            "Recipient added to group successfully!",
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
        Self::new(Config::default())
    }
}
