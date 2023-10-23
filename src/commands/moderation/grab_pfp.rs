use poise::serenity_prelude as serenity;
use scorched::*;

use crate::{
    utils::perm_check::{has_perm, PermissionLevel::Staff},
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command, user_cooldown = 5)]
pub async fn grab_pfp(
    ctx: Context<'_>,
    #[description = "Target User"] user: serenity::User,
) -> Result<(), Error> {
    let server_id = ctx.guild_id().unwrap().to_string();

    let member = match ctx.guild_id().unwrap().member(&ctx, ctx.author().id).await {
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

    if !has_perm(
        server_id,
        ctx.author().id.to_string().parse::<u64>().unwrap(),
        member.roles.clone(),
        Staff,
    )
    .await
    {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("You do not have permission to use this command.")
                    .field("Lacking permissions:", "Staff", false)
                    .color(0x8B0000)
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

        return Ok(());
    }

    ctx.say(user.face())
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");
    Ok(())
}
