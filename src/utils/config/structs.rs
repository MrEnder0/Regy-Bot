use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct MetaData {
    pub version: u8,
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
    #[cfg(feature = "legacy-staff")]
    pub staff: Vec<u64>,
    pub staff_roles: Vec<u64>,
    pub log_channel: u64,
    pub dead_zones: Vec<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct UserOffenses {
    pub global_infractions: u64,
    pub regy_bans: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub meta: MetaData,
    pub global: GlobalOptions,
    pub servers: HashMap<String, ServerOptions>,
    pub user_global_offenses: HashMap<String, UserOffenses>,
}
