use poise::serenity_prelude::CreateEmbed;
use scorched::*;

use crate::{
    utils::{
        config,
        perm_check::{has_perm, PermissionLevel::Staff},
    },
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

struct Regex {
    id: String,
    phrase: String,
    is_rti: bool,
}

#[poise::command(
    prefix_command,
    slash_command,
    user_cooldown = 45,
    channel_cooldown = 30
)]
pub async fn list_regex(ctx: Context<'_>) -> Result<(), Error> {
    let server_id = ctx.guild_id().unwrap().to_string();

    if !has_perm(
        server_id.clone(),
        ctx.author().id.to_string().parse::<u64>().unwrap(),
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

    let status_msg = ctx
        .send(|m| m.embed(|e| e.title("Sending regex phrases, this may take a few seconds...")))
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

    let block_phrases = match { config::list_regex(server_id).await } {
        Some(block_phrases) => block_phrases,
        None => {
            ctx.send(|cr| {
                cr.embed(|ce| {
                    ce.title("Database error")
                        .description("This server does not exist in the database, please run `config_setup` first; if you have already done this please add a regex phrase before trying to list them.")
                        .color(0x8B0000)
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");

            return Ok(());
        }
    };

    // TODO: Create paging system for regex phrases
    let mut embed = CreateEmbed::default();
    embed.title("Regex phrases");
    embed.description("The current regex being used are **[WARNING CONTAINS SENSITIVE MESSAGES]**");
    for item in block_phrases.iter() {
        let regex = Regex {
            id: item.uuid.clone(),
            phrase: item.phrase.clone(),
            is_rti: item.is_rti,
        };

        embed.field(
            format!("{} | {}", regex.id, regex.is_rti),
            format!("||{}||", regex.phrase),
            false,
        );
    }

    ctx.channel_id()
        .send_message(&ctx, |m| m.set_embed(embed))
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

    status_msg
        .edit(ctx, |m| {
            m.embed(|e| e.title("Finished sending regex phrases to the channel."))
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to edit message");

    Ok(())
}
