use scorched::*;

use crate::{
    utils::perm_check::{has_perm, PermissionLevel::Developer},
    Data, IpmStruct,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Debug, poise::ChoiceParameter)]
pub enum ResetEnum {
    #[name = "Global IPM reset"]
    Global,
    #[name = "Server IPM reset"]
    Server,
}

#[poise::command(slash_command, guild_cooldown = 5)]
pub async fn reset_ipm(
    ctx: Context<'_>,
    #[description = "Reset Level"] reset_level: ResetEnum,
) -> Result<(), Error> {
    let server_id = ctx.guild_id().unwrap().to_string();

    if !has_perm(
        server_id,
        ctx.author().id.to_string().parse::<u64>().unwrap(),
        Developer,
    )
    .await
    {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("You do not have permission to use this command.")
                    .field("Lacking permissions:", "Developer", false)
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

        return Ok(());
    }

    match reset_level {
        ResetEnum::Global => {
            IpmStruct::global_reset();

            ctx.send(|cr| {
                cr.embed(|ce| ce.title("IPM Reset").description("Reset global IPM to 0"))
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
        ResetEnum::Server => {
            IpmStruct::set_server(ctx.guild_id().unwrap().into(), 0);

            ctx.send(|cr| {
                cr.embed(|ce| ce.title("IPM Reset").description("Reset server IPM to 0"))
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
    }

    Ok(())
}
