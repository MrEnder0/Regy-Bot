mod toml_manager;

use serenity::{async_trait, framework::standard::{CommandResult, macros::{command, group}, StandardFramework}, model::{channel::Message, gateway::Ready, prelude::{ChannelId, UserId}}, prelude::*};
use std::path::Path;
use regex::Regex;

use toml_manager::{gen_config, get_config, add_block_phrase, list_block_phrases};

struct Handler;

#[group]
#[commands(dev, staff, user)]
struct General;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
    async fn message(&self, ctx: Context, msg: Message) {
        let content = msg.content.chars().rev().collect::<String>();
        if !content.is_empty() {
            //Ignores messages from bots
            if msg.author.bot {
                return;
            }
            
            //Reply to pings
            if msg.mentions_user_id(ctx.cache.current_user_id().await) {
                let ctx = ctx.clone();
                msg.reply(ctx, "To use Regy use the prefix `<|`").await.expect("Unable to reply to ping");
            }

            //Ignores moderation from devs
            if msg.author.id == 687897073047306270 || msg.author.id == 598280691066732564  {
                return;
            }

            //Ignores moderation from staff
            for staff in get_config().staff {
                if msg.author.id == UserId(staff.parse::<u64>().unwrap()) {
                    return;
                }
            }

            let list_block_phrases = list_block_phrases();

            for phrase in list_block_phrases {
                let re = Regex::new(&phrase).unwrap();
                if re.is_match(&msg.content) {
                    if let Err(why) = msg.delete(&ctx.http).await {
                        println!("Error deleting message: {:?}", why);
                    }
                    let message_id = msg.channel_id.say(&ctx.http, format!("<@{}> You are not allowed to send that due to the server setup regex rules", msg.author.id)).await.unwrap().id;
                    msg.author.dm(&ctx.http, |m| m.content("You are not allowed to send that due to the server setup regex rules, this has been reported to the server staff, continued offenses will result in greater punishment.")).await.expect("Unable to dm user");
                    //send message in log channel
                    let log_channel = ChannelId(977663676574204054);
                    log_channel.say(&ctx.http, format!("<@{}> sent a message that matched a regex pattern, their message is the following below:\n||```{}```||", msg.author.id, msg.content)).await.unwrap();
                    println!("{} sent a message that matched a blocked regex pattern, their message is the following below:\n{}", msg.author.id, msg.content);
                    let ctx_clone = ctx.clone();
                    tokio::spawn(async move {
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                        if let Err(why) = msg.channel_id.delete_message(&ctx_clone.http, message_id).await {
                            println!("Error deleting message: {:?}", why);
                        }
                    });
                    return;
                }
            }            
        }
    }
}

#[command]
async fn dev(ctx: &Context, msg: &Message) -> CommandResult {
    //Ignore message from non-devs
    if msg.author.id != 687897073047306270 && msg.author.id != 598280691066732564 && msg.author.id != 1056383394470182922  {
        msg.reply(ctx, "You are not dev you skid :skull:").await?;
        return Ok(());
    }

    let mut args = msg.content.split(' ');
    args.next();
    let arg = args.next().unwrap_or("none");
    if arg == "none" {
        msg.reply(ctx, "You need to specify an dev command you silly uwu kitten :heart:").await?;
        return Ok(());
    
    } else if arg == "help" {
        msg.reply(ctx, "The dev commands are:\n`dev help` - Shows this message\n`dev echo` - Echoes the message\n`dev am_dev` - Says if you are dev").await?;
    
    } else if arg == "echo" {
        if let Err(why) = msg.delete(&ctx.http).await {
            println!("Error deleting message: {:?}", why);
        }
        let mut echo = String::new();
        for arg in args {
            echo.push_str(arg);
            echo.push(' ');
        }
        msg.channel_id.say(ctx, echo).await?;
    } else if arg == "am_dev" {
        msg.reply(ctx, "Yes master uwu xo").await?;

    // Disabled due to Gatito from Discord Developer Compliance being an ass
    /*} else if arg == "notify" {
        let guild_id = msg.guild_id.expect("This is a guild");
        let mut members = guild_id.members(&ctx, None, None).await.expect("Could not get members");

        members.retain(|member| !member.user.bot);
        for member in members {
            let user_id = member.user.id;
            let channel = user_id.create_dm_channel(&ctx).await.expect("Could not create dm channel");
            match channel.send_message(&ctx, |m| {
                m.content("Hello there! **This is a test notification from Regy. (maybe your second...)** In the future this **will only be used for major info**, **if you wish to not receive these notifications** right-click my pfp on our dms and select mute and then until I turn it back on this will make your discord not get notified by these messages.")
            }).await {
                Ok(_) => {
                    println!("Sent notification to {}", user_id);
                    tokio::time::sleep(tokio::time::Duration::from_millis(25)).await;
                },
                Err(e) => {
                    eprintln!("Could not send message to {}: {:?}", user_id, e);
                }
            }
        }
        println!("Notified all members");
        }*/

    } else {
        let invalid_arg_message = format!("Invalid argument '{}' but its ok I still care abt u :heart:", arg);
        msg.reply(ctx, invalid_arg_message).await?;
    }

    Ok(())
}

