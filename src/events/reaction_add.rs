use poise::{
    serenity_prelude as serenity,
    serenity_prelude::{ChannelId, CreateEmbed, ReactionType, UserId},
};
use scorched::*;

use crate::utils::config::*;

enum EmbedType {
    Add,
    Update,
}

pub async fn reaction_add_event(ctx: &serenity::Context, add_reaction: &serenity::Reaction) {
    let server_id = add_reaction.guild_id.unwrap().to_string();
    let user_id = add_reaction.user_id.unwrap().to_string();

    //Check if server exists in config
    if !read_config().await.servers.contains_key(&server_id) {
        return;
    }

    //ignore reactions from the bot
    if add_reaction.user_id.unwrap() == ctx.cache.current_user_id() {
        return;
    }

    //ignore events except for staff
    if !read_config()
        .await
        .servers
        .get(&server_id)
        .unwrap()
        .staff
        .contains(&user_id.parse::<u64>().unwrap())
    {
        return;
    }

    //Check if the reaction is a dismiss reaction
    if add_reaction.channel_id
        == ChannelId(
            read_config()
                .await
                .servers
                .get(&server_id)
                .unwrap()
                .log_channel,
        )
    {
        //ignore events except for the 🚫 reaction
        if add_reaction.emoji != ReactionType::Unicode("🚫".to_string()) {
            return;
        }

        let ctx_clone = ctx.clone();
        let reaction_clone = add_reaction.clone();
        tokio::spawn(async move {
            let mut msg = reaction_clone
                .channel_id
                .message(&ctx_clone.http, reaction_clone.message_id)
                .await
                .unwrap();
            let user_id =
                &msg.embeds[0].fields[0].value[2..msg.embeds[0].fields[0].value.len() - 1];

            log_this(LogData {
                importance: LogImportance::Info,
                message: format!("{} Has dismissed a report", reaction_clone.user_id.unwrap()),
            })
            .await;

            dismiss_infraction(server_id, user_id.parse::<u64>().unwrap()).await;

            let user = UserId(user_id.parse::<u64>().unwrap())
                .to_user(&ctx_clone.http)
                .await
                .unwrap();
            let blocked_content =
                &msg.embeds[0].fields[1].value[2..msg.embeds[0].fields[1].value.len() - 2];
            let dm_msg = format!("Your report has been dismissed by a staff member due to it being found as being a false positive.\n\n\
                                    The message that was blocked is below:\n\
                                    ||{}||", blocked_content);

            user.dm(&ctx_clone.http, |m| m.content(dm_msg))
                .await
                .log_expect(LogImportance::Warning, "Unable to dm user");

            let mut embed = CreateEmbed::default();
            embed.color(0x556B2F);
            embed.title("Message was blocked due to matching a set regex pattern");
            embed.field(
                "The user who broke a regx pattern is below:",
                format!("<@{}>", user_id),
                false,
            );
            embed.field(
                "Their message was the following below:",
                format!(
                    "||{}||",
                    &msg.embeds[0].fields[1].value[2..msg.embeds[0].fields[1].value.len() - 2]
                ),
                false,
            );
            embed.thumbnail(
            "https://raw.githubusercontent.com/MrEnder0/Regy-Bot/master/.github/assets/secure.png",
        );
            embed.footer(|f| f.text("This infraction has been dismissed by a staff member"));
            msg.edit(&ctx_clone.http, |m| m.set_embed(embed)).await.ok();

            msg.delete_reaction_emoji(&ctx_clone.http, ReactionType::Unicode("🚫".to_string()))
                .await
                .ok();

            //Delete the embed
            /*if let Err(why) = msg.delete(&ctx_clone.http).await {
            //    println!("Error deleting message: {:?}", why);
            }*/
        });
    } else {
        let ctx_clone = ctx.clone();
        let reaction_clone = add_reaction.clone();

        let mut msg = reaction_clone
            .channel_id
            .message(&ctx_clone.http, reaction_clone.message_id)
            .await
            .unwrap();

        let embed_type = match msg.embeds[0].title.as_mut().unwrap().to_string().as_str() {
            "Are you sure you want to update the RTI package?" => EmbedType::Update,
            "Results found" => EmbedType::Add,
            _ => return,
        };

        match embed_type {
            EmbedType::Add => {
                if add_reaction.emoji != ReactionType::Unicode("✅".to_string()) {
                    return;
                }

                let phrase_ver = &msg.embeds[0].fields[0].value;
                let phrase_desc = &msg.embeds[0].fields[2].value;
                let phrase_phrase = &msg.embeds[0].fields[3].value;

                println!("{}", server_id);

                add_regex(
                    server_id,
                    format!("{} ", phrase_phrase),
                    true,
                    phrase_desc.to_string(),
                    phrase_ver.parse().unwrap(),
                )
                .await;

                log_this(LogData {
                    importance: LogImportance::Info,
                    message: format!(
                        "{} Has added a RTI package to their server",
                        reaction_clone.user_id.unwrap()
                    ),
                })
                .await;

                //edit embed
                let mut embed = CreateEmbed::default();
                embed.color(0x556B2F);
                embed.title("RTI package added to server");
                embed.field("Version", phrase_ver, false);
                embed.field("Description", phrase_desc, false);
                embed.field("Phrase", phrase_phrase, false);
                embed.footer(|f| f.text("This RTI package has been added to your server"));
                embed.thumbnail(
                    "https://raw.githubusercontent.com/MrEnder0/Regy-Bot/master/.github/assets/secure.png",
                );

                msg.edit(&ctx_clone.http, |m| m.set_embed(embed)).await.ok();
            }
            EmbedType::Update => {
                if add_reaction.emoji != ReactionType::Unicode("✅".to_string()) {
                    update_regexes(server_id).await;

                    let mut embed = CreateEmbed::default();
                    embed.color(0x556B2F);
                    embed.title("RTI package updated");
                    embed.description("All RTI packages have been updated to the latest version");
                    embed.footer(|f| {
                        f.text("This RTI package has been updated to the latest version")
                    });
                    embed.thumbnail(
                        "https://raw.githubusercontent.com/MrEnder0/Regy-Bot/master/.github/assets/secure.png",
                    );

                    msg.edit(&ctx_clone.http, |m| m.set_embed(embed)).await.ok();
                } else if add_reaction.emoji != ReactionType::Unicode("❌".to_string()) {
                    let mut embed = CreateEmbed::default();
                    embed.color(0xFFA500);
                    embed.title("RTI package update cancelled");
                    embed.description("The RTI package update has been cancelled");
                    embed.footer(|f| f.text("This RTI package update has been cancelled"));
                    embed.thumbnail(
                        "https://raw.githubusercontent.com/MrEnder0/Regy-Bot/master/.github/assets/secure.png",
                    );

                    msg.edit(&ctx_clone.http, |m| m.set_embed(embed)).await.ok();
                }
            }
        }
    }
}
