use super::rti::read_rti;
use base64::{engine::general_purpose, Engine as _};
use ron::{
    self,
    de::from_reader,
    ser::{to_string_pretty, PrettyConfig},
};
use scorched::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, path::Path};
use toml;
use uuid::Uuid;

static CONFIG_VERSION: u8 = 6;

#[derive(Serialize, Deserialize)]
struct MetaData {
    version: u8,
}

#[derive(Serialize, Deserialize)]
pub struct GlobalOptions {
    pub token: String,
    pub user_delete_on_ban: bool,
    pub allow_shutdown: bool,
    pub rti_download_frequency: u64,
}

#[derive(Serialize, Deserialize)]
pub struct BlockPhrase {
    pub uuid: String,
    pub phrase: String,
    pub is_rti: bool,
    pub description: String,
    pub version: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ServerOptions {
    pub infractions: HashMap<String, u64>,
    pub block_phrases: Vec<BlockPhrase>,
    pub staff: Vec<u64>,
    pub log_channel: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    meta: MetaData,
    pub global: GlobalOptions,
    pub servers: HashMap<String, ServerOptions>,
}

pub async fn gen_config() {
    let mut phr = HashMap::new();
    phr.insert(
        Uuid::new_v4(),
        general_purpose::STANDARD_NO_PAD.encode("regy test phrase"),
    );

    let data = Config {
        meta: MetaData {
            version: CONFIG_VERSION,
        },
        global: GlobalOptions {
            token: "token".to_string(),
            user_delete_on_ban: true,
            allow_shutdown: true,
            rti_download_frequency: 8,
        },
        servers: HashMap::new(),
    };

    // Writes base config to file
    let config = PrettyConfig::new()
        .depth_limit(4)
        .separate_tuple_members(true)
        .enumerate_arrays(true);

    let config_str = to_string_pretty(&data, config).expect("Serialization failed");
    std::fs::write("config.ron", config_str).unwrap();

    log_this(LogData {
        importance: LogImportance::Info,
        message: "Config file has been generated.".to_string(),
    })
    .await;
}

pub async fn read_config() -> Config {
    let config_file =
        File::open("config.ron").log_expect(LogImportance::Error, "Config file not found");
    let config: Config = match from_reader(config_file) {
        Ok(x) => x,
        Err(e) => {
            log_this(LogData {
                importance: LogImportance::Error,
                message: format!(
                    "Unable to read config file because of the following error:\n{}",
                    e
                ),
            })
            .await;

            std::process::exit(0);
        }
    };

    config
}

pub async fn gen_server(guid_id: String, log_channel_id: u64) {
    let mut data = read_config().await;
    let guild_id = guid_id.clone();
    data.servers.insert(
        guild_id,
        ServerOptions {
            infractions: HashMap::new(),
            block_phrases: Vec::new(),
            staff: Vec::new(),
            log_channel: log_channel_id,
        },
    );

    let config = PrettyConfig::new()
        .depth_limit(4)
        .separate_tuple_members(true)
        .enumerate_arrays(true);

    let config_str = to_string_pretty(&data, config).expect("Serialization failed");
    std::fs::write("config.ron", config_str).unwrap();

    log_this(LogData {
        importance: LogImportance::Info,
        message: format!(
            "A server with the id {} has been added to the config file.",
            guid_id
        ),
    })
    .await;
}

pub async fn server_exists(guid_id: String) -> bool {
    let data = read_config().await;
    data.servers.contains_key(&guid_id)
}

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

pub async fn add_infraction(server_id: String, id: u64) -> bool {
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

    let infractions = data
        .servers
        .get_mut(&server_id)
        .unwrap()
        .infractions
        .entry(id.to_string())
        .or_insert(0);
    *infractions += 1;

    let config = PrettyConfig::new()
        .depth_limit(4)
        .separate_tuple_members(true)
        .enumerate_arrays(true);

    let config_str = to_string_pretty(&data, config).expect("Serialization failed");
    std::fs::write("config.ron", config_str).unwrap();

    true
}

pub async fn dismiss_infraction(server_id: String, id: u64) -> bool {
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
        .infractions
        .contains_key(&id.to_string())
    {
        let infractions = data
            .servers
            .get_mut(&server_id)
            .unwrap()
            .infractions
            .entry(id.to_string())
            .or_insert(0);

        if *infractions > 0 {
            *infractions -= 1;
        } else {
            return false;
        }

        let config = PrettyConfig::new()
            .depth_limit(4)
            .separate_tuple_members(true)
            .enumerate_arrays(true);

        let config_str = to_string_pretty(&data, config).expect("Serialization failed");
        std::fs::write("config.ron", config_str).unwrap();
    } else {
        return false;
    }

    true
}

pub async fn list_infractions(server_id: String, id: u64) -> Option<u64> {
    let mut config = read_config().await;

    // Checks if server does not exist
    if !server_exists(server_id.clone()).await {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!("A server with the id {} does not exist.", server_id),
        })
        .await;

