use crate::{
    utils::logger::LogExpect,
    utils::toml,
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command, user_cooldown = 45, required_permissions = "ADMINISTRATOR")]
pub async fn list_regex(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let status_msg = ctx
        .say("Sending regex phrases this may take a few seconds...")
        .await
        .log_expect("Unable to send message");

    let blocked_phrases = toml::list_block_phrases();
    let mut formatted_blocked_phrases = String::new();
    for (id, phrase) in blocked_phrases {
        formatted_blocked_phrases.push_str(&id.to_string());
        formatted_blocked_phrases.push_str(" | ");
        formatted_blocked_phrases.push_str(&phrase);
        formatted_blocked_phrases.push('\n');
    }

    let status_message = format!("The current regex being used are **[WARNING CONTAINS SENSITIVE MESSAGES]**\n||```                  ID                 | REGEX\n{}```||", formatted_blocked_phrases);
    let channel_id = ctx.channel_id();

    if status_message.len() > 2000 {
        channel_id.say(ctx, "The current regex being used are **[WARNING CONTAINS SENSITIVE MESSAGES]**").await?;
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
                    .log_expect("Unable to send message");
                split_status_message = String::new();
                line_count = 0;
                tokio::time::sleep(std::time::Duration::from_millis(40)).await;
            }
        }
    } else {
        ctx.say(status_message)
            .await
            .log_expect("Unable to send message");
    }

    status_msg
        .edit(ctx, |m| {
            m.content("Finished sending regex phrases to the channel.")
        })
        .await
        .log_expect("Unable to edit message");
    Ok(())
}