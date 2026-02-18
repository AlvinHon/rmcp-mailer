use log::LevelFilter;
use log4rs::{
    Config,
    append::file::FileAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
};

use crate::config::LoggerConfig;

pub fn init_logging(logger_config: &LoggerConfig) {
    // Define a file appender
    let file_appender = FileAppender::builder()
        // Set the log format
        .encoder(Box::new(PatternEncoder::new("{d} {l}::{m}{n}")))
        // Specify the log file path
        .build(logger_config.file_path.clone())
        .unwrap();

    // Configure the logging
    let config = Config::builder()
        .appender(Appender::builder().build("file_log", Box::new(file_appender)))
        .build(
            Root::builder()
                .appender("file_log")
                .build(LevelFilter::Info),
        ) // Set the default minimum log level to INFO
        .unwrap();

    // Initialize the logger
    log4rs::init_config(config).unwrap();
}
