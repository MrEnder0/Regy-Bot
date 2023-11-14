use base64::{engine::general_purpose, Engine as _};
use ron::{
    self,
    ser::{to_string_pretty, PrettyConfig},
};
use scorched::*;
use uuid::Uuid;

use crate::utils::crc::CrcStruct;

use super::{non_async_read_config, read_config, server_exists, structs::BlockPhrase};

pub async fn add_regex(
    server_id: String,
    phrase: String,
    is_rti: bool,
    description: String,
    version: u32,
) -> bool {
    let mut data = read_config().await;

    // Checks if server does not exist
    if !server_exists(server_id.clone()).await {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!("A server with the id {} does not exist.", server_id),
        })
        .await;

        return false;
    }

    // Checks if phrase already exists
    for current_phrase in &data.servers.get(&server_id).unwrap().block_phrases {
        if current_phrase.phrase == phrase {
            log_this(LogData {
                importance: LogImportance::Warning,
                message: format!(
                    "A phrase with the value '{}' already exists.",
                    current_phrase.phrase
                ),
            })
            .await;

            return false;
        }
    }

    let id = Uuid::new_v4();
    data.servers
        .get_mut(&server_id)
        .unwrap()
        .block_phrases
        .push(BlockPhrase {
            uuid: id.to_string(),
            phrase: general_purpose::STANDARD_NO_PAD.encode(&phrase),
            is_rti,
            description,
            version,
        });

    let config = PrettyConfig::new()
        .depth_limit(4)
        .separate_tuple_members(true)
        .enumerate_arrays(true);

    let config_str = to_string_pretty(&data, config).expect("Serialization failed");
    std::fs::write("config.ron", config_str).unwrap();

    CrcStruct::build_server_cache(server_id.parse::<u64>().unwrap());

    true
}

pub async fn remove_regex(server_id: String, id: Uuid) -> bool {
    let mut data = read_config().await;

    // Checks if server does not exist
    if !server_exists(server_id.clone()).await {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!("A server with the id {} does not exist.", server_id),
        })
        .await;

        return false;
    }

    if data
        .servers
        .get(&server_id)
        .unwrap()
        .block_phrases
        .iter()
        .any(|x| x.uuid == id.to_string())
    {
        data.servers
            .get_mut(&server_id)
            .unwrap()
            .block_phrases
            .retain(|x| x.uuid != id.to_string());

        let config = PrettyConfig::new()
            .depth_limit(4)
            .separate_tuple_members(true)
            .enumerate_arrays(true);

        let config_str = to_string_pretty(&data, config).expect("Serialization failed");
        std::fs::write("config.ron", config_str).unwrap();
    } else {
        return false;
    }

    CrcStruct::build_server_cache(server_id.parse::<u64>().unwrap());

    true
}

pub async fn list_regex(server_id: String) -> Option<Vec<BlockPhrase>> {
    let data = read_config().await;

    // Checks if server does not exist
    if !server_exists(server_id.clone()).await {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!(
                "A server with the id '{}' does not exist or does not have any regex phrases.",
                server_id
            ),
        })
        .await;

        return None;
    }

    let mut block_phrases: Vec<BlockPhrase> = Vec::new();
    for phrase in &data.servers.get(&server_id).unwrap().block_phrases {
        let decoded_phrase = String::from_utf8(
            general_purpose::STANDARD_NO_PAD
                .decode(phrase.phrase.as_bytes())
                .log_expect(LogImportance::Warning, "Unable to decode regex phrase"),
        )
        .unwrap();

        block_phrases.push(BlockPhrase {
            uuid: phrase.uuid.clone(),
            phrase: decoded_phrase,
            is_rti: phrase.is_rti,
            description: phrase.description.clone(),
            version: phrase.version,
        });
    }

    Some(block_phrases)
}

pub fn non_async_list_regex(server_id: String) -> Vec<BlockPhrase> {
    let data = non_async_read_config();

    let mut block_phrases: Vec<BlockPhrase> = Vec::new();

    for phrase in &data.servers.get(&server_id).unwrap().block_phrases {
        let decoded_phrase = String::from_utf8(
            general_purpose::STANDARD_NO_PAD
                .decode(phrase.phrase.as_bytes())
                .log_expect(LogImportance::Warning, "Unable to decode regex phrase"),
        )
        .unwrap();

        block_phrases.push(BlockPhrase {
            uuid: phrase.uuid.clone(),
            phrase: decoded_phrase,
            is_rti: phrase.is_rti,
            description: phrase.description.clone(),
            version: phrase.version,
        });
    }

    block_phrases
}
