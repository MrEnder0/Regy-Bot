use scorched::*;
use self_replace;
use std::path::Path;

pub fn local_update(new_bin: &str) -> i8 {
    if !Path::new(new_bin).exists() {
        return 0;
    }

    if Path::new("updated").exists() {
        return -1;
    }

    self_replace::self_replace(new_bin).log_expect(LogImportance::Error, "Failed to update binary");
    std::fs::remove_file(new_bin).log_expect(LogImportance::Error, "Failed to remove temp binary");
    std::fs::File::create("updated").log_expect(LogImportance::Error, "Failed to create temp file");

    if !Path::new("regy_bot_update_helper.exe").exists() {
        return 1;
    }

    2
}
