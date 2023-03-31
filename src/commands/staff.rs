use serenity::{framework::standard::{CommandResult, macros::command}, model::{channel::Message, prelude::UserId}, prelude::*};
use crate::toml;

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
    let arg = args.next().unwrap_or("none");
    if arg == "none" {
        msg.reply(ctx, "You need to specify a command, I expect higher of you, you should know how to use this bot correctly").await?;
        return Ok(());

    } else if arg == "help" {
        msg.reply(ctx, "The staff commands are:\n
                        `staff help` - Shows this message\n
                        `staff add_regex` - Add a new regex phrase to the list\n
                        `staff list_regex` - Lists all the current blocked regex phrases\n
                        `staff grab_pfp` - Grabs a specified users pfp\n
                        `staff grab_banner` - Grabs a specified users banner\n
                        `staff am_staff` - Says if you are staff"
                    ).await?;
    
    } else if arg == "add_regex" {
        let mut args = msg.content.split(' ');
        args.next();
        args.next();
        let mut new_block_phrase = String::new();
        for arg in args {
            new_block_phrase.push_str(arg);
            new_block_phrase.push(' ');
        }

        //Prevents for empty regex
        if new_block_phrase.is_empty() || new_block_phrase == " " || new_block_phrase.len() < 3 {
            msg.reply(ctx, "You need to specify a regex phrase to add; it cant be empty and it also cant be less than 3 characters long.").await?;
            return Ok(());
        }

        let new_block_phrase_clone = new_block_phrase.clone();
        toml::add_block_phrase(new_block_phrase);

        let status_message = format!("Added the regex phrase:\n||```{}```||", new_block_phrase_clone);
        msg.reply(ctx, status_message).await?;


    } else if arg == "list_regex" {
        let blocked_phrases = toml::list_block_phrases();
        let mut formatted_blocked_phrases = String::new();
        for phrase in blocked_phrases {
            formatted_blocked_phrases.push_str(&phrase);
            formatted_blocked_phrases.push('\n');
        }

        let status_message = format!("The current regex being used are **[WARNING CONTAINS SENSITIVE MESSAGES]**\n||```{}```||", formatted_blocked_phrases);
        msg.reply(ctx, status_message).await?;

    } else if arg == "grab_pfp" {
        let user_id = args.next().unwrap_or("none");
        if user_id == "none" {
            msg.reply(ctx, "You need to specify a user id you silly kitten :heart:").await?;
            return Ok(());
        }
        let user_id = user_id.parse::<u64>().unwrap();
        let user = UserId(user_id).to_user(ctx).await?;
        msg.reply(ctx, user.face()).await?;
    
    } else if arg == "grab_banner" {
        let user_id = args.next().unwrap_or("none");
        if user_id == "none" {
            msg.reply(ctx, "You need to specify a user id you silly kitten :heart:").await?;
            return Ok(());
        }
        let user_id = user_id.parse::<u64>().unwrap();
        let user = UserId(user_id).to_user(ctx).await?;
        msg.reply(ctx, user.banner_url().unwrap_or("This user does not have a banner".to_string())).await?;

    } else if arg == "am_staff" {
        msg.reply(ctx, "Yes master uwu xo... Now do some moderation and stop making me do it :|").await?;

    } else {
        let invalid_arg_message = format!("Invalid argument '{}' but its ok I still care abt u :heart:", arg);
        msg.reply(ctx, invalid_arg_message).await?;
    }

    Ok(())
}