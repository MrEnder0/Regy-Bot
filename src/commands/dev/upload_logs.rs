use scorched::*;

use crate::{
    utils::perm_check::{has_perm, PermissionLevel::Developer},
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

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

    if !std::path::Path::new("logs").exists() {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("Log upload failed")
                    .description("There currently is no log folder.")
                    .color(0x8B0000)
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

        return Ok(());
    }

    if std::fs::read_dir("logs").unwrap().count() == 0 {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("Log upload failed")
                    .description("There currently are no log files in the log dir.")
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

    Ok(())
}
