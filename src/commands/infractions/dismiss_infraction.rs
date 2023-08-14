use poise::serenity_prelude as serenity;
use scorched::*;

use crate::{
    utils::{
        config,
        perm_check::{has_perm, PermissionLevel::Staff},
        type_conversions::userid_to_u64,
    },
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(
    context_menu_command = "Dismiss Infraction",
    slash_command,
    user_cooldown = 5
)]
pub async fn dismiss_infraction(
    ctx: Context<'_>,
    #[description = "Target User"] user: serenity::User,
) -> Result<(), Error> {
    let server_id = ctx.guild_id().unwrap().to_string();

    if !has_perm(
        server_id,
        ctx.author().id.to_string().parse::<u64>().unwrap(),
        Staff,
    )
    .await
    {
        ctx.say("You do not have permission to use this command.")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        return Ok(());
    }

    let server_id = ctx.guild_id().unwrap().0.to_string();
    let userid = user.clone().id;

    if !config::server_exists(server_id.clone()) {
        ctx.say("Server does not exist in config")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        return Ok(());
    }

    config::dismiss_infraction(server_id, userid_to_u64(userid));
    ctx.say(format!("Dismissed a infraction from {}", user.clone().name))
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

    user.dm(ctx, |m| {
        m.content(format!(
            "You have has a infraction dismissed from {}",
            ctx.author().name
        ))
    })
    .await
    .log_expect(LogImportance::Warning, "Unable to dm user");

    Ok(())
}
