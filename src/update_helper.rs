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
    std::thread::sleep(Duration::from_secs(3));

    let regy_bin = "regy_bot.exe";

    if !Path::new(regy_bin).exists() {
        let data = LogData {
            importance: LogImportance::Error,
            message: "[Update Helper] Updated Regy binary does not exist, shutting down.".to_string(),
        };
        log_this(data);
        return;
    }

    if !Path::new("updated").exists() {
        let data = LogData {
            importance: LogImportance::Warning,
            message: "[Update Helper] Regy is not in update state, shutting down.".to_string(),
        };
        log_this(data);
        return;
    }

    let data = LogData {
        importance: LogImportance::Info,
        message: "[Update Helper] Regy has finished updating restarting Regy.".to_string(),
    };
    log_this(data);
    std::fs::remove_file("updated").log_expect(LogImportance::Error, "Failed to remove updated file");
    Command::new("regy_bot.exe").spawn().log_expect(LogImportance::Error, "Failed to start regy_bot.exe");

    let data = LogData {
        importance: LogImportance::Info,
        message: "[Update Helper] Update helper has finished, closing update helper.".to_string(),
    };
    log_this(data);

    std::process::exit(0);
}