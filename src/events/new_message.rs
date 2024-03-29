use ::regex::Regex;
use poise::{
    serenity_prelude as serenity,
    serenity_prelude::{ChannelId, CreateEmbed, ReactionType, UserId},
};
use scorched::*;

use crate::{
    utils::{
        config::{
            dead_zones::is_dead_zone,
            infractions::{add_infraction, list_infractions},
            read_config,
        },
        perm_check::{highest_unlocked_perm, PermissionLevel},
        word_prep::*,
    },
    CrcStruct, IpmStruct,
};

pub async fn new_message_event(ctx: &serenity::Context, new_message: &serenity::Message) {
    // Ignores messages from bots
    if new_message.author.bot {
        return;
    }

    // Reply to dm messages
    if new_message.guild_id.is_none() {
        new_message.reply(ctx, "I wish I could dm you but because to my new fav Discord Developer Compliance worker Gatito I cant. :upside_down: Lots of love to you :heart:").await.log_expect(LogImportance::Warning, "Unable to reply to dm");
        return;
    }

    let server_id = new_message.guild_id.unwrap().to_string();

    // Check if server exists in config
    if !read_config().await.servers.contains_key(&server_id) {
        return;
    }

    // Ignore message if in dead zone
    if is_dead_zone(server_id.clone(), new_message.channel_id.into()).await {
        return;
    }

    // Reply standard to pings
    if new_message.mentions_user_id(ctx.cache.current_user_id()) {
        let ctx = ctx.clone();
        new_message.reply(ctx, "To use Regy please use the slash commands, ex '/help' to setup server run '/config_setup'").await.log_expect(LogImportance::Warning, "Unable to reply to ping");
    }

    let filtered_message = filter_characters(&new_message.content.to_lowercase());

    if !CrcStruct::check_cache(
        server_id
            .parse::<u64>()
            .log_expect(LogImportance::Warning, "Unable to parse server id"),
    ) {
        CrcStruct::build_server_cache(
            server_id
                .parse::<u64>()
                .log_expect(LogImportance::Warning, "Unable to parse server id"),
        );
    }

    let cached_regex = CrcStruct::load_server_cache(
        server_id
            .parse::<u64>()
            .log_expect(LogImportance::Warning, "Unable to parse server id"),
    )
    .regex;

    for regex_phrase in cached_regex {
        // Ignores moderation from devs
        if new_message.author.id == 687897073047306270
            || new_message.author.id == 598280691066732564
        {
            if new_message.mentions_user_id(ctx.cache.current_user_id()) {
                let ctx = ctx.clone();
                new_message
                    .reply(ctx, "OMG ITS A SUPER COOL REGY DEV!!!")
                    .await
                    .log_expect(LogImportance::Warning, "Unable to reply to ping");
            }

            break;
        }

        // Ignores moderation from staff
        if highest_unlocked_perm(
            server_id.clone(),
            new_message.author.id.into(),
            new_message
                .member(ctx)
                .await
                .unwrap()
                .roles(ctx)
                .unwrap()
                .iter_mut()
                .map(|role| role.id)
                .collect::<Vec<_>>(),
        )
        .await
            != PermissionLevel::User
        {
            break;
        }

        if regex_phrase.is_match(&format!("{} #", filtered_message)) {
            new_message
                .delete(&ctx.http)
                .await
                .log_expect(LogImportance::Warning, "Unable to delete message");

            let server_id = new_message.guild_id.unwrap().to_string();
            let log_channel = ChannelId(
                read_config()
                    .await
                    .servers
                    .get(&server_id)
                    .unwrap()
                    .log_channel,
            );

            #[cfg(not(feature = "test-deploy"))]
            add_infraction(
                new_message.guild_id.unwrap().to_string(),
                new_message.author.id.into(),
            )
            .await;

            let user_infractions = list_infractions(server_id, new_message.author.id.into()).await;

            let user_infractions = match user_infractions {
                Some(infractions) => infractions,
                None => {
                    log_this(LogData {
                        importance: LogImportance::Warning,
                        message: format!(
                            "Unable to get infractions for user {}",
                            new_message.author.id
                        ),
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
                        format!("<@{}>", new_message.author.id),
                        true,
                    );
                    embed.thumbnail("https://raw.githubusercontent.com/MrEnder0/Regy-Bot/master/.github/assets/secure.png");
                    log_channel
                        .send_message(&ctx.http, |m| m.set_embed(embed))
                        .await
                        .log_expect(LogImportance::Warning, "Unable to send embed");

                    let user = UserId(new_message.author.id.into())
                        .to_user(&ctx.http)
                        .await
                        .ok();

                    let dm_msg = "You have been banned from a server due to having 15 infractions, if you believe this is a mistake please contact the server staff.";
                    user.unwrap()
                        .dm(&ctx.http, |m| m.content(dm_msg))
                        .await
                        .log_expect(LogImportance::Warning, "Unable to dm user");

                    new_message
                        .guild(ctx)
                        .unwrap()
                        .ban_with_reason(&ctx, new_message.author.id, 0, "15 infractions")
                        .await
                        .log_expect(LogImportance::Warning, "Unable to ban user");

                    return;
                }

                let mut embed = CreateEmbed::default();
                embed.color(0x8B0000);
                embed.title(":warning: High infraction count");
                embed.description("This message will appear for every 5 infractions a user gets, note users get banned at 15 infractions");
                embed.field(
                    "The user with the high infractions warning is:",
                    format!("<@{}>", new_message.author.id),
                    true,
                );
                embed.field(
                    "The user has the following amount of infractions:",
                    format!("{}", user_infractions),
                    true,
                );
                embed.thumbnail("https://raw.githubusercontent.com/MrEnder0/Regy-Bot/master/.github/assets/warning.png");
                log_channel
                    .send_message(&ctx.http, |m| m.set_embed(embed))
                    .await
                    .log_expect(LogImportance::Warning, "Unable to send embed");

                let user = UserId(new_message.author.id.into())
                    .to_user(&ctx.http)
                    .await
                    .ok();

                let mut embed = CreateEmbed::default();
                embed.title("High infraction count");
                embed.description("This message will appear for every 5 infractions a user gets, note users get banned at 15 infractions");
                embed.field(
                    "You have these infractions in:",
                    new_message.guild_id.unwrap().to_string(),
                    true,
                );
                embed.footer(|f| {
                    f.text("Think this is a mistake? Contact the specified server staff for help")
                });
                embed.thumbnail("https://raw.githubusercontent.com/MrEnder0/Regy-Bot/master/.github/assets/warning.png");

                user.unwrap()
                    .dm(&ctx.http, |m| m.set_embed(embed))
                    .await
                    .log_expect(LogImportance::Warning, "Unable to dm user");
            }

            IpmStruct::increment_server(
                new_message
                    .guild_id
                    .unwrap()
                    .to_string()
                    .parse::<u64>()
                    .unwrap(),
            );

            log_this(LogData {
                importance: LogImportance::Info,
                message: format!(
                    "{} Has sent a message which is not allowed due to the set regex patterns",
                    new_message.author.id
                ),
            });

            let mut embed = CreateEmbed::default();
            embed.color(0xFFA500);
            embed.title("Message blocked due to matching a set regex pattern");
            embed.field(
                "The user who broke a regex pattern is below:",
                format!("<@{}>", new_message.author.id),
                false,
            );
            embed.field(
                "Their message is the following below:",
                format!("||{}||", new_message.content),
                false,
            );
            embed.footer(|f| f.text("React with 🚫 to dismiss this infraction"));
            embed.thumbnail("https://raw.githubusercontent.com/MrEnder0/Regy-Bot/master/.github/assets/warning.png");
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

            let temp_msg_content = format!(
                "<@{}> You are not allowed to send that due to the server setup regex rules",
                new_message.author.id
            );
            let temp_msg = new_message
                .channel_id
                .say(&ctx.http, temp_msg_content)
                .await
                .log_expect(LogImportance::Warning, "Unable to send message");

            let ctx_clone = ctx.clone();
            tokio::spawn(async move {
                std::thread::sleep(std::time::Duration::from_secs(5));
                temp_msg
                    .delete(&ctx_clone.http)
                    .await
                    .log_expect(LogImportance::Warning, "Unable to delete message");
            });

            let mut dm_embed = CreateEmbed::default();
            dm_embed.color(0xFFA500);
            dm_embed.title("Your message has been blocked due to breaking the servers regex rules");
            dm_embed.field(
                "Your message that was removed is the following:",
                format!("||{}||", new_message.content),
                false,
            );
            dm_embed.field(
                "The server that this interference was in is:",
                new_message.guild_id.unwrap().to_string(),
                false,
            );
            dm_embed.footer(|f| {
                f.text("Think this is a mistake? Contact the specified server staff for help.")
            });

            UserId(new_message.author.id.into())
                .to_user(&ctx.http)
                .await
                .ok()
                .unwrap()
                .dm(&ctx.http, |m| m.set_embed(dm_embed))
                .await
                .log_expect(LogImportance::Warning, "Unable to dm user");

            return;
        }
    }

    // Poll detection
    let poll_re = Regex::new("(?i)\\b(?:let'?’?s|start|begin|initiate)\\s+(?:a\\s+)?(?:poll|vote|survey|poll|questionnaire)\\b|\\bdo\\s+you(?:\\s+guys|\\s+all)?\\s+like\\b|\\bvote\\s+if\\s+you(?:\\s+guys|\\s+all)?\\s+like\\b").unwrap();
    if poll_re.is_match(&new_message.content) {
        new_message
            .react(&ctx.http, ReactionType::Unicode("👍".to_string()))
            .await
            .ok();
        new_message
            .react(&ctx.http, ReactionType::Unicode("👎".to_string()))
            .await
            .ok();
    }
}
