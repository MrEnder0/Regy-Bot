use poise::serenity_prelude::CreateEmbed;

use crate::{
    utils::perm_check::{has_perm, PermissionLevel::Developer},
    utils::{logger::{LogExpect, LogData, log_this, LogImportance}, toml::get_config},
    Data
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, ephemeral = true)]
pub async fn shutdown(
    ctx: Context<'_>,
) -> Result<(), Error> {
    if !has_perm(ctx.author().id.to_string().parse::<u64>().unwrap(), Developer).await {
        ctx.say("You do not have permission to use this command.")
            .await
            .log_expect("Unable to send message");
        return Ok(());
    }

    if !get_config().allow_shutdown {
        ctx.say("Remote shutdown is not enabled on host.").await?;
        return Ok(());
    }

    let msg_author = ctx.author().id;
    println!("Shutdown from dev commands sent from {}", msg_author);

    log_this(LogData {
        importance: LogImportance::Info,
        message: format!("Shutdown from dev commands sent from {}", msg_author),
    });

    ctx.say("Initialized shutdown countdown for 90 seconds")
        .await
        .log_expect("Unable to send message");

    for i in 0..90 {
        let mut embed = CreateEmbed::default();
        embed.color(0x565e6e);
        embed.title("Regy Shutdown");
        if i > 80 {
            embed.description(format!(
                ":warning: Regy will be shutdown in the following seconds: {}",
                90 - i
            ));
        } else {
            embed.description(format!(
                "Regy will be shutdown in the following seconds: {}",
                90 - i
            ));
        }
        embed.thumbnail("https://raw.githubusercontent.com/MrEnder0/Regy-Bot/master/.github/assets/shutdown.png");
        embed.footer(|f| f.text("This force shutdown was sent from a dev"));
        let embed_message = ctx
            .channel_id()
            .send_message(&ctx, |m| m.set_embed(embed))
            .await
            .log_expect("Unable to send shutdown embed")
            .id;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        ctx.channel_id()
            .delete_message(&ctx, embed_message)
            .await
            .ok();
    }
    ctx.say("Countdown finished, shutting down...").await?;
    std::process::exit(0);
}