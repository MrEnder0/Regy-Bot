mod commands;
mod events;
mod utils;

use poise::{serenity_prelude as serenity, Event};
use std::path::Path;
use utils::config::{management::gen_config, updating::update_config};

use crate::events::*;
use crate::utils::config::*;
use crate::utils::ipm::*;

pub struct Data {}

#[tokio::main]
async fn main() {
    // Check for config file
    if !Path::new("config.ron").exists() {
        if Path::new("config.toml").exists() {
            update_config().await;
        } else {
            gen_config().await;
        }
    }

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::commands(),
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

                        Event::GuildMemberAddition { new_member } => {
                            guild_member_join::guild_member_join_event(ctx, new_member).await;
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
