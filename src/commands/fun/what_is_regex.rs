use scorched::*;

use crate::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Explains what regex is
#[poise::command(prefix_command, slash_command, channel_cooldown = 60)]
pub async fn what_is_regex(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(|cr| {
        cr.embed(|ce| {
            ce.title("What is regex?")
                .description("Regex, short for Regular Expression, is a sequence of characters that defines a search pattern. It is used to search, replace, and manipulate text in programming and text editing tools. It provides a powerful and flexible way to match and manipulate strings of text based on certain patterns or rules.")
        })
    })
    .await
    .log_expect(LogImportance::Warning, "Unable to send message");

    Ok(())
}
