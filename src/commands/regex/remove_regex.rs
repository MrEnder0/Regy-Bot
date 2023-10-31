use scorched::*;
use uuid::Uuid;

use crate::{
    utils::{
        config::regex::{self, list_regex},
        perm_check::{has_perm, PermissionLevel::Staff},
    },
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command, user_cooldown = 15)]
pub async fn remove_regex(
    ctx: Context<'_>,
    #[description = "Regex id"]
    #[min_length = 32]
    #[max_length = 36]
    mut id: String,
) -> Result<(), Error> {
    let server_id = ctx.guild_id().unwrap().to_string();

    let member = match ctx.guild_id().unwrap().member(&ctx, ctx.author().id).await {
        Ok(user) => user,
        Err(_) => {
            ctx.send(|cr| {
                cr.embed(|ce| {
                    ce.title("Unable to get user")
                        .description("Please try again later.")
                        .color(0x8B0000)
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");

            return Ok(());
        }
    };

    if !has_perm(
        server_id.clone(),
        ctx.author().id.to_string().parse::<u64>().unwrap(),
        member.roles.clone(),
        Staff,
    )
    .await
    {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("You do not have permission to use this command.")
                    .field("Lacking permissions:", "Staff", false)
                    .color(0x8B0000)
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

        return Ok(());
    }

    id = {
        if id.len() == 32 {
            format!(
                "{}-{}-{}-{}-{}",
                &id[0..8],
                &id[8..12],
                &id[12..16],
                &id[16..20],
                &id[20..32]
            )
        } else {
            id
        }
    };

    if id.len() != 36 {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("Invalid UUID")
                    .description(
                        "Please provide a valid length (36 or 32 characters) UUID to remove.",
                    )
                    .color(0x8B0000)
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

        return Ok(());
    }

    let id = id.parse::<Uuid>().unwrap();

    // Checks if regex with specified id exists in server in config
    let server_id = ctx.guild_id().unwrap().0.to_string();
    let block_phrases = {
        let phrases = list_regex(server_id.clone()).await;
        match phrases {
            Some(phrases) => phrases,
            None => {
                log_this(LogData {
                    importance: LogImportance::Warning,
                    message: format!("Unable to get regex phrases for server {}", server_id),
                })
                .await;

                ctx.send(|cr| {
                    cr.embed(|ce| {
                        ce.title("This server does not exist in the database, please run `config_setup` first; if you have already done this please add a regex phrase before trying to list them.")
                            .color(0x8B0000)
                    })
                })
                .await
                .log_expect(LogImportance::Warning, "Unable to send message");

                return Ok(());
            }
        }
    };

    if !block_phrases.iter().any(|x| x.uuid == id.to_string()) {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("Invalid UUID")
                    .description("The UUID you provided does not exist in the database for the current server.")
                    .color(0x8B0000)
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

        return Ok(());
    }

    let server_id = ctx.guild_id().unwrap().0.to_string();
    regex::remove_regex(server_id, id).await;

    ctx.send(|cr| {
        cr.embed(|ce| {
            ce.title("Regex phrase removed")
                .description(format!("Removed the regex phrase with UUID: {}", id))
        })
    })
    .await
    .log_expect(LogImportance::Warning, "Unable to send message");

    Ok(())
}
