#[cfg(feature = "legacy-staff")]
pub mod legacy;

use ron::{
    self,
    ser::{to_string_pretty, PrettyConfig},
};
use scorched::*;

use crate::utils::config::{read_config, server_exists};

pub async fn add_staff_role(server_id: String, id: u64) -> bool {
    let mut data = read_config().await;

    //Checks if server does not exist
    if !server_exists(server_id.clone()).await {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!("A server with the id '{}' does not exist.", server_id),
        })
        .await;

        return false;
    }

    if data
        .servers
        .get(&server_id)
        .unwrap()
        .staff_roles
        .contains(&id)
    {
        false
    } else {
        data.servers
            .get_mut(&server_id)
            .unwrap()
            .staff_roles
            .push(id);

        let config = PrettyConfig::new()
            .depth_limit(4)
            .separate_tuple_members(true)
            .enumerate_arrays(true);

        let config_str = to_string_pretty(&data, config).expect("Serialization failed");
        std::fs::write("config.ron", config_str).unwrap();

        true
    }
}

pub async fn remove_staff_role(server_id: String, id: u64) -> bool {
    let mut data = read_config().await;

    // Checks if server does not exist
    if !server_exists(server_id.clone()).await {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!("A server with the id '{}' does not exist.", server_id),
        })
        .await;

        return false;
    }

    if data
        .servers
        .get(&server_id)
        .unwrap()
        .staff_roles
        .contains(&id)
    {
        data.servers
            .get_mut(&server_id)
            .unwrap()
            .staff_roles
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

pub async fn list_staff_roles(server_id: String) -> Option<Vec<u64>> {
    let config = read_config().await;

    // Checks if server does not exist
    if !server_exists(server_id.clone()).await {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!(
                "A server with the id '{}' does not exist or does not have any staff roles.",
                server_id
            ),
        })
        .await;

        return None;
    }

    let mut staff_roles: Vec<u64> = Vec::new();
    for id in &config.servers.get(&server_id).unwrap().staff_roles {
        staff_roles.push(*id);
    }

    Some(staff_roles)
}
