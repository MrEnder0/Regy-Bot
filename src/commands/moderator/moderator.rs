use poise::serenity_prelude::UserId;

use crate::{
    utils::{logger::LogExpect, toml, type_conversions},
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn moderator(
    ctx: Context<'_>,
    #[description = "Commands for staff members; run help for more info"] command_arg: Option<
        String,
    >,
) -> Result<(), Error> {
    let mut moderators = toml::get_config().moderators;
    for admin in toml::get_config().admins {
        moderators.push(admin);
    }

    let user_id = ctx.author().id.to_string();
    if !moderators.contains(&user_id) {
        ctx.say("You are not staff you skid :skull:").await?;
        return Ok(());
    }

    let arg = type_conversions::string_to_static_str(
        command_arg.log_expect("did not specify command arg"),
    );
    let args = arg.split_whitespace().collect::<Vec<&str>>();
    match args[0] {
        "none" => {
            ctx.say("You need to specify a command.").await?;
            Ok(())
        }
        "help" => {
            ctx.say(
                "The staff commands are:\n\
                    `staff help` - Shows this message\n\
                    `staff add_infraction <user>` - Adds an infraction to a user\n\
                    `staff remove_infraction <user>` - Removes an infraction from a user\n\
                    `staff list_infractions <user>` - Lists the infractions of a user\n\
                    `staff grab_pfp <user>` - Grabs a specified user's pfp\n\
                    `staff grab_banner <user>` - Grabs a specified users banner\n\
                    `staff am_mod` - Says if you are a mod",
            )
            .await
            .log_expect("Unable to send message");
            Ok(())
        }
        "add_infraction" => {
            let user_id = arg.split_whitespace().nth(1).unwrap_or("none");
            if user_id == "none" {
                ctx.say("You need to specify a user id.")
                    .await
                    .log_expect("Unable to send message");
                return Ok(());
            }
            let user_id = user_id.parse::<u64>().unwrap();
            toml::add_infraction(user_id);
            ctx.say("Added infraction to the specified user.")
                .await
                .log_expect("Unable to send message");
            Ok(())
        }
        "remove_infraction" => {
            let user_id = arg.split_whitespace().nth(1).unwrap_or("none");
            if user_id == "none" {
                ctx.say("You need to specify a user id.")
                    .await
                    .log_expect("Unable to send message");
                return Ok(());
            }
            let user_id = user_id.parse::<u64>().unwrap();
            toml::dismiss_infraction(user_id);
            ctx.say("Removed 1 infraction from the specified user.")
                .await
                .log_expect("Unable to send message");
            Ok(())
        }
        "list_infractions" => {
            let user_id = arg.split_whitespace().nth(1).unwrap_or("none");
            if user_id == "none" {
                ctx.say("You need to specify a user id.")
                    .await
                    .log_expect("Unable to send message");
                return Ok(());
            }
            let user_id = user_id.parse::<u64>().unwrap();
            let infractions = toml::list_infractions(user_id);
            let formatted_infractions = format!("Infractions for {} is:\n{}", user_id, infractions);
            ctx.say(formatted_infractions)
                .await
                .log_expect("Unable to send message");
            Ok(())
        }
        "grab_pfp" => {
            let user_id = arg.split_whitespace().nth(1).unwrap_or("none");
            if user_id == "none" {
                ctx.say("You need to specify a user id.")
                    .await
                    .log_expect("Unable to send message");
                return Ok(());
            }
            let user_id = user_id.parse::<u64>().unwrap();
            let user = UserId(user_id).to_user(ctx).await?;
            ctx.say(user.face())
                .await
                .log_expect("Unable to send message");
            Ok(())
        }
        "grab_banner" => {
            let user_id = arg.split_whitespace().nth(1).unwrap_or("n");
            if user_id == "n" {
                ctx.say("You need to specify a user id.")
                    .await
                    .log_expect("Unable to send message");
                return Ok(());
            }
            let user_id = user_id.parse::<u64>().unwrap();
            let user = UserId(user_id).to_user(ctx).await?;
            ctx.say(
                user.banner_url()
                    .unwrap_or("This user does not have a banner!".to_string()),
            )
            .await
            .log_expect("Unable to send message");
            Ok(())
        }
        "am_mod" => {
            ctx.say("Yes, now do some moderation and stop making me do it. :|")
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
