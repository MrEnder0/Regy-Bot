use poise::serenity_prelude::Channel;
use scorched::*;

use crate::{
    utils::config::{gen_server, server_exists},
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(
    prefix_command,
    slash_command,
    guild_cooldown = 240,
    required_permissions = "ADMINISTRATOR"
)]
pub async fn config_setup(
    ctx: Context<'_>,
    #[description = "Log channel"] log_channel: Channel,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().to_string();
    let log_channel_id = log_channel.id().to_string().parse::<u64>().unwrap();

    if server_exists(guild_id.clone()).await {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("Unable to add server to database.")
                    .description(
                        "The current server already exists in the database, therefore it does not need to be added again.",
                    )
                    .color(0x8B0000)
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

        return Ok(());
    }

    gen_server(guild_id, log_channel_id).await;

    ctx.send(|cr| {
        cr.embed(|ce| {
            ce.title("Welcome to Regy bot! 🎉")
                .description("The current server has been added to the database.")
                .footer(|fe| fe.text("For a list of commands, use /help <category>"))
        })
    })
    .await
    .log_expect(LogImportance::Warning, "Unable to send message");

    Ok(())
}
