use crate::{
    utils::logger::LogExpect,
    utils::toml,
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, user_cooldown = 5, required_permissions = "ADMINISTRATOR")]
pub async fn list_staff(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let staff = toml::list_staff();

    let mut staff_list = String::new();
    for staff_member in staff.clone() {
        staff_list.push_str(&format!("{}\n", staff_member));
    }

    ctx.say(format!(
        "Staff List:\n{}",
        staff_list
    ))
    .await
    .log_expect("Unable to send message");

    Ok(())
}