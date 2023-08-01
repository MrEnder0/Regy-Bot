use scorched::*;
use uuid::Uuid;

use crate::{utils::toml, Data};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command, user_cooldown = 15)]
pub async fn remove_regex(
    ctx: Context<'_>,
    #[description = "Regex id"] id: String,
) -> Result<(), Error> {
    if id == " " || id.is_empty() {
        ctx.say("You need to specify a target UUID.")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        return Ok(());
    }

    let id = id.parse::<Uuid>().unwrap();

    //Check if regex with specified id exists in server in config
    let server_id = ctx.guild_id().unwrap().0.to_string();
    let block_phrases_hashmap = toml::list_regex(server_id);
    if !block_phrases_hashmap.as_ref().unwrap().contains_key(&id) {
        ctx.say("A regex phrase with that UUID does not exist.")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        return Ok(());
    }

    let server_id = ctx.guild_id().unwrap().0.to_string();
    toml::remove_regex(server_id, id);

    let status_message = format!("Removed the regex phrase with UUID: {}", id);
    ctx.say(status_message)
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

    Ok(())
}
