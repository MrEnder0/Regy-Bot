use base64::{engine::general_purpose, Engine as _};
use poise::serenity_prelude::guild;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::logger::{log_this, LogData, LogImportance};

#[derive(Serialize, Deserialize)]
pub struct GlobalOptions {
    pub token: String,
    pub user_delete_on_ban: bool,
    pub max_activity_influx: u16,
    pub allow_shutdown: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ServerOptions {
    //user-id | infraction-count
    pub infractions: HashMap<String, u64>,
    //uuid | phrase
    pub block_phrases: HashMap<String, String>,
    //user-id
    pub staff: Vec<u64>,
    //channel-id
    pub log_channel: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub global: GlobalOptions,
    pub servers: HashMap<String, ServerOptions>,
}

pub fn gen_config() {
    let mut phr = HashMap::new();
    phr.insert(
        Uuid::new_v4(),
        general_purpose::STANDARD_NO_PAD.encode("regy test phrase"),
    );
    let config = Config {
        global: GlobalOptions {
            token: "token".to_string(),
            user_delete_on_ban: true,
            max_activity_influx: 10,
            allow_shutdown: true,
        },
        servers: HashMap::new(),
    };
    //Write base config to file
    let toml = toml::to_string(&config).unwrap();
    std::fs::write("config.toml", toml).unwrap();

    log_this(LogData {
        importance: LogImportance::Info,
        message: "Config file has been generated.".to_string(),
    });
}

pub fn read_config() -> Config {
    let toml = std::fs::read_to_string("config.toml").unwrap();
    let config: Config = toml::from_str(&toml).unwrap();
    config
}

pub fn gen_server(guid_id: String, log_channel: u64) {
    let mut config = read_config();
    let guild_id = guid_id.clone();
    config.servers.insert(
        guild_id,
        ServerOptions {
            infractions: HashMap::new(),
            block_phrases: HashMap::new(),
            staff: Vec::new(),
            log_channel: log_channel,
        },
    );
    let toml = toml::to_string(&config).unwrap();
    std::fs::write("config.toml", toml).unwrap();

    log_this(LogData {
        importance: LogImportance::Info,
        message: format!(
            "A server with the id {} has been added to the config file.",
            guid_id
        ),
    });
}

pub fn server_exists(guid_id: String) -> bool {
    let config = read_config();
    config.servers.contains_key(&guid_id)
}

pub fn add_regex(server_id: String, phrase: String) -> bool {
    let mut config = read_config();
    //config.block_phrases.insert(Uuid::new_v4(),general_purpose::STANDARD_NO_PAD.encode(&phrase));
    //let toml = toml::to_string(&config).unwrap();
    //std::fs::write("config.toml", toml).unwrap();

    //Checks if server exists
    if server_exists(server_id.clone()) == false {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!("A server with the id {} does not exist.", server_id),
        });
        return false;
    }

    config
        .servers
        .get_mut(&server_id)
        .unwrap()
        .block_phrases
        .insert(
            Uuid::new_v4().to_string(),
            general_purpose::STANDARD_NO_PAD.encode(&phrase),
        );
    let toml = toml::to_string(&config).unwrap();
    std::fs::write("config.toml", toml).unwrap();

    true
}

pub fn remove_regex(server_id: String, id: Uuid) -> bool {
    let mut config = read_config();
    //config.block_phrases.remove(&id);
    //let toml = toml::to_string(&config).unwrap();
    //std::fs::write("config.toml", toml).unwrap();

    //Checks if server exists
    if server_exists(server_id.clone()) == false {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!("A server with the id {} does not exist.", server_id),
        });
        return false;
    }

    config
        .servers
        .get_mut(&server_id)
        .unwrap()
        .block_phrases
        .remove(&id.to_string());
    let toml = toml::to_string(&config).unwrap();
    std::fs::write("config.toml", toml).unwrap();

    true
}

pub fn list_regex(server_id: String) -> Option<HashMap<Uuid, String>> {
    let config = read_config();

    //Checks if server exists
    if server_exists(server_id.clone()) == false {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!("A server with the id {} does not exist.", server_id),
        });
        return None;
    }

    let mut phrases: HashMap<Uuid, String> = HashMap::new();

    for (id, phrase) in &config.servers.get(&server_id).unwrap().block_phrases {
        let phrase =
            String::from_utf8(general_purpose::STANDARD_NO_PAD.decode(&phrase).unwrap()).unwrap();
        let phrase = &phrase[..phrase.len() - 1];
        phrases.insert(id.parse::<Uuid>().unwrap(), phrase.to_string());
    }

    Some(phrases)
}

pub fn add_infraction(server_id: String, id: u64) -> bool {
    let mut config = read_config();
    //let infractions = config.infractions.entry(id.to_string()).or_insert(0);
    //*infractions += 1;
    //let toml = toml::to_string(&config).unwrap();
    //std::fs::write("config.toml", toml).unwrap();

    //Checks if server exists
    if server_exists(server_id.clone()) == false {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!("A server with the id {} does not exist.", server_id),
        });
        return false;
    }

    let infractions = config
        .servers
        .get_mut(&server_id)
        .unwrap()
        .infractions
        .entry(id.to_string())
        .or_insert(0);
    *infractions += 1;
    let toml = toml::to_string(&config).unwrap();
    std::fs::write("config.toml", toml).unwrap();

    true
}

