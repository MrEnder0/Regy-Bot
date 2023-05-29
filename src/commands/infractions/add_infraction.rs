use poise::serenity_prelude as serenity;

use crate::{
    utils::perm_check::{has_perm, PermissionLevel::Staff},
    utils::type_conversions::userid_to_u64,
    utils::logger::LogExpect,
    utils::toml,
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(context_menu_command = "Add Infraction", slash_command, user_cooldown = 5)]
pub async fn add_infraction(
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
    toml::add_infraction(userid_to_u64(userid));
    ctx.say(format!(
        "Added an infraction to {}",
        user.clone().name
    ))
    .await
    .log_expect("Unable to send message");
    Ok(())
}