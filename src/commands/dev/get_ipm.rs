use scorched::*;

use crate::{
    utils::perm_check::{has_perm, PermissionLevel::Developer},
    Data, IpmStruct,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, guild_cooldown = 5)]
pub async fn get_ipm(ctx: Context<'_>) -> Result<(), Error> {
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

    ctx.send(|cr| {
        cr.embed(|ce| {
            ce.title("Server IPM").field(
                "IPM:",
                IpmStruct::get_server(ctx.guild_id().unwrap().into()),
                false,
            )
        })
    })
    .await
    .log_expect(LogImportance::Warning, "Unable to send message");

    Ok(())
}
