use scorched::*;

use crate::{
    utils::perm_check::{has_perm, PermissionLevel::Developer},
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Uploads all log files in the log dir
#[poise::command(slash_command, global_cooldown = 5)]
pub async fn upload_logs(ctx: Context<'_>) -> Result<(), Error> {
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

    if !std::path::Path::new("temp/logs").exists() {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("Log upload failed")
                    .description("There is not a log folder currently.")
                    .color(0x8B0000)
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

        return Ok(());
    }

    if std::fs::read_dir("temp/logs").unwrap().count() == 0 {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("Log upload failed")
                    .description("There are no logs in the log folder.")
                    .color(0x8B0000)
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

        return Ok(());
    }

    for file in std::fs::read_dir("logs").unwrap() {
        if !file.as_ref().unwrap().path().ends_with(".log") {
            continue;
        }

        let log_file = std::fs::read_to_string(file.unwrap().path())
            .log_expect(LogImportance::Warning, "Unable to read log file");
        let file_name = log_file.split(' ').collect::<Vec<&str>>()[0];

        ctx.channel_id()
            .send_files(ctx, vec![(log_file.as_bytes(), "logs.log")], |m| {
                m.content(format!("Logs for {}:", file_name))
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to upload log file");
    }

    if std::path::Path::new("temp/logs/update_helper").exists() {
        for file in std::fs::read_dir("temp/logs/update_helper").unwrap() {
            if !file.as_ref().unwrap().path().ends_with(".log") {
                continue;
            }

            let log_file = std::fs::read_to_string(file.unwrap().path())
                .log_expect(LogImportance::Warning, "Unable to read log file");
            let file_name = log_file.split(' ').collect::<Vec<&str>>()[0];

            ctx.channel_id()
                .send_files(ctx, vec![(log_file.as_bytes(), "logs.log")], |m| {
                    m.content(format!("Logs for update_helper {}:", file_name))
                })
                .await
                .log_expect(LogImportance::Warning, "Unable to upload log file");
        }
    }

    if std::path::Path::new("logs.zip").exists() {
        let log_archive = std::fs::read_to_string("logs.zip")
                .log_expect(LogImportance::Warning, "Unable to read log file");

        ctx.channel_id()
            .send_files(ctx, vec![(log_archive.as_bytes(), "logs.zip")], |m| {
                m.content("Logs archive:")
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to upload log archive");

        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("Uploaded log archive")
                    .description("Found and uploaded log archive")
                    .color(0x8B0000)
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");
    }

    Ok(())
}
