use std::fmt::format;

use scorched::*;
use screenshots::Screen;

use crate::{
    utils::perm_check::{has_perm, PermissionLevel::Developer},
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Debug, poise::ChoiceParameter)]
pub enum ResetEnum {
    #[name = "Global CRC reset"]
    Global,
    #[name = "Server CRC reset"]
    Server,
}

/// Captures and uploads a screenshot of the bot host system for debugging purposes
#[poise::command(slash_command, global_cooldown = 15)]
pub async fn capture_screen(ctx: Context<'_>) -> Result<(), Error> {
    let server_id = ctx.guild_id().unwrap().to_string();

    if !has_perm(
        server_id,
        ctx.author().id.to_string().parse::<u64>().unwrap(),
        Vec::new(),
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

    if !std::path::Path::new("temp").exists() {
        std::fs::create_dir("temp")
            .log_expect(LogImportance::Error, "Unable to create temp dir");
    }

    let screens = Screen::all().log_expect(LogImportance::Error, "Unable to get screen info");
    let capture = screens[0].capture().unwrap();
    capture.save("temp/capture.png").unwrap();

    ctx.send(|cr| {
        cr.embed(|ce| {
            ce.title("Screen Capture")
                .description("Screenshot of the bot host system")
        })
    })
    .await
    .log_expect(LogImportance::Warning, "Unable to send message");

    ctx.channel_id()
        .send_files(ctx, vec!["temp/capture.png"], |m| m)
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

    log_this(LogData {
        importance: LogImportance::Info,
        message: format!("{} ({}) has sent screen capture command in a server with the id {}", ctx.author().id, ctx.author().name, ctx.guild_id().unwrap().0),
    })
    .await;

    Ok(())
}
