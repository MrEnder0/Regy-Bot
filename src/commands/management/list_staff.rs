use crate::{
    utils::{
        logger::{LogExpect, LogImportance},
        toml
    },
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, guild_cooldown = 5, required_permissions = "ADMINISTRATOR")]
pub async fn list_staff(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let server_id = ctx.guild_id().unwrap().0.to_string();
    let staff = toml::list_staff(server_id);

    match staff {
        Some(staff) => {
            let mut staff_list = String::new();
            for staff_member in staff.clone() {
                staff_list.push_str(&format!("{}\n", staff_member));
            }

            ctx.say(format!(
                "Staff List:\n{}",
                staff_list
            ))
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        },
        None => {
            ctx.say("There are no staff members, try adding some with /add_staff or /config_server if the server has not been configured yet.")
                .await
                .log_expect(LogImportance::Warning, "Unable to send message");
        }
    }

    Ok(())
}