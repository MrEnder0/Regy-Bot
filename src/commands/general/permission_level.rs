use poise::serenity_prelude as serenity;
use scorched::*;

use crate::{utils::perm_check::*, Data};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Returns the permission level of a user
#[poise::command(
    context_menu_command = "Permission Level",
    slash_command,
    user_cooldown = 15,
    ephemeral = true
)]
pub async fn permission_level(
    ctx: Context<'_>,
    #[description = "Target User"] user: serenity::User,
) -> Result<(), Error> {
    let server_id = ctx.guild_id().unwrap().to_string();

    let user_id = user.clone().id.to_string().parse::<u64>().unwrap();

    let member = match ctx.guild_id().unwrap().member(&ctx, user.id).await {
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

    let perm = match highest_unlocked_perm(server_id, user_id, member.roles.clone()).await {
        PermissionLevel::User => "User",
        PermissionLevel::Staff => "Staff",
        PermissionLevel::Developer => "Developer",
    };

    // Reply with the users highest unlocked permission level
    ctx.send(|cr| {
        cr.embed(|ce| {
            ce.title("Permission Level")
                .field("Username", user.clone().name, false)
                .field("Permission Level", perm, false)
        })
    })
    .await
    .log_expect(LogImportance::Warning, "Unable to send message");

    Ok(())
}
