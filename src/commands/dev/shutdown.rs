use scorched::*;

use crate::{
    utils::{
        perm_check::{has_perm, PermissionLevel::Developer},
        toml::read_config,
    },
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, ephemeral = true)]
pub async fn shutdown(ctx: Context<'_>) -> Result<(), Error> {
    let server_id = ctx.guild_id().unwrap().to_string();

    if !has_perm(
        server_id,
        ctx.author().id.to_string().parse::<u64>().unwrap(),
        Developer,
    )
    .await
    {
        ctx.say("You do not have permission to use this command.")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        return Ok(());
    }

    if !read_config().global.allow_shutdown {
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

    ctx.framework()
        .shard_manager()
        .lock()
        .await
        .shutdown_all()
        .await;
    std::process::exit(0);
}
