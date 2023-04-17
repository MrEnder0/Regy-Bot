mod commands;
mod utils;

use poise::{
    Event,
    serenity_prelude as serenity,
    serenity_prelude::{
        CreateEmbed,
        ReactionType,
        UserId,
        ChannelId
    }
};
use std::path::Path;
use regex::Regex;

use crate::utils::toml::*;
use crate::utils::logger::*;
use crate::commands::dev::*;
use crate::commands::staff::*;
use crate::commands::user::*;

pub struct Data {}

#[tokio::main]
async fn main() {
    //check for config file
    if !Path::new("config.toml").exists() {
        gen_config();
    }

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![user(), staff(), dev()],
            event_handler: |ctx, event, _framework, _data| {
                Box::pin(async move {
                    match event {
                        Event::Ready { data_about_bot } => {
                            println!("{} is connected!", data_about_bot.user.name);
                        }
                        
                        Event::ReactionAdd { add_reaction, .. } => {
                            //ignore reactions from the bot
                            if add_reaction.user_id.unwrap() == ctx.cache.current_user_id() {
                                return Ok(());
                            }

                            //only look at reactions in the log channel
                            if add_reaction.channel_id != ChannelId(get_config().log_channel) {
                                return Ok(());
                            }

                            //ignore events except for staff
                            if !get_config().staff.contains(&add_reaction.user_id.unwrap().to_string()) {
                                return Ok(());
                            }

                            //ignore events except for the 🚫 reaction
                            if add_reaction.emoji != ReactionType::Unicode("🚫".to_string()) {
                                return Ok(());
                            }

                            let ctx_clone = ctx.clone();
                            let reaction_clone = add_reaction.clone();
                            tokio::spawn(async move {
                                let mut msg = reaction_clone.channel_id.message(&ctx_clone.http, reaction_clone.message_id).await.unwrap();
                                let user_id = &msg.embeds[0].fields[0].value[2..msg.embeds[0].fields[0].value.len() - 1];
                            
                                let data = LogData {
                                    importance: "INFO".to_string(),
                                    message: format!("{} Has dismissed a report", reaction_clone.user_id.unwrap()),
                                };
                                log_this(data);
                            
                                dismiss_infraction(user_id.parse::<u64>().unwrap());
                            
                                let user = UserId(user_id.parse::<u64>().unwrap()).to_user(&ctx_clone.http).await.unwrap();
                                user.dm(&ctx_clone.http, |m| m.content("Your report has been dismissed by a staff member due to it being found as being a false positive.")).await.expect("Unable to dm user");
                            
                                let mut embed = CreateEmbed::default();
                                embed.color(0x00FF00);
                                embed.title("Message blocked due to matching a set regex pattern");
                                embed.field("The user who broke a regx pattern is below:", format!("<@{}>", user_id), false);
                                embed.field("Their message is the following below:", format!("||{}||", &msg.embeds[0].fields[1].value[2..msg.embeds[0].fields[1].value.len() - 2]), false);
                                embed.footer(|f| f.text("This infraction has been dismissed by a staff member"));
                                msg.edit(&ctx_clone.http, |m| m.set_embed(embed)).await.ok();
                            
                                msg.delete_reaction_emoji(&ctx_clone.http, ReactionType::Unicode("🚫".to_string())).await.ok();
                            
                                //Delete the embed
                                /*if let Err(why) = msg.delete(&ctx_clone.http).await {
                                //    println!("Error deleting message: {:?}", why);
                                }*/
                            });
                        
                        }
                        Event::Message { new_message } => {
                            //ignore messages from bots
                            if new_message.author.bot {
                                return Ok(());
                            }
                            //Reply to dm messages
                            if new_message.guild_id.is_none() {
                                new_message.reply(ctx, "I wish I could dm you but because to my new fav Discord Developer Compliance worker Gatito I cant. :upside_down: Lots of to you :heart:").await.expect("Unable to reply to dm");
                                return Ok(());
                            }
                            //Reply to pings
                            if new_message.mentions_user_id(ctx.cache.current_user_id()) {
                                let ctx = ctx.clone();
                                new_message.reply(ctx, "To use Regy please use the slash commands, ex '/user help'").await.expect("Unable to reply to ping");
                            }

                            //Poll detection
                            let poll_re = Regex::new("\\b(?:let'?s|start|begin|initiate)\\s+(?:a\\s+)?(?:poll|vote|survey|opinion poll|questionnaire)\\b|\\bdo\\s+you(?:\\s+guys|\\s+all)?\\s+like\\b|\\bvote\\s+if\\s+you(?:\\s+guys|\\s+all)?\\s+like\\b").unwrap();
                            if poll_re.is_match(&new_message.content) {
                                new_message.react(&ctx.http, ReactionType::Unicode("👍".to_string())).await.ok();
                                new_message.react(&ctx.http, ReactionType::Unicode("👎".to_string())).await.ok();
                            }

                            //Ignores moderation from devs
                            if new_message.author.id == 687897073047306270 || new_message.author.id == 598280691066732564 {
                                return Ok(());
                            }
                        
                            //Ignores moderation from staff
                            for staff in get_config().staff {
                                if new_message.author.id == UserId(staff.parse::<u64>().unwrap()) {
                                    return Ok(());
                                }
                            }
                            
                            let list_block_phrases = list_block_phrases();
                            for (_id, phrase) in list_block_phrases {
                                let re = Regex::new(&phrase).unwrap();
                                if re.is_match(&new_message.content) {
                                    if let Err(why) = new_message.delete(&ctx.http).await {
                                        println!("Error deleting message: {:?}", why);
                                    }
                
                                    let temp_msg_content = format!("<@{}> You are not allowed to send that due to the server setup regex rules", new_message.author.id).to_string();
                                    let temp_msg = new_message.channel_id.say(&ctx.http, temp_msg_content).await.expect("Unable to send message");
                                    let ctx_clone = ctx.clone();
                                    tokio::spawn(async move {
                                        std::thread::sleep(std::time::Duration::from_secs(5));
                                        temp_msg.delete(&ctx_clone.http).await.ok();
                                    });
                
                                    new_message.author.dm(&ctx.http, |m| m.content("You are not allowed to send that due to the server setup regex rules, this has been reported to the server staff, continued infractions will result in greater punishment.")).await.expect("Unable to dm user");
                                    let log_channel = ChannelId(get_config().log_channel);
                
                                    let mut embed = CreateEmbed::default();
                                    embed.color(0xFFA500);
                                    embed.title("Message blocked due to matching a set regex pattern");
                                    embed.field("The user who broke a regx pattern is below:", format!("<@{}>", new_message.author.id), false);
                                    embed.field("Their message is the following below:", format!("||{}||", new_message.content), false);
                                    embed.footer(|f| f.text("React with 🚫 to dismiss this infraction"));
                                    let embed_message_id = log_channel.send_message(&ctx.http, |m| m.set_embed(embed)).await.expect("Unable to send embed").id;
                                    let embed_message = log_channel.message(&ctx.http, embed_message_id).await.ok();
                                    embed_message.unwrap().react(&ctx.http, ReactionType::Unicode("🚫".to_string())).await.ok();
                
                                    //log_channel.say(&ctx.http, format!("<@{}> sent a message that matched a regex pattern, their message is the following below:\n||```{}```||", msg.author.id, msg.content.replace('`', "\\`"))).await.unwrap();
                
                                    let data = LogData {
                                        importance: "INFO".to_string(),
                                        message: format!("{} has sent a message which is not allowed due to the set regex patterns", new_message.author.id),
                                    };
                
                                    log_this(data);
                
                                    println!("{} sent a message that matched a blocked regex pattern, their message is the following below:\n{}\n\nThere message broke the following pattern:\n{}", new_message.author.id, new_message.content, phrase);
                                    add_infraction(new_message.author.id.into());
                                    return Ok(());
                                }
                            }      
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
                ctx.set_activity(serenity::Activity::playing("Scanning peoples messages with regex")).await;
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
    });

    framework.run().await.unwrap();
}