use poise::serenity_prelude as serenity;

use crate::{
    utils::perm_check::*,
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn permissions(
    ctx: Context<'_>,
    #[description = "Checks users permissions"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let userid = user.as_ref().unwrap().id.to_string();

    let perm = match highest_unlocked_perm(userid.parse::<u64>().unwrap()).await {
        PermissionLevel::User => "User",
        PermissionLevel::Moderator => "Moderator",
        PermissionLevel::Admin => "Admin",
        PermissionLevel::Developer => "Developer",
    };

    //reply with the users highest unlocked permission level
    ctx.say(format!(
        "The highest permission level {} has unlocked is {}",
        user.unwrap().name, perm
    )).await?;

    Ok(())
}
