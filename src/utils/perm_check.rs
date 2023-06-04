use crate::utils::toml;

pub enum PermissionLevel {
    User,
    Staff,
    Developer
}

const DEVELOPERS : [&str; 3] = [
    "687897073047306270",
    "598280691066732564",
    "275787354688585730",
];

pub async fn has_perm(userid: u64, permission_level: PermissionLevel) -> bool {
    let user_id = userid.to_string();

    match permission_level {
        PermissionLevel::User => {
            true
        }
        PermissionLevel::Staff => {
            if toml::get_config().staff.contains(&user_id) {
                true
            } else {
                false
            }
        }
        PermissionLevel::Developer => {
            if DEVELOPERS.contains(&&user_id[..]) {
                true
            } else {
                false
            }
        }
    }
}

pub async fn highest_unlocked_perm(userid: u64) -> PermissionLevel {
    let user_id = userid.to_string();

    if DEVELOPERS.contains(&&user_id[..]) {
        PermissionLevel::Developer
    } else if toml::get_config().staff.contains(&user_id) {
        PermissionLevel::Staff
    } else {
        PermissionLevel::User
    }
}