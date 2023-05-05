use std::sync::atomic::Ordering;

use poise::serenity_prelude::CreateEmbed;

use crate::{IPM, Data, utils::{logger::*, type_conversions, toml::get_config, log_on_error::LogExpect}};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn dev(
    ctx: Context<'_>,
    #[description = "Commands for devs; run help for more info"] command_arg: Option<String>,
) -> Result<(), Error> {
    //Ignore message from non-devs
    let dev = ["687897073047306270", "598280691066732564", "275787354688585730"];
    if !dev.contains(&ctx.author().id.to_string().as_str()) {
        ctx.say("You are not dev you skid :skull:").await?;
        return Ok(());
    }

    let arg = type_conversions::string_to_static_str(command_arg.log_expect("did not specify command arg"));
    let args = arg.split_whitespace().collect::<Vec<&str>>();
    match args[0] {
        "none" => {
            ctx.say(
                "You need to specify an dev command you silly uwu kitten :heart:",
            ).await?;
            Ok(())
        }
        "help" => {
            ctx.say(
                "The dev commands are:\n\
                            `dev help` - Shows this message\n\
                            `dev shutdown` - Shuts down the bot after a 120 second countdown\n\
                            `dev clean` - Deletes the log file and other temp files\n\
                            `dev upload_logs` - Uploads the log file to the current channel\n\
                            `dev echo <message>` - Says the message back\n\
                            `dev hai` - Says hello back :3\n\
                            `dev IPM` - Shows the current server IPM\n\
                            `dev am_dev` - Says if you are dev",
            ).await?;
            Ok(())
        }
        "shutdown" => {
            let msg_author = ctx.author().id;
            println!("Shutdown from dev commands sent from {}", msg_author);

            log_this(LogData {
                importance: "INFO".to_string(),
                message: format!("Shutdown from dev commands sent from {}", msg_author),
            });

            ctx.say("Initialized shutdown countdown for 90 seconds").await?;

            for i in 0..90 {
                let mut embed = CreateEmbed::default();
                embed.color(0x565e6e);
                embed.title("Regy Shutdown");
                if i > 80 {
                    embed.description(format!(":warning: Regy will be shutdown in the following seconds: {}", 90-i));
                } else {
                    embed.description(format!("Regy will be shutdown in the following seconds: {}", 90-i));
                }
                embed.thumbnail("https://raw.githubusercontent.com/MrEnder0/Regy-Bot/master/.github/assets/shutdown.png");
                embed.footer(|f| f.text("This force shutdown was sent from a dev"));
                let embed_message = ctx.channel_id().send_message(&ctx, |m| m.set_embed(embed)).await.log_expect("Unable to send shutdown embed").id;
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                ctx.channel_id().delete_message(&ctx, embed_message).await.ok();
            }
            ctx.say("Countdown finished, shutting down...").await?;
            std::process::exit(0);
        }
        "am_dev" => {
            ctx.say("Yes master uwu xo").await?;
            Ok(())
        }
        "hai" => {
            ctx.say("Hai hai! 😸🐾 \n \nI am Discord kitten, nya~ 🐱🌸 \n \nI will do my best to fulfill your requests, uwu~ 😊 \n \nLet's pawty and have some kawaii fun, nya~ 🎉🎀 \n \nDon't worry, I'll try not to mispurr too many words, nya~ 😸👌").await?;
            Ok(())
        }
        "clean" => {
            if std::path::Path::new("regy.log").exists() {
                if let Err(_e) = std::fs::remove_file("regy.log") {
                    std::fs::remove_file("regy.log").log_expect("Unable to delete log file or file does not exist");
                    ctx.say("Log file deleted").await?;
                    return Ok(());
                }
            }
            ctx.say("Log file does not exist").await?;
            Ok(())
        }
        "upload_logs" => {
            if std::path::Path::new("regy.log").exists() {
                //CURRENT Channel 
                let log_file = std::fs::read_to_string("regy.log").log_expect("Unable to read log file");
                let log_file = log_file.as_bytes();
                ctx.channel_id().send_files(ctx, vec![(log_file, "regy.log")], |m| m.content("Log file")).await?;
                ctx.say("Log file uploaded").await?;
                return Ok(());
            }
            ctx.say("Log file does not exist").await?;
            Ok(())
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
            Ok(())
        }
        "echo" => {
            let channel_id = ctx.channel_id();
            let echo_msg = args[1..].join(" ");
            channel_id.say(ctx, echo_msg).await?;
            Ok(())
        }
        _ => {
            let invalid_arg_message = format!(
                "Invalid argument '{}' but its ok I still care abt u :heart:",
                arg.replace('@', "\\@")
            );
            ctx.say(invalid_arg_message).await?;
            Ok(())
        }
    }
}