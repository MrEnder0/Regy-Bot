use poise::{
    serenity_prelude::User
};

use crate::utils::toml::*;

pub async fn guild_ban_event(banned_user: &User) {
    if get_config().user_delete_on_ban {
        delete_user(banned_user.id.into());
    }
}