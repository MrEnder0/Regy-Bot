use std::{
    fs::OpenOptions,
    io::prelude::*,
    fmt::Debug
};
use owo_colors::{OwoColorize, colors::{css::*}};
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

    match data.importance {
        LogImportance::Error => {
            file.unwrap().write_all(format!("{} [ERROR] {}\n", formatted_time, data.message).as_bytes()).unwrap();
            println!("{} {} {}", formatted_time, "[ERROR]".fg::<Black>().bg::<Red>(), data.message);
        },
        LogImportance::Warning => {
            file.unwrap().write_all(format!("{} [WARNING] {}\n", formatted_time, data.message).as_bytes()).unwrap();
            println!("{} {} {}", formatted_time, "[WARNING]".fg::<Black>().bg::<Yellow>(), data.message);
        },
        LogImportance::Info => {
            file.unwrap().write_all(format!("{} [INFO] {}\n", formatted_time, data.message).as_bytes()).unwrap();
            println!("{} {} {}", formatted_time, "[INFO]".fg::<Black>().bg::<LightGray>(), data.message);
        }
        LogImportance::Debug => {
            file.unwrap().write_all(format!("{} [DEBUG] {}\n", formatted_time, data.message).as_bytes()).unwrap();
            println!("{} {} {}", formatted_time, "[DEBUG]".fg::<Black>().bg::<LightBlue>(), data.message);
        }
        _ => {
            file.unwrap().write_all(format!("{} [UNKNOWN] {}\n", formatted_time, data.message).as_bytes()).unwrap();
            println!("{} {} {}", formatted_time, "[UNKNOWN]".fg::<Black>().bg::<Magenta>(), data.message);
        }
    }
}

pub trait LogExpect<T, E: Debug> {
    fn log_expect(self, importance: LogImportance, msg: &str) -> T;
}

impl<T, E: Debug> LogExpect<T, E> for Result<T, E> {
    fn log_expect(self, importance: LogImportance, msg: &str) -> T {
        match self {
            Ok(val) => val,
            Err(err) => {
                let data = LogData {
                    importance,
                    message: format!("{}: {:?}", msg, err),
                };
                log_this(data);
                panic!("{}: {:?}", msg, err);
            }
        }
    }
}

impl<T> LogExpect<T, ()> for Option<T> {
    fn log_expect(self, importance: LogImportance, msg: &str) -> T {
        match self {
            Some(val) => val,
            None => {
                let data = LogData {
                    importance,
                    message: msg.to_string(),
                };
                log_this(data);
                panic!("{}", msg);
            }
        }
    }
}