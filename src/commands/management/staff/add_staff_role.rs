use poise::serenity_prelude as serenity;
use scorched::*;

use crate::{
    utils::{config, type_conversions::roleid_to_u64},
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(
    slash_command,
    guild_cooldown = 5,
    required_permissions = "ADMINISTRATOR"
)]
pub async fn add_staff_role(
    ctx: Context<'_>,
    #[description = "Target Role"] role: serenity::Role,
) -> Result<(), Error> {
    let roleid = role.clone().id;

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

    let add_staff_role_status = config::add_staff_role(
        ctx.guild_id().unwrap().0.to_string(),
        roleid_to_u64(roleid).await,
    )
    .await;

    match add_staff_role_status {
        true => {
            ctx.send(|cr| {
                cr.embed(|ce| {
                    ce.title("Added staff role")
                        .description(format!("Added {} to staff roles", role.clone().name))
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
        false => {
            ctx.send(|cr| {
                cr.embed(|ce| {
                    ce.title("Unable to add staff role")
                        .description(format!(
                            "Unable to add {} to staff roles",
                            role.clone().name
                        ))
                        .color(0x8B0000)
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
    }

    Ok(())
}
