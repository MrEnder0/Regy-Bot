use poise::serenity_prelude::{CreateEmbed, MessageId};
use scorched::*;

use crate::{
    utils::perm_check::{has_perm, PermissionLevel::Staff},
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command, user_cooldown = 18, ephemeral = true)]
pub async fn nuke(
    ctx: Context<'_>,
    #[description = "Nuke size"] nuke_size: u64,
) -> Result<(), Error> {
    let server_id = ctx.guild_id().unwrap().to_string();

    if !has_perm(
        server_id,
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

    let mut messages = ctx
        .channel_id()
        .messages(&ctx, |retriever| retriever.limit(nuke_size))
        .await
        .log_expect(LogImportance::Warning, "Failed to get messages");

    let mut message_ids: Vec<MessageId> = Vec::new();
    for message in messages.iter_mut() {
        message_ids.push(MessageId::from(message.id));
    }

    if nuke_size < 3 {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title(format!(
                    "Nuking {} messages, this may take a second.",
                    nuke_size
                ))
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

        for message in messages.iter_mut() {
            message.delete(&ctx).await?;
        }
    } else if nuke_size > 300 {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("Invalid nuke size")
                    .description("The nuke size cant be larger than 300")
                    .field("Defined nuke size:", nuke_size, true)
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

        return Ok(());
    } else if nuke_size > 100 {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title(format!(
                    "Nuking {} messages, this may take a second.",
                    nuke_size
                ))
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

        // Deletes messages in chunks of 100
        let mut message_ids: Vec<MessageId> = Vec::new();
        for message in messages.iter_mut() {
            message_ids.push(MessageId::from(message.id));
        }

        let mut message_ids_chunked: Vec<Vec<MessageId>> = message_ids
            .chunks(100)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<Vec<MessageId>>>();

        for chunk in message_ids_chunked.iter_mut() {
            ctx.channel_id().delete_messages(&ctx, chunk).await?;
        }
    } else {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title(format!(
                    "Nuking {} messages, this may take a second.",
                    nuke_size
                ))
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

        ctx.channel_id().delete_messages(&ctx, message_ids).await?;
    }

    let mut embed = CreateEmbed::default();
    embed.title("Nuked Deployed!".to_string());
    embed.field("Nuked by:", ctx.author().name.clone(), true);
    embed.field("Messages nuked:", nuke_size, true);
    embed.thumbnail(
        "https://raw.githubusercontent.com/MrEnder0/Regy-Bot/master/.github/assets/nuke.png",
    );
    embed.footer(|f| f.text("That was a super epic boom boom!"));

    let embed_msg = ctx
        .channel_id()
        .send_message(&ctx, |m| m.set_embed(embed))
        .await?;

    embed_msg.react(&ctx, '🤯').await?;

    Ok(())
}
