use std::{fs::OpenOptions, str::FromStr};

pub use log::*;
use simplelog::{CombinedLogger, ConfigBuilder, WriteLogger};
pub fn init_log() {
    let log_level = std::env::var("LOG_LEVEL").unwrap_or("INFO".to_string());
    let log_level = LevelFilter::from_str(&log_level).unwrap_or(LevelFilter::Info);

    let log_file = std::env::var("LOG_FILE").unwrap_or("./app.log".to_string());
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)
        .unwrap();

    let config = ConfigBuilder::new().build();
    CombinedLogger::init(vec![
        WriteLogger::new(log_level, config.clone(), log_file),
        WriteLogger::new(log_level, config, std::io::stdout()),
    ])
    .unwrap();
}
