use super::config;

pub enum PermissionLevel {
    User,
    Staff,
    Developer,
}

const DEVELOPERS: [&str; 3] = [
    "687897073047306270",
    "598280691066732564",
    "275787354688585730",
];

pub async fn has_perm(server_id: String, userid: u64, permission_level: PermissionLevel) -> bool {
    match permission_level {
        PermissionLevel::User => true,
        PermissionLevel::Staff => config::read_config()
            .servers
            .get(&server_id)
            .unwrap()
            .staff
            .contains(&userid),
        PermissionLevel::Developer => DEVELOPERS.contains(&&userid.to_string()[..]),
    }
}

pub async fn highest_unlocked_perm(server_id: String, userid: u64) -> PermissionLevel {
    if DEVELOPERS.contains(&&userid.to_string()[..]) {
        PermissionLevel::Developer
    } else if config::read_config()
        .servers
        .get(&server_id)
        .unwrap()
        .staff
        .contains(&userid)
    {
        PermissionLevel::Staff
    } else {
        PermissionLevel::User
    }
}
