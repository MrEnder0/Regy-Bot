use scorched::*;

use crate::{
    utils::config::{add_regex, list_regex, server_exists},
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
    if !server_exists(target_server_id.clone()).await {
        ctx.say("This server does not exist in the database.")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
    } else {
        //check if current server exists in database
        if !server_exists(ctx.guild_id().unwrap().0.to_string()).await {
            ctx.say("This server does not exist in the database, please run `config_setup` first.")
                .await
                .log_expect(LogImportance::Warning, "Unable to send message");

            return Ok(());
        }

        let target_block_phrases = match { list_regex(target_server_id.clone()).await } {
            Some(phrases) => phrases,
            None => {
                log_this(LogData {
                    importance: LogImportance::Warning,
                    message: format!(
                        "Unable to get regex phrases for server {}",
                        target_server_id
                    ),
                })
                .await;

                ctx.say(format!("Unable to get regex phrases for server {} double check the id and make sure they have phrases to clone.", target_server_id))
                    .await
                    .log_expect(LogImportance::Warning, "Unable to send message");

                return Ok(());
            }
        };

        for phrase in target_block_phrases.iter() {
            add_regex(
                ctx.guild_id().unwrap().0.to_string(),
                phrase.phrase.clone(),
                false,
                "No description provided.".to_string(),
                0,
            )
            .await;
        }

        ctx.say("Regex phrases cloned successfully.")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
    }

    return Ok(());
}
