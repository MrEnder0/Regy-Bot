use scorched::*;

use crate::{
    utils::{config, type_conversions::userid_to_u64},
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command, user_cooldown = 25, ephemeral = true)]
pub async fn my_infractions(ctx: Context<'_>) -> Result<(), Error> {
    let server_id = ctx.guild_id().unwrap().to_string();
    let user_id = userid_to_u64(ctx.author().id);

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

    let user_infractions = config::list_infractions(server_id, user_id).await;

    match user_infractions {
        Some(user_infractions) => {
            ctx.send(|cr| {
                cr.embed(|ce| {
                    ce.title("Infractions")
                        .description(format!("You have {} infraction(s).", user_infractions))
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
        None => {
            ctx.send(|cr| {
                cr.embed(|ce| {
                    ce.title("Infractions")
                        .description("You have no infractions.")
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
    }

    Ok(())
}
