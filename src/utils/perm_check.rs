use poise::{
    serenity_prelude as serenity,
};

use crate::utils::{toml, log_on_error::LogExpect};

pub enum PermissionLevel {
    Moderator,
    Admin,
    Developer
}

pub async fn has_perm(ctx: &serenity::Context, permission_level: PermissionLevel) -> bool {
    let user = ctx.cache.current_user();
    let user_id = user.id.to_string();

    match permission_level {
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
            let devs = [
                "687897073047306270",
                "598280691066732564",
                "275787354688585730",
            ];

            if devs.contains(&&user_id[..]) {
                true
            } else {
                false
            }
        }
    }
}