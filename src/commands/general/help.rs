use crate::{
    utils::logger::LogExpect,
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Debug, poise::ChoiceParameter)]
pub enum HelpChoice {
    #[name = "General commands help"]
    General,
    #[name = "Information commands help"]
    Information,
    #[name = "Infraction commands help"]
    Infraction,
    #[name = "Regex commands help"]
    Regex,
    #[name = "Moderation commands help"]
    Moderation,
    #[name = "Developer commands help"]
    Developer,

}

#[poise::command(slash_command, prefix_command, user_cooldown = 25, ephemeral = true)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Target Permission"] choice: HelpChoice,
) -> Result<(), Error> {
    match choice {
        HelpChoice::General => {
            ctx.say(
                "The general commands are:\n\
                `/help <choice>` - Shows help on a given subset of commands\n\
                `/permission_level <user> - Shows the specified user's permission level",
            )
            .await
            .log_expect("Unable to send message");
        },
        HelpChoice::Information => {
            ctx.say(
                "The information commands are:\n\
                `/info` - Tells you a full description of what Regy is\n\
                `/skid` - Explains what a skid is\n\
                `/why_rust` - Shows why rust is the best language\n\
                `/what_is_regex` - Explains what regex is\n\
                `/my_infractions` - Shows how many infractions you have",
            )
            .await
            .log_expect("Unable to send message");
        },
        HelpChoice::Infraction => {
            ctx.say(
                "The infraction commands are:\n\
                `/add_infraction <user>` - Adds an infraction to the specified user\n\
                `/dismiss_infraction <user>` - Removes an infraction from the specified user\n\
                `/list_infractions <user>` - Lists the infractions of the specified user",
            )
            .await
            .log_expect("Unable to send message");
        },
        HelpChoice::Regex => {
            ctx.say(
                "The regex commands are:\n\
                `/add_regex <phrase>` - Adds a new regex phrase to the list\n\
                `/remove_regex <id>` - Removes the specified regex phrase from the list\n\
                `/list_regex` - Lists all the current blocked regex phrases",
            )
            .await
            .log_expect("Unable to send message");
        },
        HelpChoice::Moderation => {
            ctx.say(
                "The moderation commands are:\n\
                `/grab_pfp <user>` - Grabs the profile picture of the specified user\n\
                `/grab_banner <user>` - Grabs the user banner of the specified user",
            )
            .await
            .log_expect("Unable to send message");
        },
        HelpChoice::Developer => {
            ctx.say(
                "The dev commands are:\n\
                `/shutdown` - Shuts down the bot after a 120 second countdown\n\
                `/clean` - Deletes the log file and other temp files\n\
                `/upload_logs` - Uploads the log file to the current channel\n\
                `/echo <message>` - Says the message back\n\
                `/IPM` - Shows the current server IPM\n\
                `/local_update` - Updates the bot from a local file",
            )
            .await
            .log_expect("Unable to send message");
        },
        #[allow(unreachable_patterns)]
        _ => {
            ctx.say(
                "Unknown permission level, the available permission levels are:\n\
                `General` - General commands\n\
                `Information` - Information commands\n\
                `Infraction` - Infraction commands\n\
                `Regex` - Regex commands\n\
                `Moderation` - Moderation commands\n\
                `Developer` - Developer commands",

            )
            .await
            .log_expect("Unable to send message");
        }
    };

    Ok(())
}
