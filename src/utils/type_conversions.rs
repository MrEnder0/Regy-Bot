use poise::serenity_prelude::UserId;

pub fn userid_to_u64(userid: UserId) -> u64 {
    userid
        .to_string()
        .replace("<@!", "")
        .replace('>', "")
        .parse::<u64>()
        .unwrap()
}
