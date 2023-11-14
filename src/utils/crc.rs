use super::config::regex::non_async_list_regex;
use regex::Regex;
use std::{collections::BTreeMap, sync::Mutex};

static CRC: Mutex<BTreeMap<u64, Server>> = Mutex::new(BTreeMap::new());

pub struct Server {
    pub hash: String,
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

        let server = match guard.get(&server_id) {
            Some(x) => Some(x),
            None => {
                Self::build_server_cache(server_id);
                guard.get(&server_id)
            }
        }
        .unwrap();

        // Loads the data from the cache
        let hash = &server.hash;
        let regex = server.regex.clone();

        Server {
            hash: hash.to_string(),
            regex,
        }
    }
    pub fn build_server_cache(server_id: u64) {
        let binding = &CRC;
        let mut guard = binding.lock().unwrap();

        // Clears the cache if it exists before it builds
        if guard.iter().any(|x| x.0 == &server_id) {
            guard.remove(&server_id);
        }

        let hash = format!(
            "{:x}",
            md5::compute(
                non_async_list_regex(server_id.to_string())
                    .iter()
                    .map(|x| x.phrase.clone())
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        );
        let regex = non_async_list_regex(server_id.to_string())
            .iter()
            .map(|x| x.phrase.clone())
            .collect::<Vec<String>>();
        let mut compiled_regex = Vec::new();

        for regex in regex {
            compiled_regex.push(Regex::new(&regex).unwrap());
        }

        // Inserts server regex cache into the cache
        guard.insert(
            server_id,
            Server {
                hash: hash.clone(),
                regex: compiled_regex,
            },
        );
    }
    pub fn check_cache(server_id: u64) -> bool {
        let binding = &CRC;
        let guard = binding.lock().unwrap();

        // Checks if server exists in cache
        if guard.iter().any(|x| x.0 == &server_id) {
            let comparison_hash = format!(
                "{:x}",
                md5::compute(
                    non_async_list_regex(server_id.to_string())
                        .iter()
                        .map(|x| x.phrase.clone())
                        .collect::<Vec<String>>()
                        .join("\n"),
                ),
            );

            let hash = &guard.iter().find(|x| x.0 == &server_id).unwrap().1.hash;

            // Validates the cache by checking if the hashes match
            if comparison_hash == *hash {
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
                guard.remove(&data);
            }
            CacheLevel::Global => {
                guard.clear();
            }
        }
    }
}
