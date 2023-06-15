use crate::{
    utils::{
        logger::{LogExpect, LogImportance},
        toml
    },
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command, user_cooldown = 10, required_permissions = "ADMINISTRATOR")]
pub async fn add_regex(
    ctx: Context<'_>,
    #[description = "Regex Phrase"] regex_phrase: String
) -> Result<(), Error> {
    if regex_phrase.is_empty() || regex_phrase == " " || regex_phrase.len() < 3 || regex_phrase.len() > 350 {
        ctx.say("You need to specify a regex phrase to add; it cant be empty and it also must be between 3 and 350 characters long.")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        return Ok(());
    }

    let server_id = ctx.guild_id().unwrap().0.to_string();
    let phrase = regex_phrase.clone();

    toml::add_regex(server_id, phrase);

    let status_message = format!(
        "Added the regex phrase:\n||```{}```||",
        regex_phrase
    );
    ctx.say(status_message)
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

    Ok(())
}