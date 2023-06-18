use poise::serenity_prelude as serenity;

use crate::{
    utils::{
        logger::{LogExpect, LogImportance},
        perm_check::{has_perm, PermissionLevel::Staff},
    },
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command, user_cooldown = 5)]
pub async fn grab_banner(
    ctx: Context<'_>,
    #[description = "Target User"] user: serenity::User,
) -> Result<(), Error> {
    let server_id = ctx.guild_id().unwrap().to_string();

    if !has_perm(
        server_id,
        ctx.author().id.to_string().parse::<u64>().unwrap(),
        Staff,
    )
    .await
    {
        ctx.say("You do not have permission to use this command.")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        return Ok(());
    }

    ctx.say(
        user.banner_url()
            .unwrap_or("This user does not have a banner.".to_string()),
    )
    .await
    .log_expect(LogImportance::Warning, "Unable to send message");
    Ok(())
}
