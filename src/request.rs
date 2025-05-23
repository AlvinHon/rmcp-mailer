use std::collections::HashMap;

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

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SendEmailWithTemplateRequest {
    pub to: Vec<String>,
    pub reply_to: Option<String>,
    pub subject: String,
    pub template_name: String,
    pub template_data: HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub enum ManageRecipientsRequest {
    Add(AddRecipientRequest),
    Remove(RemoveRecipientRequest),
    Update(UpdateRecipientRequest),
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct AddRecipientRequest {
    pub name: String,
    pub email: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct UpdateRecipientRequest {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_email: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct RemoveRecipientRequest {
    pub email: String,
}
