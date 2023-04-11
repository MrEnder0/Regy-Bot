use serenity::{
    framework::standard::CommandResult,
    model::channel::Message,
    prelude::*,
};

pub async fn temp_msg(ctx: &Context, msg: &Message, content: &str) -> CommandResult {
    let temp_msg = msg.channel_id.say(&ctx.http, content).await?;
    let ctx_clone = ctx.clone();
    let msg_clone = msg.clone();
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        if let Err(why) = msg_clone.channel_id.delete_message(&ctx_clone.http, temp_msg).await {
            eprintln!("Error deleting message: {:?}", why);
        }
    });
    Ok(())
}