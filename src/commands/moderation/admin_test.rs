use crate::{
    utils::logger::LogExpect,
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command, user_cooldown = 5, required_permissions = "ADMINISTRATOR")]
pub async fn admin_test(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.say(
        "You have permission to use this command."
    )
    .await
    .log_expect("Unable to send message");
    Ok(())
}