use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{
        channel::{ChannelType, Message},
        id::UserId,
    },
    prelude::TypeMapKey,
};

#[derive(Default)]
struct BanList;

impl TypeMapKey for BanList {
    type Value = Vec<UserId>;
}

#[command]
async fn dev(ctx: &Context, msg: &Message) -> CommandResult {
    // Ignore message from non-devs
    if ![687897073047306270, 598280691066732564, 1056383394470182922].contains(&msg.author.id) {
        msg.reply(&ctx, "You are not a dev!").await?;
        return Ok(());
    }

    let mut args = msg.content.split(' ');
    args.next();
    let arg = args.next().unwrap_or("none");

    match arg {
        "none" => {
            msg.reply(&ctx, "You need to specify a dev command!").await?;
        }
        "help" => {
            msg.reply(
                &ctx,
                "The dev commands are:\n\
                        `dev help` - Shows this message\n\
                        `dev echo` - Echoes the message\n\
                        `dev shutdown` - Shuts down the bot after a 120 second countdown\n\
                        `dev hai` - Says hello back :3\n\
                        `dev am_dev` - Says if you are dev\n\
                        `dev ban` - Bans a user from the server\n\
                        `dev unban` - Unbans a user from the server",
            )
            .await?;
        }
        "echo" => {
            if let Err(why) = msg.delete(&ctx).await {
                println!("Error deleting message: {:?}", why);
            }

            let echo: String = args.collect::<Vec<&str>>().join(" ");
            msg.channel_id.say(&ctx, &echo).await?;
        }
        "shutdown" => {
            msg.reply(&ctx, "Regy will shut down in 120 seconds...").await?;

            let msg_clone = msg.clone();
            tokio::spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_secs(120)).await;
                println!("Shutdown from dev commands sent from {}", msg_clone.author.id);
                std::process::exit(0);
            });
        }
        "am_dev" => {
            msg.reply(&ctx, "Yes master!").await?;
        }
        "ban" => {
            if let Some(user_id) = args.next() {
                let user_id = match user_id.parse::<u64>() {
                    Ok(user_id) => UserId(user_id),
                    Err(_) => {
                        msg.reply(&ctx, "Invalid user ID provided!").await?;
                        return Ok(());
                    }
                };

                let guild_id = match msg.guild_id {
                    Some(guild_id) => guild_id,
                    None => {
                        msg.reply(&ctx, "Command not available in DMs!").await?;
                        return Ok(());
                    }
                };

                if let Some(user) = guild_id.member(&ctx, user_id).await.ok() {
                    let can_ban = user.permissions(&ctx).await.ok().map_or(false, |permissions| {
                        permissions.ban_members()
                    });

                    if can_ban {
                        let ban_reason = args.collect::<Vec<&str>>().join(" ");
                        guild_id.ban_with_reason(&ctx, user_id, 0, &ban_reason).await?;

                        let mut ban_list =
