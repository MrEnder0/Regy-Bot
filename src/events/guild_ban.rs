use poise::serenity_prelude::{GuildId, User};

use crate::utils::config::*;

pub async fn guild_ban_event(server_id: GuildId, banned_user: &User) {
    // Checks if server exists in config
    if !read_config()
        .await
        .servers
        .contains_key(&server_id.to_string())
    {
        return
    }

    if read_config().await.global.user_delete_on_ban {
        delete_user(server_id.to_string(), banned_user.id.into()).await;
    }
}
