use serenity::{framework::standard::{CommandResult, macros::command}, model::channel::Message, prelude::*};

#[command]
async fn user(ctx: &Context, msg: &Message) -> CommandResult {
    let mut args = msg.content.split(' ');
    args.next();
    let arg = args.next().unwrap_or("none");
    if arg == "none" {
        msg.reply(ctx, "You need to specify a command, I expect higher of you, you should know how to use this bot correctly").await?;
        return Ok(());

    } else if arg == "help" {
        msg.reply(ctx, "The user commands are:\n`user help` - Shows this message\n`user why_rust` - Shows why rust is the best language\n`user info` - Tells you a full description of what Regy is\n`user whats_regex` - Explains what regex is\n`user am_user` - Says if you are a user...").await?;

    } else if arg == "am_user" {
        msg.reply(ctx, "Why would you not be a user you skid :skull:").await?;

    } else if arg == "why_rust" {
        msg.reply(ctx, "Rust is an excellent programming language that offers a unique combination of safety, speed, and concurrency. It is a modern language designed to provide low-level control and system-level programming, while also ensuring memory safety and preventing many common programming errors such as null pointer dereferences and buffer overflows. Rust achieves this by using a system of ownership and borrowing that guarantees at compile-time that programs are free of these errors. Additionally, Rust's concurrency model allows developers to write efficient and safe concurrent code, making it an ideal choice for building scalable and high-performance applications.\n\nAnother reason why Rust is the best language is its vibrant and growing community. Rust has a passionate and dedicated community of developers who actively contribute to the language, libraries, and tools. This community is committed to creating high-quality and reliable software that is both performant and secure. Additionally, Rust's popularity is on the rise, and many companies, including Mozilla, Dropbox, and Cloudflare, have adopted Rust for their critical systems and applications. As a result, there are numerous resources available for learning Rust, including online courses, books, and tutorials, making it easy for new developers to get started with the language. Overall, Rust's unique combination of safety, speed, and community support makes it an excellent choice for building robust and scalable software systems.").await.expect("Sadly could not say why Rust is the best programming language.");

    } else if arg == "info" {
        msg.reply(ctx, "Regy is a Discord regex auto-moderation bot developed mainly by Mr.Ender#0001 with a few contributions by 3kh0#6969 and 1984#0001, also the profile picture created by 1984.").await?;

    } else if arg == "whats_regex" {
        msg.reply(ctx, "Regex, short for Regular Expression, is a sequence of characters that defines a search pattern. It is used to search, replace, and manipulate text in programming and text editing tools. It provides a powerful and flexible way to match and manipulate strings of text based on certain patterns or rules.").await?;

    } else {
        let invalid_arg_message = format!("Invalid argument '{}' but its ok I still care abt u :heart:", arg);
        msg.reply(ctx, invalid_arg_message).await?;
    }

    Ok(())
}