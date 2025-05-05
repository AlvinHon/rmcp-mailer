use rmcp::schemars;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SendEmailRequest {
    pub to: String,
    pub reply_to: Option<String>,
    pub subject: String,
    pub body: String,
}
