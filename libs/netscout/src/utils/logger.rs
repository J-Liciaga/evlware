use log::{
    Level,
    LevelFilter,
    Metadata,
    Record,
};
use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use anyhow::Result;

pub struct Logger;

impl log::Log for Logger {
    fn enabled(
        &self,
        metadata: &Metadata,
    ) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(
        &self,
        record: &Record,
    ) {
        if self.enabled(record.metadata()) {
            let now = Local::now();
            let message = format!(
                "{} - {} - {}",
                now.format("%Y-%m-%d %H:%M:%S"),
            );

            println!("{}", message);

            if let Err(e) = self.log_to_file(&message) {
                eprintln!(
                    "Faied to write to log file: {}", 
                    e
                );
            }
        }
    }

    fn flush() {}
}

impl Logger {
    pub fn init() -> Result<()> {
        let logger = Box::new(SecurityLogger);
        log::set_boxed_logger(logger)?;
        log::set_max_level(LevelFilter::Debug);
        Ok(())
    }

    fn log_to_file(&self, message: &str) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("security_test.log")?;
        
        writeln!(file, "{}", message)?;
        Ok(())
    }
}

// Convenience macros for logging
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        log::info!($($arg)*);
    }
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        log::warn!($($arg)*);
    }
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        log::error!($($arg)*);
    }
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        log::debug!($($arg)*);
    }
}