use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, message::header::ContentType,
    transport::smtp::authentication::Credentials,
};

use crate::{config::MailerConfig, error::MailerError, request::SendEmailRequest};

#[derive(Debug, Clone)]
pub struct Mailer {
    config: MailerConfig,
}

impl Mailer {
    pub fn new(config: MailerConfig) -> Self {
        Self { config }
    }

    pub async fn send(&self, email_request: &SendEmailRequest) -> Result<(), MailerError> {
        let email = self.build_email(email_request)?;
        let mailer = self.build_transport()?;

        // Send the email
        mailer
            .send(email)
            .await
            .map(|_| ())
            .map_err(MailerError::from)
    }

    fn build_email(&self, email_request: &SendEmailRequest) -> Result<Message, MailerError> {
        let mut msg_builder = Message::builder()
            .from(self.config.mailer_email.parse().unwrap())
            .to(email_request.to.parse().unwrap())
            .subject(&email_request.subject)
            .header(ContentType::TEXT_PLAIN);

        if let Some(reply_to) = &email_request.reply_to {
            msg_builder = msg_builder.reply_to(reply_to.parse().unwrap());
        }

        msg_builder
            .body(email_request.body.clone())
            .map_err(MailerError::from)
    }

    fn build_transport(&self) -> Result<AsyncSmtpTransport<Tokio1Executor>, MailerError> {
        if self.config.is_authenication() {
            let creds = Credentials::new(
                self.config.smtp_username.clone().unwrap().to_owned(),
                self.config.smtp_password.clone().unwrap().to_owned(),
            );

            let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&self.config.smtp_host)
                .unwrap()
                .port(self.config.smtp_port)
                .credentials(creds)
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
