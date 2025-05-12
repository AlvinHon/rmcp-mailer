#[derive(Debug, Clone)]
pub struct MailerError {
    pub message: String,
}

impl std::fmt::Display for MailerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mailer error: {}", self.message)
    }
}

impl std::error::Error for MailerError {}

impl From<lettre::error::Error> for MailerError {
    fn from(error: lettre::error::Error) -> Self {
        MailerError {
            message: format!("Mailer error: {}", error),
        }
    }
}

impl From<lettre::transport::smtp::Error> for MailerError {
    fn from(error: lettre::transport::smtp::Error) -> Self {
        MailerError {
            message: format!("Mailer error: {}", error),
        }
    }
}

impl From<diesel::result::Error> for MailerError {
    fn from(error: diesel::result::Error) -> Self {
        MailerError {
            message: format!("Database error: {}", error),
        }
    }
}

impl From<MailerError> for rmcp::Error {
    fn from(error: MailerError) -> Self {
        rmcp::Error::new(rmcp::model::ErrorCode::INTERNAL_ERROR, error.message, None)
    }
}

pub fn new_rmcp_error(message: &str) -> MailerError {
    MailerError {
        message: format!("RMCP error: {}", message),
    }
}
