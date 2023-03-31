use serenity::{framework::standard::{CommandResult, macros::command}, model::{channel::Message}, prelude::*};

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
        msg.reply(ctx, "The dev commands are:\n\
                        `dev help` - Shows this message\n\
                        `dev echo` - Echoes the message\n\
                        `dev shutdown` - Shuts down the bot after a 120 second countdown\n\
                        `dev hai` - Says hello back :3\n\
                        `dev am_dev` - Says if you are dev"
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

    } else if arg == "hai" {
        msg.reply(ctx, "Hai hai! 😸🐾 \n \nI am Discord kitten, nya~ 🐱🌸 \n \nI will do my best to fulfill your requests, uwu~ 😊 \n \nLet's pawty and have some kawaii fun, nya~ 🎉🎀 \n \nDon't worry, I'll try not to mispurr too many words, nya~ 😸👌").await?;
    
    } else {
        let invalid_arg_message = format!("Invalid argument '{}' but its ok I still care abt u :heart:", arg);
        msg.reply(ctx, invalid_arg_message).await?;
    }

    Ok(())
}
