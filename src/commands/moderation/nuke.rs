use poise::serenity_prelude::{CreateEmbed, MessageId};
use scorched::*;

use crate::{
    utils::perm_check::{has_perm, PermissionLevel::Staff},
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command, user_cooldown = 20, ephemeral = true)]
pub async fn nuke(
    ctx: Context<'_>,
    #[description = "Nuke size"] nuke_size: Option<u64>,
) -> Result<(), Error> {
    let server_id = ctx.guild_id().unwrap().to_string();

    if !has_perm(
        server_id,
        ctx.author().id.to_string().parse::<u64>().unwrap(),
        Staff,
    )
    .await
    {
        ctx.say("You do not have permission to use this command.")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        return Ok(());
    }

    let nuke_size = match nuke_size {
        Some(nuke_size) => {
            if nuke_size > 100 {
                100
            } else if nuke_size < 3 {
                3
            } else {
                nuke_size
            }
        }
        None => 25,
    };

    let mut messages = ctx
        .channel_id()
        .messages(&ctx, |retriever| retriever.limit(nuke_size))
        .await.log_expect(LogImportance::Warning, "Failed to get messages");

    let mut message_ids: Vec<MessageId> = Vec::new();
    for message in messages.iter_mut() {
        message_ids.push(MessageId::from(message.id));
    }

    ctx.channel_id().delete_messages(&ctx, message_ids).await?;

    ctx.say(format!("Nuked {} messages!", nuke_size))
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

    let mut embed = CreateEmbed::default();
    embed.title("Nuked Deployed!".to_string());
    embed.field("Nuked by:", ctx.author().name.clone(), true);
    embed.field("Messages nuked:", nuke_size, true);
    embed.thumbnail("https://raw.githubusercontent.com/MrEnder0/Regy-Bot/master/.github/assets/nuke.png");
    embed.footer(|f| f.text("That was a super epic boom boom!"));

    let embed_msg = ctx.channel_id()
        .send_message(&ctx, |m| m.set_embed(embed))
        .await?;
    
    embed_msg.react(&ctx, '🤯').await?;

    Ok(())
}