        return None;
    }

    let infractions = config
        .servers
        .get_mut(&server_id)
        .unwrap()
        .infractions
        .entry(id.to_string())
        .or_insert(0);

    Some(*infractions)
}

pub async fn add_staff(server_id: String, id: u64) -> bool {
    let mut data = read_config().await;

    //Checks if server does not exist
    if !server_exists(server_id.clone()).await {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!("A server with the id {} does not exist.", server_id),
        })
        .await;

        return false;
    }

    if data.servers.get(&server_id).unwrap().staff.contains(&id) {
        false
    } else {
        data.servers.get_mut(&server_id).unwrap().staff.push(id);

        let config = PrettyConfig::new()
            .depth_limit(4)
            .separate_tuple_members(true)
            .enumerate_arrays(true);

        let config_str = to_string_pretty(&data, config).expect("Serialization failed");
        std::fs::write("config.ron", config_str).unwrap();

        true
    }
}

pub async fn remove_staff(server_id: String, id: u64) -> bool {
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

    if data.servers.get(&server_id).unwrap().staff.contains(&id) {
        data.servers
            .get_mut(&server_id)
            .unwrap()
            .staff
            .retain(|&x| x != id);

        let config = PrettyConfig::new()
            .depth_limit(4)
            .separate_tuple_members(true)
            .enumerate_arrays(true);

        let config_str = to_string_pretty(&data, config).expect("Serialization failed");
        std::fs::write("config.ron", config_str).unwrap();

        true
    } else {
        false
    }
}

pub async fn list_staff(server_id: String) -> Option<Vec<u64>> {
    let config = read_config().await;

    // Checks if server does not exist
    if !server_exists(server_id.clone()).await {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!("A server with the id {} does not exist.", server_id),
        })
        .await;

        return None;
    }

    let mut staff: Vec<u64> = Vec::new();
    for id in &config.servers.get(&server_id).unwrap().staff {
        staff.push(*id);
    }

    Some(staff)
}

pub async fn delete_user(server_id: String, id: u64) {
    let mut data = read_config().await;

    // Checks if server does not exist
    if !server_exists(server_id.clone()).await {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!("A server with the id {} does not exist.", server_id),
        })
        .await;
    }

    // Checks if user is on infraction list and removes them if they are
    if data
        .servers
        .get(&server_id)
        .unwrap()
        .infractions
        .contains_key(&id.to_string())
    {
        data.servers
            .get_mut(&server_id)
            .unwrap()
            .infractions
            .remove(&id.to_string());
    }

    // Checks if user is on staff list and removes them if they are
    if data
        .servers
        .get(&server_id)
        .unwrap()
        .staff
        .iter()
        .any(|x| *x == id)
    {
        remove_staff(server_id.clone(), id).await;
    }

    let config = PrettyConfig::new()
        .depth_limit(4)
        .separate_tuple_members(true)
        .enumerate_arrays(true);

    let config_str = to_string_pretty(&data, config).expect("Serialization failed");
    std::fs::write("config.ron", config_str).unwrap();
}

