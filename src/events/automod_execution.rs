use poise::{
    serenity_prelude as serenity,
    serenity_prelude::{ActionExecution, ChannelId, CreateEmbed, ReactionType},
};
use scorched::*;
use std::sync::atomic::Ordering;

use crate::{
    utils::toml::{add_infraction, list_infractions, read_config},
    IPM,
};

pub async fn automod_execution_event(ctx: &serenity::Context, execution: &ActionExecution) {
    //Check if server exists in config
    if !read_config()
        .servers
        .contains_key(&execution.guild_id.to_string())
    {
        return;
    }

    //If action is BlockMessage
    if execution.action != serenity::model::guild::automod::Action::BlockMessage {
        return;
    }

    let user = execution
        .user_id
        .to_user(&ctx.http)
        .await
        .log_expect(LogImportance::Warning, "Unable to get user");
    add_infraction(execution.guild_id.to_string(), user.id.into());

    IPM.store(IPM.load(Ordering::SeqCst) + 1, Ordering::SeqCst);

    log_this(LogData {
        importance: LogImportance::Info,
        message: format!(
            "{} Has sent a message which breaks an auto-mod rule.",
            user.id
        ),
    });

    let log_channel = ChannelId(
        read_config()
            .servers
            .get(&execution.guild_id.to_string())
            .unwrap()
            .log_channel,
    );

    let mut embed = CreateEmbed::default();
    embed.color(0xFFA500);
    embed.title("Message blocked due to matching a set auto-mod pattern");
    embed.field(
        "The user who broke a auto-mod pattern is below:",
        format!("{}", user),
        false,
    );
    embed.field(
        "The detected content is below:",
        format!("||{}||", execution.content),
        false,
    );
    embed.thumbnail(
        "https://raw.githubusercontent.com/MrEnder0/Regy-Bot/master/.github/assets/warning.png",
    );
    embed.footer(|f| f.text("React with 🚫 to dismiss this infraction"));
    let embed_message_id = log_channel
        .send_message(&ctx.http, |m| m.set_embed(embed))
        .await
        .log_expect(LogImportance::Warning, "Unable to send embed")
        .id;
    let embed_message = log_channel.message(&ctx.http, embed_message_id).await.ok();
    embed_message
        .unwrap()
        .react(&ctx.http, ReactionType::Unicode("🚫".to_string()))
        .await
        .ok();

    let user_infractions = list_infractions(execution.guild_id.to_string(), user.id.into());
    let user_infractions = match user_infractions {
        Some(infractions) => infractions,
        None => {
            log_this(LogData {
                importance: LogImportance::Warning,
                message: format!("Unable to get infractions for user {}", user.id),
            });
            return;
        }
    };

    match (user_infractions >= 10, user_infractions % 5) {
        (true, 0) => {
            let mut embed = CreateEmbed::default();
            embed.color(0x8B0000);
            embed.title(":warning: High infraction count");
            embed.field(
                "The user with the high infractions warning is below:",
                format!("<@{}>", user.id),
                false,
            );
            embed.field(
                "The amount of infractions they have is below:",
                format!("{}", user_infractions),
                false,
            );
            embed.footer(|f| {
                f.text("This message will appear for users with high infraction counts")
            });
            embed.thumbnail("https://raw.githubusercontent.com/MrEnder0/Regy-Bot/master/.github/assets/warning.png");
            log_channel
                .send_message(&ctx.http, |m| m.set_embed(embed))
                .await
                .log_expect(LogImportance::Warning, "Unable to send embed");
        }
        _ => {}
    }

    let dm_msg = format!("You are not allowed to send messages with blocked content which breaks the server's setup regex rules, this has been reported to the server staff, continued infractions will result in greater punishment.");
    user.dm(&ctx.http, |m| m.content(dm_msg))
        .await
        .log_expect(LogImportance::Warning, "Unable to dm user");
}
