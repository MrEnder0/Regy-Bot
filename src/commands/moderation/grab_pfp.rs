use poise::serenity_prelude as serenity;

use crate::{
    utils::perm_check::{has_perm, PermissionLevel::Moderator},
    utils::logger::LogExpect,
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command, user_cooldown = 5)]
pub async fn grab_pfp(
    ctx: Context<'_>,
    #[description = "Target User"] user: serenity::User,
) -> Result<(), Error> {
    if !has_perm(ctx.author().id.to_string().parse::<u64>().unwrap(), Moderator).await {
        ctx.say("You do not have permission to use this command.")
            .await
            .log_expect("Unable to send message");
        return Ok(());
    }

    ctx.say(user.face())
    .await
    .log_expect("Unable to send message");
    Ok(())
}