use std::fs::read;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server_host: String,
    pub db_config: DatabaseConfig,
    pub mailer_config: MailerConfig,
    pub logger_config: LoggerConfig,
}

impl Config {
    pub fn read_from_file() -> Self {
        let config = match read("config.toml") {
            Ok(bytes) => toml::from_str(&String::from_utf8_lossy(&bytes))
                .expect("config.toml must have valid toml format."),
            Err(_) => Config::default(),
        };

        // Validate the config
        if config.server_host.is_empty() {
            panic!("server_host must be set in the config.toml");
        }

        if config.db_config.db_path.is_empty() {
            panic!("db_config.db_path must be set in the config.toml");
        }

        if config.mailer_config.smtp_host.is_empty() {
            panic!("mailer_config.smtp_host must be set in the config.toml");
        }

        if config.mailer_config.senders.is_empty() {
            panic!(
                "mailer_config.senders must have at least one sender configured in the config.toml"
            );
        }

        if config.mailer_config.senders[0]
            .email
            .parse::<lettre::Address>()
            .is_err()
        {
            panic!(
                "mailer_config.senders[0].email (default email) must be a valid email address in the config.toml"
            );
        }

        config
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_host: "127.0.0.1:3000".to_string(),
            db_config: Default::default(),
            mailer_config: Default::default(),
            logger_config: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailerConfig {
    pub smtp_port: u16,
    pub smtp_host: String,
    pub senders: Vec<MailSender>,
}

impl MailerConfig {
    pub fn default_sender(&self) -> &MailSender {
        self.senders
            .first()
            .expect("At least one sender must be configured")
    }

    /// Finds the sender by email address and returns its credentials if available.
    ///
    /// Conditions to find a sender:
    ///
    /// 1. The sender's email must match the provided email address.
    /// 2. If the email address is not valid, it checks if the user part of the email matches.
    ///
    /// Returns `None` if no matching sender is found.
    pub fn find_sender(&self, sender: &str) -> Option<&MailSender> {
        self.senders.iter().find(|s| {
            if let Ok(sender_email) = s.email.parse::<lettre::Address>() {
                sender_email.to_string().as_str() == sender
            } else {
                let s_email_user = s.email.parse::<lettre::Address>().unwrap();
                s_email_user.user() == sender
            }
        })
    }
}

impl Default for MailerConfig {
    fn default() -> Self {
        Self {
            smtp_port: 2525,
            smtp_host: "localhost".to_string(),
            senders: vec![MailSender {
                email: "test@test.com".to_string(),
                credentials: None,
            }],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailSender {
    pub email: String,
    pub credentials: Option<SMTPCredentials>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMTPCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub db_path: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            db_path: "mailer.db".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggerConfig {
    pub config_file_path: String,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            config_file_path: "log4rs.yaml".to_string(),
        }
    }
}

#[test]
fn test_toml_config() {
    let toml_str = r#"
    server_host = "127.0.0.1:3000"
    [db_config]
    db_path = "mailer.db"
    [mailer_config]
    smtp_port = 2525
    smtp_host = "localhost"
    [[mailer_config.senders]]
    email = "test@test.com"
    [mailer_config.senders.credentials]
    username = "testuser"
    password = "testpassword"
    [[mailer_config.senders]]
    email = "test2@test.com"
    [logger_config]
    config_file_path = "log4rs.yaml"
    "#;

    let config = toml::from_str::<Config>(toml_str).unwrap();
    assert_eq!(config.server_host, "127.0.0.1:3000");

    // check [db_config]
    assert_eq!(config.db_config.db_path, "mailer.db");

    // check [mailer_config]
    assert_eq!(config.mailer_config.smtp_port, 2525);
    assert_eq!(config.mailer_config.smtp_host, "localhost");
    assert_eq!(config.mailer_config.senders.len(), 2);

    let first_sender = config.mailer_config.default_sender();
    assert_eq!(first_sender.email, "test@test.com");
    assert_eq!(
        first_sender.credentials.as_ref().unwrap().username,
        "testuser"
    );
    assert_eq!(
        first_sender.credentials.as_ref().unwrap().password,
        "testpassword"
    );
    let second_sender = config.mailer_config.senders.get(1).unwrap();

    assert_eq!(second_sender.email, "test2@test.com");
    assert!(second_sender.credentials.is_none());

    assert_eq!(config.logger_config.config_file_path, "log4rs.yaml");
}
