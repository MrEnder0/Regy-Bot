use scorched::*;

use crate::{utils::toml, Data};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

struct Regex {
    id: String,
    phrase: String,
}

#[poise::command(
    prefix_command,
    slash_command,
    user_cooldown = 45,
    channel_cooldown = 30,
)]
pub async fn list_regex(ctx: Context<'_>) -> Result<(), Error> {
    let status_msg = ctx
        .say("Sending regex phrases this may take a few seconds...")
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

    let server_id = ctx.guild_id().unwrap().0.to_string();
    let block_phrases = match { toml::list_regex(server_id) } {
        Some(block_phrases) => block_phrases,
        None => {
            ctx.say("This server does not exist in the database, please run `config_setup` first; if you have already done this please add a regex phrase before trying to list them.")
                .await
                .log_expect(LogImportance::Warning, "Unable to send message");
            return Ok(());
        }
    };

    let mut formatted_blocked_phrases = String::new();
    for item in block_phrases.iter() {
        let regex = Regex {
            id: item.0.to_string(),
            phrase: item.1.to_string(),
        };

        formatted_blocked_phrases.push_str(&format!("{} | {}\n", regex.id, regex.phrase));
    }

    let status_message = format!("The current regex being used are **[WARNING CONTAINS SENSITIVE MESSAGES]**\n||```                  ID                 | REGEX\n{}```||", formatted_blocked_phrases);
    let channel_id = ctx.channel_id();

    if status_message.len() > 2000 {
        channel_id
            .say(
                ctx,
                "The current regex being used are **[WARNING CONTAINS SENSITIVE MESSAGES]**",
            )
            .await?;
        let mut split_status_message = String::new();
        //remove the warning message
        let status_message = status_message[75..status_message.len()].to_string();
        let status_message = status_message[5..status_message.len() - 5].to_string();
        let mut line_count = 0;
        for line in status_message.lines() {
            split_status_message.push_str(line);
            split_status_message.push('\n');
            line_count += 1;
            if line_count == 5 {
                let message_part = format!("```{}```", split_status_message);
                channel_id
                    .say(ctx, message_part)
                    .await
                    .log_expect(LogImportance::Warning, "Unable to send message");
                split_status_message = String::new();
                line_count = 0;
                tokio::time::sleep(std::time::Duration::from_millis(40)).await;
            }
        }
    } else {
        ctx.say(status_message)
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
    }

    status_msg
        .edit(ctx, |m| {
            m.content("Finished sending regex phrases to the channel.")
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to edit message");

    Ok(())
}
