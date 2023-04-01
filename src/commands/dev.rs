use serenity::{client::Context, framework::standard::{macros::command, CommandResult}, model::channel::{ChannelType, Message}};

#[command]
async fn dev(ctx: &Context, msg: &Message) -> CommandResult {
    //Ignore message from non-devs
    if msg.author.id != 687897073047306270 && msg.author.id != 598280691066732564 && msg.author.id != 1056383394470182922 {
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
        msg.reply(ctx, "The dev commands are:\n\
                        `dev help` - Shows this message\n\
                        `dev echo` - Echoes the message\n\
                        `dev shutdown` - Shuts down the bot after a 120 second countdown\n\
                        `dev hai` - Says hello back :3\n\
                        `dev am_dev` - Says if you are dev\n\
                        `dev ban` - Bans a user from the server"
                    ).await?;

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

    } else if arg == "shutdown" {
        msg.reply(ctx, "Regy will down in 120 seconds...").await?;
        let msg_clone = msg.clone();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(120)).await;
            println!("Shutdown from dev commands sent from {}", msg_clone.author.id);
            std::process::exit(0);
        });

    } else if arg == "am_dev" {
        msg.reply(ctx, "Yes master uwu xo").await?;

    } else if arg == "ban" {
        if let Some(user_id) = args.next() {
            if let Ok(user_id) = user_id.parse::<u64>() {
                if let Some(user) = ctx.cache.user(user_id).await {
                    if let Ok(guild_id) = msg.guild_id.unwrap().to_partial_guild_id().to_guild_cached(&ctx.cache).await {
                        if let Ok(()) = guild_id.ban_with_reason(&ctx.http, user_id, 0, "Banned by developer").await {
                            msg.reply(ctx, format!("User {} has been banned from the server", user.tag())).await?;
                        } else {
                            msg.reply(ctx, "Failed to ban user. Check the bot's permissions and ensure the user isn't an admin/mod").await?;
                        }
                    } else {
                        msg.reply(ctx, "Failed to get server information").await?;
                    }
                } else {
                    msg.reply(ctx, "Invalid user ID").await?;
                }
            } else {
                msg.reply(ctx, "Invalid user ID").await?;
            }
        } else {
            msg.reply(ctx, "No user ID provided").await?;
        }

    } else if arg == "hai" {
        msg.reply(ctx, "Hai hai!
