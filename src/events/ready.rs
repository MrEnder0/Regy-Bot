use poise::{
    serenity_prelude::{self as serenity, Ready},
    serenity_prelude::{ChannelId, CreateEmbed},
};
use scorched::*;
use std::net::TcpStream;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::{utils::toml::read_config, IpmStruct};

static OFFLINE_TIME: AtomicUsize = AtomicUsize::new(0);

pub async fn ready_event(data_about_bot: &Ready, ctx: &serenity::Context) {
    log_this(LogData {
        importance: LogImportance::Info,
        message: format!(
            "{} has started and connected to discord.",
            data_about_bot.user.name
        ),
    });

    let ctx_clone = ctx.clone();

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
            if overflow.len() > 0 {
                for server in overflow {
                    log_this(LogData {
                        importance: LogImportance::Info,
                        message: "Possible raid detected due to IPM influx.".to_string(),
                    });

                    let log_channel = ChannelId(
                        read_config()
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
                        f.text("False detection? Try increasing the min influx in the config.toml file")
                    });
                    log_channel
                        .send_message(&ctx_clone.http, |m| {
                            m.content("<@&1009589625230213200>").set_embed(embed)
                        })
                        .await
                        .log_expect(LogImportance::Warning, "Unable to send embed")
                        .id;

                    IpmStruct::set_server(server.try_into().unwrap(), 0);
                }
            }
        }
    });
    // Checks if bot is online
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;

            let result = TcpStream::connect("discord.com:443");
            match result {
                Ok(_) => {
                    if OFFLINE_TIME.load(Ordering::SeqCst) > 0 {
                        log_this(LogData {
                            importance: LogImportance::Info,
                            message: format!("The bot has reconnected to Discord after being offline for {} minutes.", OFFLINE_TIME.load(Ordering::SeqCst)+1),
                        });
                        OFFLINE_TIME.store(0, Ordering::SeqCst);
                    }
                }
                Err(_) => {
                    log_this(LogData {
                        importance: LogImportance::Warning,
                        message: format!(
                            "The bot has lost connection, and has been offline for {} minutes.",
                            OFFLINE_TIME.load(Ordering::SeqCst) + 1
                        ),
                    });
                }
            }
        }
    });
}
