use poise::{
    serenity_prelude as serenity,
    serenity_prelude::{ChannelId, CreateEmbed, MessageUpdateEvent, ReactionType, UserId},
};
use regex::Regex;
use scorched::*;

use crate::{
    utils::{config::*, word_prep::*},
    IpmStruct,
};

pub async fn update_message_event(ctx: &serenity::Context, event: &MessageUpdateEvent) {
    let updated_message = event.content.clone().log_expect(
        LogImportance::Warning,
        "Unable to get updated message content",
    );
    let author = event.author.clone().unwrap();
    let guild_id = event.guild_id;
    let channel_id = event.channel_id;
    let message_id = event.id;

    // Ignore messages from bots
    if author.bot {
        return;
    }

    // Reply to dm messages
    if guild_id.is_none() {
        channel_id.send_message(&ctx.http, |m| m.content("I wish I could dm you but because to my new fav Discord Developer Compliance worker Gatito I cant. :upside_down: Lots of to you :heart:")).await.log_expect(LogImportance::Warning, "Unable to send message");
        return;
    }

    // Checks if server exists in config
    if !read_config()
        .await
        .servers
        .contains_key(&guild_id.unwrap().to_string())
    {
        return;
    }

    // Ignores moderation from devs
    if author.id == 687897073047306270 || author.id == 598280691066732564 {
        return;
    }

    // Ignores moderation from staff
    for user in read_config()
        .await
        .servers
        .get(&guild_id.unwrap().to_string())
        .unwrap()
        .staff
        .iter()
    {
        if author.id == UserId(*user) {
            return;
        }
    }

    let filtered_message = filter_characters(&updated_message.to_lowercase());

    let block_phrases = match { list_regex(guild_id.unwrap().to_string()).await } {
        Some(phrases) => phrases,
        None => {
            log_this(LogData {
                importance: LogImportance::Warning,
                message: format!(
                    "Unable to get regex phrases for server {}",
                    guild_id.unwrap()
                ),
            })
            .await;

            return;
        }
    };

    for regex_phrase in block_phrases {
        if Regex::new(&regex_phrase.phrase)
            .unwrap()
            .is_match(&format!("{} #", filtered_message))
        {
            channel_id
                .delete_message(&ctx.http, message_id)
                .await
                .log_expect(LogImportance::Warning, "Unable to delete message");

            let server_id = guild_id.unwrap().to_string();
            let log_channel = ChannelId(
                read_config()
                    .await
                    .servers
                    .get(&server_id)
                    .unwrap()
                    .log_channel,
            );

            #[cfg(not(feature = "test-deploy"))]
            add_infraction(guild_id.unwrap().to_string(), author.id.into()).await;

            let user_infractions = list_infractions(server_id, author.id.into()).await;

            let user_infractions = match user_infractions {
                Some(infractions) => infractions,
                None => {
                    log_this(LogData {
                        importance: LogImportance::Warning,
                        message: format!("Unable to get infractions for user {}", author.id),
                    })
                    .await;

                    return;
                }
            };

            match (user_infractions >= 10, user_infractions % 5) {
                (true, 0) => {
                    if user_infractions >= 15 {
                        let mut embed = CreateEmbed::default();
                        embed.color(0x556B2F);
                        embed.title("User banned");
                        embed.description("User was banned for reaching 15 infractions");
                        embed.field(
                            "The user who was terminated from the server is:",
                            format!("<@{}>", author.id),
                            true,
                        );
                        embed.thumbnail("https://raw.githubusercontent.com/MrEnder0/Regy-Bot/master/.github/assets/secure.png");
                        log_channel
                            .send_message(&ctx.http, |m| m.set_embed(embed))
                            .await
                            .log_expect(LogImportance::Warning, "Unable to send embed");

                        let user = UserId(author.id.into()).to_user(&ctx.http).await.ok();

                        let dm_msg = "You have been banned from a server due to having 15 infractions, if you believe this is a mistake please contact the server staff.";
                        user.unwrap()
                            .dm(&ctx.http, |m| m.content(dm_msg))
                            .await
                            .log_expect(LogImportance::Warning, "Unable to dm user");

                        let guild = guild_id.unwrap().to_guild_cached(ctx);

                        guild
                            .unwrap()
                            .ban(&ctx, author.id, 0)
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
                        format!("<@{}>", author.id),
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

                    let user = UserId(author.id.into()).to_user(&ctx.http).await.ok();

                    let mut embed = CreateEmbed::default();
                    embed.title("High infraction count");
                    embed.description("This message will appear for every 5 infractions a user gets, note users get banned at 15 infractions");
                    embed.field(
                        "You have these infractions in:",
                        guild_id.unwrap().to_string(),
                        true,
                    );
                    embed.footer(|f| {
                        f.text(
                            "Think this is a mistake? Contact the specified server staff for help",
                        )
                    });
                    embed.thumbnail("https://raw.githubusercontent.com/MrEnder0/Regy-Bot/master/.github/assets/warning.png");

                    user.unwrap()
                        .dm(&ctx.http, |m| m.set_embed(embed))
                        .await
                        .log_expect(LogImportance::Warning, "Unable to dm user");
                }
                _ => {}
            }

            IpmStruct::increment_server(guild_id.unwrap().to_string().parse::<u64>().unwrap());

            log_this(LogData {
                importance: LogImportance::Info,
                message: format!("{} Has edited a message a message which no longer is not allowed due to the set regex patterns", author.id),
            }).await;

            let mut embed = CreateEmbed::default();
            embed.color(0xFFA500);
            embed.title("Message blocked due to matching a set regex pattern");
            embed.field(
                "The user who broke a regex pattern is below:",
                format!("<@{}>", author.id),
                false,
            );
            embed.field(
                "Their message is the following below:",
                format!("||{}||", updated_message),
                false,
            );
            embed.thumbnail("https://raw.githubusercontent.com/MrEnder0/Regy-Bot/master/.github/assets/warning.png");
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

            let temp_msg_content = format!("<@{}> You are not allowed to edit your message to have that due to the server setup regex rules", author.id);
            let temp_msg = channel_id
                .send_message(&ctx.http, |m| m.content(temp_msg_content))
                .await
                .log_expect(LogImportance::Warning, "Unable to send message");
            let ctx_clone = ctx.clone();
            tokio::spawn(async move {
                std::thread::sleep(std::time::Duration::from_secs(5));
                temp_msg.delete(&ctx_clone.http).await.ok();
            });

            let mut dm_embed = CreateEmbed::default();
            dm_embed.color(0xFFA500);
            dm_embed.title("Your message has been blocked due to breaking the servers regex rules");
            dm_embed.field(
                "Your message that was removed is the following:",
                format!("||{}||", updated_message),
                false,
            );
            dm_embed.field(
                "The server that this interference was in is:",
                guild_id.unwrap().to_string(),
                false,
            );
            dm_embed.footer(|f| {
                f.text("Think this is a mistake? Contact the specified server staff for help.")
            });

            UserId(author.id.into())
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
}
