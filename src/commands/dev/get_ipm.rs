use crate::{
    utils::perm_check::{has_perm, PermissionLevel::Developer},
    utils::logger::LogExpect,
    Data,
    IPM
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, user_cooldown = 5)]
pub async fn get_ipm(
    ctx: Context<'_>,
) -> Result<(), Error> {
    if !has_perm(ctx.author().id.to_string().parse::<u64>().unwrap(), Developer).await {
        ctx.say("You do not have permission to use this command.")
            .await
            .log_expect("Unable to send message");
        return Ok(());
    }

    ctx.say(format!(
        "Current server IPM: {}",
        IPM.load(std::sync::atomic::Ordering::Relaxed)
    )).await.log_expect("Unable to send message");

    Ok(())
}