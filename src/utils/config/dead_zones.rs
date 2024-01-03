use ron::{
    self,
    ser::{to_string_pretty, PrettyConfig},
};
use scorched::*;

use super::{read_config, server_exists};

pub async fn list_dead_zones(server_id: String) -> Option<Vec<u64>> {
    let config = read_config().await;

    // Checks if server does not exist
    if !server_exists(server_id.clone()).await {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!(
                "A server with the id '{}' does not exist or does not have any dead zones.",
                server_id
            ),
        });

        return None;
    }

    Some(config.servers.get(&server_id).unwrap().dead_zones.clone())
}

pub async fn is_dead_zone(server_id: String, channel_id: u64) -> bool {
    // Checks if server does not exist
    if !server_exists(server_id.clone()).await {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!(
                "A server with the id '{}' does not exist or does not have any dead zones.",
                server_id
            ),
        });

        return false;
    }

    list_dead_zones(server_id.clone())
        .await
        .unwrap()
        .contains(&channel_id)
}

pub async fn add_dead_zone(server_id: String, channel_id: u64) -> bool {
    let mut data = read_config().await;

    //Checks if server does not exist
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
        .dead_zones
        .contains(&channel_id)
    {
        false
    } else {
        data.servers
            .get_mut(&server_id)
            .unwrap()
            .dead_zones
            .push(channel_id);

        let config = PrettyConfig::new()
            .depth_limit(4)
            .separate_tuple_members(true)
            .enumerate_arrays(true);

        let config_str = to_string_pretty(&data, config).expect("Serialization failed");
        std::fs::write("config.ron", config_str).unwrap();

        true
    }
}

pub async fn remove_dead_zone(server_id: String, channel_id: u64) -> bool {
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
        .dead_zones
        .contains(&channel_id)
    {
        data.servers
            .get_mut(&server_id)
            .unwrap()
            .dead_zones
            .retain(|&x| x != channel_id);

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
