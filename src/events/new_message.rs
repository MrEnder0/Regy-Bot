use poise::{
    serenity_prelude as serenity,
    serenity_prelude::{ChannelId, CreateEmbed, ReactionType, UserId},
};
use regex::Regex;
use scorched::*;

use crate::{utils::toml::*, IpmStruct};

pub async fn new_message_event(ctx: &serenity::Context, new_message: &serenity::Message) {
    //ignore messages from bots
    if new_message.author.bot {
        return;
    }

    //Reply to dm messages
    if new_message.guild_id.is_none() {
        new_message.reply(ctx, "I wish I could dm you but because to my new fav Discord Developer Compliance worker Gatito I cant. :upside_down: Lots of love to you :heart:").await.log_expect(LogImportance::Warning, "Unable to reply to dm");
        return;
    }

    let server_id = new_message.guild_id.unwrap().to_string();

    //Poll detection
    let poll_re = Regex::new("\\b(?:let'?’?s|start|begin|initiate)\\s+(?:a\\s+)?(?:poll|vote|survey|opinion poll|questionnaire)\\b|\\bdo\\s+you(?:\\s+guys|\\s+all)?\\s+like\\b|\\bvote\\s+if\\s+you(?:\\s+guys|\\s+all)?\\s+like\\b").unwrap();
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

    //Reply to pings
    if new_message.mentions_user_id(ctx.cache.current_user_id()) {
        let ctx = ctx.clone();
        new_message.reply(ctx, "To use Regy please use the slash commands, ex '/help' to setup server run '/config_setup'").await.log_expect(LogImportance::Warning, "Unable to reply to ping");
    }

    //Check if server exists in config
    if !read_config().servers.contains_key(&server_id) {
        return;
    }

    //Ignores moderation from devs
    if new_message.author.id == 687897073047306270 || new_message.author.id == 598280691066732564 {
        return;
    }

    //Ignores moderation from staff
    for user in read_config().servers.get(&server_id).unwrap().staff.iter() {
        if new_message.author.id == UserId(*user) {
            return;
        }
    }

    let block_phrases_hashmap = list_regex(server_id);
    for phrase in block_phrases_hashmap.as_ref().unwrap().values() {
        let re = Regex::new(&phrase).unwrap();
        if re.is_match(&new_message.content) {
            if let Err(why) = new_message.delete(&ctx.http).await {
                println!("Error deleting message: {:?}", why);
            }

            let server_id = new_message.guild_id.unwrap().to_string();
            add_infraction(server_id, new_message.author.id.into());

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

            let server_id = new_message.guild_id.unwrap().to_string();
            let log_channel = ChannelId(read_config().servers.get(&server_id).unwrap().log_channel);

            let mut embed = CreateEmbed::default();
            embed.color(0xFFA500);
            embed.title("Message blocked due to matching a set regex pattern");
            embed.field(
                "The user who broke a regx pattern is below:",
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

            let user_infractions = list_infractions(server_id, new_message.author.id.into());

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

            match (user_infractions >= 10, user_infractions % 5) {
                (true, 0) => {
                    let mut embed = CreateEmbed::default();
                    embed.color(0x8B0000);
                    embed.title(":warning: High infraction count");
                    embed.field(
                        "The user with the high infractions warning is below:",
                        format!("<@{}>", new_message.author.id),
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
                temp_msg.delete(&ctx_clone.http).await.ok();
            });

            let dm_msg = format!("You are not allowed to send that due to the server setup regex rules, this has been reported to the server staff, continued infractions will result in greater punishment.\n\n\
                                The message which has been blocked is below:\n\
                                ||{}||", new_message.content);

            new_message
                .author
                .dm(&ctx.http, |m| m.content(dm_msg))
                .await
                .log_expect(LogImportance::Warning, "Unable to dm user");

            return;
        }
    }
}
