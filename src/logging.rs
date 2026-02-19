use std::path::Path;

use log::LevelFilter;
use log4rs::{
    Config,
    append::file::FileAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
};

use crate::config::LoggerConfig;

pub fn init_logging(logger_config: &LoggerConfig) {
    // check if the log4rs configuration file exists
    if Path::new(&logger_config.config_file_path).exists() {
        log4rs::init_file(&logger_config.config_file_path, Default::default())
            .expect("Failed to initialize logging from config file");
    } else {
        // If the config file doesn't exist, use the default configuration
        log4rs::init_config(default_config()).expect("Failed to initialize default logging");
    }
}

fn default_config() -> Config {
    // Define a file appender
    let file_appender = FileAppender::builder()
        // Set the log format
        .encoder(Box::new(PatternEncoder::new("{d} {l}::{m}{n}")))
        // Specify the log file path
        .build("logs/mailer.log")
        .unwrap();

    // Configure the logging
    Config::builder()
        .appender(Appender::builder().build("file_log", Box::new(file_appender)))
        .build(
            Root::builder()
                .appender("file_log")
                .build(LevelFilter::Info),
        ) // Set the default minimum log level to INFO
        .unwrap()
}
