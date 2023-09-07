use scorched::*;

use crate::{
    utils::{
        config::read_config,
        perm_check::{has_perm, PermissionLevel::Developer},
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
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("You do not have permission to use this command.")
                    .field("Lacking permissions:", "Developer", false)
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

        return Ok(());
    }

    if !read_config().await.global.allow_shutdown {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("Unable to remote shutdown").field(
                    "Reason:",
                    "Remote shutdown is not enabled on host",
                    false,
                )
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

        return Ok(());
    }

    log_this(LogData {
        importance: LogImportance::Info,
        message: format!("Shutdown from dev commands sent from {}", ctx.author().id),
    })
    .await;

    ctx.send(|cr| cr.embed(|ce| ce.title("Shutting down...")))
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
