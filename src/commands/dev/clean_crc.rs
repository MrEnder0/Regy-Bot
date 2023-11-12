use scorched::*;

use crate::{
    utils::{
        crc::CacheLevel,
        perm_check::{has_perm, PermissionLevel::Developer},
    },
    CrcStruct, Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Debug, poise::ChoiceParameter)]
pub enum ResetEnum {
    #[name = "Global CRC reset"]
    Global,
    #[name = "Server CRC reset"]
    Server,
}

/// Resets the CRC cache
#[poise::command(slash_command, global_cooldown = 10)]
pub async fn clean_crc(
    ctx: Context<'_>,
    #[description = "Reset Level"] reset_level: ResetEnum,
) -> Result<(), Error> {
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

    match reset_level {
        ResetEnum::Global => {
            CrcStruct::clear_cache(CacheLevel::Global);

            ctx.send(|cr| {
                cr.embed(|ce| ce.title("CRC Reset").description("Reset global CRC cache"))
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
        ResetEnum::Server => {
            CrcStruct::clear_cache(CacheLevel::Server {
                data: ctx.guild_id().unwrap().into(),
            });

            ctx.send(|cr| {
                cr.embed(|ce| ce.title("CRC Reset").description("Reset server CRC cache"))
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
    }

    Ok(())
}
