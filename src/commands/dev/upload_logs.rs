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
        Developer,
    )
    .await
    {
        ctx.say("You do not have permission to use this command.")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");

        return Ok(());
    }

    for file in std::fs::read_dir("logs").unwrap() {
        let log_file = std::fs::read_to_string(file.unwrap().path())
            .log_expect(LogImportance::Warning, "Unable to read log file");
        let file_name = log_file.split(" ").collect::<Vec<&str>>()[0];
        ctx.channel_id()
            .send_files(ctx, vec![(log_file.as_bytes(), "logs.zip")], |m| {
                m.content(format!("Logs for {}:", file_name))
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to upload log file");
    }

    Ok(())
}
