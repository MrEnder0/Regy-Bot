use poise::serenity_prelude as serenity;
use scorched::*;

use crate::{
    utils::{config, type_conversions::userid_to_u64},
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(
    slash_command,
    guild_cooldown = 5,
    required_permissions = "ADMINISTRATOR"
)]
pub async fn remove_staff(
    ctx: Context<'_>,
    #[description = "Target User"] user: serenity::User,
) -> Result<(), Error> {
    let server_id = ctx.guild_id().unwrap().0.to_string();
    let userid = user.clone().id;

    if !config::server_exists(server_id.clone()) {
        ctx.say("Server does not exist in config")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        return Ok(());
    }

    let remove_staff_status = config::remove_staff(server_id, userid_to_u64(userid));

    match remove_staff_status {
        true => {
            ctx.say(format!("Removed {} from staff", user.clone().name))
                .await
                .log_expect(LogImportance::Warning, "Unable to send message");

            user.dm(ctx, |m| {
                m.content(format!(
                    "You have been revoked of Regy staff permissions from {} inside {}.",
                    ctx.author().name,
                    ctx.guild().unwrap().name
                ))
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to dm user");
        }
        false => {
            ctx.say(format!("{} is not staff", user.clone().name))
                .await
                .log_expect(LogImportance::Warning, "Unable to send message");
        }
    }

    Ok(())
}
