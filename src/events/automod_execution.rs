use poise::{
    serenity_prelude as serenity,
    serenity_prelude::{ActionExecution, ChannelId, CreateEmbed, ReactionType},
};
use scorched::*;

use crate::{
    utils::config::{
        infractions::{add_infraction, list_infractions},
        read_config,
    },
    IpmStruct,
};

pub async fn automod_execution_event(ctx: &serenity::Context, execution: &ActionExecution) {
    // Checks if server exists in config
    if !read_config()
        .await
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

    #[cfg(not(feature = "test-deploy"))]
    add_infraction(execution.guild_id.to_string(), user.id.into()).await;

    IpmStruct::increment_server(execution.guild_id.into());

    log_this(LogData {
        importance: LogImportance::Info,
        message: format!(
            "{} Has sent a message which breaks an auto-mod rule.",
            user.id
        ),
    });

    let log_channel = ChannelId(
        read_config()
            .await
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
        format!("<@{}>", user.id),
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

    let user_infractions =
        match list_infractions(execution.guild_id.to_string(), user.id.into()).await {
            Some(infractions) => infractions,
            None => {
                log_this(LogData {
                    importance: LogImportance::Warning,
                    message: format!("Unable to get infractions for user {}", user.id),
                });

                return;
            }
        };

    if (user_infractions >= 10, user_infractions % 5) == (true, 0) {
        if user_infractions >= 15 {
            let mut embed = CreateEmbed::default();
            embed.color(0x556B2F);
            embed.title("User banned");
            embed.description("User was banned for reaching 15 infractions");
            embed.field(
                "The user who was terminated from the server is:",
                format!("<@{}>", user.id),
                true,
            );
            embed.thumbnail("https://raw.githubusercontent.com/MrEnder0/Regy-Bot/master/.github/assets/secure.png");
            log_channel
                .send_message(&ctx.http, |m| m.set_embed(embed))
                .await
                .log_expect(LogImportance::Warning, "Unable to send embed");

            let dm_msg = "You have been banned from a server due to having 15 infractions, if you believe this is a mistake please contact the server staff.";
            user.dm(&ctx.http, |m| m.content(dm_msg))
                .await
                .log_expect(LogImportance::Warning, "Unable to dm user");

            let guild = execution
                .guild_id
                .to_guild_cached(&ctx.cache)
                .log_expect(LogImportance::Warning, "Unable to get guild");

            guild
                .ban_with_reason(&ctx.http, user.id, 0, "15 infractions")
                .await
                .log_expect(LogImportance::Warning, "Unable to ban user");

            return;
        }

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
        embed.footer(|f| f.text("This message will appear for users with high infraction counts"));
        embed.thumbnail(
            "https://raw.githubusercontent.com/MrEnder0/Regy-Bot/master/.github/assets/warning.png",
        );
        log_channel
            .send_message(&ctx.http, |m| m.set_embed(embed))
            .await
            .log_expect(LogImportance::Warning, "Unable to send embed");
    }

    let mut dm_embed = CreateEmbed::default();
    dm_embed.color(0xFFA500);
    dm_embed.title("Your message has been blocked due to breaking the servers auto-mod rules");
    dm_embed.field(
        "Your message that was removed is the following:",
        format!("||{}||", execution.content),
        false,
    );
    dm_embed.field(
        "The server that this interference was in is:",
        format!("{}", execution.guild_id),
        false,
    );
    dm_embed.footer(|f| {
        f.text("Think this is a mistake? Contact the specified server staff for help.")
    });

    user.dm(&ctx.http, |m| m.set_embed(dm_embed))
        .await
        .log_expect(LogImportance::Warning, "Unable to dm user");
}
