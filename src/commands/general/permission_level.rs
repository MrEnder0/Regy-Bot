use poise::serenity_prelude as serenity;

use crate::{
    utils::logger::LogExpect,
    utils::perm_check::*,
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(context_menu_command = "Permission Level", slash_command, user_cooldown = 15)]
pub async fn permission_level(
    ctx: Context<'_>,
    #[description = "Target User"] user: serenity::User,
) -> Result<(), Error> {
    let userid = user.clone().id.to_string();

    let perm = match highest_unlocked_perm(userid.parse::<u64>().unwrap()).await {
        PermissionLevel::User => "User",
        PermissionLevel::Staff => "Staff",
        PermissionLevel::Developer => "Developer",
    };

    //reply with the users highest unlocked permission level
    ctx.say(format!(
        "The highest permission **{}** has is **{}**",
        user.clone().name,
        perm
    ))
    .await
    .log_expect("Unable to send message");

    Ok(())
}