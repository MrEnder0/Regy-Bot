mod utils;

use scorched::*;
use std::{path::Path, process::Command, time::Duration};

pub const VERSION: &str = "1.5.1";

fn main() {
    // Sets logging path
    set_logging_path("temp/logs/update_helper/");

    // Waits for Regy to fully shutdown
    log_this(LogData {
        importance: LogImportance::Info,
        message: "[Update Helper] Waiting for Regy to fully shutdown.".to_string(),
    });

    std::thread::sleep(Duration::from_millis(500));

    for tries in 1..3 {
        if !Path::new("update.lock").exists() {
            if tries == 3 {
                log_this(LogData {
                    importance: LogImportance::Error,
                    message: "[Update Helper] Regy has failed to enter update state, shutting down update-helper."
                        .to_string(),
                });

                std::process::exit(0);
            }

            log_this(LogData {
                importance: LogImportance::Info,
                message: format!(
                    "[Update Helper] Regy is not in update state, retrying (Tries: {}/3)",
                    tries
                )
                .to_string(),
            });

            std::thread::sleep(Duration::from_millis(1000));
        } else {
            break;
        }
    }

    let regy_bin = "regy_bot.exe";

    if !Path::new(regy_bin).exists() {
        log_this(LogData {
            importance: LogImportance::Error,
            message: "[Update Helper] Updated Regy binary does not exist, shutting down."
                .to_string(),
        });

        std::process::exit(0);
    }

    if !Path::new("update.lock").exists() {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: "[Update Helper] Regy is not in update state, shutting down update-helper."
                .to_string(),
        });

        std::process::exit(0);
    }

    log_this(LogData {
        importance: LogImportance::Info,
        message: "[Update Helper] Regy has finished updating, cleaning up.".to_string(),
    });

    std::fs::remove_file("update.lock")
        .log_expect(LogImportance::Error, "Failed to remove update.lock file");

    // Checks if the old binary exists, if it does, it will delete it
    if Path::new("regy_update.exe").exists() {
        if !Path::new("regy_bot.exe").exists() {
            log_this(LogData {
                importance: LogImportance::Error,
                message: "[Update Helper] Updated Regy binary does not exist, shutting down all services."
                    .to_string(),
            });

            std::process::exit(0);
        }

        log_this(LogData {
            importance: LogImportance::Warning,
            message: "[Update Helper] Found update file after updating, everything else seems good, removing update binary."
                .to_string(),
        });

        std::fs::remove_file("regy_update.exe").log_expect(
            LogImportance::Error,
            "Failed to remove used update file file",
        );
    }

    Command::new("regy_bot.exe")
        .spawn()
        .log_expect(LogImportance::Error, "Failed to start regy_bot.exe");

    log_this(LogData {
        importance: LogImportance::Info,
        message: "[Update Helper] Update helper has finished, closing update helper.".to_string(),
    });

    std::process::exit(0);
}
