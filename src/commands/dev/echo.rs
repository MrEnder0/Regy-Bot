use scorched::*;

use crate::{
    utils::perm_check::{has_perm, PermissionLevel::Developer},
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, ephemeral = true)]
pub async fn echo(
    ctx: Context<'_>,
    #[description = "Message"] echo_msg: String,
) -> Result<(), Error> {
    let server_id = ctx.guild_id().unwrap().to_string();

    if !has_perm(
        server_id,
        ctx.author().id.to_string().parse::<u64>().unwrap(),
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

    let channel_id = ctx.channel_id();
    channel_id
        .say(ctx, echo_msg)
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

    ctx.send(|cr| {
        cr.embed(|ce| {
            ce.title("Echoed message")
                .description("Your provided message has been echoed by Regy.")
        })
    })
    .await
    .log_expect(LogImportance::Warning, "Unable to send message");

    Ok(())
}
