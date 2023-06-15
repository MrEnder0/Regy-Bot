use crate::{
    utils::{
        perm_check::{has_perm, PermissionLevel::Developer},
        logger::{LogExpect, LogImportance}
    },
    Data,
    IPM
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, guild_cooldown = 5)]
pub async fn reset_ipm(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let server_id = ctx.guild_id().unwrap().to_string();

    if !has_perm(server_id, ctx.author().id.to_string().parse::<u64>().unwrap(), Developer).await {
        ctx.say("You do not have permission to use this command.")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        return Ok(());
    }

    IPM.store(0, std::sync::atomic::Ordering::Relaxed);
    ctx.say("Reset server IPM to 0").await.log_expect(LogImportance::Warning, "Unable to send message");

    Ok(())
}