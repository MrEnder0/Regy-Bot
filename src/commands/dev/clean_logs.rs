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
        Vec::new(),
        Developer,
    )
    .await
    {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("You do not have permission to use this command.")
                    .field("Lacking permissions:", "Developer", false)
                    .color(0x8B0000)
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

        return Ok(());
    }

    if std::path::Path::new("logs").exists() {
        std::fs::remove_dir_all("logs")
            .log_expect(LogImportance::Warning, "Unable to delete log folder");

        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("Log folder")
                    .description("Found and deleted log folder")
                    .thumbnail("https://raw.githubusercontent.com/MrEnder0/Regy-Bot/master/.github/assets/trashcan.png")
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");
    } else {
        log_this(LogData {
            importance: LogImportance::Info,
            message: "Log folder does not exist".to_string(),
        })
        .await;

        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("Log folder")
                    .description("Log folder does not exist, unable to delete")
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");
    }

    if std::path::Path::new("logs.zip").exists() {
        std::fs::remove_file("logs.zip")
            .log_expect(LogImportance::Warning, "Unable to delete found log archive");

        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("Log archive")
                    .description("Found and deleted log archive")
                    .thumbnail("https://raw.githubusercontent.com/MrEnder0/Regy-Bot/master/.github/assets/trashcan.png")
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");
    }

    Ok(())
}
