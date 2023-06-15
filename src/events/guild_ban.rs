use poise::{
    serenity_prelude::{User, GuildId}
};

use crate::utils::toml::*;

pub async fn guild_ban_event(server_id: GuildId, banned_user: &User) {
    //Check if server exists in config
    if !read_config().servers.contains_key(&server_id.to_string()) {
        return;
    }

    if read_config().global.user_delete_on_ban {
        delete_user(server_id.to_string(), banned_user.id.into());
    }
}