use scorched::*;

use crate::{
    utils::{
        config,
        perm_check::{has_perm, PermissionLevel::Staff},
        rti::fuzzy_search_rti,
    },
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command, user_cooldown = 10)]
pub async fn search_rti(
    ctx: Context<'_>,
    #[description = "Search Phrase"] search_phrase: String,
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

    if search_phrase.is_empty() || search_phrase == " " || search_phrase.len() < 3 {
        ctx.say("You need to specify a search phrase to search; it cant be empty and it also must be at least 3 characters long.")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        return Ok(());
    }

    let relevant_objects = fuzzy_search_rti(search_phrase);

    let mut return_string = String::new();

    for rti_object in relevant_objects {
        return_string.push_str(&format!(
            "```Version: {}\nDescription: {}\nPhrase: {}```",
            rti_object.version, rti_object.description, rti_object.phrase
        ));
    }

    ctx.say(return_string)
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

    Ok(())
}
