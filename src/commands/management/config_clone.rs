use scorched::*;

use crate::{
    utils::config::{add_regex, list_regex, server_exists},
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(
    prefix_command,
    slash_command,
    guild_cooldown = 240,
    required_permissions = "ADMINISTRATOR"
)]
pub async fn config_clone_regex(
    ctx: Context<'_>,
    #[description = "Guild ID"] target_server_id: String,
) -> Result<(), Error> {
    // Checks if target server exists in database
    if !server_exists(target_server_id.clone()).await {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("Target server does not exist in the database.")
                    .description(
                        "Please make sure the target server exists in the database or that the server id is correct and try again.",
                    )
                    .color(0x8B0000)
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");
    } else {
        // Checks if current server exists in database
        if !server_exists(ctx.guild_id().unwrap().0.to_string()).await {
            ctx.send(|cr| {
                cr.embed(|ce| {
                    ce.title("Current server does not exist in the database.")
                        .description(
                            "Please add the server to the config using /config_setup if you are the owner of the server.",
                        )
                        .color(0x8B0000)
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");

            return Ok(());
        }

        let target_block_phrases = match { list_regex(target_server_id.clone()).await } {
            Some(phrases) => phrases,
            None => {
                log_this(LogData {
                    importance: LogImportance::Warning,
                    message: format!(
                        "Unable to get regex phrases for server {}",
                        target_server_id
                    ),
                })
                .await;

                ctx.send(|cr| {
                    cr.embed(|ce| {
                        ce.title("Unable to get regex phrases for target server.")
                            .description(
                                "Please make sure the target server exists in the database or that the server id is correct and try again.",
                            )
                            .color(0x8B0000)
                    })
                })
                .await
                .log_expect(LogImportance::Warning, "Unable to send message");

                return Ok(());
            }
        };

        for phrase in target_block_phrases.iter() {
            add_regex(
                ctx.guild_id().unwrap().0.to_string(),
                phrase.phrase.clone(),
                false,
                "No description provided.".to_string(),
                0,
            )
            .await;
        }

        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("Successfully cloned regex phrases from target server.")
                    .description(format!(
                        "Successfully cloned {} regex phrases from target server.",
                        target_block_phrases.len()
                    ))
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");
    }

    Ok(())
}
