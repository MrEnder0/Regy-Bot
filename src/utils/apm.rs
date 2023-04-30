use std::sync::atomic::Ordering;

use crate::APM;

pub fn update_apm() {
    APM.store(APM.load(Ordering::SeqCst) + 1, Ordering::SeqCst);
}

pub fn check_apm_lock() {
    if APM.load(Ordering::SeqCst) >= 50 {
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

pub fn init_apm_clock() {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        APM.store(0, Ordering::SeqCst);
    }
}