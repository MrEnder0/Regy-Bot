use chrono::Local;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub struct LogData {
    pub importance: String,
    pub message: String,
}

pub fn log_this(data: LogData) {
    let date = Local::now();
    let formatted_time = date.format("%Y-%m-%d %H:%M:%S");

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("regy.log");

    file.unwrap()
        .write_all(
            format!(
                "{} [{}] {}\n",
                formatted_time, data.importance, data.message
            )
            .as_bytes(),
        )
        .unwrap();
}
