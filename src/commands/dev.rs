use serenity::{
    framework::standard::{CommandResult, macros::command},
    model::{channel::Message, id::ChannelId},
    prelude::*,
};

#[command]
async fn dev(ctx: &Context, msg: &Message) -> CommandResult {
    // Ignore message from non-devs
    if msg.author.id != 687897073047306270 && msg.author.id != 598280691066732564 && msg.author.id != 1056383394470182922 {
        msg.reply(ctx, "You are not dev you skid :skull:").await?;
        return Ok(());
    }

    let mut args = msg.content.split(' ');
    args.next();
    let arg = args.next().unwrap_or("none");
    if arg == "none" {
        msg.reply(ctx, "You need to specify a dev command you silly uwu kitten :heart:").await?;
        return Ok(());
    } else if arg == "help" {
        msg.reply(ctx, "The dev commands are:\n\
                        `dev help` - Shows this message\n\
                        `dev echo` - Echoes the message\n\
                        `dev shutdown` - Shuts down the bot after a 120 second countdown\n\
                        `dev hai` - Says hello back :3\n\
                        `dev am_dev` - Says if you are dev\n\
                        `dev ban` - Bans a user\n\
                        `dev lock` - Locks all channels").await?;
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
        let user_id = args.next().and_then(|s| s.parse::<u64>().ok());
        if let Some(user_id) = user_id {
            let reason = args.collect::<Vec<&str>>().join(" ");
            let guild_id = msg.guild_id.expect("This command only works in a server");
            guild_id.ban_with_reason(&ctx.http, user_id, 7, &reason).await?;
            msg.reply(ctx, format!("User with ID {} has been banned with reason: {}", user_id, reason)).await?;
        } else {
            msg.reply(ctx, "Please provide a user ID to ban.").await?;
        }
    } else if arg == "lock" {
        let channels = msg.guild_id.expect("This command only works in a server")
            .channels(&ctx.http)
            .await?;
        for channel in channels {
            if let Ok(text_channel) = channel.to_channel_text() {
                if text_channel.send_message(&ctx.http, |m| m.content("This channel has been locked.")).await.is_ok() {
                    println!("Locked channel: {}", text_channel.id);
                }
            }
        }
        msg.reply(ctx
