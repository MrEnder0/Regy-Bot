use crate::{
    utils::perm_check::{has_perm, PermissionLevel::Developer},
    utils::logger::LogExpect,
    Data
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, ephemeral = true)]
pub async fn echo(
    ctx: Context<'_>,
    #[description = "Message"] echo_msg: String,
) -> Result<(), Error> {
    if !has_perm(ctx.author().id.to_string().parse::<u64>().unwrap(), Developer).await {
        ctx.say("You do not have permission to use this command.")
            .await
            .log_expect("Unable to send message");
        return Ok(());
    }

    if ctx.guild_id().is_none() {
        ctx.say("This command can only be used in a server.")
            .await
            .log_expect("Unable to send message");
        return Ok(());
    }

    let channel_id = ctx.channel_id();
    channel_id.say(ctx, echo_msg).await.log_expect("Unable to send message");

    ctx.say("Message sent.").await.log_expect("Unable to send message");

    Ok(())
}