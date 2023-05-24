use crate::utils::toml;

pub enum PermissionLevel {
    User,
    Moderator,
    Admin,
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
        PermissionLevel::Moderator => {
            let mut moderators = toml::get_config().moderators;
            for admin in toml::get_config().admins {
                moderators.push(admin);
            }

            if moderators.contains(&user_id) {
                true
            } else {
                false
            }
        }
        PermissionLevel::Admin => {
            let admins = toml::get_config().admins;
            if admins.contains(&user_id) {
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

    let mut moderators = toml::get_config().moderators;
    for admin in toml::get_config().admins {
        moderators.push(admin);
    }

    if DEVELOPERS.contains(&&user_id[..]) {
        PermissionLevel::Developer
    } else if moderators.contains(&user_id) {
        PermissionLevel::Moderator
    } else if toml::get_config().admins.contains(&user_id) {
        PermissionLevel::Admin
    } else {
        PermissionLevel::User
    }
}