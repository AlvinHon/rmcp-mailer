use std::collections::HashMap;

use rmcp::schemars;
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(description = "Request to send an email to one or more recipients.")]
pub struct SendEmailRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(
        description = "Optional sender email address. If not provided, the default sender will be used."
    )]
    pub from: Option<String>,
    #[schemars(description = "List of recipient email addresses.")]
    pub to: Vec<String>,
    #[schemars(description = "Optional reply-to email address.")]
    pub reply_to: Option<String>,
    #[schemars(description = "Subject of the email.")]
    pub subject: String,
    #[schemars(description = "Body of the email.")]
    pub body: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(description = "Request to send an email to all members of a specified group.")]
pub struct SendGroupEmailRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(
        description = "Optional sender email address. If not provided, the default sender will be used."
    )]
    pub from: Option<String>,
    #[schemars(description = "The name of the group to send the email to.")]
    pub group_name: String,
    #[schemars(description = "Optional reply-to email address.")]
    pub reply_to: Option<String>,
    #[schemars(description = "Subject of the email.")]
    pub subject: String,
    #[schemars(description = "Body of the email.")]
    pub body: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(description = "Request to send an email using a predefined template with dynamic data.")]
pub struct SendEmailWithTemplateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(
        description = "Optional sender email address. If not provided, the default sender will be used."
    )]
    pub from: Option<String>,
    #[schemars(description = "List of recipient email addresses.")]
    pub to: Vec<String>,
    #[schemars(description = "Optional reply-to email address.")]
    pub reply_to: Option<String>,
    #[schemars(description = "Subject of the email.")]
    pub subject: String,
    #[schemars(description = "The unique name of the email template to use for this email.")]
    pub template_name: String,
    #[schemars(
        description = "A map of key-value pairs to be used as dynamic data for the email template. The keys should correspond to the placeholders defined in the template's format string."
    )]
    pub template_data: HashMap<String, String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(
    description = "Request to manage groups, including adding, removing, and updating groups."
)]
pub enum ManageGroupsRequest {
    #[schemars(description = "Request to add a new group.")]
    Add(AddGroupRequest),
    #[schemars(description = "Request to remove an existing group.")]
    Remove(RemoveGroupRequest),
    #[schemars(description = "Request to update an existing group.")]
    Update(UpdateGroupRequest),
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(description = "Request to add a new group with the specified name.")]
pub struct AddGroupRequest {
    #[schemars(description = "The unique name of the group to be added.")]
    pub name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(
    description = "Request to update an existing group, allowing for changing the group's name."
)]
pub struct UpdateGroupRequest {
    #[schemars(description = "The unique name of the group to be updated.")]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "The new name for the group, if it is being updated.")]
    pub new_name: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(description = "Request to remove an existing group by its unique name.")]
pub struct RemoveGroupRequest {
    #[schemars(description = "The unique name of the group to be removed.")]
    pub name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(
    description = "Request to manage recipients, including adding, removing, and updating recipients."
)]
pub enum ManageRecipientsRequest {
    #[schemars(description = "Request to add a new recipient.")]
    Add(AddRecipientRequest),
    #[schemars(description = "Request to remove an existing recipient.")]
    Remove(RemoveRecipientRequest),
    #[schemars(description = "Request to update an existing recipient's information.")]
    Update(UpdateRecipientRequest),
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(
    description = "Request to add a new recipient with the specified name and email address."
)]
pub struct AddRecipientRequest {
    #[schemars(description = "The name of the recipient to be added.")]
    pub name: String,
    #[schemars(description = "The email address of the recipient to be added.")]
    pub email: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(
    description = "Request to update an existing recipient's information, allowing for changing the recipient's name and/or email address."
)]
pub struct UpdateRecipientRequest {
    #[schemars(description = "The email address of the recipient to be updated.")]
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "The new name for the recipient, if it is being updated.")]
    pub new_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "The new email address for the recipient, if it is being updated.")]
    pub new_email: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(description = "Request to remove an existing recipient by their email address.")]
pub struct RemoveRecipientRequest {
    #[schemars(description = "The email address of the recipient to be removed.")]
    pub email: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(
    description = "Request to add an existing recipient to an existing group, specified by the recipient's email address and the group's name."
)]
pub struct AddRecipientToGroupRequest {
    #[schemars(description = "The name of the group to which the recipient will be added.")]
    pub group_name: String,
    #[schemars(description = "The email address of the recipient to be added to the group.")]
    pub email: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(description = "Request to get the an email template by its unique name.")]
