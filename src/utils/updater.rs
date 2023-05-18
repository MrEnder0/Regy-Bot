use std::path::Path;
use self_replace;

use crate::utils::log_on_error::LogExpect;

pub fn local_update(new_bin: &str) -> i8 {
    if !Path::new(new_bin).exists() {
        return 0;
    }

    self_replace::self_replace(&new_bin).log_expect("Failed to update binary");
    std::fs::remove_file(&new_bin).log_expect("Failed to remove temp binary");
    std::thread::sleep(std::time::Duration::from_secs(1));

    if !Path::new("regy_bot_update_helper.exe").exists() {
        return 1;
    }

    return 2;
}