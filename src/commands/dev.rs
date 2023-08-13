use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

use crate::utils::logger::*;

#[command]
async fn dev(ctx: &Context, msg: &Message) -> CommandResult {
    //Ignore message from non-devs
    match msg.author.id.as_u64() {
        687897073047306270 | 598280691066732564 => {
            let mut args = msg.content.split(' ');
            args.next();
            let arg = args.next().unwrap_or("none");
            match arg {
                "none" => {
                    msg.reply(
                        ctx,
                        "You need to specify an dev command you silly uwu kitten :heart:",
                    )
                    .await?;
                    return Ok(());
                }
                "help" => {
                    msg.reply(
                        ctx,
                        "The dev commands are:\n\
                                    `dev help` - Shows this message\n\
                                    `dev echo` - Echoes the message\n\
                                    `dev shutdown` - Shuts down the bot after a 120 second countdown\n\
                                    `dev hai` - Says hello back :3\n\
                                    `dev am_dev` - Says if you are dev",
                    )
                    .await?;
                    return Ok(());
                }
                "echo" => {
                    if let Err(why) = msg.delete(&ctx.http).await {
                        println!("Error deleting message: {:?}", why);
                    }
                    let mut echo = String::new();
                    for arg in args {
                        echo.push_str(arg);
                        echo.push(' ');
                    }
                    msg.channel_id.say(ctx, echo).await?;
                    return Ok(());
                }
                "shutdown" => {
                    msg.reply(ctx, "Regy will down in 120 seconds...").await?;
                    let msg_clone = msg.clone();
                    tokio::spawn(async move {
                        tokio::time::sleep(tokio::time::Duration::from_secs(120)).await;
                        println!(
                            "Shutdown from dev commands sent from {}",
                            msg_clone.author.id
                        );

                        let log_data = LogData {
                            importance: "INFO".to_string(),
                            message: format!(
                                "Shutdown from dev commands sent from {}",
                                msg_clone.author.id
                            ),
                        };
                        log_this(log_data);
                        std::process::exit(0);
                    });
                    return Ok(());
                }
                "am_dev" => {
                    msg.reply(ctx, "Yes master uwu xo").await?;
                    return Ok(());
                }
                "hai" => {
                    msg.reply(ctx, "Hai hai! 😸🐾 \n \nI am Discord kitten, nya~ 🐱🌸 \n \nI will do my best to fulfill your requests, uwu~ 😊 \n \nLet's pawty and have some kawaii fun, nya~ 🎉🎀 \n \nDon't worry, I'll try not to mispurr too many words, nya~ 😸👌").await?;
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
        _ => {
            msg.reply(ctx, "You are not dev you skid :skull:").await?;
            return Ok(());
        }
    }
}
