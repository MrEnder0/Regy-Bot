use crate::{
    utils::{
        logger::{LogExpect, LogImportance},
        toml::{add_regex, list_regex, server_exists},
    },
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(
    prefix_command,
    slash_command,
    guild_cooldown = 240,
    required_permissions = "ADMINISTRATOR"
)]
pub async fn config_clone_regex(
    ctx: Context<'_>,
    #[description = "Guild ID"] guild_id: String,
) -> Result<(), Error> {
    //check if server exists in database
    if !server_exists(guild_id.clone()) {
        ctx.say("This server does not exist in the database.")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
    } else {
        let server_id = ctx.guild_id().unwrap().0.to_string();
        let block_phrases_hashmap = list_regex(server_id);
        for phrase in block_phrases_hashmap.as_ref().unwrap().values() {
            let phrase = phrase.to_string();
            add_regex(guild_id.clone(), phrase);
        }

        ctx.say("Regex phrases cloned.")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
    }

    return Ok(());
}
