use poise::serenity_prelude::Channel;
use scorched::*;

use crate::{utils::config::dead_zones, Data};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Adds a target channel to the dead zones
#[poise::command(
    slash_command,
    guild_cooldown = 10,
    required_permissions = "ADMINISTRATOR"
)]
pub async fn add_dead_zone(
    ctx: Context<'_>,
    #[description = "Target Channel"] channel: Option<Channel>,
) -> Result<(), Error> {
    let channel_id = match channel {
        Some(channel) => channel.id().0,
        None => ctx.channel_id().0,
    };

    match dead_zones::add_dead_zone(ctx.guild_id().unwrap().0.to_string(), channel_id).await {
        true => {
            ctx.send(|cr| {
                cr.embed(|ce| {
                    ce.title("Added dead zone")
                        .description(format!("Added <#{}> to dead zones", channel_id))
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }

        false => {
            ctx.send(|cr| {
                cr.embed(|ce| {
                    ce.title("Failed to add dead zone")
                        .description(format!("Failed to add <#{}> to dead zones", channel_id))
                        .color(0x8B0000)
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
    }

    Ok(())
}
