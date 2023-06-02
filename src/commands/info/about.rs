use crate::{
    utils::logger::LogExpect,
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Regy is a Discord regex auto-moderation bot developed mainly by Mr.Ender#0001 with contributions by Endercass#0001 and 1984#0001, pfp by 1984.").await.log_expect("Unable to send message");
    Ok(())
}