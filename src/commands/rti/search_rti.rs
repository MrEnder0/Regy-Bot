use poise::serenity_prelude::CreateEmbed;
use scorched::*;

use crate::{
    utils::{
        perm_check::{has_perm, PermissionLevel::Staff},
        rti::fuzzy_search_rti,
    },
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command, user_cooldown = 5)]
pub async fn search_rti(
    ctx: Context<'_>,
    #[description = "Search Phrase"] search_phrase: String,
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
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");

        return Ok(());
    }

    if search_phrase.is_empty() || search_phrase == " " || search_phrase.len() < 2 {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("You need to specify a search phrase to search").description(
                    "The search query cant be empty and it also must be at least 2 characters long.",
                )
            })
        }).await.log_expect(LogImportance::Warning, "Unable to send message");
        return Ok(());
    }

    let relevant_objects = fuzzy_search_rti(search_phrase.clone()).await;

    if relevant_objects.is_none() {
        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("No results found")
                    .field(
                        "The search phrase you entered did not match any results",
                        format!("Search phrase: {}", search_phrase),
                        false,
                    )
                    .footer(|fe| {
                        fe.text(
                            "If there is a package you want to add create a PR on our github repo",
                        )
                    })
            })
        })
        .await?;

        return Ok(());
    } else {
        for rti_object in relevant_objects.clone().unwrap() {
            let mut embed = CreateEmbed::default();
            embed.title("RTI Package Found".to_string());
            embed.color(0x556B2F);
            embed.field("Version", rti_object.version, false);
            embed.field("UUID", rti_object.uuid, false);
            embed.field("Description", rti_object.description, false);
            embed.field("Phrase", rti_object.phrase, false);
            embed.footer(|fe| fe.text("React with ✅ to add this package to your server"));

            let channel_id = ctx.channel_id();
            let embed_msg = channel_id
                .send_message(&ctx, |m| m.set_embed(embed))
                .await?;

            embed_msg.react(&ctx, '✅').await?;
        }

        ctx.send(|cr| {
            cr.embed(|ce| {
                ce.title("Results found".to_string())
                    .color(0x556B2F)
                    .description(format!(
                        "The search phrase you entered matched {} results",
                        relevant_objects.unwrap().len()
                    ))
            })
        })
        .await
        .log_expect(LogImportance::Warning, "Unable to send message");
    }

    Ok(())
}
