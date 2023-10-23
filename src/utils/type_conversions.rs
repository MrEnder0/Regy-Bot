use poise::serenity_prelude::{RoleId, UserId};

pub async fn userid_to_u64(userid: UserId) -> u64 {
    userid
        .to_string()
        .replace("<@!", "")
        .replace('>', "")
        .parse::<u64>()
        .unwrap()
}

pub async fn roleid_to_u64(roleid: RoleId) -> u64 {
    roleid
        .to_string()
        .replace("<@&", "")
        .replace('>', "")
        .parse::<u64>()
        .unwrap()
}
