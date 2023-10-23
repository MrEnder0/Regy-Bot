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
    context_menu_command = "List Infractions",
    slash_command,
    user_cooldown = 8
)]
pub async fn list_infractions(
    ctx: Context<'_>,
    #[description = "Target User"] user: serenity::User,
) -> Result<(), Error> {
    let server_id = ctx.guild_id().unwrap().to_string();

    let member = match ctx.guild_id().unwrap().member(&ctx, ctx.author().id).await {
        Ok(user) => user,
        Err(_) => {
            ctx.send(|cr| {
                cr.embed(|ce| {
                    ce.title("Unable to get user")
                        .description("Please try again later.")
                        .color(0x8B0000)
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");

            return Ok(());
        }
    };

    if !has_perm(
        server_id,
        ctx.author().id.to_string().parse::<u64>().unwrap(),
        member.roles.clone(),
        Staff,
    )
    .await
    {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("You do not have permission to use this command.")
                    .field("Lacking permissions:", "Staff", false)
                    .color(0x8B0000)
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

        return Ok(());
    }

    let server_id = ctx.guild_id().unwrap().0.to_string();
    let userid = user.clone().id;

    if !config::server_exists(server_id.clone()).await {
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

    let infraction_count = config::list_infractions(server_id, userid_to_u64(userid).await).await;

    match infraction_count {
        Some(infraction_count) => {
            ctx.send(|cr| {
                cr.embed(|ce| {
                    ce.title("Infractions").description(format!(
                        "User {} has {} infraction(s).",
                        user.clone().name,
                        infraction_count
                    ))
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
        None => {
            ctx.send(|cr| {
                cr.embed(|ce| {
                    ce.title("Infractions")
                        .description(format!("User {} has no infractions.", user.clone().name))
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
    }

    Ok(())
}
