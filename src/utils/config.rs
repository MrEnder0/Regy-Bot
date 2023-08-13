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

static CONFIG_VERSION: u16 = 5;

#[derive(Serialize, Deserialize)]
struct MetaData {
    version: u16,
}

#[derive(Serialize, Deserialize)]
pub struct GlobalOptions {
    pub token: String,
    pub user_delete_on_ban: bool,
    pub max_activity_influx: u16,
    pub allow_shutdown: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ServerOptions {
    pub infractions: HashMap<String, u64>,
    pub block_phrases: HashMap<String, String>,
    pub staff: Vec<u64>,
    pub log_channel: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    meta: MetaData,
    pub global: GlobalOptions,
    pub servers: HashMap<String, ServerOptions>,
}

pub fn gen_config() {
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
            max_activity_influx: 10,
            allow_shutdown: true,
        },
        servers: HashMap::new(),
    };

    //Write base config to file
    let config = PrettyConfig::new()
        .depth_limit(4)
        .separate_tuple_members(true)
        .enumerate_arrays(true);

    let config_str = to_string_pretty(&data, config).expect("Serialization failed");
    std::fs::write("config.ron", config_str).unwrap();

    log_this(LogData {
        importance: LogImportance::Info,
        message: "Config file has been generated.".to_string(),
    });
}

pub fn read_config() -> Config {
    let config_file =
        File::open("config.ron").log_expect(LogImportance::Error, "Config file not found");
    let config: Config = match from_reader(config_file) {
        Ok(x) => x,
        Err(e) => {
            log_this(LogData {
                importance: LogImportance::Error,
                message: format!("Unable to read config file, shutting down: {}", e),
            });

            std::process::exit(0);
        }
    };

    config
}

pub fn check_config() {
    let config = read_config();
    if config.meta.version != CONFIG_VERSION {
        log_this(LogData {
            importance: LogImportance::Info,
            message: "Config file is out of date, updating config.".to_string(),
        });
        update_config();

        std::process::exit(0);
    }
}

pub fn gen_server(guid_id: String, log_channel: u64) {
    let mut data = read_config();
    let guild_id = guid_id.clone();
    data.servers.insert(
        guild_id,
        ServerOptions {
            infractions: HashMap::new(),
            block_phrases: HashMap::new(),
            staff: Vec::new(),
            log_channel: log_channel,
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
    });
}

pub fn server_exists(guid_id: String) -> bool {
    let data = read_config();
    data.servers.contains_key(&guid_id)
}

pub fn add_regex(server_id: String, phrase: String) -> bool {
    let mut data = read_config();

    //Checks if server exists
    if server_exists(server_id.clone()) == false {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!("A server with the id {} does not exist.", server_id),
        });
        return false;
    }

    let id = Uuid::new_v4();
    data.servers
        .get_mut(&server_id)
        .unwrap()
        .block_phrases
        .insert(
            id.to_string(),
            general_purpose::STANDARD_NO_PAD.encode(&phrase),
        );

    let config = PrettyConfig::new()
        .depth_limit(4)
        .separate_tuple_members(true)
        .enumerate_arrays(true);

    let config_str = to_string_pretty(&data, config).expect("Serialization failed");
    std::fs::write("config.ron", config_str).unwrap();

    true
}

pub fn remove_regex(server_id: String, id: Uuid) -> bool {
    let mut data = read_config();

    //Checks if server exists
    if server_exists(server_id.clone()) == false {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!("A server with the id {} does not exist.", server_id),
        });
        return false;
    }

    if data
        .servers
        .get(&server_id)
        .unwrap()
        .block_phrases
        .contains_key(&id.to_string())
    {
        data.servers
            .get_mut(&server_id)
            .unwrap()
            .block_phrases
            .remove(&id.to_string());

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

pub fn list_regex(server_id: String) -> Option<HashMap<Uuid, String>> {
    let data = read_config();

    //Checks if server exists
    if server_exists(server_id.clone()) == false {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!(
                "A server with the id '{}' does not exist or does not have any regex phrases.",
                server_id
            ),
        });
        return None;
    }

    let mut phrases: HashMap<Uuid, String> = HashMap::new();
    for (key, value) in &data.servers.get(&server_id).unwrap().block_phrases {
        let phrase = String::from_utf8(
            general_purpose::STANDARD_NO_PAD
                .decode(&value)
                .log_expect(LogImportance::Warning, "Unable to decode regex phrase"),
        )
        .unwrap();

        phrases.insert(
            Uuid::parse_str(key).unwrap(),
            phrase[..phrase.len() - 1].to_owned(),
        );
    }

    Some(phrases)
}

pub fn add_infraction(server_id: String, id: u64) -> bool {
    let mut data = read_config();

    //Checks if server exists
    if server_exists(server_id.clone()) == false {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!("A server with the id {} does not exist.", server_id),
        });
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

pub fn dismiss_infraction(server_id: String, id: u64) -> bool {
    let mut data = read_config();

    //Checks if server exists
    if server_exists(server_id.clone()) == false {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!("A server with the id {} does not exist.", server_id),
        });
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

