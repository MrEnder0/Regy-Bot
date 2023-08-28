use scorched::*;

use crate::{
    utils::{
        config,
        perm_check::{has_perm, PermissionLevel::Staff},
    },
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command, user_cooldown = 10)]
pub async fn add_regex(
    ctx: Context<'_>,
    #[description = "Regex Phrase"] regex_phrase: String,
) -> Result<(), Error> {
    let server_id = ctx.guild_id().unwrap().to_string();

    if !has_perm(
        server_id.clone(),
        ctx.author().id.to_string().parse::<u64>().unwrap(),
        Staff,
    )
    .await
    {
        ctx.say("You do not have permission to use this command.")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        return Ok(());
    }

    if regex_phrase.is_empty()
        || regex_phrase == " "
        || regex_phrase.len() < 3
        || regex_phrase.len() > 350
        || regex_phrase == ".*+"
    {
        ctx.say("You need to specify a regex phrase to add; it cant be empty and it also must be between 3 and 350 characters long.")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        return Ok(());
    }

    let phrase = regex_phrase.clone();

    config::add_regex(
        server_id,
        format!("{} ", phrase),
        false,
        "No description provided.".to_string(),
        0,
    )
    .await;

    let status_message = format!("Added the regex phrase:\n||```{}```||", regex_phrase);
    ctx.say(status_message)
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

    Ok(())
}
