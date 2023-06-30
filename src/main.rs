mod commands;
mod events;
mod utils;

use once_cell::sync::Lazy;
use poise::{serenity_prelude as serenity, Event};
use std::{collections::HashMap, path::Path, sync::Mutex};

use crate::commands::*;
use crate::events::*;
use crate::utils::toml::*;

pub struct Data {}

const IPM: Lazy<Mutex<HashMap<u64, u16>>> = Lazy::new(|| Mutex::new(HashMap::new()));

struct IpmStruct {}

impl IpmStruct {
    pub fn get_server(server_id: u64) -> u16 {
        let binding = IPM;
        let guard = binding.lock().unwrap();

        guard[&server_id]
    }
    pub fn set_server(server_id: u64, value: u16) {
        let binding = IPM;
        let mut guard = binding.lock().unwrap();
        guard.insert(server_id, value);
    }
    pub fn get_overflow() -> Vec<u64> {
        let mut overflow: Vec<u64> = Vec::new();
        let binding = IPM;
        let guard = binding.lock().unwrap();
        for (key, value) in guard.iter() {
            if value > &read_config().global.max_activity_influx {
                overflow.push(*key);
            }
        }

        overflow
    }
    pub fn increment_server(server_id: u64) {
        let binding = IPM;
        let mut guard = binding.lock().unwrap();
        if !guard.contains_key(&server_id) {
            guard.insert(server_id, 1);
        } else {
            let original = guard[&server_id];
            guard.insert(server_id, original + 1);
        }
    }
    pub fn global_reset() {
        IPM.lock().unwrap().clear();
    }
}

#[tokio::main]
async fn main() {
    //check for config file
    if !Path::new("config.toml").exists() {
        gen_config();
    }

    check_config();

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
                moderation::grab_banner::grab_banner(),
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
        .token(read_config().global.token)
        .intents(serenity::GatewayIntents::all())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                let ctx_clone = ctx.clone();
                tokio::spawn(async move {
                    loop {
                        std::thread::sleep(std::time::Duration::from_secs(60));
                        let guild_count = ctx_clone.cache.guilds().len();
                        let activity_msg =
                            format!("over with powerful regex in {} servers.", guild_count);
                        ctx_clone
                            .set_activity(serenity::Activity::watching(&activity_msg))
                            .await;
                    }
                });
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
