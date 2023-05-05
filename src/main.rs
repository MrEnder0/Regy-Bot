mod events;
mod commands;
mod utils;

use poise::{
    Event,
    serenity_prelude::{self as serenity},
};
use std::{
    path::Path,
    sync::atomic::AtomicUsize
};

use crate::events::{ready::*, reaction_add::*, new_message::*, update_message::*, automod_execution::*, guild_ban::*};
use crate::utils::toml::*;
use crate::commands::{user::*, moderator::*, admin::*, dev::*};

pub struct Data {}

static IPM: AtomicUsize = AtomicUsize::new(0);

#[tokio::main]
async fn main() {
    //check for config file
    if !Path::new("config.toml").exists() {
        gen_config();
    }

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![user(), moderator(), admin(), dev()],
            event_handler: |ctx, event, _framework, _data| {
                Box::pin(async move {
                    match event {
                        Event::Ready { data_about_bot } => {
                            ready_event(data_about_bot, ctx).await;
                            return Ok(());
                        }
                        
                        Event::ReactionAdd { add_reaction, .. } => {
                            reaction_add_event(ctx, add_reaction).await;
                            return Ok(());
                        }

                        Event::Message { new_message } => {
                            new_message_event(ctx, new_message).await;
                            return Ok(());
                        }
                        Event::MessageUpdate { old_if_available: _, new: _, event } => {
                            update_message_event(ctx, event).await;
                            return Ok(());
                        }
                        Event::AutoModerationActionExecution { execution } => {
                            automod_execution_event(ctx, execution).await;
                        }
                        Event::GuildBanAddition { guild_id: _, banned_user } => {
                            guild_ban_event(banned_user).await;
                        }
                        _ => {}
                    }
                    Ok(())
                })
            },
            ..Default::default()
        })
        .token(get_config().token)
        .intents(serenity::GatewayIntents::all())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                let ctx_clone = ctx.clone();
                tokio::spawn(async move {
                    loop {
                        std::thread::sleep(std::time::Duration::from_secs(60));
                        let guild_count = ctx_clone.cache.guilds().len();
                        let activity_msg = format!("Scanning channels with powerful regex in {} servers", guild_count);
                        ctx_clone.set_activity(serenity::Activity::playing(activity_msg)).await;
                    }
                });
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
    });

    framework.run().await.unwrap();
}