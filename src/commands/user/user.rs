use crate::{
    utils::{logger::LogExpect, toml, type_conversions},
    Data,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn user(
    ctx: Context<'_>,
    #[description = "Commands for standard users; run help for more info"] command_arg: Option<
        String,
    >,
) -> Result<(), Error> {
    let arg = type_conversions::string_to_static_str(
        command_arg.log_expect("did not specify command arg"),
    );
    let args = arg.split_whitespace().collect::<Vec<&str>>();
    match args[0] {
        "none" => {
            ctx.say("You need to specify a command, I expect higher of you, you should know how to use this bot correctly").await?;
            Ok(())
        }
        "info" => {
            ctx.say("Regy is a Discord regex auto-moderation bot developed mainly by Mr.Ender#0001 with a few contributions by 3kh0#6969 and 1984#0001, pfp by 1984 <3.").await.log_expect("Unable to send message");
            Ok(())
        }
        "what_is_regex" => {
            ctx.say("Regex, short for Regular Expression, is a sequence of characters that defines a search pattern. It is used to search, replace, and manipulate text in programming and text editing tools. It provides a powerful and flexible way to match and manipulate strings of text based on certain patterns or rules.").await.log_expect("Unable to send message");
            Ok(())
        }
        "skid" => {
            ctx.say("The term 'skid' <@&1087534862937890896> can have different meanings depending on the context in which it is used. Here are a few possible definitions:\n\
                    A skid <@&1087534862937890896> is a flat, wooden or metal platform that is used for transporting heavy loads. The platform is placed on top of a set of runners or wheels, and the load is placed on the platform. The platform is then pulled or pushed along the ground.\n\
                    In driving, a skid <@&1087534862937890896> occurs when a vehicle's tires lose traction with the road surface, causing the vehicle to slide or spin out of control. Skids can occur for various reasons, such as wet or icy roads, sharp turns taken at high speeds, or sudden braking.\n\
                    In the oil and gas industry, a skid <@&1087534862937890896> refers to a modular system that contains equipment for processing or controlling fluids, such as oil or gas. The skid is designed to be easily transported and installed, and can be connected to other skids to form a larger processing or control system.\n\
                    In construction, a skid <@&1087534862937890896> steer is a type of compact, maneuverable loader that is used for digging, pushing, and carrying materials. The loader is mounted on four wheels or tracks, and can be operated by a single person.\n\
                    These are just a few examples of the different meanings of the term 'skid.' <@&1087534862937890896> The exact meaning of the term will depend on the context in which it is used."
                ).await.log_expect("Unable to send message");
            Ok(())
        }
        "my_infractions" => {
            let user_id = type_conversions::userid_to_u64(ctx.author().id);
            let user_infractions = toml::list_infractions(user_id);
            let infractions_message = format!("You have {} infractions.", user_infractions);
            ctx.say(infractions_message)
                .await
                .log_expect("Unable to send message");
            Ok(())
        }
        _ => {
            let invalid_arg_message = format!(
                "Invalid argument '{}' but its ok I still care abt u :heart:",
                arg.replace('@', "\\@")
            );
            ctx.say(invalid_arg_message)
                .await
                .log_expect("Unable to send message");
            Ok(())
        }
    }
}