pub fn list_infractions(server_id: String, id: u64) -> Option<u64> {
    let mut config = read_config();

    //Checks if server exists
    if server_exists(server_id.clone()) == false {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!("A server with the id {} does not exist.", server_id),
        });
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

pub fn add_staff(server_id: String, id: u64) -> bool {
    let mut data = read_config();

    //Checks if server exists
    if server_exists(server_id.clone()) == false {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!("A server with the id {} does not exist.", server_id),
        });
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

pub fn remove_staff(server_id: String, id: u64) -> bool {
    let mut data = read_config();

    //Checks if server exists
    if server_exists(server_id.clone()) == false {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!("A server with the id {} does not exist.", server_id),
        });
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

pub fn list_staff(server_id: String) -> Option<Vec<u64>> {
    let config = read_config();

    //Checks if server exists
    if server_exists(server_id.clone()) == false {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!("A server with the id {} does not exist.", server_id),
        });
        return None;
    }

    let mut staff: Vec<u64> = Vec::new();
    for id in &config.servers.get(&server_id).unwrap().staff {
        staff.push(*id);
    }

    Some(staff)
}

pub fn delete_user(server_id: String, id: u64) {
    let mut data = read_config();

    //Checks if server exists
    if server_exists(server_id.clone()) == false {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!("A server with the id {} does not exist.", server_id),
        });
    }

    //Checks if user is on infraction list and removes them if they are
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

    //Checks if user is on staff list and removes them if they are
    if data
        .servers
        .get(&server_id)
        .unwrap()
        .staff
        .iter()
        .any(|x| *x == id)
    {
        remove_staff(server_id.clone(), id);
    }

    let config = PrettyConfig::new()
        .depth_limit(4)
        .separate_tuple_members(true)
        .enumerate_arrays(true);

    let config_str = to_string_pretty(&data, config).expect("Serialization failed");
    std::fs::write("config.ron", config_str).unwrap();
}

pub fn update_config() {
    if !Path::new("config.ron").exists() {
        if Path::new("config.toml").exists() {
            log_this(LogData {
                importance: LogImportance::Info,
                message: "Legacy Toml config found, updating to ron config format.".to_string(),
            });

            std::fs::rename("config.toml", "config.toml.bak").unwrap();

            let old_config_file = std::fs::read_to_string("config.toml.bak").unwrap();
            let config_data: toml::Value = toml::from_str(&old_config_file).unwrap();

            //convert config_data to new config format
            let mut converted_config_data = Config {
                meta: MetaData {
                    version: CONFIG_VERSION,
                },
                global: GlobalOptions {
                    token: config_data["global"]["token"].as_str().unwrap().to_string(),
                    user_delete_on_ban: config_data["global"]["user_delete_on_ban"]
                        .as_bool()
                        .unwrap(),
                    max_activity_influx: config_data["global"]["max_activity_influx"]
                        .as_integer()
                        .unwrap() as u16,
                    allow_shutdown: config_data["global"]["allow_shutdown"].as_bool().unwrap(),
                },
                servers: HashMap::new(),
            };

            //convert servers
            for (key, value) in config_data["servers"].as_table().unwrap() {
                let mut server_options = ServerOptions {
                    infractions: HashMap::new(),
                    block_phrases: HashMap::new(),
                    staff: Vec::new(),
                    log_channel: value["log_channel"].as_integer().unwrap() as u64,
                };

                //convert infractions
                for (key, value) in value["infractions"].as_table().unwrap() {
                    server_options
                        .infractions
                        .insert(key.to_string(), value.as_integer().unwrap() as u64);
                }

                //convert block_phrases
                for (key, value) in value["block_phrases"].as_table().unwrap() {
                    let cleaned_value = &value.to_string()[1..value.to_string().len() - 1];

                    server_options
                        .block_phrases
                        .insert(key.to_string(), cleaned_value.to_string());
                }

                //convert staff
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
            });
            gen_config();
            std::process::exit(0);
        }
    } else {
        log_this(LogData {
            importance: LogImportance::Info,
            message: "Updating ron based config version, this is necessary to implement new features into Regy, it may take a few updates to get up to date.".to_string(),
        });

        //Commented due to lack of any ron based config updates

        while read_config().meta.version < CONFIG_VERSION {
            match read_config().meta.version {
                5 => {
                    log_this(LogData {
                        importance: LogImportance::Info,
                        message: "Updating config version 5 to 6, this config patch adds support for rti packages.".to_string(),
                    });

                    //TODO: Add rti package support
                }
                6 => {
                    log_this(LogData {
                        importance: LogImportance::Info,
                        message: "Updating config version 6 to 7, this config patch adds support for staff roles.".to_string(),
                    });

                    //TODO: Add staff role support
                }
                _ => {
                    log_this(LogData {
                        importance: LogImportance::Error,
                        message: "Unknown/Invalid config version, unable to update config, please resolve conflict manually.".to_string(),
                    });
                    std::process::exit(0);
                }
            }
        }
    }
}
