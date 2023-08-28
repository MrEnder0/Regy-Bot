mod commands;
mod events;
mod utils;

use poise::{serenity_prelude as serenity, Event};
use std::path::Path;

use crate::commands::*;
use crate::events::*;
use crate::utils::config::*;
use crate::utils::ipm::*;

pub struct Data {}

#[tokio::main]
async fn main() {
    //check for config file
    if !Path::new("config.ron").exists() {
        if Path::new("config.toml").exists() {
            update_config().await;
        } else {
            gen_config().await;
        }
    }

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                general::help::help(),
                general::permission_level::permission_level(),
                info::about::about(),
                info::why_rust::why_rust(),
                info::what_is_regex::what_is_regex(),
                info::skid::skid(),
                infractions::my_infractions::my_infractions(),
                infractions::add_infraction::add_infraction(),
                infractions::dismiss_infraction::dismiss_infraction(),
                infractions::list_infractions::list_infractions(),
                moderation::grab_pfp::grab_pfp(),
                regex::add_regex::add_regex(),
                regex::remove_regex::remove_regex(),
                regex::list_regex::list_regex(),
                management::add_staff::add_staff(),
                management::remove_staff::remove_staff(),
                management::list_staff::list_staff(),
                management::config_setup::config_setup(),
                management::config_clone::config_clone_regex(),
                dev::upload_logs::upload_logs(),
                dev::clean_logs::clean_logs(),
                dev::get_ipm::get_ipm(),
                dev::reset_ipm::reset_ipm(),
                dev::echo::echo(),
                dev::shutdown::shutdown(),
                dev::local_update::update(),
                rti::search_rti::search_rti(),
                rti::update_rti::update_rti(),
            ],
            event_handler: |ctx, event, _framework, _data| {
                Box::pin(async move {
                    match event {
                        Event::Ready { data_about_bot } => {
                            ready::ready_event(data_about_bot, ctx).await;
                            return Ok(());
                        }

                        Event::ReactionAdd { add_reaction, .. } => {
                            reaction_add::reaction_add_event(ctx, add_reaction).await;
                            return Ok(());
                        }

                        Event::Message { new_message } => {
                            new_message::new_message_event(ctx, new_message).await;
                            return Ok(());
                        }

                        Event::MessageUpdate {
                            old_if_available: _,
                            new: _,
                            event,
                        } => {
                            update_message::update_message_event(ctx, event).await;
                            return Ok(());
                        }

                        Event::AutoModerationActionExecution { execution } => {
                            automod_execution::automod_execution_event(ctx, execution).await;
                        }

                        Event::GuildBanAddition {
                            guild_id,
                            banned_user,
                        } => {
                            guild_ban::guild_ban_event(*guild_id, banned_user).await;
                        }

                        _ => {}
                    }
                    Ok(())
                })
            },
            ..Default::default()
        })
        .token(read_config().await.global.token)
        .intents(serenity::GatewayIntents::all())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
