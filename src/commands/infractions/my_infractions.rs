use crate::{
    utils::{
        type_conversions::userid_to_u64,
        logger::{LogExpect, LogImportance},
        toml
    },
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command, user_cooldown = 25, ephemeral = true)]
pub async fn my_infractions(ctx: Context<'_>) -> Result<(), Error> {
    let user_id = userid_to_u64(ctx.author().id);

    let user_infractions = toml::list_infractions(user_id);
    let infractions_message = format!("You have {} infraction(s).", user_infractions);
    ctx.say(infractions_message)
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");
    Ok(())
}