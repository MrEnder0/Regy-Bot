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

    //ignore messages from bots
    if author.bot {
        return;
    }

    //Reply to dm messages
    if guild_id.is_none() {
        channel_id.send_message(&ctx.http, |m| m.content("I wish I could dm you but because to my new fav Discord Developer Compliance worker Gatito I cant. :upside_down: Lots of to you :heart:")).await.log_expect(LogImportance::Warning, "Unable to send message");
        return;
    }

    //Check if server exists in config
    if guild_id.is_some() {
        if !read_config()
            .servers
            .contains_key(&guild_id.unwrap().to_string())
        {
            return;
        }
    }

    //Ignores moderation from devs
    if author.id == 687897073047306270 || author.id == 598280691066732564 {
        return;
    }

    //Ignores moderation from staff
    for user in read_config()
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

    let block_phrases_hashmap = list_regex(guild_id.unwrap().to_string());
    for phrase in block_phrases_hashmap.as_ref().unwrap().values() {
        let re = Regex::new(&phrase).unwrap();
        if re.is_match(&filtered_message) {
            if let Err(why) = channel_id.delete_message(&ctx.http, message_id).await {
                println!("Error deleting message: {:?}", why);
            }

            let server_id = guild_id.unwrap().to_string();
            add_infraction(server_id, author.id.into());

            IpmStruct::increment_server(guild_id.unwrap().to_string().parse::<u64>().unwrap());

            log_this(LogData {
                importance: LogImportance::Info,
                message: format!("{} Has edited a message a message which no longer is not allowed due to the set regex patterns", author.id),
            });

            let server_id = guild_id.unwrap().to_string();
            let log_channel = ChannelId(read_config().servers.get(&server_id).unwrap().log_channel);

            let mut embed = CreateEmbed::default();
            embed.color(0xFFA500);
            embed.title("Message blocked due to matching a set regex pattern");
            embed.field(
                "The user who broke a regx pattern is below:",
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

            let user_infractions = list_infractions(server_id, author.id.into());

            let user_infractions = match user_infractions {
                Some(infractions) => infractions,
                None => {
                    log_this(LogData {
                        importance: LogImportance::Warning,
                        message: format!("Unable to get infractions for user {}", author.id),
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
                        format!("<@{}>", author.id),
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

            //TODO: Change message to embed

            let dm_msg = format!("You are not allowed to edit your messages to have blocked content which breaks the server's setup regex rules, this has been reported to the server staff, continued infractions will result in greater punishment.\n\n\
                                        The message which has been blocked is below:\n\
                                        ||{}||", updated_message);

            author
                .dm(&ctx.http, |m| m.content(dm_msg))
                .await
                .log_expect(LogImportance::Warning, "Unable to dm user");

            return;
        }
    }
}
