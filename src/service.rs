use std::{sync::Arc, vec};

use lettre::Message;
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
    model::{recipient::Recipient, template::Template},
    request::{
        CreateEventRequest, GetEmailHistoryRequest, ManageGroupsRequest, ManageRecipientsRequest,
        ManageTemplatesRequest, SendEmailRequest, SendEmailWithTemplateRequest,
        SendEventInvitationRequest, SendGroupEmailRequest, is_valid_start_end_time,
        parse_start_end_time,
    },
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
        let sent_message = self.mailer.send(&email_request).await?;

        // Save the recipient record in the database
        let recipient_ids = self
            .save_recipient_record(&sent_message)
            .await
            .expect("Failed to save recipient record");

        // Save email record with recipient IDs
        self.save_email_record_with_recipient_ids(
            email_request.subject,
            email_request.body,
            recipient_ids,
        )
        .await
        .expect("Failed to save email record");

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
            from: email_request.from,
            to,
            reply_to: email_request.reply_to,
            subject: email_request.subject,
            body: email_request.body,
        };

        let sent_message = self.mailer.send(&request).await?;

        // Save the recipient record in the database
        let recipient_ids = self
            .save_recipient_record(&sent_message)
            .await
            .expect("Failed to save recipient record");

        // Save email record with recipient IDs
        self.save_email_record_with_recipient_ids(request.subject, request.body, recipient_ids)
            .await
            .expect("Failed to save email record");

        Ok(CallToolResult::success(vec![Content::text(
            "Email sent to group successfully!",
        )]))
    }

    #[tool(description = "Send an email with template")]
    async fn send_email_with_template(
        &self,
        #[tool(aggr)] email_request: SendEmailWithTemplateRequest,
    ) -> Result<CallToolResult, rmcp::Error> {
        let mut db = self.db.lock().await;
        let res_template = db
            .find_template_by_name(email_request.template_name.clone())
            .map_err(|_| new_rmcp_error("Template not found"))?;

        let body = res_template
            .format(email_request.template_data.clone())
            .map_err(|e| new_rmcp_error(&e))?;

        let request = SendEmailRequest {
            from: email_request.from,
            to: email_request.to,
            reply_to: email_request.reply_to,
            subject: email_request.subject,
            body,
        };

        let sent_message = self.mailer.send(&request).await?;

        // Save the recipient record in the database
        let recipient_ids = self
            .save_recipient_record(&sent_message)
            .await
            .expect("Failed to save recipient record");

        // Save email record with recipient IDs
        self.save_email_record_with_recipient_ids(request.subject, request.body, recipient_ids)
            .await
            .expect("Failed to save email record");

        Ok(CallToolResult::success(vec![Content::text(
            "Email sent with template successfully!",
        )]))
    }

    #[tool(
        description = "Describe the phone book. It includes the information about the recipients and groups"
    )]
    async fn describe_phone_book(&self) -> Result<CallToolResult, rmcp::Error> {
        let (recipients, groups) = {
            let mut db = self.db.lock().await;
            let recipients = db.list_recipients()?;
            let groups = db.list_groups()?;

            (recipients, groups)
        };

        let mut result = recipients
            .into_iter()
            .map(|r| Content::text(format!("Recipient: {r:?}")))
            .collect::<Vec<_>>();
        result.extend(
            groups
                .into_iter()
                .map(|g| Content::text(format!("Group: {g:?}")))
                .collect::<Vec<_>>(),
        );

        Ok(CallToolResult::success(result))
    }

    #[tool(description = "Manage mail groups: add, remove, update")]
    async fn manage_mail_group(
        &self,
        #[tool(aggr)] manage_group_request: ManageGroupsRequest,
    ) -> Result<CallToolResult, rmcp::Error> {
        let mut db = self.db.lock().await;

        let result_message = match manage_group_request {
            ManageGroupsRequest::Add(add_request) => {
                db.new_group(add_request.name)?;

                vec![Content::text("Group added successfully!")]
            }
            ManageGroupsRequest::Remove(remove_request) => {
                let group_id = db
                    .find_group_by_name(remove_request.name.clone())
                    .map(|g| g.id)
                    .map_err(|_| new_rmcp_error("Group not found"))?;
                db.remove_group(group_id)?;

                vec![Content::text("Group removed successfully!")]
            }
            ManageGroupsRequest::Update(update_request) => {
                let group = db
                    .find_group_by_name(update_request.name.clone())
                    .map(|g| g.id)
                    .map_err(|_| new_rmcp_error("Group not found"))?;
                let new_name = update_request.new_name.unwrap_or(update_request.name);
                db.update_group(group, new_name)?;

                vec![Content::text("Group updated successfully!")]
            }
        };

        Ok(CallToolResult::success(result_message))
    }

    #[tool(description = "Recipient management: add, remove, update")]
    async fn manage_recipient(
        &self,
        #[tool(aggr)] manage_recipient_request: ManageRecipientsRequest,
    ) -> Result<CallToolResult, rmcp::Error> {
        let mut db = self.db.lock().await;

        let result_message = match manage_recipient_request {
            ManageRecipientsRequest::Add(add_request) => {
                db.new_recipient(add_request.name, add_request.email)?;

                vec![Content::text("Recipient added successfully!")]
            }
            ManageRecipientsRequest::Remove(remove_request) => {
                let recipient_id = db
                    .find_recipient_by_email(remove_request.email.clone())
                    .map(|r| r.id)
                    .map_err(|_| new_rmcp_error("Recipient not found"))?;
                db.remove_recipient(recipient_id)?;

                vec![Content::text("Recipient removed successfully!")]
            }
            ManageRecipientsRequest::Update(update_request) => {
                let recipient = db
                    .find_recipient_by_email(update_request.email.clone())
                    .map(|r| Recipient {
                        id: r.id,
                        name: update_request.new_name.unwrap_or(r.name),
                        email: update_request.new_email.unwrap_or(r.email),
                        status: r.status,
                    })
                    .map_err(|_| new_rmcp_error("Recipient not found"))?;
                db.update_recipient(recipient.id, recipient.name, recipient.email)?;

                vec![Content::text("Recipient updated successfully!")]
            }
        };

        Ok(CallToolResult::success(result_message))
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

    #[tool(description = "Describe email templates.")]
    async fn describe_email_template(&self) -> Result<CallToolResult, rmcp::Error> {
        let templates = {
            let mut db = self.db.lock().await;
            db.list_templates()?
        };

        let result = templates
            .into_iter()
            .map(|t| Content::text(format!("Template: {t:?}")))
            .collect::<Vec<_>>();

        Ok(CallToolResult::success(result))
    }

    #[tool(description = "Get email template")]
    async fn get_email_template(
        &self,
        #[tool(param)]
        #[schemars(description = "Template name")]
        template_name: String,
    ) -> Result<CallToolResult, rmcp::Error> {
        let template = self
            .db
            .lock()
            .await
            .find_template_by_name(template_name.clone())
            .map_err(|_| new_rmcp_error("Template not found"))?;

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Template: {}",
            template.format_string
        ))]))
    }

    #[tool(description = "Manage email template: add, remove, update")]
    async fn manage_email_template(
        &self,
        #[tool(aggr)] manage_template_request: ManageTemplatesRequest,
    ) -> Result<CallToolResult, rmcp::Error> {
        let mut db = self.db.lock().await;

        let result_message = match manage_template_request {
            ManageTemplatesRequest::Add(add_request) => {
                db.new_template(add_request.name, add_request.format_string)?;

                vec![Content::text("Email template added successfully!")]
            }
            ManageTemplatesRequest::Remove(remove_request) => {
                let template_id = db
                    .find_template_by_name(remove_request.name.clone())
                    .map(|t| t.id)
                    .map_err(|_| new_rmcp_error("Template not found"))?;

                db.remove_template(template_id)?;

                vec![Content::text("Email template removed successfully!")]
            }
            ManageTemplatesRequest::Update(update_request) => {
                let template = db
                    .find_template_by_name(update_request.name.clone())
                    .map(|t| Template {
                        id: t.id,
                        name: update_request.new_name.unwrap_or(t.name),
                        format_string: update_request.new_format_string.unwrap_or(t.format_string),
                    })
                    .map_err(|_| new_rmcp_error("Template not found"))?;

                db.update_template(template.id, template.name, template.format_string)?;

                vec![Content::text("Email template updated successfully!")]
            }
        };

        Ok(CallToolResult::success(result_message))
    }

    #[tool(description = "Get email records filter by recipients or group by time range")]
    async fn get_email_records(
        &self,
        #[tool(aggr)] get_email_history_request: GetEmailHistoryRequest,
    ) -> Result<CallToolResult, rmcp::Error> {
        if !get_email_history_request.is_valid() {
            return Err(rmcp::Error::from(new_rmcp_error(
                "Invalid request: At least one filter must be provided (to, start_date, end_date)",
            )));
        }

        let mut db = self.db.lock().await;

        let start_end_time = parse_start_end_time(
            get_email_history_request.start_date.as_ref(),
            get_email_history_request.end_date.as_ref(),
        );

        let recipient_id = get_email_history_request.to.and_then(|recipient_email| {
            db.find_recipient_by_email(recipient_email)
                .ok()
                .map(|recipient| recipient.id)
        });

        let result = db
            .list_email_records_by_criteria(start_end_time, recipient_id)
            .map_err(|e| new_rmcp_error(&format!("Failed to list email records: {}", e)))?
            .into_iter()
            .map(|r| Content::text(format!("Email Record: {r:?}")))
            .collect::<Vec<_>>();

        Ok(CallToolResult::success(result))
    }

    #[tool(description = "Create an event in the calendar")]
    async fn create_event(
        &self,
        #[tool(aggr)] event_request: CreateEventRequest,
    ) -> Result<CallToolResult, rmcp::Error> {
        let mut db = self.db.lock().await;

        let new_event = db.add_event(
            event_request.title,
            event_request.description,
            event_request.start_time,
            event_request.end_time,
            event_request.is_all_day,
        )?;

        if let Ok(overlapping_events) =
            db.list_events(event_request.start_time, event_request.end_time)
        {
            let overlapping_events = overlapping_events
                .iter()
                .filter(|e| e.id != new_event.id)
                .collect::<Vec<_>>();

            return Ok(CallToolResult::success(vec![Content::text(format!(
                "Event created successfully: {new_event:?}. Overlapping events: {overlapping_events:?}"
            ))]));
        }

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Event created successfully: {new_event:?}"
        ))]))
    }

    #[tool(description = "List events in the calendar")]
    async fn list_events(
        &self,
        #[tool(param)]
        #[schemars(description = "Start date in RFC3339 format")]
        start_date: Option<String>,
        #[tool(param)]
        #[schemars(description = "End date in RFC3339 format")]
        end_date: Option<String>,
    ) -> Result<CallToolResult, rmcp::Error> {
        let mut db = self.db.lock().await;

        if !is_valid_start_end_time(start_date.as_ref(), end_date.as_ref()) {
            return Err(rmcp::Error::from(new_rmcp_error(
                "Invalid request: start_date and end_date must be valid RFC3339 dates with reasonable range",
            )));
        }

        let start_end_time = parse_start_end_time(start_date.as_ref(), end_date.as_ref());

        let events = match start_end_time {
            Some((start, end)) => db.list_events(start, Some(end))?,
            None => db.list_events(chrono::NaiveDateTime::MIN, None)?,
        };

        let result = events
            .into_iter()
            .map(|e| Content::text(format!("Event: {e:?}")))
            .collect::<Vec<_>>();

        Ok(CallToolResult::success(result))
    }

    #[tool(description = "Send event invitation to a recipient or group")]
    async fn send_event_invitation(
        &self,
        #[tool(aggr)] invitation_request: SendEventInvitationRequest,
    ) -> Result<CallToolResult, rmcp::Error> {
        let mut db = self.db.lock().await;

        let event = db
            .find_event_by_id(invitation_request.event_id)
            .map_err(|_| new_rmcp_error("Event not found"))?;

        let recipients = invitation_request
            .to
            .groups
            .iter()
            .filter_map(|group_name| {
                db.find_group_by_name(group_name.clone())
                    .ok()
                    .and_then(|g| db.find_recipients_by_group_id(g.id).ok())
                    .map(|recipients| recipients.into_iter().map(|r| r.email).collect::<Vec<_>>())
            })
            .flatten()
            .chain(invitation_request.to.individuals)
            .collect();

        // send invitations via email
        let email_request = SendEmailRequest {
            from: invitation_request.from,
            to: recipients,
            reply_to: None,
            subject: invitation_request.subject,
            body: invitation_request.body,
        };

        let sent_message = self.mailer.send(&email_request).await?;

        // Save the recipient record in the database
        let recipient_ids = self
            .save_recipient_record(&sent_message)
            .await
            .expect("Failed to save recipient record");

        // Save email record with recipient IDs
        self.save_email_record_with_recipient_ids(
            email_request.subject,
            email_request.body,
            recipient_ids.clone(),
        )
        .await
        .expect("Failed to save email record");

        // Set event attendees in the database
        self.save_event_attendee(event.id, recipient_ids)
            .await
            .expect("Failed to save event attendee");

        Ok(CallToolResult::success(vec![Content::text(
            "Event invitations sent successfully!",
        )]))
    }

    /// Save recipient records in the database for each email in the sent message.
    /// Returns a vector of recipient IDs.
    async fn save_recipient_record(&self, sent_message: &Message) -> Result<Vec<i32>, rmcp::Error> {
        let mut recipient_ids = Vec::new();
        let mut db = self.db.lock().await;

        for email in sent_message.envelope().to() {
            let email_str = email.to_string();
            let new_recipient_id = match db.find_recipient_by_email(email_str.clone()) {
                Ok(recipient) => recipient.id, // If recipient exists, use their ID
                Err(_) => {
                    // If recipient does not exist, create a new one
                    let recipient = db
                        .new_recipient(email.user().to_string(), email_str)
                        .map_err(|e| new_rmcp_error(&format!("Failed to save recipient: {}", e)))?;
                    recipient.id
                }
            };
            recipient_ids.push(new_recipient_id);
        }

        Ok(recipient_ids)
    }

    async fn save_email_record_with_recipient_ids(
        &self,
        email_subject: String,
        email_body: String,
        recipient_ids: Vec<i32>,
    ) -> Result<(), rmcp::Error> {
        let mut db = self.db.lock().await;

        let email_record = db.add_email_record(email_subject, email_body)?;
        for recipient_id in recipient_ids {
            db.add_recipient_email_record(email_record.id, recipient_id)?;
        }
        Ok(())
    }

    async fn save_event_attendee(
        &self,
        event_id: i32,
        recipient_ids: Vec<i32>,
    ) -> Result<(), rmcp::Error> {
        let mut db = self.db.lock().await;

        for recipient_id in recipient_ids {
            db.add_event_attendee(event_id, recipient_id)
                .map_err(|e| new_rmcp_error(&format!("Failed to add event attendee: {}", e)))?;
        }
        Ok(())
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
