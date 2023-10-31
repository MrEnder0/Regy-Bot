use crate::utils::rti::read_rti;
use ron::{
    self,
    ser::{to_string_pretty, PrettyConfig},
};
use scorched::*;
use std::path::Path;

use super::{management::gen_config, read_config};

pub async fn update_config() {
    #[cfg(feature = "toml-updating")]
    {
        use toml::{from_str, Value};

        if !Path::new("config.ron").exists() {
            if Path::new("config.toml").exists() {
                log_this(LogData {
                    importance: LogImportance::Info,
                    message: "Legacy Toml config found, updating to ron config format.".to_string(),
                })
                .await;

                std::fs::rename("config.toml", "config.toml.bak").unwrap();

                let old_config_file = std::fs::read_to_string("config.toml.bak").unwrap();
                let config_data: Value = from_str(&old_config_file).unwrap();

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
                        dead_zones: Vec::new(),
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
                            description: "No description provided, legacy TOML port.".to_string(),
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

    // Minimal behavior with no toml dep
    #[cfg(not(feature = "toml-updating"))]
    {
        if !Path::new("config.ron").exists() {
            if Path::new("config.toml").exists() {
                log_this(LogData {
                    importance: LogImportance::Error,
                    message: "Legacy Toml config found, please compile with the toml-updating feature to convert toml to ron config format.".to_string(),
                })
                .await;
            }

            log_this(LogData {
                importance: LogImportance::Error,
                message: "Ron config file not found, generating default.".to_string(),
            })
            .await;
            gen_config().await;

            std::process::exit(0);
        }
    }
}

pub async fn update_rti_regexes(server_id: String) {
    let mut data = read_config().await;
    let rti = read_rti().await;

    for phrase in &mut data.servers.get_mut(&server_id).unwrap().block_phrases {
        if phrase.is_rti {
            let filtered_rti_objects = rti
                .packages
                .iter()
                .find(|rti_object| rti_object.description == phrase.description);

            match filtered_rti_objects {
                Some(_) => {
                    phrase.phrase = filtered_rti_objects.unwrap().phrase.clone();
                    phrase.description = filtered_rti_objects.unwrap().description.clone();
                    phrase.version = filtered_rti_objects.unwrap().version;
                }
                None => {
                    log_this(LogData {
                        importance: LogImportance::Warning,
                        message: format!(
                            "Unable to update found RTI object with the description '{}'.",
                            phrase.description
                        ),
                    })
                    .await;
                }
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
