use poise::serenity_prelude as serenity;

use crate::{
    utils::type_conversions::userid_to_u64,
    utils::logger::LogExpect,
    utils::toml,
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, user_cooldown = 5, required_permissions = "ADMINISTRATOR")]
pub async fn add_staff(
    ctx: Context<'_>,
    #[description = "Target User"] user: serenity::User,
) -> Result<(), Error> {
    let userid = user.clone().id;
    let add_staff_status = toml::add_staff(userid_to_u64(userid));

    match add_staff_status {
        true => {
            ctx.say(format!(
                "Added {} to staff",
                user.clone().name
            ))
            .await
            .log_expect("Unable to send message");
        
            user.dm(ctx, |m| {
                m.content(format!(
                    "You have received Regy staff permissions from {} inside {}.",
                    ctx.author().name,
                    ctx.guild().unwrap().name
                ))
            }).await.log_expect("Unable to dm user");
        },
        false => {
            ctx.say(format!(
                "{} is already staff",
                user.clone().name
            ))
            .await
            .log_expect("Unable to send message");
        }
    }

    Ok(())
}