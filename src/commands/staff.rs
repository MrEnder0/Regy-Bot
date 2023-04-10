use crate::managers::toml;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::{channel::Message, prelude::UserId},
    prelude::*,
};
use uuid::Uuid;

#[command]
async fn staff(ctx: &Context, msg: &Message) -> CommandResult {
    //Ignore message from non-staff
    let staff = toml::get_config().staff;
    let user_id = msg.author.id.to_string();
    if !staff.contains(&user_id) {
        msg.reply(ctx, "You are not staff you skid :skull:").await?;
        return Ok(());
    }

    let mut args = msg.content.split(' ');
    args.next();
    let arg: &str = args.next().unwrap_or("none");
    match arg {
        "none" => {
            msg.reply(ctx, "You need to specify a command, I expect higher of you, you should know how to use this bot correctly").await?;
            return Ok(());
        }
        "help" => {
            msg.reply(
                ctx,
                "The staff commands are:\n\
                            `staff help` - Shows this message\n\
                            `staff add_regex` - Add a new regex phrase to the list\n\
                            `staff remove_regex` - Remove a regex phrase from the list\n\
                            `staff list_regex` - Lists all the current blocked regex phrases\n\
                            `staff grab_pfp` - Grabs a specified user's pfp\n\
                            `staff grab_timestamp` - Find out when a specified user's account was made \n\
                            `staff grab_banner` - Grabs a specified users banner\n\
                            `staff am_staff` - Says if you are staff",
            )
            .await?;
            return Ok(());
        }
        "add_regex" => {
            let mut args = msg.content.split(' ');
            args.next();
            args.next();
            let mut new_block_phrase = String::new();
            for arg in args {
                new_block_phrase.push_str(arg);
                new_block_phrase.push(' ');
            }

            //Prevents for empty regex
            if new_block_phrase.is_empty() || new_block_phrase == " " || new_block_phrase.len() < 3
            {
                msg.reply(ctx, "You need to specify a regex phrase to add; it cant be empty and it also cant be less than 3 characters long.").await?;
                return Ok(());
            }

            let new_block_phrase_clone = new_block_phrase.clone();
            toml::add_block_phrase(new_block_phrase);

            let status_message = format!(
                "Added the regex phrase:\n||```{}```||",
                new_block_phrase_clone
            );
            msg.reply(ctx, status_message).await?;
            return Ok(());
        }
        "remove_regex" => {
            let id = args.next().unwrap_or("none");
            if id == "none" {
                msg.reply(
                    ctx,
                    "You need to specify a UUID you silly kitten :heart:",
                )
                .await?;
                return Ok(());
            }
            let id = id.parse::<Uuid>().unwrap();
            toml::remove_block_phrase(id);
            let status_message = format!(
                "Removed the regex phrase with UUID: {}",
                id
            );
            msg.reply(ctx, status_message).await?;
            return Ok(());
        }
        "list_regex" => {
            let blocked_phrases = toml::list_block_phrases();
            let mut formatted_blocked_phrases = String::new();
            for (id, phrase) in blocked_phrases {
                formatted_blocked_phrases.push_str(&id.to_string());
                formatted_blocked_phrases.push_str(" | ");
                formatted_blocked_phrases.push_str(&phrase);
                formatted_blocked_phrases.push('\n');
            }

            let status_message = format!("The current regex being used are **[WARNING CONTAINS SENSITIVE MESSAGES]**\n||```                  ID                 | REGEX\n{}```||", formatted_blocked_phrases);
            let channel_id = msg.channel_id;

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
                        channel_id.say(ctx, message_part).await?;
                        split_status_message = String::new();
                        line_count = 0;
                        tokio::time::sleep(std::time::Duration::from_millis(40)).await;
                    }
                }
            } else {
                msg.reply(ctx, status_message).await?;
            }
            return Ok(());
        }
        "add_infraction" => {
            let user_id = args.next().unwrap_or("none");
            if user_id == "none" {
                msg.reply(
                    ctx,
                    "You need to specify a user id you silly kitten :heart:",
                )
                .await?;
                return Ok(());
            }
            let user_id = user_id.parse::<u64>().unwrap();
            toml::add_infraction(user_id);
            msg.reply(ctx, "Added infraction to the specified user.").await?;
            return Ok(());
        }
        "list_infractions" => {
            let user_id = args.next().unwrap_or("none");
            if user_id == "none" {
                msg.reply(
                    ctx,
                    "You need to specify a user id you silly kitten :heart:",
                )
                .await?;
                return Ok(());
            }
            let user_id = user_id.parse::<u64>().unwrap();
            let infractions = toml::list_infractions(user_id);
            let formatted_infractions = format!("Infractions for {} is:\n{}", user_id, infractions);
            msg.reply(ctx, formatted_infractions).await?;
            return Ok(());
        }
        "grab_pfp" => {
            let user_id = args.next().unwrap_or("none");
            if user_id == "none" {
                msg.reply(
                    ctx,
                    "You need to specify a user id you silly kitten :heart:",
                )
                .await?;
                return Ok(());
            }
            let user_id = user_id.parse::<u64>().unwrap();
            let user = UserId(user_id).to_user(ctx).await?;
            msg.reply(ctx, user.face()).await?;
            return Ok(());
        }
        "grab_banner" => {
            let user_id = args.next().unwrap_or("none");
            if user_id == "none" {
                msg.reply(
                    ctx,
                    "You need to specify a user id you silly kitten :heart:",
                )
                .await?;
                return Ok(());
            }
            let user_id = user_id.parse::<u64>().unwrap();
            let user = UserId(user_id).to_user(ctx).await?;
            msg.reply(
                ctx,
                user.banner_url()
                    .unwrap_or("This user does not have a banner".to_string()),
            )
            .await?;
            return Ok(());
        }
        "am_staff" => {
            msg.reply(
                ctx,
                "Yes master uwu xo... Now do some moderation and stop making me do it :|",
            )
            .await?;
            return Ok(());
        }
        _ => {
            let invalid_arg_message = format!(
                "Invalid argument '{}' but its ok I still care abt u :heart:",
                arg.replace('@', "\\@")
            );
            msg.reply(ctx, invalid_arg_message).await?;
            return Ok(());
        }
    }
}
