use poise::{
    serenity_prelude as serenity,
    serenity_prelude::Member,
};
use regex::Regex;
use scorched::{LogExpect, LogImportance};

use crate::utils::{config::*, word_prep::filter_characters};

pub async fn guild_member_join_event(ctx: &serenity::Context, guild_member: &Member) {
    // Checks if server exists in config
    if !read_config()
        .await
        .servers
        .contains_key(&guild_member.guild_id.to_string())
    {
        return
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
        println!("filtered username or nickname");

        // Dm user they need to change their username or nickname
        guild_member
            .user
            .dm(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("Username contains blocked phrase");
                    e.description("Your username or nickname contains a banned word, please change it. You will be given 2 minutes to change it, if you do not change it you will be kicked from the server, you will be able to rejoin if you do get kicked when you fix your username or nickname. If you do not know what you did wrong, please contact a staff member. Your username or nickname")
                })
            })
            .await
            .log_expect(LogImportance::Error, "Failed to dm user");

        let ctx_clone = ctx.clone();
        let guild_member_clone = guild_member.clone();

        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

            // If user has not changed their username or nickname, kick them
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

                return
            }
        });
    }
}
