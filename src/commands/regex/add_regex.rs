use scorched::*;

use crate::{
    utils::{
        config,
        perm_check::{has_perm, PermissionLevel::Staff},
    },
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command, user_cooldown = 10)]
pub async fn add_regex(
    ctx: Context<'_>,
    #[description = "Regex Phrase"]
    #[min_length = 3]
    #[max_length = 350]
    regex_phrase: String,
) -> Result<(), Error> {
    let server_id = ctx.guild_id().unwrap().to_string();

    if !has_perm(
        server_id.clone(),
        ctx.author().id.to_string().parse::<u64>().unwrap(),
        Staff,
    )
    .await
    {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("You do not have permission to use this command.")
                    .field("Lacking permissions:", "Staff", false)
                    .color(0x8B0000)
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

        return Ok(());
    }

    if regex_phrase == ".*"
        || regex_phrase == ".*+"
        || regex_phrase == "[a-zA-Z0-9]"
        || regex_phrase == "[a-zA-Z0-9]+"
        || regex_phrase == "^.*$"
    {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("Invalid regex phrase")
                    .description("Looks like you tried to add a regex phrase that would match everything. For your server's convenience this wont be added.")
                    .color(0x8B0000)
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

        return Ok(());
    }

    config::add_regex(
        server_id,
        format!("{} ", regex_phrase.clone()),
        false,
        "No description provided.".to_string(),
        0,
    )
    .await;

    ctx.send(|cr| {
        cr.embed(|ce| {
            ce.title("Regex phrase added").description(format!(
                "Added the regex phrase:\n||```{}```||",
                regex_phrase
            ))
        })
    })
    .await
    .log_expect(LogImportance::Warning, "Unable to send message");

    Ok(())
}
