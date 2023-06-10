use poise::serenity_prelude::CreateEmbed;

use crate::{
    utils::perm_check::{has_perm, PermissionLevel::Developer},
    utils::{logger::{LogExpect, LogData, log_this, LogImportance}, toml::get_config},
    Data
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, ephemeral = true)]
pub async fn shutdown(
    ctx: Context<'_>,
) -> Result<(), Error> {
    if !has_perm(ctx.author().id.to_string().parse::<u64>().unwrap(), Developer).await {
        ctx.say("You do not have permission to use this command.")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        return Ok(());
    }

    if !get_config().allow_shutdown {
        ctx.say("Remote shutdown is not enabled on host.").await?;
        return Ok(());
    }

    let msg_author = ctx.author().id;
    println!("Shutdown from dev commands sent from {}", msg_author);

    log_this(LogData {
        importance: LogImportance::Info,
        message: format!("Shutdown from dev commands sent from {}", msg_author),
    });

    ctx.say("Shutting down...")
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

    ctx.framework().shard_manager().lock().await.shutdown_all().await;
    std::process::exit(0);
}