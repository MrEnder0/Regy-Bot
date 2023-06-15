use poise::serenity_prelude as serenity;

use crate::{
    utils::{
        perm_check::{has_perm, PermissionLevel::Staff},
        type_conversions::userid_to_u64,
        logger::{LogExpect, LogImportance},
        toml
    },
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(context_menu_command = "List Infractions", slash_command, user_cooldown = 8)]
pub async fn list_infractions(
    ctx: Context<'_>,
    #[description = "Target User"] user: serenity::User,
) -> Result<(), Error> {
    let server_id = ctx.guild_id().unwrap().to_string();

    if !has_perm(server_id, ctx.author().id.to_string().parse::<u64>().unwrap(), Staff).await {
        ctx.say("You do not have permission to use this command.")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        return Ok(());
    }

    let server_id = ctx.guild_id().unwrap().0.to_string();
    let userid = user.clone().id;

    let infraction_count = toml::list_infractions(server_id, userid_to_u64(userid));

    match infraction_count {
        Some(infraction_count) => {
            let infractions_message = format!("User {} has {} infraction(s).", user.clone().name, infraction_count);
            ctx.say(infractions_message)
                .await
                .log_expect(LogImportance::Warning, "Unable to send message");
        },
        None => {
            ctx.say("You have no infractions.")
                .await
                .log_expect(LogImportance::Warning, "Unable to send message");
        }
    }

    Ok(())
}