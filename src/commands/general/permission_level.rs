use poise::serenity_prelude as serenity;

use crate::{
    utils::{
        logger::{LogExpect, LogImportance},
        perm_check::*
    },
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(context_menu_command = "Permission Level", slash_command, user_cooldown = 15, ephemeral = true)]
pub async fn permission_level(
    ctx: Context<'_>,
    #[description = "Target User"] user: serenity::User,
) -> Result<(), Error> {
    let server_id = ctx.guild_id().unwrap().to_string();
    let user_id = user.clone().id.to_string().parse::<u64>().unwrap();

    let perm = match highest_unlocked_perm(server_id, user_id).await {
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
    .log_expect(LogImportance::Warning, "Unable to send message");

    Ok(())
}