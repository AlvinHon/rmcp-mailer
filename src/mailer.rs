use crate::{
    config::{MailSender, MailerConfig},
    error::{MailerError, new_rmcp_error},
    request::SendEmailRequest,
};
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, message::header::ContentType,
    transport::smtp::authentication::Credentials,
};

#[derive(Debug, Clone)]
pub struct Mailer {
    config: MailerConfig,
}

impl Mailer {
    pub fn new(config: MailerConfig) -> Self {
        Self { config }
    }

    pub async fn send(&self, email_request: &SendEmailRequest) -> Result<Message, MailerError> {
        let sender = email_request
            .from
            .as_ref()
            .and_then(|from| self.config.find_sender(from))
            .unwrap_or(self.config.default_sender());

        let email = self.build_email(email_request, sender)?;
        let transport = self.build_transport(sender)?;

        // Send the email
        transport
            .send(email.clone())
            .await
            .map(|_| ())
            .map_err(MailerError::from)?;

        Ok(email)
    }

    fn build_email(
        &self,
        email_request: &SendEmailRequest,
        from: &MailSender,
    ) -> Result<Message, MailerError> {
        let from = from
            .email
            .parse::<lettre::message::Mailbox>()
            .map_err(|_| new_rmcp_error("Invalid sender email"))?;

        let mut msg_builder = Message::builder()
            .from(from)
            .subject(&email_request.subject)
            .header(ContentType::TEXT_PLAIN);

        for recipient in &email_request.to {
            msg_builder = msg_builder.to(recipient
                .parse()
                .map_err(|_| new_rmcp_error("Invalid recipient email"))?);
        }

        if let Some(reply_to) = &email_request.reply_to {
            msg_builder = msg_builder.reply_to(reply_to.parse().unwrap());
        }

        msg_builder
            .body(email_request.body.clone())
            .map_err(MailerError::from)
    }

    fn build_transport(
        &self,
        sender: &MailSender,
    ) -> Result<AsyncSmtpTransport<Tokio1Executor>, MailerError> {
        let credentials = sender
            .credentials
            .as_ref()
            .map(|creds| Credentials::new(creds.username.clone(), creds.password.clone()));

        if let Some(credentials) = credentials {
            let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&self.config.smtp_host)
                .unwrap()
                .port(self.config.smtp_port)
                .credentials(credentials)
                .build();

            return Ok(mailer);
        }

        let mailer =
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&self.config.smtp_host)
                .port(self.config.smtp_port)
                .build();

        Ok(mailer)
    }
}
