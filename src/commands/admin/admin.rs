use uuid::Uuid;

use crate::{
    utils::{log_on_error::LogExpect, toml, type_conversions},
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn admin(
    ctx: Context<'_>,
    #[description = "Commands for admins; run help for more info"] command_arg: Option<String>,
) -> Result<(), Error> {
    //Ignore message from non-staff
    let staff = toml::get_config().admins;
    let user_id = ctx.author().id.to_string();
    if !staff.contains(&user_id) {
        ctx.say("You are not a admin you skid :skull:").await?;
        return Ok(());
    }

    let arg = type_conversions::string_to_static_str(
        command_arg.log_expect("did not specify command arg"),
    );
    let args = arg.split_whitespace().collect::<Vec<&str>>();
    match args[0] {
        "none" => {
            ctx.say("You need to specify a command.").await?;
            Ok(())
        }
        "help" => {
            ctx.say(
                "The staff commands are:\n\
                    `staff help` - Shows this message\n\
                    `staff add_regex <phrase>` - Add a new regex phrase to the list\n\
                    `staff remove_regex <id>` - Remove a regex phrase from the list\n\
                    `staff list_regex` - Lists all the current blocked regex phrases\n\
                    `staff am_admin` - Says if you are a admin",
            )
            .await
            .log_expect("Unable to send message");
            Ok(())
        }
        "add_regex" => {
            let args = arg.split_whitespace().skip(1).collect::<Vec<&str>>();
            println!("{:?}", args);
            let mut new_block_phrase = String::new();
            for arg in args {
                new_block_phrase.push_str(arg);
                new_block_phrase.push(' ');
            }

            //Prevents for empty regex
            if new_block_phrase.is_empty() || new_block_phrase == " " || new_block_phrase.len() < 3
            {
                ctx.say("You need to specify a regex phrase to add; it cant be empty and it also cant be less than 3 characters long.").await.log_expect("Unable to send message");
                return Ok(());
            }

            let new_block_phrase_clone = new_block_phrase.clone();
            toml::add_block_phrase(new_block_phrase);

            let status_message = format!(
                "Added the regex phrase:\n||```{}```||",
                new_block_phrase_clone
            );
            ctx.say(status_message)
                .await
                .log_expect("Unable to send message");
            Ok(())
        }
        "remove_regex" => {
            let id = arg.split_whitespace().nth(1).unwrap_or("none");
            if id == "none" {
                ctx.say("You need to specify a target UUID.")
                    .await
                    .log_expect("Unable to send message");
                return Ok(());
            }
            let id = id.parse::<Uuid>().unwrap();
            toml::remove_block_phrase(id);
            let status_message = format!("Removed the regex phrase with UUID: {}", id);
            ctx.say(status_message)
                .await
                .log_expect("Unable to send message");
            Ok(())
        }
        "list_regex" => {
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
        "am_admin" => {
            ctx.say("Yes, now do some admin thingies and stop making me do it. :|")
                .await
                .log_expect("Unable to send message");
            Ok(())
        }
        _ => {
            let invalid_arg_message = format!("Invalid argument '{}'", arg.replace('@', "\\@"));
            ctx.say(invalid_arg_message)
                .await
                .log_expect("Unable to send message");
            Ok(())
        }
    }
}
