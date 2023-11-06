use futures::executor::block_on;
use regex::Regex;
use std::{collections::HashMap, sync::Mutex};

use super::config::regex::list_regex;

static CRC: Mutex<Vec<HashMap<u64, Server>>> = Mutex::new(Vec::new());

pub struct Server {
    pub hash: u64,
    pub regex: Vec<Regex>,
}

pub enum CacheLevel<U64> {
    Server { data: U64 },
    Global,
}

pub struct CrcStruct {}

impl CrcStruct {
    pub fn load_server_cache(server_id: u64) -> Server {
        let binding = &CRC;
        let guard = binding.lock().unwrap();

        // Loads the data from the cache
        let hash = guard
            .iter()
            .find(|x| x.contains_key(&server_id))
            .unwrap()
            .get(&server_id)
            .unwrap()
            .hash;
        let regex = guard
            .iter()
            .find(|x| x.contains_key(&server_id))
            .unwrap()
            .get(&server_id)
            .unwrap()
            .regex
            .clone();

        Server { hash, regex }
    }
    pub fn build_server_cache(server_id: u64) {
        let binding = &CRC;
        let mut guard = binding.lock().unwrap();

        // Clears the cache if it exists before it builds
        Self::clear_cache(CacheLevel::Server { data: server_id });

        let hash = format!(
            "{:x}",
            md5::compute(
                block_on(list_regex(server_id.to_string()))
                    .unwrap()
                    .iter()
                    .map(|x| x.phrase.clone())
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        );
        let regex = block_on(list_regex(server_id.to_string()))
            .unwrap()
            .iter()
            .map(|x| x.phrase.clone())
            .collect::<Vec<String>>();
        let mut compiled_regex = Vec::new();

        for regex in regex {
            compiled_regex.push(Regex::new(&regex).unwrap());
        }

        // Inserts server regex cache into the cache
        guard.push(HashMap::new());
        guard.last_mut().unwrap().insert(
            server_id,
            Server {
                hash: hash.parse::<u64>().unwrap(),
                regex: compiled_regex,
            },
        );
    }
    pub fn check_cache(server_id: u64) -> bool {
        let binding = &CRC;
        let guard = binding.lock().unwrap();

        // Checks if the cache is empty
        if guard.iter().any(|x| x.contains_key(&server_id)) {
            // Validates cache
            let comparison_hash = format!(
                "{:x}",
                md5::compute(
                    block_on(list_regex(server_id.to_string()))
                        .unwrap()
                        .iter()
                        .map(|x| x.phrase.clone())
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            );
            let hash = guard
                .iter()
                .find(|x| x.contains_key(&server_id))
                .unwrap()
                .get(&server_id)
                .unwrap()
                .hash;

            if comparison_hash == hash.to_string() {
                return true;
            }
        }
        false
    }
    pub fn clear_cache(level: CacheLevel<u64>) {
        let binding = &CRC;
        let mut guard = binding.lock().unwrap();

        // Clears the cache
        match level {
            CacheLevel::Server { data } => {
                guard.iter_mut().for_each(|x| {
                    if x.contains_key(&data) {
                        x.remove(&data);
                    }
                });
            }
            CacheLevel::Global => {
                guard.clear();
            }
        }
    }
}
