mod utils;

use scorched::*;
use std::{path::Path, process::Command, time::Duration};

#[tokio::main]
async fn main() {
    // Wait for Regy to fully shutdown
    log_this(LogData {
        importance: LogImportance::Info,
        message: "[Update Helper] Waiting for Regy to fully shutdown.".to_string(),
    })
    .await;

    std::thread::sleep(Duration::from_secs(2));

    let regy_bin = "regy_bot.exe";

    if !Path::new(regy_bin).exists() {
        log_this(LogData {
            importance: LogImportance::Error,
            message: "[Update Helper] Updated Regy binary does not exist, shutting down."
                .to_string(),
        })
        .await;

        return
    }

    if !Path::new("update.lock").exists() {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: "[Update Helper] Regy is not in update state, shutting down.".to_string(),
        })
        .await;

        return
    }

    log_this(LogData {
        importance: LogImportance::Info,
        message: "[Update Helper] Regy has finished updating restarting Regy.".to_string(),
    })
    .await;

    std::fs::remove_file("update.lock")
        .log_expect(LogImportance::Error, "Failed to remove update.lock file");

    Command::new("regy_bot.exe")
        .spawn()
        .log_expect(LogImportance::Error, "Failed to start regy_bot.exe");

    log_this(LogData {
        importance: LogImportance::Info,
        message: "[Update Helper] Update helper has finished, closing update helper.".to_string(),
    })
    .await;

    std::process::exit(0);
}
