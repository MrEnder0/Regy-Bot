use crate::{
    utils::logger::{LogExpect, LogImportance},
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command, channel_cooldown = 60)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Regy is a Discord regex auto-moderation bot which uses user defined regex patterns to help moderate and prevent raids in your server, it is developed mainly by Mr.Ender#0001 with contributions by Endercass#0001 and 1984#0001, pfp by 1984.").await.log_expect(LogImportance::Warning, "Unable to send message");
    Ok(())
}