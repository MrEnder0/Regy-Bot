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
pub async fn add_staff(
    ctx: Context<'_>,
    #[description = "Target User"] user: serenity::User,
) -> Result<(), Error> {
    let userid = user.clone().id;

    if !config::server_exists(ctx.guild_id().unwrap().0.to_string()).await {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("Server does not exist in config")
                    .description(
                        "Please add the server to the config using /config_setup if you are the owner of the server.",
                    )
                    .color(0x8B0000)
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

        return Ok(());
    }

    let add_staff_status = config::add_staff(
        ctx.guild_id().unwrap().0.to_string(),
        userid_to_u64(userid).await,
    )
    .await;

    match add_staff_status {
        true => {
            ctx.send(|cr| {
                cr.embed(|ce| {
                    ce.title("Added staff")
                        .description(format!("Added {} to staff", user.clone().name))
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");

            user.dm(ctx, |m| {
                m.embed(|ce| {
                    ce.title("Received staff perms").description(format!(
                        "You have been given Regy staff perms in {}",
                        ctx.guild_id().unwrap().0.to_string()
                    ))
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to dm user");
        }
        false => {
            ctx.send(|cr| {
                cr.embed(|ce| {
                    ce.title("Failed to add staff")
                        .description(format!("Failed to add {} to staff", user.clone().name))
                        .color(0x8B0000)
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
    }

    Ok(())
}
