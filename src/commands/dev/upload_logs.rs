use crate::{
    utils::perm_check::{has_perm, PermissionLevel::Developer},
    utils::logger::{LogExpect, LogImportance, LogData, log_this},
    Data
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, user_cooldown = 5)]
pub async fn upload_logs(
    ctx: Context<'_>,
) -> Result<(), Error> {
    if !has_perm(ctx.author().id.to_string().parse::<u64>().unwrap(), Developer).await {
        ctx.say("You do not have permission to use this command.")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");

        return Ok(());
    }

    if std::path::Path::new("regy.log").exists() {
        ctx.say("Uploading log file, this may take a few seconds...")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");

        let log_file = std::fs::read_to_string("regy.log").log_expect(LogImportance::Error, "Unable to read log file");
        let log_file = log_file.as_bytes();

        ctx.channel_id()
            .send_files(ctx, vec![(log_file, "regy.log")], |m| {
                m.content("Log file:")
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to upload log file");

        Ok(())
    } else {
        log_this(LogData {
            importance: LogImportance::Warning,
            message: "Log file does not exist".to_string(),
        });

        ctx.say("Log file does not exist")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");

        Ok(())
    }
}