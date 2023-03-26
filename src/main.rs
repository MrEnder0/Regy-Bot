use serenity::{async_trait, framework::standard::{CommandResult, macros::{command, group}, StandardFramework}, model::{channel::Message, gateway::Ready, prelude::{ChannelId, GuildId}}, prelude::*};
use std::{fs::{File, OpenOptions}, io::{Read, Write, BufReader, BufRead}};
use regex::Regex;

struct Handler;

#[group]
#[commands(dev, staff)]
struct General;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let mut run_id_file = File::open("runID").expect("Unable to open runID");
        let mut run_id = String::new();
        run_id_file.read_to_string(&mut run_id).expect("Unable to read runID");
        let run_id = run_id.parse::<u64>().expect("Unable to parse runID");
        let mut run_id_file = File::create("runID").expect("Unable to open runID");
        run_id_file.write_all((run_id + 1).to_string().as_bytes()).expect("Unable to write runID");
    }
    async fn message(&self, ctx: Context, msg: Message) {
        let content = msg.content.chars().rev().collect::<String>();
        if !content.is_empty() {
            if !msg.author.bot {
                if msg.author.id != 687897073047306270 {
                    let regex_file = File::open("regex").expect("Unable to open regex");
                    let reader = BufReader::new(regex_file);
                    let regex: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

                    for regex in regex {
                        if Regex::new(&regex).expect("Unable to parse regex").is_match(msg.content.as_str()) {
                            if let Err(why) = msg.delete(&ctx.http).await {
                                println!("Error deleting message: {:?}", why);
                            }
                            let message_id = msg.channel_id.say(&ctx.http, format!("<@{}> You are not allowed to send that due to the server setup regex rules", msg.author.id)).await.unwrap().id;
                            msg.author.dm(&ctx.http, |m| m.content(format!("You are not allowed to send that due to the server setup regex rules, this has been reported to the server staff, continued offenses will result in greater punishment."))).await.expect("Unable to dm user");
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
    }
}

#[command]
async fn dev(ctx: &Context, msg: &Message) -> CommandResult {
    let mut dev_file = File::open("devs").expect("Unable to open devs");
    let mut devs = String::new();
    dev_file.read_to_string(&mut devs).expect("Unable to read devs");
    let devs = devs.split(" ").collect::<Vec<&str>>();
    if !devs.contains(&msg.author.id.to_string().as_str()) {
        msg.reply(ctx, "You are not dev you skid :skull:").await?;
        return Ok(());
    }

    let mut args = msg.content.split(" ");
    args.next();
    let arg = args.next().unwrap_or("none");
    if arg == "none" {
        msg.reply(ctx, "You need to specify an dev command you silly uwu kitten :heart:").await?;
        return Ok(());
    } else if arg == "help" {
        msg.reply(ctx, "The dev commands are:\n`dev help` - Shows this message\n`dev runid` - Shows the runID of the bot\n`dev rust` - Says why Rust is the best programming language\n`dev echo` - Echoes the message\n`dev amdev` - Says if you are dev").await?;
    } else if arg == "runid" {
        let mut run_id_file = File::open("runID").expect("Unable to open runID");
        let mut run_id = String::new();
        run_id_file.read_to_string(&mut run_id).expect("Unable to read runID");
        let status_message = format!("The session runID is:\n`{}`", run_id);
        msg.reply(ctx, status_message).await?;
    } else if arg == "rust" {
        msg.reply(ctx, "Rust is an excellent programming language that offers a unique combination of safety, speed, and concurrency. It is a modern language designed to provide low-level control and system-level programming, while also ensuring memory safety and preventing many common programming errors such as null pointer dereferences and buffer overflows. Rust achieves this by using a system of ownership and borrowing that guarantees at compile-time that programs are free of these errors. Additionally, Rust's concurrency model allows developers to write efficient and safe concurrent code, making it an ideal choice for building scalable and high-performance applications.\n\nAnother reason why Rust is the best language is its vibrant and growing community. Rust has a passionate and dedicated community of developers who actively contribute to the language, libraries, and tools. This community is committed to creating high-quality and reliable software that is both performant and secure. Additionally, Rust's popularity is on the rise, and many companies, including Mozilla, Dropbox, and Cloudflare, have adopted Rust for their critical systems and applications. As a result, there are numerous resources available for learning Rust, including online courses, books, and tutorials, making it easy for new developers to get started with the language. Overall, Rust's unique combination of safety, speed, and community support makes it an excellent choice for building robust and scalable software systems.").await.expect("Sadly could not say why Rust is the best programming language.");
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
    } else if arg == "amdev" {
        msg.reply(ctx, "Yes master uwu xo").await?;
    } else if arg == "notify" {
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


        //let guild = msg.guild(&ctx.cache).await.unwrap();
        //println!("Pending to send message to {} members in guild", guild.member_count);
        //let members = guild.members(&ctx.http, None, None).await.unwrap();
        //for member in members {
        //    if let Err(why) = member.user.dm(&ctx.http, |m| m.content("Hello there! **This is a test notification from Regy.** In the future this **will only be used for major info**, **if you wish to not receive these notifications** right-click my pfp on our dms and select mute and then until I turn it back on this will make your discord not get notified by these messages.")).await {
        //        println!("Error sending message: {:?}", why);
        //    } else {
        //        println!("Sent message to {}", member.user.name);
        //    }
        //    tokio::time::sleep(tokio::time::Duration::from_millis(22)).await;
        //}

    } else {
        let invalid_arg_message = format!("Invalid argument '{}' but its ok I still care abt u :heart:", arg);
        msg.reply(ctx, invalid_arg_message).await?;
    }

    Ok(())
}

#[command]
async fn staff(ctx: &Context, msg: &Message) -> CommandResult {
    let mut staff_file = File::open("staff").expect("Unable to open staff");
    let mut staff = String::new();
    staff_file.read_to_string(&mut staff).expect("Unable to read staff");
    let staff = staff.split(" ").collect::<Vec<&str>>();
    if !staff.contains(&msg.author.id.to_string().as_str()) {
        msg.reply(ctx, "You are not staff you skid :skull:").await?;
        return Ok(());
    }

    let mut args = msg.content.split(" ");
    args.next();
    let arg = args.next().unwrap_or("none");
    if arg == "none" {
        msg.reply(ctx, "You need to specify a command, I expect higher of you, you should know how to use this bot correctly").await?;
        return Ok(());
    } else if arg == "help" {
        msg.reply(ctx, "The staff commands are:\n`staff help` - Shows this message\n`staff addregex` - Add a new regex phrase to the list\n`staff listregex` - Lists all the current blocked regex phrases\n`staff amstaff` - Says if you are staff").await?;
    } else if arg == "addregex" {
        //run a for loop on the args and add them to the regex file all together on one line
        let mut args = msg.content.split(" ");
        args.next();
        args.next();
        let mut regex = String::new();
        for arg in args {
            regex.push_str(arg);
            regex.push_str(" ");
        }
        let mut regex_file = OpenOptions::new().append(true).open("regex").expect("Unable to open regex");
        regex_file.write_all(regex.as_bytes()).expect("Unable to write to regex");
        regex_file.write_all("\n".as_bytes()).expect("Unable to write to regex");
        let reply_message = format!("Successfully added the following phrase to the blocked regex phrases:\n||```{}```||", regex);
        msg.reply(ctx, reply_message).await?;
    } else if arg == "listregex" {
        let mut regex_file = File::open("regex").expect("Unable to open regex");
        let mut regex = String::new();
        regex_file.read_to_string(&mut regex).expect("Unable to read regex");
        if regex.is_empty() {
            msg.reply(ctx, "There are no regex").await?;
            return Ok(());
        }
        let status_message = format!("The current regex being used are **[WARNING CONTAINS SENSITIVE MESSAGES]**\n||```{}```||", regex);
        msg.reply(ctx, status_message).await?;
    } else if arg == "amstaff" {
        msg.reply(ctx, "Yes master uwu xo").await?;
    } else {
        let invalid_arg_message = format!("Invalid argument '{}' but its ok I still care abt u :heart:", arg);
        msg.reply(ctx, invalid_arg_message).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let token = "token";

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("regy<|"))
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
