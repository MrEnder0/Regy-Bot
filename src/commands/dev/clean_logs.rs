use scorched::*;

use crate::{
    utils::perm_check::{has_perm, PermissionLevel::Developer},
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, global_cooldown = 10)]
pub async fn clean_logs(ctx: Context<'_>) -> Result<(), Error> {
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

    if std::path::Path::new("logs").exists() {
        std::fs::remove_dir_all("logs")
            .log_expect(LogImportance::Warning, "Unable to delete log folder");
        ctx.say("Log folder deleted")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
    } else {
        log_this(LogData {
            importance: LogImportance::Info,
            message: "Log folder does not exist".to_string(),
        })
        .await;

        ctx.say("Log folder does not exist")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
    }

    if std::path::Path::new("logs.zip").exists() {
        std::fs::remove_file("logs.zip")
            .log_expect(LogImportance::Warning, "Unable to delete found log archive");
        ctx.say("Found and deleted log archive")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
    }

    Ok(())
}
