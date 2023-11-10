use scorched::*;

use crate::{utils::config::dead_zones, Data};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Lists all dead zones in the current server
#[poise::command(
    slash_command,
    guild_cooldown = 10,
    required_permissions = "ADMINISTRATOR"
)]
pub async fn list_dead_zones(ctx: Context<'_>) -> Result<(), Error> {
    let dead_zones = dead_zones::list_dead_zones(ctx.guild_id().unwrap().0.to_string()).await;

    if dead_zones.is_none() {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("Unable to list dead zones")
                    .description("There are no dead zones in the current server.")
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

        return Ok(());
    }

    let mut dead_zones_string = String::new();

    for dead_zone in dead_zones.unwrap() {
        dead_zones_string.push_str(&format!("<#{}>\n", dead_zone));
    }

    ctx.send(|cr| cr.embed(|ce| ce.title("Dead Zones").description(dead_zones_string)))
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

    Ok(())
}