pub struct GetEmailTemplatesRequest {
    #[schemars(description = "The unique name of the email template to retrieve.")]
    pub name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(
    description = "Request to manage email templates, including adding, removing, and updating templates."
)]
pub enum ManageTemplatesRequest {
    #[schemars(description = "Request to add a new email template.")]
    Add(AddTemplateRequest),
    #[schemars(description = "Request to remove an existing email template.")]
    Remove(RemoveTemplateRequest),
    #[schemars(description = "Request to update an existing email template.")]
    Update(UpdateTemplateRequest),
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(description = "Request to add a new email template.")]
pub struct AddTemplateRequest {
    #[schemars(description = "The unique name of the email template.")]
    pub name: String,
    #[schemars(
        description = "The format string of the email template, which may include placeholders for dynamic content."
    )]
    pub format_string: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(description = "Request to update an existing email template.")]
pub struct UpdateTemplateRequest {
    #[schemars(description = "The unique name of the email template to be updated.")]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "The new name for the email template, if it is being updated.")]
    pub new_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(
        description = "The new format string for the email template, if it is being updated. This may include placeholders for dynamic content."
    )]
    pub new_format_string: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(description = "Request to remove an existing email template.")]
pub struct RemoveTemplateRequest {
    #[schemars(description = "The unique name of the email template to be removed.")]
    pub name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(
    description = "Request to retrieve email history with optional filtering by recipient and date range."
)]
pub struct GetEmailHistoryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Optional email address to filter the email history by recipient.")]
    pub to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(
        description = "Optional start date to filter the email history. Should be in ISO 8601 format (e.g., \"2023-10-01T00:00:00\")."
    )]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(
        description = "Optional end date to filter the email history. Should be in ISO 8601 format (e.g., \"2023-10-31T23:59:59\")."
    )]
    pub end_date: Option<String>,
}

impl GetEmailHistoryRequest {
    /// Validates the request by checking that at least one field is provided and that the provided fields are in the correct format.
    pub fn is_valid(&self) -> bool {
        // If all fields are None, the request is considered invalid
        if self.to.is_none() && self.start_date.is_none() && self.end_date.is_none() {
            return false;
        }

        // Validate the 'to' field if it is provided
        if let Some(to) = &self.to
            && to.parse::<lettre::Address>().is_err()
        {
            return false;
        }

        // Validate the start and end dates if they are provided
        is_valid_start_end_time(self.start_date.as_ref(), self.end_date.as_ref())
    }
}

/// Validates that the provided start and end dates are in the correct format and that the start date is not after the end date.
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
    if let (Some(parsed_start_datetime), Some(parsed_end_datetime)) = (&parsed_start, &parsed_end)
        && parsed_start_datetime > parsed_end_datetime
    {
        return false;
    }
    // If only start date is provided, it must not be future-dated
    if let Some(parsed_start_datetime) = parsed_start
        && parsed_start_datetime > chrono::Utc::now()
    {
        return false;
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

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(description = "Request to create a new calendar event with the specified details.")]
pub struct CreateEventRequest {
    #[schemars(description = "The title of the event.")]
    pub title: String,
    #[schemars(description = "An optional description of the event.")]
    pub description: Option<String>,
    #[schemars(
        description = "The start time of the event in ISO 8601 format (e.g., \"2023-10-01T14:00:00\")."
    )]
    pub start_time: chrono::NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(
        description = "The end time of the event in ISO 8601 format (e.g., \"2023-10-01T16:00:00\")."
    )]
    pub end_time: Option<chrono::NaiveDateTime>,
    #[schemars(description = "Indicates whether the event is an all-day event.")]
    pub is_all_day: bool,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(description = "Request to list calendar events with optional filtering by date range.")]
pub struct ListEventsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(
        description = "Optional start date to filter events. Should be in ISO 8601 format (e.g., \"2023-10-01T00:00:00\")."
    )]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(
        description = "Optional end date to filter events. Should be in ISO 8601 format (e.g., \"2023-10-31T23:59:59\")."
    )]
    pub end_date: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(
    description = "Request to send an invitation for a calendar event to specified groups and/or individuals."
)]
pub struct SendEventInvitationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(
        description = "Optional sender email address. If not provided, the default sender will be used."
    )]
    pub from: Option<String>, // Optional sender email address. If not provided, the default sender will be used.
    #[schemars(
        description = "The unique identifier of the event for which the invitation is being sent."
    )]
    pub event_id: i32,
    #[schemars(
        description = "The recipients of the event invitation, which can include both groups and individuals."
    )]
    pub to: SendEventInvitationTo,
    #[schemars(description = "The subject of the event invitation email.")]
    pub subject: String,
    #[schemars(description = "The body of the event invitation email.")]
    pub body: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(
    description = "The recipients of an event invitation, which can include both groups and individuals."
)]
pub struct SendEventInvitationTo {
    #[schemars(description = "Groups to send the invitation to. The string is the group name.")]
    pub groups: Vec<String>,
    #[schemars(
        description = "Individuals to send the invitation to. The string is the individual's user name."
    )]
    pub individuals: Vec<String>,
}
