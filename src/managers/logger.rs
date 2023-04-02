use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono::prelude::*;

pub struct LogData {
    pub importance: String,
    pub message: String,
}

pub fn log_this(data: LogData) {
    let current_time = Local::now();
    let formatted_time = current_time.format("%Y-%m-%d %H:%M:%S").to_string();

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("regy.log");

    file.unwrap().write_all(format!("{} [{}] {}\n", formatted_time, data.importance, data.message).as_bytes()).unwrap();
}