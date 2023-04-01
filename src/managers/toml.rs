use std::{collections::HashMap, hash::Hash};

use uuid::Uuid;
use base64::{Engine as _, engine::general_purpose};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub token: String,
    pub staff: Vec<String>,
    pub log_channel: u64,
    // pub block_phrases: Vec<String>,
    pub block_phrases: HashMap<Uuid, String>
}

pub fn gen_config() {
    let mut phr = HashMap::new();
    phr.insert(Uuid::new_v4(), general_purpose::STANDARD_NO_PAD.encode("regy test phrase"));
    let config = Config {
        token: "token".to_string(),
        staff: vec!["000000000000000000".to_string()],
        log_channel: 000000000000000000,
        // block_phrases: vec![general_purpose::STANDARD_NO_PAD.encode("regy test phrase")],
        block_phrases: phr
    };
    //write to file
    let toml = toml::to_string(&config).unwrap();
    std::fs::write("config.toml", toml).unwrap();
}

pub fn get_config() -> Config {
    let toml = std::fs::read_to_string("config.toml").unwrap();
    let config: Config = toml::from_str(&toml).unwrap();
    config
}

pub fn add_block_phrase (phrase: String) {
    let mut config = get_config();
    config.block_phrases.insert(Uuid::new_v4(),general_purpose::STANDARD_NO_PAD.encode(phrase));
    let toml = toml::to_string(&config).unwrap();
    std::fs::write("config.toml", toml).unwrap();
}

pub fn list_block_phrases () -> HashMap<Uuid, String> {
    let config = get_config();
    // let mut phrases: Vec<String> = Vec::new();
    let mut phrases: HashMap<Uuid, String> = HashMap::new();

    for (id, phrase) in config.block_phrases {
        let phrase = String::from_utf8(general_purpose::STANDARD_NO_PAD.decode(&phrase).unwrap()).unwrap();
        let phrase = &phrase[..phrase.len() - 1];
        phrases.insert(id, phrase.to_string());
    }
    phrases
}