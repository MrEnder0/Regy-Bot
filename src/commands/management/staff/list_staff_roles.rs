use scorched::*;

use crate::{
    utils::config::{self, staff},
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(
    slash_command,
    guild_cooldown = 5,
    required_permissions = "ADMINISTRATOR"
)]
pub async fn list_staff_roles(ctx: Context<'_>) -> Result<(), Error> {
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

    let staff_roles = staff::list_staff_roles(server_id).await;

    match staff_roles {
        Some(staff_roles) => {
            let mut staff_role_list = String::new();

            for staff_role in staff_roles.clone() {
                let role_name = ctx
                    .guild()
                    .unwrap()
                    .roles
                    .iter_mut()
                    .find(|role| role.0.as_u64() == &staff_role)
                    .unwrap()
                    .1
                    .name
                    .clone();

                staff_role_list.push_str(&format!("{}\n", role_name));
            }

            ctx.send(|cr| {
                cr.embed(|ce| {
                    ce.title("Staff List")
                        .description(format!("There are {} staff roles.", staff_roles.len()))
                        .field("Staff Roles", staff_role_list, false)
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
        None => {
            ctx.send(|cr| {
                cr.embed(|ce| {
                    ce.title("Unable to get staff role list")
                        .description("Please add a staff role to the server using /add_staff_role.")
                        .color(0x8B0000)
                })
            })
            .await
            .log_expect(LogImportance::Warning, "Unable to send message");
        }
    }

    Ok(())
}
