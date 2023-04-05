mod managers;
mod commands;

use serenity::{
    async_trait,
    framework::standard::{macros::group, StandardFramework},
    model::{channel::Message, gateway::Ready, prelude::{ChannelId, UserId},
    channel::{Reaction, ReactionType}},
    builder::CreateEmbed,
    prelude::*
};
use std::path::Path;
use regex::Regex;

use crate::managers::toml::*;
use crate::managers::logger::*;
use crate::commands::dev::*;
use crate::commands::staff::*;
use crate::commands::user::*;

struct Handler;

#[group]
#[commands(dev, staff, user)]
struct General;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let content = msg.content.chars().rev().collect::<String>();
        if !content.is_empty() {
            //Ignores messages from bots
            if msg.author.bot {
                return;
            }

            //Reply to dm messages
            if msg.guild_id.is_none() {
                msg.reply(ctx, "I wish I could dm you but because to my new fav Discord Developer Compliance worker named Gatito I cant. :upside_down: Lots of to you :heart:").await.expect("Unable to reply to dm");
                return;
            }
            
            //Reply to pings
            if msg.mentions_user_id(ctx.cache.current_user_id().await) {
                let ctx = ctx.clone();
                msg.reply(ctx, "To use Regy use the prefix `<|`").await.expect("Unable to reply to ping");
            }

            //Ignores moderation from devs
            if msg.author.id == 687897073047306270 || msg.author.id == 598280691066732564 {
                return;
            }

            //Ignores moderation from staff
            for staff in get_config().staff {
                if msg.author.id == UserId(staff.parse::<u64>().unwrap()) {
                    return;
                }
            }

            let list_block_phrases = list_block_phrases();

            for (_id, phrase) in list_block_phrases {
                let re = Regex::new(&phrase).unwrap();
                if re.is_match(&msg.content) {
                    if let Err(why) = msg.delete(&ctx.http).await {
                        println!("Error deleting message: {:?}", why);
                    }
                    let reply_msg = msg.channel_id.say(&ctx.http, format!("<@{}> You are not allowed to send that due to the server setup regex rules", msg.author.id)).await.unwrap().id;
                    msg.author.dm(&ctx.http, |m| m.content("You are not allowed to send that due to the server setup regex rules, this has been reported to the server staff, continued offenses will result in greater punishment.")).await.expect("Unable to dm user");
                    let log_channel = ChannelId(get_config().log_channel);

                    let mut embed = CreateEmbed::default();
                    embed.color(0xFFA500);
                    embed.title("Message blocked due to matching a set regex pattern");
                    embed.field("The user who broke a regx pattern is below:", format!("<@{}>", msg.author.id), false);
                    embed.field("Their message is the following below:", format!("||{}||", msg.content), false);
                    embed.footer(|f| f.text("React with 🚫 to dismiss this offense"));
                    let embed_message_id = log_channel.send_message(&ctx.http, |m| m.set_embed(embed)).await.expect("Unable to send embed").id;
                    let embed_message = log_channel.message(&ctx.http, embed_message_id).await.ok();
                    embed_message.unwrap().react(&ctx.http, ReactionType::Unicode("🚫".to_string())).await.ok();

                    //log_channel.say(&ctx.http, format!("<@{}> sent a message that matched a regex pattern, their message is the following below:\n||```{}```||", msg.author.id, msg.content.replace('`', "\\`"))).await.unwrap();

                    let data = LogData {
                        importance: "INFO".to_string(),
                        message: format!("{} has sent a message which is not allowed due to the set regex patterns", msg.author.id),
                    };

                    log_this(data);

                    println!("{} sent a message that matched a blocked regex pattern, their message is the following below:\n{}\n\nThere message broke the following pattern:\n{}", msg.author.id, msg.content, phrase);
                    add_offense(msg.author.id.into());

                    let ctx_clone = ctx.clone();
                    tokio::spawn(async move {
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                        if let Err(why) = msg.channel_id.delete_message(&ctx_clone.http, reply_msg).await {
                            eprintln!("Error deleting message: {:?}", why);
                        }
                    });
                    return;
                }
            }            
        }
    }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        //Only looks in the log channel
        if reaction.channel_id != ChannelId(get_config().log_channel) {
            return;
        }

        //Only allow staff to use reactions
        if !get_config().staff.contains(&reaction.user_id.unwrap().to_string()) {
            return;
        }

        //Ignores reactions from the bot
        if reaction.user_id.unwrap() == ctx.cache.current_user_id().await {
            return;
        }

        if reaction.user_id.unwrap() == ctx.cache.current_user_id().await {
            return;
        }
        if reaction.emoji == ReactionType::Unicode("🚫".to_string()) {
            let ctx_clone = ctx.clone();
            let reaction_clone = reaction.clone();
            tokio::spawn(async move {
                let mut msg = reaction_clone.channel_id.message(&ctx_clone.http, reaction_clone.message_id).await.unwrap();
                let user_id = &msg.embeds[0].fields[0].value[2..msg.embeds[0].fields[0].value.len() - 1];

                let data = LogData {
                    importance: "INFO".to_string(),
                    message: format!("{} Has dismissed a report", reaction_clone.user_id.unwrap()),
                };
                log_this(data);

                dismiss_offense(user_id.parse::<u64>().unwrap());

                let user = UserId(user_id.parse::<u64>().unwrap()).to_user(&ctx_clone.http).await.unwrap();
                user.dm(&ctx_clone.http, |m| m.content("Your report has been dismissed by a staff member due to it being found as being a false positive.")).await.expect("Unable to dm user");

                let mut embed = CreateEmbed::default();
                embed.color(0x00FF00);
                embed.title("Message blocked due to matching a set regex pattern");
                embed.field("The user who broke a regx pattern is below:", format!("<@{}>", user_id), false);
                embed.field("Their message is the following below:", format!("||{}||", &msg.embeds[0].fields[1].value[2..msg.embeds[0].fields[1].value.len() - 2]), false);
                embed.footer(|f| f.text("This offense has been dismissed by a staff member"));
                msg.edit(&ctx_clone.http, |m| m.set_embed(embed)).await.ok();

                msg.delete_reaction_emoji(&ctx_clone.http, ReactionType::Unicode("🚫".to_string())).await.ok();

                //Delete the embed
                /*if let Err(why) = msg.delete(&ctx_clone.http).await {
                //    println!("Error deleting message: {:?}", why);
                }*/
            });
        }
    }
}

#[tokio::main]
async fn main() {
    //check for config file
    if !Path::new("config.toml").exists() {
        gen_config();
    }

    //load token from config file
    let token = get_config().token;

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("<|"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    let data = LogData {
        importance: "INFO".to_string(),
        message: "Starting Regy".to_string(),
    };
    log_this(data);

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
        let data = LogData {
            importance: "ERROR".to_string(),
            message: format!("Client error: {:?}", why),
        };
        log_this(data);

    }
}