pub async fn update_config() {
    if !Path::new("config.ron").exists() {
        if Path::new("config.toml").exists() {
            log_this(LogData {
                importance: LogImportance::Info,
                message: "Legacy Toml config found, updating to ron config format.".to_string(),
            })
            .await;

            std::fs::rename("config.toml", "config.toml.bak").unwrap();

            let old_config_file = std::fs::read_to_string("config.toml.bak").unwrap();
            let config_data: toml::Value = toml::from_str(&old_config_file).unwrap();

            let mut converted_config_data = Config {
                meta: MetaData {
                    version: CONFIG_VERSION,
                },
                global: GlobalOptions {
                    token: config_data["global"]["token"].as_str().unwrap().to_string(),
                    user_delete_on_ban: config_data["global"]["user_delete_on_ban"]
                        .as_bool()
                        .unwrap(),
                    allow_shutdown: config_data["global"]["allow_shutdown"].as_bool().unwrap(),
                    rti_download_frequency: 8,
                },
                servers: HashMap::new(),
            };

            for (key, value) in config_data["servers"].as_table().unwrap() {
                let mut server_options = ServerOptions {
                    infractions: HashMap::new(),
                    block_phrases: Vec::new(),
                    staff: Vec::new(),
                    log_channel: value["log_channel"].as_integer().unwrap() as u64,
                };

                for (key, value) in value["infractions"].as_table().unwrap() {
                    server_options
                        .infractions
                        .insert(key.to_string(), value.as_integer().unwrap() as u64);
                }

                for (key, value) in value["block_phrases"].as_table().unwrap() {
                    let cleaned_value = &value.to_string()[1..value.to_string().len() - 1];

                    let phrase = BlockPhrase {
                        uuid: key.to_string(),
                        phrase: general_purpose::STANDARD_NO_PAD.encode(cleaned_value),
                        is_rti: false,
                        description: "No description provided.".to_string(),
                        version: 0,
                    };

                    server_options.block_phrases.push(phrase);
                }

                for value in value["staff"].as_array().unwrap() {
                    server_options
                        .staff
                        .push(value.as_integer().unwrap() as u64);
                }

                converted_config_data
                    .servers
                    .insert(key.to_string(), server_options);
            }

            let config = PrettyConfig::new()
                .depth_limit(4)
                .separate_tuple_members(true)
                .enumerate_arrays(true);
            let new_config =
                to_string_pretty(&converted_config_data, config).expect("Serialization failed");

            std::fs::write("config.ron", new_config).unwrap();
        } else {
            log_this(LogData {
                importance: LogImportance::Error,
                message: "Config file not found, generating default.".to_string(),
            })
            .await;
            gen_config().await;

            std::process::exit(0);
        }
    }
}

pub async fn update_regexes(server_id: String) {
    let mut data = read_config().await;
    let rti = read_rti().await;

    for phrase in &mut data.servers.get_mut(&server_id).unwrap().block_phrases {
        if phrase.is_rti {
            let filtered_rti_objects = rti
                .packages
                .iter()
                .find(|rti_object| rti_object.description == phrase.description);

            if filtered_rti_objects.is_some() {
                phrase.phrase = filtered_rti_objects.unwrap().phrase.clone();
                phrase.description = filtered_rti_objects.unwrap().description.clone();
                phrase.version = filtered_rti_objects.unwrap().version;
            }
        }
    }

    let config = PrettyConfig::new()
        .depth_limit(4)
        .separate_tuple_members(true)
        .enumerate_arrays(true);

    let config_str = to_string_pretty(&data, config).expect("Serialization failed");
    std::fs::write("config.ron", config_str).unwrap();
}
