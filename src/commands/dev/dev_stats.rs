use std::path::Path;

use scorched::*;

use crate::{
    utils::{
        config::read_config,
        perm_check::{has_perm, PermissionLevel::Developer},
        rti::read_rti,
    },
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Gets stats about the bot
#[poise::command(slash_command, ephemeral = true)]
pub async fn dev_stats(ctx: Context<'_>) -> Result<(), Error> {
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

    let version = env!("CARGO_PKG_VERSION");
    let scorched_version = scorched::VERSION;
    let os = std::env::consts::OS;
    let config_version = read_config().await.meta.version;
    let rti_version = read_rti().await.meta.version;
    let rti_length = read_rti().await.packages.len();
    let update_helper_available = Path::new("regy_bot_update_helper.exe").exists();

    let update_helper_available = if update_helper_available {
        "Available"
    } else {
        "Not available"
    };

    ctx.send(|cr| {
        cr.embed(|ce| {
            ce.title("Dev Stats")
                .field("Version", version, true)
                .field("Scorched version", scorched_version, true)
                .field("OS", os, true)
                .field("Config version", config_version, true)
                .field("RTI version", rti_version, true)
                .field("RTI package count", rti_length, true)
                .field("Update helper status", update_helper_available, true)
                .footer(|fe| {
                    fe.text("Regy's source can be found at https://github.com/MrEnder0/Regy-Bot")
                })
        })
    })
    .await
    .log_expect(LogImportance::Warning, "Unable to send message");

    Ok(())
}