#[command]
async fn staff(ctx: &Context, msg: &Message) -> CommandResult {
    //Ignore message from non-staff
    let staff = get_config().staff;
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
        msg.reply(ctx, "The staff commands are:\n`staff help` - Shows this message\n`staff add_regex` - Add a new regex phrase to the list\n`staff list_regex` - Lists all the current blocked regex phrases\n`staff grab_pfp` - Grabs a specified users pfp\n`staff grab_banner` - Grabs a specified users banner\n`staff am_staff` - Says if you are staff").await?;
    } else if arg == "add_regex" {
        let mut args = msg.content.split(' ');
        args.next();
        args.next();
        let mut new_block_phrase = String::new();
        for arg in args {
            new_block_phrase.push_str(arg);
            new_block_phrase.push_str(" ");
        }

        //Prevents for empty regex
        if new_block_phrase.is_empty() || new_block_phrase == " " || new_block_phrase.len() < 3 {
            msg.reply(ctx, "You need to specify a regex phrase to add; it cant be empty and it also cant be less than 3 characters long.").await?;
            return Ok(());
        }

        let new_block_phrase_clone = new_block_phrase.clone();
        add_block_phrase(new_block_phrase);

        let status_message = format!("Added the regex phrase:\n||```{}```||", new_block_phrase_clone);
        msg.reply(ctx, status_message).await?;


    } else if arg == "list_regex" {
        let blocked_phrases = list_block_phrases();
        let mut formated_blocked_phrases = String::new();
        for phrase in blocked_phrases {
            formated_blocked_phrases.push_str(&phrase);
            formated_blocked_phrases.push_str("\n");
        }

        let status_message = format!("The current regex being used are **[WARNING CONTAINS SENSITIVE MESSAGES]**\n||```{}```||", formated_blocked_phrases);
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

#[command]
async fn user(ctx: &Context, msg: &Message) -> CommandResult {
    let mut args = msg.content.split(' ');
    args.next();
    let arg = args.next().unwrap_or("none");
    if arg == "none" {
        msg.reply(ctx, "You need to specify a command, I expect higher of you, you should know how to use this bot correctly").await?;
        return Ok(());

    } else if arg == "help" {
        msg.reply(ctx, "The user commands are:\n`user help` - Shows this message\n`user why_rust` - Shows why rust is the best language\n`user info` - Tells you a full description of what Regy is\n`user whats_regex` - Explains what regex is\n`user am_user` - Says if you are a user...").await?;

    } else if arg == "am_user" {
        msg.reply(ctx, "Why would you not be a user you skid :skull:").await?;

    } else if arg == "why_rust" {
        msg.reply(ctx, "Rust is an excellent programming language that offers a unique combination of safety, speed, and concurrency. It is a modern language designed to provide low-level control and system-level programming, while also ensuring memory safety and preventing many common programming errors such as null pointer dereferences and buffer overflows. Rust achieves this by using a system of ownership and borrowing that guarantees at compile-time that programs are free of these errors. Additionally, Rust's concurrency model allows developers to write efficient and safe concurrent code, making it an ideal choice for building scalable and high-performance applications.\n\nAnother reason why Rust is the best language is its vibrant and growing community. Rust has a passionate and dedicated community of developers who actively contribute to the language, libraries, and tools. This community is committed to creating high-quality and reliable software that is both performant and secure. Additionally, Rust's popularity is on the rise, and many companies, including Mozilla, Dropbox, and Cloudflare, have adopted Rust for their critical systems and applications. As a result, there are numerous resources available for learning Rust, including online courses, books, and tutorials, making it easy for new developers to get started with the language. Overall, Rust's unique combination of safety, speed, and community support makes it an excellent choice for building robust and scalable software systems.").await.expect("Sadly could not say why Rust is the best programming language.");

    } else if arg == "info" {
        msg.reply(ctx, "Regy is a Discord regex auto-moderation bot developed mainly by Mr.Ender#0001 with a few contributions by 3kh0#6969 and 1984#0001, also the profile picture created by 1984.").await?;

    } else if arg == "whats_regex" {
        msg.reply(ctx, "Regex, short for Regular Expression, is a sequence of characters that defines a search pattern. It is used to search, replace, and manipulate text in programming and text editing tools. It provides a powerful and flexible way to match and manipulate strings of text based on certain patterns or rules.").await?;

    } else {
        let invalid_arg_message = format!("Invalid argument '{}' but its ok I still care abt u :heart:", arg);
        msg.reply(ctx, invalid_arg_message).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    //check for config file
    if !Path::new("config.toml").exists() {
        gen_config();
    }

    //load token from config file
    let token = get_config().token;

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("<|"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
