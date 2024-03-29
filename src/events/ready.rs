use poise::{
    serenity_prelude::{self as serenity, Ready},
    serenity_prelude::{ChannelId, CreateEmbed},
};
use scorched::*;
use std::net::TcpStream;

use crate::{
    utils::{
        config::{management::clean_config, read_config},
        rti::download_rti,
    },
    IpmStruct,
};

pub async fn ready_event(data_about_bot: &Ready, ctx: &serenity::Context) {
    log_this(LogData {
        importance: LogImportance::Info,
        message: format!(
            "{} has started and connected to discord.",
            data_about_bot.user.name
        ),
    });

    #[cfg(feature = "test-deploy")]
    log_this(LogData {
        importance: LogImportance::Debug,
        message: "This is a test deployment, infractions will not be tracked.".to_string(),
    })
    .await;

    let ctx_clone = ctx.clone();

    // Downloads RTI on startup
    tokio::spawn(async move {
        download_rti().await;
    });

    // Clean config
    tokio::spawn(async move {
        clean_config().await;
    });

    // Sets bot activity
    let bot_activity_ctx = ctx.clone();
    tokio::spawn(async move {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(60));
            let guild_count = bot_activity_ctx.cache.guilds().len();
            let activity_msg = format!("over with powerful regex in {} servers.", guild_count);
            bot_activity_ctx
                .set_activity(serenity::Activity::watching(&activity_msg))
                .await;
        }
    });
    // Resets IPM every min
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            IpmStruct::global_reset();
        }
    });
    // Checks IPM if breaking max activity influx
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            let overflow = IpmStruct::get_overflow();
            if !overflow.is_empty() {
                for server in overflow {
                    log_this(LogData {
                        importance: LogImportance::Info,
                        message: "Possible raid detected due to IPM influx.".to_string(),
                    });

                    let log_channel = ChannelId(
                        read_config()
                            .await
                            .servers
                            .get(&server.to_string())
                            .unwrap()
                            .log_channel,
                    );
                    let mut embed = CreateEmbed::default();
                    embed.color(0x8B0000);
                    embed.title(":warning: Raid Detection");
                    embed.field("Possible raid detected due to IPM influx.", "", false);
                    embed.thumbnail("https://raw.githubusercontent.com/MrEnder0/Regy-Bot/master/.github/assets/denied.png");
                    embed.footer(|f| {
                        f.text(
                            "False detection? Request a increase the min influx in the ron config.",
                        )
                    });
                    log_channel
                        .send_message(&ctx_clone.http, |m| {
                            m.content("<@&1009589625230213200>").set_embed(embed)
                        })
                        .await
                        .log_expect(LogImportance::Warning, "Unable to send embed");

                    IpmStruct::set_server(server, 0);
                }
            }
        }
    });
    // Checks if bot is online or offline
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;

            let result = TcpStream::connect("discord.com:443");
            match result {
                Ok(_) => {}
                Err(_) => {
                    log_this(LogData {
                        importance: LogImportance::Error,
                        message: "Bot is unable to connect to discord offline.".to_string(),
                    });
                }
            }
        }
    });
}
