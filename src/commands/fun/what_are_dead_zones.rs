use scorched::*;

use crate::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command, channel_cooldown = 60)]
pub async fn what_are_dead_zones(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(|cr| {
        cr.embed(|ce| {
            ce.title("What is a dead zone?")
                .description("Dead zones are a feature of Regy that allow you to specify a channel that Regy will loosen moderation in, Regy will still provide infractions for auto mod related things, but will not provide infractions or block any messages that are not controlled by auto mod. This is useful for if you want to have a 18+ channel or a channel where you can say whatever you want.")
        })
    })
    .await
    .log_expect(LogImportance::Warning, "Unable to send message");

    Ok(())
}
