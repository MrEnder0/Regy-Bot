use ron::{
    self,
    ser::{to_string_pretty, PrettyConfig},
};
use scorched::*;

#[cfg(feature = "legacy-staff")]
pub async fn add_staff(server_id: String, id: u64) -> bool {
    use crate::utils::config::{read_config, server_exists};

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

#[cfg(feature = "legacy-staff")]
pub async fn remove_staff(server_id: String, id: u64) -> bool {
    use crate::utils::config::{read_config, server_exists};

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

#[cfg(feature = "legacy-staff")]
pub async fn list_staff(server_id: String) -> Option<Vec<u64>> {
    use crate::utils::config::{read_config, server_exists};

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
