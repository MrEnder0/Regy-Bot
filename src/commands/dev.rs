use crate::{Data, utils::{logger::{LogData, log_this}, s_t_ss}};

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

    let arg = s_t_ss::string_to_static_str(command_arg.unwrap());
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
                            `dev echo` - Echoes the message\n\
                            `dev shutdown` - Shuts down the bot after a 120 second countdown\n\
                            `dev hai` - Says hello back :3\n\
                            `dev am_dev` - Says if you are dev",
            ).await?;
            return Ok(());
        }
        "shutdown" => {
            ctx.say("Regy will down in 120 seconds...").await?;
            let msg_author = ctx.author().id.clone();
            tokio::spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_secs(120)).await;
                println!("Shutdown from dev commands sent from {}", msg_author);
                let log_data = LogData {
                    importance: "INFO".to_string(),
                    message: format!("Shutdown from dev commands sent from {}", msg_author),
                };
                log_this(log_data);
                std::process::exit(0);
            });
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