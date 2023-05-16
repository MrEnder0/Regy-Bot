use poise::{
    serenity_prelude::{self as serenity, Ready},
    serenity_prelude::{
        CreateEmbed,
        ChannelId
    }
};
use std::sync::atomic::Ordering;

use crate::utils::{toml::*, logger::*, log_on_error::LogExpect};
use crate::IPM;

pub async fn ready_event(data_about_bot: &Ready, ctx: &serenity::Context) {
    println!("{} is connected!", data_about_bot.user.name);
    let ctx_clone = ctx.clone();
    /* Prints IPM for debug
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            println!("IPM: {}", IPM.load(Ordering::SeqCst));
        }
    });
    */
    // Resets IPM every min
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            IPM.store(0, std::sync::atomic::Ordering::Relaxed);
        }
    });
    // Checks IPM if breaking max activity influx
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            if IPM.load(Ordering::SeqCst) >= get_config().max_activity_influx.into() {
                let data = LogData {
                    importance: "INFO".to_string(),
                    message: "Possible raid detected due to IPM influx.".to_string(),
                };

                log_this(data);
                println!("Possible raid detected due to IPM influx.");

                let log_channel = ChannelId(get_config().log_channel);
                let mut embed = CreateEmbed::default();
                embed.color(0x8B0000);
                embed.title(":warning: Raid Detection");
                embed.field("Possible raid detected due to IPM influx.", "", false);
                embed.thumbnail("https://raw.githubusercontent.com/MrEnder0/Regy-Bot/master/.github/assets/denied.png");
                embed.footer(|f| f.text("False detection? Try increasing the max influx in the config.toml file"));
                log_channel.send_message(&ctx_clone.http, |m| m.content("<@&1009589625230213200>").set_embed(embed)).await.log_expect("Unable to send embed").id;
                IPM.store(0, Ordering::SeqCst);
            }
        }
    });
}