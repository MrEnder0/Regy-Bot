use poise::serenity_prelude::UserId;

pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn userid_to_u64(userid: UserId) -> u64 {
    let userid = userid.to_string();
    let userid = userid.replace("<@!", "");
    let userid = userid.replace('>', "");
    let userid = userid.parse::<u64>().unwrap();
    userid
}