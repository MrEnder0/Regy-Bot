use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

use crate::utils::toml::read_config;

const IPM: Lazy<Mutex<HashMap<u64, u16>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub struct IpmStruct {}

impl IpmStruct {
    pub fn get_server(server_id: u64) -> u16 {
        let binding = IPM;
        let guard = binding.lock().unwrap();

        guard[&server_id]
    }
    pub fn set_server(server_id: u64, value: u16) {
        let binding = IPM;
        let mut guard = binding.lock().unwrap();
        guard.insert(server_id, value);
    }
    pub fn get_overflow() -> Vec<u64> {
        let mut overflow: Vec<u64> = Vec::new();
        let binding = IPM;
        let guard = binding.lock().unwrap();
        for (key, value) in guard.iter() {
            if value > &read_config().global.max_activity_influx {
                overflow.push(*key);
            }
        }

        overflow
    }
    pub fn increment_server(server_id: u64) {
        let binding = IPM;
        let mut guard = binding.lock().unwrap();
        if !guard.contains_key(&server_id) {
            guard.insert(server_id, 1);
        } else {
            let original = guard[&server_id];
            guard.insert(server_id, original + 1);
        }
    }
    pub fn global_reset() {
        IPM.lock().unwrap().clear();
    }
}
