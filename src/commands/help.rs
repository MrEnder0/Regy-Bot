use crate::{
    utils::logger::LogExpect,
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Check available commands"] role: String,
) -> Result<(), Error> {
    let arg = role.as_str();

    match arg {
        "user" => {
            ctx.say(
                "The user commands are:\n\
                `user help` - Shows this message\n\
                `user info` - Tells you a full description of what Regy is\n\
                `user skid` - Explains what a skid is\n\
                `user why_rust` - Shows why rust is the best language\n\
                `user what_is_regex` - Explains what regex is\n\
                `user my_infractions` - Shows how many infractions you have\n\
                `user am_user` - Says if you are a user...",
            )
            .await
            .log_expect("Unable to send message");
        },
        "moderator" => {
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
        },
        "admin" => {
            ctx.say(
                "The staff commands are:\n\
                `staff help` - Shows this message\n\
                `staff add_regex <phrase>` - Add a new regex phrase to the list\n\
                `staff remove_regex <id>` - Remove a regex phrase from the list\n\
                `staff list_regex` - Lists all the current blocked regex phrases\n\
                `staff am_admin` - Says if you are a admin",
            )
            .await
            .log_expect("Unable to send message");
        },
        "developer" => {
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
        },
        _ => {
            ctx.say(
                "Invalid permission level, the available permission levels are:\n\
                `user`\n\
                `moderator`\n\
                `admin`\n\
                `developer`",
            )
            .await
            .log_expect("Unable to send message");
        }
    };

    Ok(())
}
