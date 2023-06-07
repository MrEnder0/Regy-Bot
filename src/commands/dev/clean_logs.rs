use crate::{
    utils::{
        perm_check::{has_perm, PermissionLevel::Developer},
        logger::{LogExpect, LogData, LogImportance, log_this}
    },
    Data
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, user_cooldown = 5)]
pub async fn clean_logs(
    ctx: Context<'_>,
) -> Result<(), Error> {
    if !has_perm(ctx.author().id.to_string().parse::<u64>().unwrap(), Developer).await {
        ctx.say("You do not have permission to use this command.")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");

        return Ok(())
    }

    if std::path::Path::new("regy.log").exists() {
        std::fs::remove_file("regy.log")
            .log_expect(LogImportance::Warning, "Unable to delete log filet");
        ctx.say("Log file deleted")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");

        Ok(())
    } else {
        let data = LogData {
            importance: LogImportance::Error,
            message: "Log file does not exist".to_string(),
        };
        log_this(data);

        ctx.say("Log file does not exist")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");

        Ok(())
    }
}