use scorched::*;

use crate::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command)]
pub async fn random_word(ctx: Context<'_>) -> Result<(), Error> {
    //send api request to https://random-word-api.herokuapp.com/word?lang=en
    let word = reqwest::get("https://random-word-api.herokuapp.com/word?lang=en")
        .await
        .log_expect(LogImportance::Warning, "Unable to get random word")
        .text()
        .await
        .log_expect(LogImportance::Warning, "Unable to get random word")
        .to_lowercase();

    ctx.send(|cr| {
        cr.embed(|ce| {
            ce.title("This is a word!")
                .field("Random word:", &word[2..word.len() - 2], false)
        })
    })
    .await
    .log_expect(LogImportance::Warning, "Unable to send message");

    Ok(())
}
