use std::fs::read;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub sse_server_host: String,
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
            mailer_config: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct MailerConfig {
    pub mailer_email: String,
    pub smtp_port: u16,
    pub smtp_host: String,
    pub smtp_username: Option<String>,
    pub smtp_password: Option<String>,
}

impl MailerConfig {
    pub fn is_authenication(&self) -> bool {
        self.smtp_username.is_some() && self.smtp_password.is_some()
    }
}

impl Default for MailerConfig {
    fn default() -> Self {
        Self {
            mailer_email: "test@test.com".to_string(),
            smtp_port: 2525,
            smtp_host: "localhost".to_string(),
            smtp_username: None,
            smtp_password: None,
        }
    }
}

#[test]
fn test_toml_config() {
    let toml_str = r#"
    sse_server_host = "127.0.0.1:3000"
    [mailer_config]
    mailer_email = "test@test.com"
    smtp_port = 2525
    smtp_host = "localhost"
    "#;

    toml::from_str(toml_str).unwrap()
}
