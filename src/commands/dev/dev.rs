use poise::serenity_prelude::CreateEmbed;
use std::sync::atomic::Ordering;

use crate::{
    utils::{
        logger::*,
        toml::get_config,
        type_conversions,
        updater::local_update,
    },
    Data, IPM,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn dev(
    ctx: Context<'_>,
    #[description = "Commands for devs; run help for more info"] command_arg: Option<String>,
) -> Result<(), Error> {
    //Ignore message from non-devs
    let dev = [
        "687897073047306270",
        "598280691066732564",
        "275787354688585730",
    ];
    if !dev.contains(&ctx.author().id.to_string().as_str()) {
        ctx.say("You are not dev you skid :skull:").await?;
        return Ok(());
    }

    let arg = type_conversions::string_to_static_str(
        command_arg.log_expect("did not specify command arg"),
    );
    let args = arg.split_whitespace().collect::<Vec<&str>>();
    match args[0] {
        "none" => {
            ctx.say("You need to specify an dev command you silly uwu kitten :heart:")
                .await?;
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
                    `dev local_update` - Updates the bot from a local file\n\
                    `dev am_dev` - Says if you are dev",
            )
            .await
            .log_expect("Unable to send message");
            Ok(())
        }
        "shutdown" => {
            if !get_config().allow_shutdown {
                ctx.say("Remote shutdown is not enabled on host.").await?;
                return Ok(());
            }

            let msg_author = ctx.author().id;
            println!("Shutdown from dev commands sent from {}", msg_author);

            log_this(LogData {
                importance: LogImportance::Info,
                message: format!("Shutdown from dev commands sent from {}", msg_author),
            });

            ctx.say("Initialized shutdown countdown for 90 seconds")
                .await
                .log_expect("Unable to send message");

            for i in 0..90 {
                let mut embed = CreateEmbed::default();
                embed.color(0x565e6e);
                embed.title("Regy Shutdown");
                if i > 80 {
                    embed.description(format!(
                        ":warning: Regy will be shutdown in the following seconds: {}",
                        90 - i
                    ));
                } else {
                    embed.description(format!(
                        "Regy will be shutdown in the following seconds: {}",
                        90 - i
                    ));
                }
                embed.thumbnail("https://raw.githubusercontent.com/MrEnder0/Regy-Bot/master/.github/assets/shutdown.png");
                embed.footer(|f| f.text("This force shutdown was sent from a dev"));
                let embed_message = ctx
                    .channel_id()
                    .send_message(&ctx, |m| m.set_embed(embed))
                    .await
                    .log_expect("Unable to send shutdown embed")
                    .id;
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                ctx.channel_id()
                    .delete_message(&ctx, embed_message)
                    .await
                    .ok();
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
                std::fs::remove_file("regy.log")
                    .log_expect("Unable to delete log file or file does not exist");
                ctx.say("Log file deleted")
                    .await
                    .log_expect("Unable to send message");
                return Ok(());
            }
            ctx.say("Log file does not exist")
                .await
                .log_expect("Unable to send message");
            Ok(())
        }
        "upload_logs" => {
            if std::path::Path::new("regy.log").exists() {
                ctx.say("Uploading log file, this may take a few seconds...")
                    .await
                    .log_expect("Unable to send message");
                let log_file =
                    std::fs::read_to_string("regy.log").log_expect("Unable to read log file");
                let log_file = log_file.as_bytes();
                ctx.channel_id()
                    .send_files(ctx, vec![(log_file, "regy.log")], |m| {
                        m.content("Log file:")
                    })
                    .await
                    .log_expect("Unable to upload log file");
                return Ok(());
            }
            ctx.say("Log file does not exist")
                .await
                .log_expect("Unable to send message");
            Ok(())
        }
        "IPM" => {
            let ipm_msg = {
                if IPM.load(Ordering::SeqCst) > get_config().max_activity_influx.into() {
                    format!(
                        "IPM is: {} :warning: This is over the server IPM",
                        IPM.load(Ordering::SeqCst)
                    )
                } else {
                    format!("IPM is: {}", IPM.load(Ordering::SeqCst))
                }
            };
            ctx.say(ipm_msg).await.log_expect("Unable to send message");
            Ok(())
        }
        "echo" => {
            let channel_id = ctx.channel_id();
            let echo_msg = args[1..].join(" ");
            channel_id.say(ctx, echo_msg).await?;
            Ok(())
        }
        "local_update" => {
            let mut embed = CreateEmbed::default();
            embed.color(0x565e6e);
            embed.title("Regy Update");
            embed.thumbnail("https://raw.githubusercontent.com/MrEnder0/Regy-Bot/master/.github/assets/update.png");
            embed.description("A local update has been initialized.");
            embed.footer(|f| f.text("If the update fails you will be notified automatically."));
            ctx.channel_id()
                .send_message(&ctx, |m| m.set_embed(embed))
                .await
                .log_expect("Unable to send update embed")
                .id;

            let update = local_update("regy_update.exe");

            match update {
                0 => {
                    let mut embed = CreateEmbed::default();
                    embed.color(0x565e6e);
                    embed.title("Regy Update");
                    embed.description("Update has failed, bot will return to normal operation.");
                    embed.footer(|f| {
                        f.text("Tip: Make sure you put the update file in the right directory")
                    });
                    ctx.channel_id()
                        .send_message(&ctx, |m| m.set_embed(embed))
                        .await
                        .log_expect("Unable to send failed update embed")
                        .id;

                    let data = LogData {
                        importance: LogImportance::Error,
                        message: "Update has failed, bot will return to normal operation.".to_string(),
                    };
                    log_this(data);

                    Ok(())
                }
                1 => {
                    let mut embed = CreateEmbed::default();
                    embed.color(0x565e6e);
                    embed.title("Regy Update");
                    embed.description("Update has been successful, but a update helper was not found, please restart the bot manually to finish the update.");
                    embed.footer(|f| f.text("Closing and reopening Regy will finish the update"));
                    ctx.channel_id()
                        .send_message(&ctx, |m| m.set_embed(embed))
                        .await
                        .log_expect("Unable to send partial update embed")
                        .id;

                    let data = LogData {
                        importance: LogImportance::Info,
                        message: "Update has been successful, but a update helper was not found, please restart the bot manually to finish the update.".to_string(),
                    };
                    log_this(data)

                    Ok(())
                }
                2 => {
                    let mut embed = CreateEmbed::default();
                    embed.color(0x565e6e);
                    embed.title("Regy Update");
                    embed.description("Update has been successful, bot will restart.");
                    embed.footer(|f| f.text("Regy will now restart to finish the update"));
                    ctx.channel_id()
                        .send_message(&ctx, |m| m.set_embed(embed))
                        .await
                        .log_expect("Unable to send successful update embed")
                        .id;

                    let data = LogData {
                        importance: LogImportance::Info,
                        message: "Update has been successful, bot will restart.".to_string(),
                    };
                    log_this(data);

                    std::process::Command::new("regy_bot_update_helper.exe")
                        .spawn()
                        .log_expect("Unable to run update helper");
                    std::process::exit(0);
                }
                _ => {
                    let mut embed = CreateEmbed::default();
                    embed.color(0x565e6e);
                    embed.title("Regy Update");
                    embed.description("Update has finished with an unknown outcome, bot will return to normal operation and ignore the update.");
                    embed.footer(|f| f.text("Tip: Try running the update helper"));
                    ctx.channel_id()
                        .send_message(&ctx, |m| m.set_embed(embed))
                        .await
                        .log_expect("Unable to send unknown update status embed")
                        .id;

                    let data = LogData {
                        importance: LogImportance::Warning,
                        message: "Update has finished with an unknown outcome, bot will return to normal operation and ignore the update.".to_string(),
                    };
                    log_this(data);

                    Ok(())
                }
            }
        }
        _ => {
            let invalid_arg_message = format!(
                "Invalid argument '{}' but its ok I still care abt u :heart:",
                arg.replace('@', "\\@")
            );
            ctx.say(invalid_arg_message)
                .await
                .log_expect("Unable to send message");
            Ok(())
        }
    }
}
