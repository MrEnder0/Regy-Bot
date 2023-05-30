use crate::{
    utils::logger::LogExpect,
    utils::toml,
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command, user_cooldown = 10, required_permissions = "ADMINISTRATOR", ephemeral = true)]
pub async fn add_regex(
    ctx: Context<'_>,
    #[description = "Regex Phrase"] regex_phrase: String
) -> Result<(), Error> {
    if regex_phrase.is_empty() || regex_phrase == " " || regex_phrase.len() < 3
    {
        ctx.say("You need to specify a regex phrase to add; it cant be empty and it also cant be less than 3 characters long.")
            .await
            .log_expect("Unable to send message");
        return Ok(());
    }

    toml::add_block_phrase(regex_phrase.clone());

    let status_message = format!(
        "Added the regex phrase:\n||```{}```||",
        regex_phrase
    );
    ctx.say(status_message)
        .await
        .log_expect("Unable to send message");
    Ok(())
}