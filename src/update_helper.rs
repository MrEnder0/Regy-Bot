mod utils;

use std::{
    process::Command,
    time::Duration,
    path::Path
};

use crate::utils::logger::*;

fn main() {
    //Wait for Regy to fully shutdown
    let data = LogData {
        importance: LogImportance::Info,
        message: "[Update Helper] Waiting for Regy to fully shutdown.".to_string(),
    };
    log_this(data);
    println!("[Update Helper INFO] Waiting for Regy to fully shutdown.");
    std::thread::sleep(Duration::from_secs(3));

    let regy_bin = "regy_bot.exe";

    if !Path::new(regy_bin).exists() {
        let data = LogData {
            importance: LogImportance::Error,
            message: "[Update Helper] Regy binary does not exist, shutting down.".to_string(),
        };
        log_this(data);
        println!("[Update Helper ERROR] Regy binary does not exist, shutting down.");
        return;
    }

    if !Path::new("updated").exists() {
        let data = LogData {
            importance: LogImportance::Warning,
            message: "[Update Helper] Regy is not in update state, shutting down.".to_string(),
        };
        log_this(data);
        println!("[Update Helper WARNING] Regy is not in update state, shutting down.");
        return;
    }

    println!("[Update Helper INFO] Regy has finished updating restarting Regy.");
    let data = LogData {
        importance: LogImportance::Info,
        message: "[Update Helper] Regy has finished updating restarting Regy.".to_string(),
    };
    log_this(data);
    std::fs::remove_file("updated").log_expect("Failed to remove updated file");
    Command::new("regy_bot.exe").spawn().log_expect("Failed to start regy_bot.exe");

    println!("[Update Helper INFO] Update helper has finished, closing update helper.");
    let data = LogData {
        importance: LogImportance::Info,
        message: "[Update Helper] Update helper has finished, closing update helper.".to_string(),
    };
    log_this(data);

    std::process::exit(0);
}