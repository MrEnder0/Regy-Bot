use uuid::Uuid;

use crate::{
    utils::{
        logger::{LogExpect, LogImportance},
        toml
    },
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command, user_cooldown = 10, required_permissions = "ADMINISTRATOR")]
pub async fn remove_regex(
    ctx: Context<'_>,
    #[description = "Regex id"] id: String
) -> Result<(), Error> {
    if id == "none" {
        ctx.say("You need to specify a target UUID.")
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        return Ok(());
    }
    let id = id.parse::<Uuid>().unwrap();
    toml::remove_block_phrase(id);
    let status_message = format!("Removed the regex phrase with UUID: {}", id);
    ctx.say(status_message)
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");
    Ok(())
}