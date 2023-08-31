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
        ctx.say("Server does not exist in config")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        return Ok(());
    }

    let user_infractions = config::list_infractions(server_id, user_id).await;

    match user_infractions {
        Some(user_infractions) => {
            let infractions_message = format!("You have {} infraction(s).", user_infractions);
            ctx.say(infractions_message)
                .await
                .log_expect(LogImportance::Warning, "Unable to send message");
        }
        None => {
            ctx.say("You have no infractions.")
                .await
                .log_expect(LogImportance::Warning, "Unable to send message");
        }
    }

    Ok(())
}
