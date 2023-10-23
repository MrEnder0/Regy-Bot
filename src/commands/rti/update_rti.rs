use poise::serenity_prelude::CreateEmbed;
use scorched::*;

use crate::{
    utils::perm_check::{has_perm, PermissionLevel::Staff},
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command, user_cooldown = 5)]
pub async fn update_rti(ctx: Context<'_>) -> Result<(), Error> {
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

    let mut embed = CreateEmbed::default();
    embed.title("Are you sure you want to update your RTI package?");
    embed.description("Doing this will get all your RTI packages to the latest version which may change the way they detect phrases.");
    embed.footer(|fe| fe.text("React with ✅ to verify or react with ❌ to cancel"));

    let channel_id = ctx.channel_id();
    let embed_msg = channel_id
        .send_message(&ctx, |m| m.set_embed(embed))
        .await?;

    embed_msg.react(&ctx, '✅').await?;
    embed_msg.react(&ctx, '❌').await?;

    ctx.send(|cr| {
        cr.embed(|ce| {
            ce.title("Sent prompt to update RTI package")
                .description("Read the embed below and react with your choice to continue")
        })
    })
    .await
    .log_expect(LogImportance::Warning, "Unable to send message");

    Ok(())
}
