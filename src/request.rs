use rmcp::schemars;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SendEmailRequest {
    pub to: Vec<String>,
    pub reply_to: Option<String>,
    pub subject: String,
    pub body: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SendGroupEmailRequest {
    pub group_name: String,
    pub reply_to: Option<String>,
    pub subject: String,
    pub body: String,
}
