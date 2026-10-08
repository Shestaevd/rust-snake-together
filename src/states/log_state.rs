use chrono::{DateTime, Utc};
use fixed_capacity_vec::FixedCapacityVec;
use std::fmt;
use std::fmt::Display;

enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

pub struct LogMessage {
    pub message: String,
    pub log_time: DateTime<Utc>,
    log_level: LogLevel,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogLevel::Info => write!(f, "Info"),
            LogLevel::Debug => write!(f, "Debug"),
            LogLevel::Warning => write!(f, "Warning"),
            LogLevel::Error => write!(f, "Error"),
        }
    }
}

pub struct LogState {
    pub buffer: FixedCapacityVec<LogMessage, 100>,
}

impl LogState {
    pub fn new() -> Self {
        LogState {
            buffer: FixedCapacityVec::new(),
        }
    }

    fn push_log(&mut self, message: String, log_level: LogLevel) {
        if self.buffer.is_full() {
            self.buffer.pop();
        }
        self.buffer.push(LogMessage {
            message,
            log_level,
            log_time: Utc::now(),
        });
    }

    pub fn push_info(&mut self, message: String) {
        self.push_log(message, LogLevel::Info);
    }

    pub fn push_debug(&mut self, message: String) {
        self.push_log(message, LogLevel::Debug);
    }

    pub fn push_error(&mut self, message: String) {
        self.push_log(message, LogLevel::Error);
    }

    pub fn push_warning(&mut self, message: String) {
        self.push_log(message, LogLevel::Warning);
    }

    pub fn get_logs(&self) -> String {
        self.buffer.iter().fold(String::new(), |acc, log| {
            format!(
                "{} \n [{}] : {} - {}",
                acc,
                log.log_time.to_rfc3339(),
                log.log_level,
                log.message
            )
        })
    }
}
