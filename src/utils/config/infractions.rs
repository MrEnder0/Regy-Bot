use ron::{
    self,
    ser::{to_string_pretty, PrettyConfig},
};
use scorched::*;

use super::{read_config, server_exists, structs::UserOffenses};

pub async fn add_infraction(server_id: String, id: u64) -> bool {
    let mut data = read_config().await;

    // Checks if server does not exist
    if !server_exists(server_id.clone()).await {
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

    add_user_offense_infraction(id).await;

    true
}

pub async fn dismiss_infraction(server_id: String, id: u64) -> bool {
    let mut data = read_config().await;

    // Checks if server does not exist
    if !server_exists(server_id.clone()).await {
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

        dismiss_user_offense_infraction(id).await;
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

pub async fn add_user_offense_infraction(id: u64) {
    let mut data = read_config().await;

    let offenses = data
        .user_global_offenses
        .entry(id.to_string())
        .or_insert(UserOffenses {
            global_infractions: 0,
            regy_bans: 0,
        });

    offenses.global_infractions += 1;

    let config = PrettyConfig::new()
        .depth_limit(4)
        .separate_tuple_members(true)
        .enumerate_arrays(true);

    let config_str = to_string_pretty(&data, config).expect("Serialization failed");
    std::fs::write("config.ron", config_str).unwrap();
}

pub async fn dismiss_user_offense_infraction(id: u64) {
    let mut data = read_config().await;

    let offenses = data
        .user_global_offenses
        .entry(id.to_string())
        .or_insert(UserOffenses {
            global_infractions: 0,
            regy_bans: 0,
        });

    if offenses.global_infractions > 0 {
        offenses.global_infractions -= 1;
    }

    let config = PrettyConfig::new()
        .depth_limit(4)
        .separate_tuple_members(true)
        .enumerate_arrays(true);

    let config_str = to_string_pretty(&data, config).expect("Serialization failed");
    std::fs::write("config.ron", config_str).unwrap();
}

pub async fn add_user_offense_ban(id: u64) {
    let mut data = read_config().await;

    let offenses = data
        .user_global_offenses
        .entry(id.to_string())
        .or_insert(UserOffenses {
            global_infractions: 0,
            regy_bans: 0,
        });

    offenses.regy_bans += 1;

    let config = PrettyConfig::new()
        .depth_limit(4)
        .separate_tuple_members(true)
        .enumerate_arrays(true);

    let config_str = to_string_pretty(&data, config).expect("Serialization failed");
    std::fs::write("config.ron", config_str).unwrap();
}

pub async fn get_user_offenses(id: u64) -> Option<UserOffenses> {
    let mut config = read_config().await;

    config
        .user_global_offenses
        .iter_mut()
        .find(|x| x.0 == &id.to_string())
        .map(|(_, offences)| offences.clone())
}