pub fn dismiss_infraction(server_id: String, id: u64) -> bool {
    let mut config = read_config();
    //let infractions = config.infractions.entry(id.to_string()).or_insert(1);
    //if *infractions == 0 {
    //    return
    //} else if *infractions == 1{
    //    *infractions = 0;
    //} else {
    //    *infractions -= 1;
    //}

    //let toml = toml::to_string(&config).unwrap();
    //std::fs::write("config.toml", toml).unwrap();

    //Checks if server exists
    if server_exists(server_id.clone()) == false {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!("A server with the id {} does not exist.", server_id),
        });
        return false;
    }

    let infractions = config
        .servers
        .get_mut(&server_id)
        .unwrap()
        .infractions
        .entry(id.to_string())
        .or_insert(1);
    if *infractions <= 0 {
        return false;
    } else {
        *infractions -= 1;
    }

    true
}

pub fn list_infractions(server_id: String, id: u64) -> Option<u64> {
    let mut config = read_config();
    //let infractions = config.infractions.entry(id.to_string()).or_insert(0);
    //*infractions

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
    let mut config = read_config();

    //if config.staff.contains(&id.to_string()) {
    //    false
    //} else {
    //    config.staff.push(id.to_string());
    //    let toml = toml::to_string(&config).unwrap();
    //    std::fs::write("config.toml", toml).unwrap();

    //    log_this(LogData {
    //        importance: LogImportance::Info,
    //        message: format!("{} Has been added to the staff list.", id),
    //    });

    //    true
    //}

    //Checks if server exists
    if server_exists(server_id.clone()) == false {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!("A server with the id {} does not exist.", server_id),
        });
        return false;
    }

    if config.servers.get(&server_id).unwrap().staff.contains(&id) {
        false
    } else {
        config.servers.get_mut(&server_id).unwrap().staff.push(id);
        let toml = toml::to_string(&config).unwrap();
        std::fs::write("config.toml", toml).unwrap();

        true
    }
}

pub fn remove_staff(server_id: String, id: u64) -> bool {
    let mut config = read_config();

    //if config.staff.contains(&id.to_string()) {
    //    config.staff.remove(config.staff.iter().position(|x| *x == id.to_string()).unwrap());
    //    let toml = toml::to_string(&config).unwrap();
    //    std::fs::write("config.toml", toml).unwrap();

    //    log_this(LogData {
    //        importance: LogImportance::Info,
    //        message: format!("{} Has been removed from the staff list.", id),
    //    });

    //    true
    //} else {
    //    false
    //}

    //Checks if server exists
    if server_exists(server_id.clone()) == false {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!("A server with the id {} does not exist.", server_id),
        });
        return false;
    }

    if config.servers.get(&server_id).unwrap().staff.contains(&id) {
        let user_index = config
            .servers
            .get(&server_id)
            .unwrap()
            .staff
            .iter()
            .position(|x| *x == id)
            .unwrap();
        config
            .servers
            .get_mut(&server_id)
            .unwrap()
            .staff
            .remove(user_index);
        let toml = toml::to_string(&config).unwrap();
        std::fs::write("config.toml", toml).unwrap();

        log_this(LogData {
            importance: LogImportance::Info,
            message: format!("{} Has been removed from the staff list.", id),
        });

        true
    } else {
        false
    }
}

pub fn list_staff(server_id: String) -> Option<Vec<u64>> {
    let config = read_config();
    //let mut staff: Vec<u64> = Vec::new();
    //for id in config.staff {
    //    staff.push(id.parse::<u64>().unwrap());
    //}
    //staff

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
    let mut config = read_config();
    //config.infractions.remove(&id.to_string());

    ////Removes from staff list if they are on it
    //if config.staff.iter().any(|x| *x == id.to_string()) {
    //    config.staff.remove(config.staff.iter().position(|x| *x == id.to_string()).unwrap());

    //    log_this(LogData {
    //        importance: LogImportance::Info,
    //        message: format!("{} Has been deleted from the staff list due to being banned.", id),
    //    });
    //}

    //let toml = toml::to_string(&config).unwrap();
    //std::fs::write("config.toml", toml).unwrap();

    //Checks if server exists
    if server_exists(server_id.clone()) == false {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: format!("A server with the id {} does not exist.", server_id),
        });
    }

    //Checks if user is on infraction list and removes them if they are
    if config
        .servers
        .get(&server_id)
        .unwrap()
        .infractions
        .contains_key(&id.to_string())
    {
        config
            .servers
            .get_mut(&server_id)
            .unwrap()
            .infractions
            .remove(&id.to_string());
    }

    //Checks if user is on staff list and removes them if they are
    if config
        .servers
        .get(&server_id)
        .unwrap()
        .staff
        .iter()
        .any(|x| *x == id)
    {
        remove_staff(server_id.clone(), id);
    }

    let toml = toml::to_string(&config).unwrap();
    std::fs::write("config.toml", toml).unwrap();
}
