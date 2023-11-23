use ::regex::Regex;
use poise::{serenity_prelude as serenity, serenity_prelude::{Member, ChannelId}};
use scorched::{LogExpect, LogImportance};

use crate::utils::{
    config::{read_config, regex::list_regex},
    word_prep::filter_characters,
};

pub async fn guild_member_join_event(ctx: &serenity::Context, guild_member: &Member) {
    // Checks if server exists in config
    if !read_config()
        .await
        .servers
        .contains_key(&guild_member.guild_id.to_string())
    {
        return;
    }

    let filtered_username = filter_characters(&guild_member.user.name.to_lowercase());

    // checks if username or nickname contains a banned word
    if list_regex(guild_member.guild_id.to_string())
        .await
        .unwrap()
        .iter()
        .any(|regex| {
            Regex::new(&regex.phrase)
                .unwrap()
                .is_match(&format!("{} #", filtered_username))
        })
    {
        // Dm user they need to change their username or nickname
        guild_member
            .user
            .dm(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("Username contains blocked phrase");
                    e.description("Your username or nickname contains a banned word, please change it. You will be given 1 and a half minutes to change it, if you do not change it you will be kicked from the server, you will be able to rejoin if you do get kicked when you fix your username or nickname. If you do not know what you did wrong, please contact a staff member. Your username or nickname")
                })
            })
            .await
            .log_expect(LogImportance::Error, "Failed to dm user");

        let ctx_clone = ctx.clone();
        let guild_member_clone = guild_member.clone();

        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(90)).await;

            // Kicks user if they have not changed their username or nickname
            if list_regex(guild_member_clone.guild_id.to_string())
                .await
                .unwrap()
                .iter()
                .any(|regex| {
                    Regex::new(&regex.phrase)
                        .unwrap()
                        .is_match(&format!("{} #", filtered_username))
                })
            {
                guild_member_clone
                    .kick(&ctx_clone.http.clone())
                    .await
                    .log_expect(LogImportance::Error, "Failed to kick user");

                // Send message to log channel
                let log_channel = ChannelId(
                    read_config()
                        .await
                        .servers
                        .get(&guild_member_clone.guild_id.to_string())
                        .unwrap()
                        .log_channel,
                );
                
                log_channel
                    .send_message(&ctx_clone.http, |m| {
                        m.embed(|e| {
                            e.title("User kicked for an offensive profile");
                            e.description(format!(
                                "{} was kicked for not changing their username or nickname, they are able to rejoin if they change their username or nickname.",
                                guild_member_clone.user.name
                            ))
                        })
                    })
                    .await
                    .log_expect(LogImportance::Error, "Failed to send message to log channel");
            }
        });
    }
}
