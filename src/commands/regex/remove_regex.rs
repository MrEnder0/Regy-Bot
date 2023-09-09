use scorched::*;
use uuid::Uuid;

use crate::{
    utils::{
        config::{self, list_regex},
        perm_check::{has_perm, PermissionLevel::Staff},
    },
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command, user_cooldown = 15)]
pub async fn remove_regex(
    ctx: Context<'_>,
    #[description = "Regex id"] id: String,
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

    if id == " " || id.is_empty() {
        ctx.say("You need to specify a target UUID.")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        return Ok(());
    }

    let id = id.parse::<Uuid>().unwrap();

    // Checks if regex with specified id exists in server in config
    let server_id = ctx.guild_id().unwrap().0.to_string();
    let block_phrases = {
        let phrases = list_regex(server_id.clone()).await;
        match phrases {
            Some(phrases) => phrases,
            None => {
                log_this(LogData {
                    importance: LogImportance::Warning,
                    message: format!("Unable to get regex phrases for server {}", server_id),
                })
                .await;

                ctx.say("This server does not exist in the database, please run `config_setup` first; if you have already done this please add a regex phrase before trying to list them.")
                    .await
                    .log_expect(LogImportance::Warning, "Unable to send message");

                return Ok(());
            }
        }
    };

    if !block_phrases.iter().any(|x| x.uuid == id.to_string()) {
        ctx.say("A regex phrase with that UUID does not exist.")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        return Ok(());
    }

    let server_id = ctx.guild_id().unwrap().0.to_string();
    config::remove_regex(server_id, id).await;

    let status_message = format!("Removed the regex phrase with UUID: {}", id);
    ctx.say(status_message)
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

    Ok(())
}
