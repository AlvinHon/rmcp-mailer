use std::collections::HashMap;

use rmcp::schemars;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SendEmailRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>, // Optional sender email address. If not provided, the default sender will be used.
    pub to: Vec<String>,
    pub reply_to: Option<String>,
    pub subject: String,
    pub body: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SendGroupEmailRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>, // Optional sender email address. If not provided, the default sender will be used.
    pub group_name: String,
    pub reply_to: Option<String>,
    pub subject: String,
    pub body: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SendEmailWithTemplateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>, // Optional sender email address. If not provided, the default sender will be used.
    pub to: Vec<String>,
    pub reply_to: Option<String>,
    pub subject: String,
    pub template_name: String,
    pub template_data: HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub enum ManageGroupsRequest {
    Add(AddGroupRequest),
    Remove(RemoveGroupRequest),
    Update(UpdateGroupRequest),
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct AddGroupRequest {
    pub name: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct UpdateGroupRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_name: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct RemoveGroupRequest {
    pub name: String,
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

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub enum ManageTemplatesRequest {
    Add(AddTemplateRequest),
    Remove(RemoveTemplateRequest),
    Update(UpdateTemplateRequest),
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct AddTemplateRequest {
    pub name: String,
    pub format_string: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct UpdateTemplateRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_format_string: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct RemoveTemplateRequest {
    pub name: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetEmailHistoryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// The start date for filtering email history.
    /// ISO 8601 format (e.g., "2023-10-01T00:00:00")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// The end date for filtering email history.
    /// ISO 8601 format (e.g., "2023-10-31T23:59:59")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

impl GetEmailHistoryRequest {
    pub fn is_valid(&self) -> bool {
        // If all fields are None, the request is considered invalid
        if self.to.is_none() && self.start_date.is_none() && self.end_date.is_none() {
            return false;
        }

        // Validate the 'to' field if it is provided
        if let Some(to) = &self.to {
            if to.parse::<lettre::Address>().is_err() {
                return false;
            }
        }

        // Validate the start and end dates if they are provided
        is_valid_start_end_time(self.start_date.as_ref(), self.end_date.as_ref())
    }
}

pub(crate) fn is_valid_start_end_time(
    start_date: Option<&String>,
    end_date: Option<&String>,
) -> bool {
    let parsed_start = start_date
        .as_ref()
        .and_then(|start| chrono::DateTime::parse_from_rfc3339(start).ok());
    // If start_date is provided but cannot be parsed, return invalid
    if start_date.is_some() && parsed_start.is_none() {
        return false;
    }

    let parsed_end = end_date
        .as_ref()
        .and_then(|end| chrono::DateTime::parse_from_rfc3339(end).ok());

    // If end_date is provided but cannot be parsed, return invalid
    if end_date.is_some() && parsed_end.is_none() {
        return false;
    }

    // If both start_date and end_date are provided, check if start_date is before end_date
    if let (Some(parsed_start_datetime), Some(parsed_end_datetime)) = (&parsed_start, &parsed_end) {
        if parsed_start_datetime > parsed_end_datetime {
            return false;
        }
    }
    // If only start date is provided, it must not be future-dated
    if let Some(parsed_start_datetime) = parsed_start {
        if parsed_start_datetime > chrono::Utc::now() {
            return false;
        }
    }

    true
}

/// Converts the request's start and end dates to a tuple of NaiveDateTime.
/// If only one date is provided, the current time or UNIX epoch is used for the other.
pub(crate) fn parse_start_end_time(
    start_date: Option<&String>,
    end_date: Option<&String>,
) -> Option<(chrono::NaiveDateTime, chrono::NaiveDateTime)> {
    match (start_date, end_date) {
        (Some(start), Some(end)) => {
            let start_time = chrono::DateTime::parse_from_rfc3339(start)
                .ok()?
                .naive_utc();
            let end_time = chrono::DateTime::parse_from_rfc3339(end).ok()?.naive_utc();
            Some((start_time, end_time))
        }
        (Some(start), None) => {
            let start_time = chrono::DateTime::parse_from_rfc3339(start)
                .ok()?
                .naive_utc();
            Some((start_time, chrono::Utc::now().naive_utc()))
        }
        (None, Some(end)) => {
            let end_time = chrono::DateTime::parse_from_rfc3339(end).ok()?.naive_utc();
            Some((chrono::DateTime::UNIX_EPOCH.naive_local(), end_time))
        }
        _ => None,
    }
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CreateEventRequest {
    pub title: String,
    pub description: Option<String>,
    pub start_time: chrono::NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<chrono::NaiveDateTime>,
    pub is_all_day: bool,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SendEventInvitationRequest {
    pub event_id: i32,
    pub to: SendEventInvitationTo,
    pub subject: String,
    pub template: InvitationTemplate,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SendEventInvitationTo {
    /// list of groups to send invitations to, where the party is the group name
    pub groups: Vec<InvitationTarget>,
    /// list of individuals to send invitations to, where the party is the email address
    pub individuals: Vec<InvitationTarget>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct InvitationTarget {
    pub party: String,
    pub is_required: bool,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub enum InvitationTemplate {
    CustomText(String),     // Custom text for the invitation
    StoredTemplate(String), // Name of the stored template
}
