use scorched::*;

use crate::{
    utils::toml::{add_regex, list_regex, server_exists},
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
    #[description = "Guild ID"] target_server_id: String,
) -> Result<(), Error> {
    //check if server exists in database
    if !server_exists(target_server_id.clone()) {
        ctx.say("This server does not exist in the database.")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
    } else {
        //check if current server exists in database
        if !server_exists(ctx.guild_id().unwrap().0.to_string()) {
            ctx.say("This server does not exist in the database, please run `config_setup` first.")
                .await
                .log_expect(LogImportance::Warning, "Unable to send message");

            return Ok(());
        }

        let target_block_phrases = list_regex(target_server_id);

        for item in target_block_phrases.as_ref().unwrap().iter() {
            let phrase = item.1.to_string();
            add_regex(ctx.guild_id().unwrap().0.to_string(), phrase);
        }

        ctx.say("Regex phrases cloned.")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
    }

    return Ok(());
}
