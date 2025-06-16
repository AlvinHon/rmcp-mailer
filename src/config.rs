use std::fs::read;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub sse_server_host: String,
    pub db_config: DatabaseConfig,
    pub mailer_config: MailerConfig,
}

impl Config {
    pub fn read_from_file() -> Self {
        match read("config.toml") {
            Ok(bytes) => toml::from_str(&String::from_utf8_lossy(&bytes))
                .expect("config.toml must have valid toml format."),
            Err(_) => Config::default(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            sse_server_host: "127.0.0.1:3000".to_string(),
            db_config: Default::default(),
            mailer_config: Default::default(),
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
    pub fn default_sender(&self) -> Option<&MailSender> {
        self.senders.first()
    }

    pub fn find_sender(&self, username: &str) -> Option<&SMTPCredentials> {
        self.senders
            .iter()
            .filter_map(|sender| sender.credentials.as_ref())
            .find(|user| user.username == username)
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

#[test]
fn test_toml_config() {
    let toml_str = r#"
    sse_server_host = "127.0.0.1:3000"
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
    "#;

    let config = toml::from_str::<Config>(toml_str).unwrap();
    assert_eq!(config.sse_server_host, "127.0.0.1:3000");

    // check [db_config]
    assert_eq!(config.db_config.db_path, "mailer.db");

    // check [mailer_config]
    assert_eq!(config.mailer_config.smtp_port, 2525);
    assert_eq!(config.mailer_config.smtp_host, "localhost");
    assert_eq!(config.mailer_config.senders.len(), 2);

    assert!(config.mailer_config.default_sender().is_some());
    let first_sender = config.mailer_config.default_sender().unwrap();
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
}
