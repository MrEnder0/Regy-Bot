use super::{
    read_config,
    structs::{Config, GlobalOptions, MetaData, ServerOptions},
    CONFIG_VERSION,
};
use ron::{
    self,
    ser::{to_string_pretty, PrettyConfig},
};
use scorched::*;
use std::collections::HashMap;

pub async fn gen_config() {
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
        user_global_offenses: HashMap::new(),
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

pub async fn clean_config() {
    log_this(LogData {
        importance: LogImportance::Info,
        message: "Running startup config cleanup.".to_string(),
    })
    .await;

    let mut data = read_config().await;

    data.servers.iter_mut().for_each(|(_, server_options)| {
        server_options.infractions.retain(|_, &mut v| v != 0);

        #[cfg(feature = "legacy-staff")]
        server_options.staff.retain(|&x| x != 0);
    });

    let config = PrettyConfig::new()
        .depth_limit(4)
        .separate_tuple_members(true)
        .enumerate_arrays(true);

    let config_str = to_string_pretty(&data, config).expect("Serialization failed");
    std::fs::write("config.ron", config_str).unwrap();

    log_this(LogData {
        importance: LogImportance::Info,
        message: "Finished startup config cleanup.".to_string(),
    })
    .await;
}

pub async fn gen_server(guid_id: String, log_channel_id: u64) {
    let mut data = read_config().await;
    let guild_id = guid_id.clone();
    data.servers.insert(
        guild_id,
        ServerOptions {
            infractions: HashMap::new(),
            block_phrases: Vec::new(),
            #[cfg(feature = "legacy-staff")]
            staff: Vec::new(),
            staff_roles: Vec::new(),
            log_channel: log_channel_id,
            dead_zones: Vec::new(),
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
