use scorched::*;

use crate::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Debug, poise::ChoiceParameter)]
pub enum HelpEnum {
    #[name = "General commands help"]
    General,
    #[name = "Information commands help"]
    Information,
    #[name = "Moderation commands help"]
    Moderation,
    #[name = "Infraction commands help"]
    Infraction,
    #[name = "Management commands help"]
    Management,
    #[name = "Regex commands help"]
    Regex,
    #[name = "Developer commands help"]
    Developer,
}

#[poise::command(slash_command, prefix_command, user_cooldown = 25, ephemeral = true)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Help Type"] help_type: HelpEnum,
) -> Result<(), Error> {
    match help_type {
        HelpEnum::General => {
            ctx.say(
                "The general commands are:\n\
                `/help <choice>` - Shows help on a given subset of commands\n\
                `/permission_level <user>` - Shows the specified user's permission level",
            )
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
        HelpEnum::Information => {
            ctx.say(
                "The information commands are:\n\
                `/about` - Tells you a little about what Regy is\n\
                `/skid` - Explains what a skid is\n\
                `/why_rust` - Shows why rust is the best language\n\
                `/what_is_regex` - Explains what regex is",
            )
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
        HelpEnum::Moderation => {
            ctx.say(
                "The moderation commands are:\n\
                `/grab_pfp <user>` - Grabs the profile picture of the specified user\n\
                `/grab_banner <user>` - Grabs the user banner of the specified user",
            )
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
        HelpEnum::Infraction => {
            ctx.say(
                "The infraction commands are:\n\
                `/add_infraction <user>` - Adds an infraction to the specified user\n\
                `/dismiss_infraction <user>` - Removes an infraction from the specified user\n\
                `/list_infractions <user>` - Lists the infractions of the specified user\n\
                `/my_infractions` - Shows how many infractions you have",
            )
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
        HelpEnum::Management => {
            ctx.say(
                "The management commands are:\n\
                `/add_staff <user>` - Adds the specified user to the staff list\n\
                `/remove_staff <user>` - Removes the specified user from the staff list\n\
                `/list_staff` - Lists all the current staff members\n\
                `/config_setup` - Adds the current server to the config file\n\
                `/config_clone_regex <guild_id>` - Clones the regex phrases from the specified server to the current server",
            )
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
        HelpEnum::Regex => {
            ctx.say(
                "The regex commands are:\n\
                `/add_regex <phrase>` - Adds a new regex phrase to the list\n\
                `/remove_regex <id>` - Removes the specified regex phrase from the list\n\
                `/list_regex` - Lists all the current blocked regex phrases",
            )
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
        HelpEnum::Developer => {
            ctx.say(
                "The dev commands are:\n\
                `/shutdown` - Shuts down the bot after a 120 second countdown\n\
                `/clean_logs` - Deletes the log file\n\
                `/upload_logs` - Uploads the log file to the current channel\n\
                `/echo <echo_msg>` - Says the message back as the bot\n\
                `/get_ipm` - Shows the current server IPM\n\
                `/reset_ipm` - Resets the IPM for the current server\n\
                `/local_update` - Updates the bot from a local file",
            )
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
        #[allow(unreachable_patterns)]
        _ => {
            ctx.say(
                "Unknown permission level, the available permission levels are:\n\
                `General` - General commands\n\
                `Information` - Information commands\n\
                `Moderation` - Moderation commands\n\
                `Infraction` - Infraction commands\n\
                `Management` - Management commands\n\
                `Regex` - Regex commands\n\
                `Developer` - Developer commands",
            )
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
    };

    Ok(())
}
