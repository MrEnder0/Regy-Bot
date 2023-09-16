use poise::serenity_prelude::CreateEmbed;
use scorched::*;

use crate::{
    utils::{
        perm_check::{has_perm, PermissionLevel::Developer},
        updater::local_update,
    },
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, ephemeral = true)]
pub async fn update(ctx: Context<'_>) -> Result<(), Error> {
    let server_id = ctx.guild_id().unwrap().to_string();

    if !has_perm(
        server_id,
        ctx.author().id.to_string().parse::<u64>().unwrap(),
        Developer,
    )
    .await
    {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("You do not have permission to use this command.")
                    .field("Lacking permissions:", "Developer", false)
                    .color(0x8B0000)
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

        return Ok(());
    }

    let mut embed = CreateEmbed::default();
    embed.color(0x565e6e);
    embed.title("Regy Update");
    embed.thumbnail(
        "https://raw.githubusercontent.com/MrEnder0/Regy-Bot/master/.github/assets/update.png",
    );
    embed.description("A local update has been initialized.");
    embed.footer(|f| f.text("If the update fails you will be notified automatically."));
    ctx.channel_id()
        .send_message(&ctx, |m| m.set_embed(embed))
        .await
        .log_expect(LogImportance::Warning, "Unable to send update embed");

    let update = local_update("regy_update.exe");

    match update {
        0 => {
            let mut embed = CreateEmbed::default();
            embed.color(0x565e6e);
            embed.title("Regy Update");
            embed.description("Update has failed, bot will return to normal operation.");
            embed.footer(|f| {
                f.text("Tip: Make sure you put the update file in the right directory")
            });
            ctx.channel_id()
                .send_message(&ctx, |m| m.set_embed(embed))
                .await
                .log_expect(LogImportance::Warning, "Unable to send failed update embed");

            log_this(LogData {
                importance: LogImportance::Error,
                message: "Update has failed, bot will return to normal operation.".to_string(),
            })
            .await;

            Ok(())
        }
        1 => {
            let mut embed = CreateEmbed::default();
            embed.color(0x565e6e);
            embed.title("Regy Update");
            embed.description("Update has been successful, but a update helper was not found, please restart the bot manually to finish the update.");
            embed.footer(|f| f.text("Closing and reopening Regy will finish the update"));
            ctx.channel_id()
                .send_message(&ctx, |m| m.set_embed(embed))
                .await
                .log_expect(
                    LogImportance::Warning,
                    "Unable to send partial update embed",
                );

            log_this(LogData {
                importance: LogImportance::Warning,
                message: "Update has been successful, but a update helper was not found, please restart the bot manually to finish the update.".to_string(),
            }).await;

            Ok(())
        }
        2 => {
            let mut embed = CreateEmbed::default();
            embed.color(0x565e6e);
            embed.title("Regy Update");
            embed.description("Update has been successful, bot will restart.");
            embed.footer(|f| f.text("Regy will now restart to finish the update"));
            ctx.channel_id()
                .send_message(&ctx, |m| m.set_embed(embed))
                .await
                .log_expect(
                    LogImportance::Warning,
                    "Unable to send successful update embed",
                );

            ctx.say("Update has been successful, bot will restart.")
                .await
                .log_expect(LogImportance::Warning, "Unable to send message");

            log_this(LogData {
                importance: LogImportance::Info,
                message: "Update has been successful, bot will restart.".to_string(),
            })
            .await;

            std::process::Command::new("regy_bot_update_helper.exe")
                .spawn()
                .log_expect(LogImportance::Warning, "Unable to run update helper");
            std::process::exit(0);
        }
        _ => {
            let mut embed = CreateEmbed::default();
            embed.color(0x565e6e);
            embed.title("Regy Update");
            embed.description("Update has finished with an unknown outcome, bot will return to normal operation and ignore the update.");
            embed.footer(|f| f.text("Tip: Try running the update helper"));
            ctx.channel_id()
                .send_message(&ctx, |m| m.set_embed(embed))
                .await
                .log_expect(
                    LogImportance::Warning,
                    "Unable to send unknown update status embed",
                );

            log_this(LogData {
                importance: LogImportance::Error,
                message: "Update has finished with an unknown outcome, bot will return to normal operation and ignore the update and its possible lingering side-effects.".to_string(),
            }).await;

            Ok(())
        }
    }
}
