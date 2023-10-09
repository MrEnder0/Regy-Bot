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
    context_menu_command = "Add Infraction",
    slash_command,
    user_cooldown = 5
)]
pub async fn add_infraction(
    ctx: Context<'_>,
    #[description = "Target User"] user: serenity::User,
) -> Result<(), Error> {
    let server_id = ctx.guild_id().unwrap().to_string();

    if !has_perm(
        server_id.clone(),
        ctx.author().id.to_string().parse::<u64>().unwrap(),
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

    let userid = user.clone().id;

    if !config::server_exists(server_id.clone()).await {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("Server does not exist in config")
                    .description("Please add the server to the config using /config_setup if you are the owner of the server.")
                    .color(0x8B0000)
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

        return Ok(());
    }

    config::add_infraction(server_id, userid_to_u64(userid)).await;

    ctx.send(|cr| {
        cr.embed(|ce| {
            ce.title("Added infraction").field(
                "The following user has received an infraction:",
                user.clone().name,
                false,
            )
        })
    })
    .await
    .log_expect(LogImportance::Warning, "Unable to send message");

    match Some(ctx.guild_id().unwrap().name(ctx)) {
        Some(_) => {
            user.dm(ctx, |m| {
                m.embed(|me| {
                    me.title("You have received an infraction")
                        .description(format!("You have received an infraction from a staff member inside {}. Please contact the server for more information.", ctx.guild_id().unwrap().name(ctx).unwrap()))
                        .color(0x8B0000)
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to dm user");
        }
        None => {
            user.dm(ctx, |m| {
                m.embed(|me| {
                    me.title("You have received an infraction")
                        .description("You have received an infraction inside an unknown sever.")
                        .color(0x8B0000)
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to dm user");
        }
    }

    Ok(())
}
