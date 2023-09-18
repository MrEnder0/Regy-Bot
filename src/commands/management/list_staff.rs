use poise::serenity_prelude::UserId;
use scorched::*;

use crate::{utils::config, Data};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(
    slash_command,
    guild_cooldown = 5,
    required_permissions = "ADMINISTRATOR"
)]
pub async fn list_staff(ctx: Context<'_>) -> Result<(), Error> {
    let server_id = ctx.guild_id().unwrap().0.to_string();

    if !config::server_exists(server_id.clone()).await {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("Server does not exist in config")
                    .description(
                        "Please add the server to the config using /config_setup if you are the owner of the server.",
                    )
                    .color(0x8B0000)
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

        return Ok(());
    }

    let staff = config::list_staff(server_id).await;

    match staff {
        Some(staff) => {
            let mut staff_list = String::new();
            for staff_member in staff.clone() {
                let staff_member_user = UserId(staff_member)
                    .to_user(&ctx)
                    .await
                    .log_expect(LogImportance::Warning, "Unable to get user");

                staff_list.push_str(&format!("{}\n", staff_member_user.name));
            }

            ctx.send(|cr| {
                cr.embed(|ce| {
                    ce.title("Staff List")
                        .description(format!("There are {} staff members.", staff.len()))
                        .field("Staff Members", staff_list, false)
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
        None => {
            ctx.send(|cr| {
                cr.embed(|ce| {
                    ce.title("Unable to get staff list")
                        .description("There are no staff members.")
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
    }

    Ok(())
}
