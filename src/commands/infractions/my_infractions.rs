use scorched::*;

use crate::{
    utils::{
        config::{self, infractions},
        type_conversions::userid_to_u64,
    },
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Returns the number of infractions you have
#[poise::command(prefix_command, slash_command, user_cooldown = 25, ephemeral = true)]
pub async fn my_infractions(ctx: Context<'_>) -> Result<(), Error> {
    let server_id = ctx.guild_id().unwrap().to_string();
    let user_id = userid_to_u64(ctx.author().id);

    if !config::server_exists(server_id.clone()).await {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("Server does not exist in config")
                    .description(
                        "If you are the owner of the server please add the server to the config using /config_setup.",
                    )
                    .color(0x8B0000)
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

        return Ok(());
    }

    let user_infractions = infractions::list_infractions(server_id, user_id.await).await;

    match user_infractions {
        Some(user_infractions) => {
            let user_infractions = if user_infractions == 0 {
                "no".to_string()
            } else {
                user_infractions.to_string()
            };

            ctx.send(|cr| {
                cr.embed(|ce| {
                    ce.title("Infractions")
                        .description(format!("You have {} infractions.", user_infractions))
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
