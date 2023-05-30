use poise::serenity_prelude::Channel;

use crate::{
    utils::logger::LogExpect,
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Debug, poise::ChoiceParameter)]
pub enum ModerationLevelChoice {
    #[name = "Basic: Raid/scam protection"]
    Basic,
    #[name = "Intermediate: Raid/scam protection, and address protection"]
    Intermediate,
    #[name = "Advanced: Raid/scam protection, address protection, and ip protection"]
    Advanced,

}

#[poise::command(prefix_command, slash_command, user_cooldown = 5, required_permissions = "ADMINISTRATOR")]
pub async fn config_server(
    ctx: Context<'_>,
    #[description = "Moderation Level"] ModerationLevel: ModerationLevelChoice,
    #[description = "Log channel"] LogChannel: Channel,
) -> Result<(), Error> {
    //TODO: Add server setup
    ctx.say(
        "Successful!"
    )
    .await
    .log_expect("Unable to send message");
    Ok(())
}