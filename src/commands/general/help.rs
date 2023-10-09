use scorched::*;

use crate::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Debug, poise::ChoiceParameter)]
pub enum HelpEnum {
    #[name = "General commands help"]
    General,
    #[name = "Fun commands help"]
    Fun,
    #[name = "Moderation commands help"]
    Moderation,
    #[name = "Infraction commands help"]
    Infraction,
    #[name = "Management commands help"]
    Management,
    #[name = "Regex commands help"]
    Regex,
    #[name = "RTI commands help"]
    Rti,
    #[cfg(feature = "developer-commands")]
    #[name = "Developer commands help"]
    Developer,
}

#[poise::command(slash_command, prefix_command, ephemeral = true)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Help Type"] help_type: HelpEnum,
) -> Result<(), Error> {
    match help_type {
        HelpEnum::General => {
            ctx.send(|cr| {
                cr.embed(|ce| {
                    ce.title("General Commands Help")
                        .field("help", "Shows help on a given subset of commands", false)
                        .field(
                            "permission_level",
                            "Shows the specified user's permission level",
                            false,
                        )
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
        HelpEnum::Fun => {
            ctx.send(|cr| {
                cr.embed(|ce| {
                    ce.title("Fun Commands Help")
                        .field("about", "Tells you a little about what Regy is", false)
                        .field("skid", "Explains what a skid is", false)
                        .field("why_rust", "Shows why rust is the best language", false)
                        .field("what_is_regex", "Explains what regex is", false)
                        .field("random_word", "Shows a random word", false)
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
        HelpEnum::Moderation => {
            ctx.send(|cr| {
                cr.embed(|ce| {
                    ce.title("Moderation Commands Help")
                        .field(
                            "grab_pfp",
                            "Grabs the profile picture of the specified user",
                            false,
                        )
                        .field("nuke", "Deletes the specified amount of messages", false)
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
        HelpEnum::Infraction => {
            ctx.send(|cr| {
                cr.embed(|ce| {
                    ce.title("Infraction Commands Help")
                        .field(
                            "add_infraction",
                            "Adds an infraction to the specified user",
                            false,
                        )
                        .field(
                            "dismiss_infraction",
                            "Removes an infraction from the specified user",
                            false,
                        )
                        .field(
                            "list_infractions",
                            "Lists the infractions of the specified user",
                            false,
                        )
                        .field(
                            "my_infractions",
                            "Shows how many infractions you have",
                            false,
                        )
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
        HelpEnum::Management => {
            ctx.send(|cr| {
                cr.embed(|ce| {
                    ce.title("Management Commands Help")
                        .field(
                            "add_staff",
                            "Adds the specified user to the staff list",
                            false,
                        )
                        .field(
                            "remove_staff",
                            "Removes the specified user from the staff list",
                            false,
                        )
                        .field(
                            "list_staff",
                            "Lists all the current staff members",
                            false,
                        )
                        .field(
                            "config_setup",
                            "Adds the current server to the config file",
                            false,
                        )
                        .field(
                            "config_clone_regex",
                            "Clones the regex phrases from the specified server to the current server",
                            false,
                        )
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
        HelpEnum::Regex => {
            ctx.send(|cr| {
                cr.embed(|ce| {
                    ce.title("Regex Commands Help")
                        .field("add_regex", "Adds a new regex phrase to the list", false)
                        .field(
                            "remove_regex",
                            "Removes the specified regex phrase from the list",
                            false,
                        )
                        .field(
                            "list_regex",
                            "Lists all the current blocked regex phrases",
                            false,
                        )
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
        HelpEnum::Rti => {
            ctx.send(|cr| {
                cr.embed(|ce| {
                    ce.title("RTI Commands Help")
                        .field(
                            "search_rti",
                            "Searches the RTI packages given a specified search",
                            false,
                        )
                        .field(
                            "update_rti",
                            "Updates the RTI packages in the current server",
                            false,
                        )
                        .field(
                            "reload_rti",
                            "Re-downloads the online RTI package list (Global Cool-down: 120 seconds)",
                            false,
                        )
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
        #[cfg(feature = "developer-commands")]
        HelpEnum::Developer => {
            ctx.send(|cr| {
                cr.embed(|ce| {
                    ce.title("Developer Commands Help")
                        .field(
                            "shutdown",
                            "Shuts down the bot after a 120 second countdown",
                            false,
                        )
                        .field("clean_logs", "Deletes the log file", false)
                        .field(
                            "upload_logs",
                            "Uploads the log file to the current channel",
                            false,
                        )
                        .field(
                            "echo",
                            "Returns the specified the message back as the bot",
                            false,
                        )
                        .field("get_ipm", "Shows the current server IPM", false)
                        .field("reset_ipm", "Resets the IPM for the current server", false)
                        .field("update", "Updates the bot from a local file", false)
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
    };

    Ok(())
}
