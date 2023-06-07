use poise::{
    serenity_prelude as serenity,
    serenity_prelude::{
        CreateEmbed,
        ChannelId, ReactionType, UserId
    }
};

use crate::utils::{toml::*, logger::*};

pub async fn reaction_add_event(ctx: &serenity::Context, add_reaction: &serenity::Reaction) {
    //ignore reactions from the bot
    if add_reaction.user_id.unwrap() == ctx.cache.current_user_id() {
        return;
    }

    //only look at reactions in the log channel
    if add_reaction.channel_id != ChannelId(get_config().log_channel) {
        return;
    }

    //ignore events except for staff
    if !get_config().staff.contains(&add_reaction.user_id.unwrap().to_string()) {
        return;
    }

    //ignore events except for the 🚫 reaction
    if add_reaction.emoji != ReactionType::Unicode("🚫".to_string()) {
        return;
    }

    let ctx_clone = ctx.clone();
    let reaction_clone = add_reaction.clone();
    tokio::spawn(async move {
        let mut msg = reaction_clone.channel_id.message(&ctx_clone.http, reaction_clone.message_id).await.unwrap();
        let user_id = &msg.embeds[0].fields[0].value[2..msg.embeds[0].fields[0].value.len() - 1];
    
        let data = LogData {
            importance: LogImportance::Info,
            message: format!("{} Has dismissed a report", reaction_clone.user_id.unwrap()),
        };
        log_this(data);
    
        dismiss_infraction(user_id.parse::<u64>().unwrap());
    
        let user = UserId(user_id.parse::<u64>().unwrap()).to_user(&ctx_clone.http).await.unwrap();
        let blocked_content = &msg.embeds[0].fields[1].value[2..msg.embeds[0].fields[1].value.len() - 2];
        let dm_msg = format!("Your report has been dismissed by a staff member due to it being found as being a false positive.\n\n\
                                    The message that was blocked is below:\n\
                                    ||{}||", blocked_content);
        user.dm(&ctx_clone.http, |m| m.content(dm_msg)).await.log_expect(LogImportance::Warning, "Unable to dm user");
    
        let mut embed = CreateEmbed::default();
        embed.color(0x556B2F);
        embed.title("Message was blocked due to matching a set regex pattern");
        embed.field("The user who broke a regx pattern is below:", format!("<@{}>", user_id), false);
        embed.field("Their message was the following below:", format!("||{}||", &msg.embeds[0].fields[1].value[2..msg.embeds[0].fields[1].value.len() - 2]), false);
        embed.thumbnail("https://raw.githubusercontent.com/MrEnder0/Regy-Bot/master/.github/assets/secure.png");
        embed.footer(|f| f.text("This infraction has been dismissed by a staff member"));
        msg.edit(&ctx_clone.http, |m| m.set_embed(embed)).await.ok();
    
        msg.delete_reaction_emoji(&ctx_clone.http, ReactionType::Unicode("🚫".to_string())).await.ok();
    
        //Delete the embed
        /*if let Err(why) = msg.delete(&ctx_clone.http).await {
        //    println!("Error deleting message: {:?}", why);
        }*/
    });
}