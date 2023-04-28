use std::sync::atomic::Ordering;

use crate::{IPM, Data, utils::{logger::{LogData, log_this}, type_conversions, toml::get_config}};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn dev(
    ctx: Context<'_>,
    #[description = "Commands for devs; run help for more info"] command_arg: Option<String>,
) -> Result<(), Error> {
    //Ignore message from non-devs
    let staff = ["687897073047306270", "598280691066732564", "275787354688585730"];
    if !staff.contains(&ctx.author().id.to_string().as_str()) {
        ctx.say("You are not staff you skid :skull:").await?;
        return Ok(());
    }

    let arg = type_conversions::string_to_static_str(command_arg.expect("did not specify command arg"));
    let args = arg.split_whitespace().collect::<Vec<&str>>();
    match args[0] {
        "none" => {
            ctx.say(
                "You need to specify an dev command you silly uwu kitten :heart:",
            ).await?;
            return Ok(());
        }
        "help" => {
            ctx.say(
                "The dev commands are:\n\
                            `dev help` - Shows this message\n\
                            `dev shutdown` - Shuts down the bot after a 120 second countdown\n\
                            `dev clean` - Deletes the log file and other temp files\n\
                            `dev hai` - Says hello back :3\n\
                            `dev IPM` - Shows the current server IPM\n\
                            `dev am_dev` - Says if you are dev",
            ).await?;
            return Ok(());
        }
        "shutdown" => {
            let shutdown_msg = ctx.say("Regy will down in 120 seconds...").await?;
            let msg_author = ctx.author().id;
            tokio::spawn(async move {
                println!("Shutdown from dev commands sent from {}", msg_author);

                let log_data = LogData {
                    importance: "INFO".to_string(),
                    message: format!("Shutdown from dev commands sent from {}", msg_author),
                };
                log_this(log_data);

                tokio::time::sleep(tokio::time::Duration::from_secs(121)).await;
                std::process::exit(0);
            });

            for i in 0..120 {
                if i > 110 {
                    shutdown_msg.edit(ctx, |m| {m.content(format!("Regy will down in {} seconds... :warning:", 120 - i));m}).await?;
                } else {
                    shutdown_msg.edit(ctx, |m| {m.content(format!("Regy will down in {} seconds...", 120 - i));m}).await?;
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
            return Ok(());
        }
        "am_dev" => {
            ctx.say("Yes master uwu xo").await?;
            return Ok(());
        }
        "hai" => {
            ctx.say("Hai hai! 😸🐾 \n \nI am Discord kitten, nya~ 🐱🌸 \n \nI will do my best to fulfill your requests, uwu~ 😊 \n \nLet's pawty and have some kawaii fun, nya~ 🎉🎀 \n \nDon't worry, I'll try not to mispurr too many words, nya~ 😸👌").await?;
            return Ok(());
        }
        "clean" => {
            if std::path::Path::new("regy.log").exists() {
                if let Err(_e) = std::fs::remove_file("regy.log") {
                    std::fs::remove_file("regy.log").expect("Unable to delete log file or file does not exist");
                    ctx.say("Log file deleted").await?;
                    return Ok(());
                }
            }
            ctx.say("Log file does not exist").await?;
            return Ok(());
        }
        "IPM" => {
            let ipm_msg = {
                if IPM.load(Ordering::SeqCst) > get_config().max_activity_influx.into() {
                    format!("IPM is: {} :warning: This is over the server IPM", IPM.load(Ordering::SeqCst))
                } else {
                    format!("IPM is: {}", IPM.load(Ordering::SeqCst))
                }
            };
            ctx.say(ipm_msg).await?;
            return Ok(());
        }
        _ => {
            let invalid_arg_message = format!(
                "Invalid argument '{}' but its ok I still care abt u :heart:",
                arg.replace('@', "\\@")
            );
            ctx.say(invalid_arg_message).await?;
            return Ok(());
        }
    }
}