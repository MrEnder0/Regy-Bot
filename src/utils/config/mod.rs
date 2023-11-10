use std::fs::File;

use ron::{
    de::from_reader,
    ser::{to_string_pretty, PrettyConfig},
};
use scorched::*;

use self::structs::Config;

pub static CONFIG_VERSION: u8 = 8;

pub mod dead_zones;
pub mod infractions;
pub mod management;
pub mod regex;
pub mod staff;
pub mod structs;
pub mod updating;

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

pub fn non_async_read_config() -> Config {
    let config_file =
        File::open("config.ron").log_expect(LogImportance::Error, "Config file not found");
    let config: Config = match from_reader(config_file) {
        Ok(x) => x,
        Err(e) => {
            log_this(LogData {
                importance: LogImportance::Error,
                message: format!("Unable to read config file with the following error {}", e),
            });

            std::process::exit(0);
        }
    };

    config
}

pub async fn server_exists(server_id: String) -> bool {
    let data = read_config().await;
    data.servers.contains_key(&server_id)
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

    #[cfg(feature = "legacy-staff")]
    {
        use crate::utils::config::staff::legacy::remove_staff;

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
    }

    let config = PrettyConfig::new()
        .depth_limit(4)
        .separate_tuple_members(true)
        .enumerate_arrays(true);

    let config_str = to_string_pretty(&data, config).expect("Serialization failed");
    std::fs::write("config.ron", config_str).unwrap();
}
