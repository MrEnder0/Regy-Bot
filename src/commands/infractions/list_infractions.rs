use poise::serenity_prelude as serenity;

use crate::{
    utils::perm_check::{has_perm, PermissionLevel::Staff},
    utils::{
        type_conversions::userid_to_u64,
        logger::LogExpect,
        toml
    },
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(context_menu_command = "List Infractions", slash_command, user_cooldown = 5, ephemeral = true)]
pub async fn list_infractions(
    ctx: Context<'_>,
    #[description = "Target User"] user: serenity::User,
) -> Result<(), Error> {
    if !has_perm(ctx.author().id.to_string().parse::<u64>().unwrap(), Staff).await {
        ctx.say("You do not have permission to use this command.")
            .await
            .log_expect("Unable to send message");
        return Ok(());
    }

    let userid = user.clone().id;
    let infraction_count = toml::list_infractions(userid_to_u64(userid));
    ctx.say(format!(
        "{} has {} infractions",
        user.clone().name,
        infraction_count
    ))
    .await
    .log_expect("Unable to send message");
    Ok(())
}