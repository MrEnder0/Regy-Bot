use poise::serenity_prelude::Channel;

use crate::{
    utils::{
        logger::{LogExpect, LogImportance},
        toml::{gen_server, server_exists}
    },
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command, guild_cooldown = 240, required_permissions = "ADMINISTRATOR")]
pub async fn config_server(
    ctx: Context<'_>,
    #[description = "Log channel"] log_channel: Channel,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap().to_string();
    let log_channel_id = log_channel.id().to_string().parse::<u64>().unwrap();

    if server_exists(guild_id.clone()) {
        ctx.say("This server already exists in the database.")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        return Ok(());
    }

    gen_server(guild_id, log_channel_id);

    ctx.say("Server added to database.")
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

    Ok(())
}