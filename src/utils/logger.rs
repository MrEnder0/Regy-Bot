use std::{
    fs::OpenOptions,
    io::prelude::*,
    fmt::Debug
};
use chrono::Local;

pub enum LogImportance {
    Error,
    Warning,
    Info,
    #[allow(dead_code)]
    Debug
}

pub struct LogData {
    pub importance: LogImportance,
    pub message: String,
}

pub fn log_this(data: LogData) {
    let date = Local::now();
    let formatted_time = date.format("%Y-%m-%d %H:%M:%S");

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("regy.log");

    let log_importance = {
        match data.importance {
            LogImportance::Error => "ERROR",
            LogImportance::Warning => "WARNING",
            LogImportance::Info => "INFO",
            LogImportance::Debug => "DEBUG",
        }
    };

    file.unwrap().write_all(format!("{} [{}] {}\n", formatted_time, log_importance, data.message).as_bytes()).unwrap();
}

pub trait LogExpect<T, E: Debug> {
    fn log_expect(self, msg: &str) -> T;
}

impl<T, E: Debug> LogExpect<T, E> for Result<T, E> {
    fn log_expect(self, msg: &str) -> T {
        match self {
            Ok(val) => val,
            Err(err) => {
                let data = LogData {
                    importance: LogImportance::Error,
                    message: format!("{}: {:?}", msg, err),
                };
                log_this(data);
                panic!("{}: {:?}", msg, err);
            }
        }
    }
}

impl<T> LogExpect<T, ()> for Option<T> {
    fn log_expect(self, msg: &str) -> T {
        match self {
            Some(val) => val,
            None => {
                let data = LogData {
                    importance: LogImportance::Error,
                    message: msg.to_string(),
                };
                log_this(data);
                panic!("{}", msg);
            }
        }
    }
}
