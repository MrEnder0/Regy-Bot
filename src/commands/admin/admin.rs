use uuid::Uuid;

use crate::{
    utils::{logger::LogExpect, toml, type_conversions},
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn admin(
    ctx: Context<'_>,
    #[description = "Commands for admins; run help for more info"] command_arg: Option<String>,
) -> Result<(), Error> {
    //Ignore message from non-staff
    //let staff = toml::get_config().admins;
    //let user_id = ctx.author().id.to_string();
    //if !staff.contains(&user_id) {
    //    ctx.say("You are not a admin you skid :skull:").await?;
    //    return Ok(());
    //}

    let arg = type_conversions::string_to_static_str(
        command_arg.log_expect("did not specify command arg"),
    );
    let args = arg.split_whitespace().collect::<Vec<&str>>();
    match args[0] {
        "none" => {
            ctx.say("You need to specify a command.").await?;
            Ok(())
        }
        "remove_regex" => {
            let id = arg.split_whitespace().nth(1).unwrap_or("none");
            if id == "none" {
                ctx.say("You need to specify a target UUID.")
                    .await
                    .log_expect("Unable to send message");
                return Ok(());
            }
            let id = id.parse::<Uuid>().unwrap();
            toml::remove_block_phrase(id);
            let status_message = format!("Removed the regex phrase with UUID: {}", id);
            ctx.say(status_message)
                .await
                .log_expect("Unable to send message");
            Ok(())
        }
        _ => {
            let invalid_arg_message = format!("Invalid argument '{}'", arg.replace('@', "\\@"));
            ctx.say(invalid_arg_message)
                .await
                .log_expect("Unable to send message");
            Ok(())
        }
    }
}
