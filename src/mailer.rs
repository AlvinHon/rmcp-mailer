use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, message::header::ContentType,
};
// use lettre::transport::smtp::authentication::Credentials;

use crate::{error::MailerError, request::SendEmailRequest};

// TODO
const MAILER_FROM: &str = "me <mailer@domain.tld>";
const SMTP_PORT: u16 = 2525;
const SMTP_HOST: &str = "localhost";
//const SMTP_USERNAME: &str = "username";
//const SMTP_PASSWORD: &str = "password";

pub struct Mailer {
    // TODO
}

impl Mailer {
    pub async fn send(email_request: &SendEmailRequest) -> Result<(), MailerError> {
        let email = Self::build_email(email_request)?;
        let mailer = Self::build_transport()?;

        // Send the email
        mailer
            .send(email)
            .await
            .map(|_| ())
            .map_err(MailerError::from)
    }

    fn build_email(email_request: &SendEmailRequest) -> Result<Message, MailerError> {
        let mut msg_builder = Message::builder()
            .from(MAILER_FROM.parse().unwrap())
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

    fn build_transport() -> Result<AsyncSmtpTransport<Tokio1Executor>, MailerError> {
        // let creds = Credentials::new(SMTP_USERNAME.to_owned(), SMTP_PASSWORD.to_owned());

        // Open a remote connection to smtp server

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(SMTP_HOST)
            .port(SMTP_PORT)
            // .credentials(creds)
            .build();

        Ok(mailer)
    }
}
