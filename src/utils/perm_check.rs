use poise::serenity_prelude::RoleId;

use super::config;

#[derive(PartialEq)]
pub enum PermissionLevel {
    User,
    Staff,
    Developer,
}

const DEVELOPERS: [&str; 1] = ["687897073047306270"];

pub async fn has_perm(
    server_id: String,
    userid: u64,
    roles: Vec<RoleId>,
    permission_level: PermissionLevel,
) -> bool {
    match permission_level {
        PermissionLevel::User => true,
        PermissionLevel::Staff => {
            #[cfg(feature = "legacy-staff")]
            if config::read_config()
                .await
                .servers
                .get(&server_id)
                .unwrap()
                .staff
                .contains(&userid)
            {
                return true;
            }

            let staff_roles = match config::staff::list_staff_roles(server_id.clone()).await {
                Some(staff_roles) => staff_roles,
                None => return false,
            };

            for role in roles {
                if staff_roles.clone().contains(role.as_u64()) {
                    return true;
                }
            }

            false
        }
        PermissionLevel::Developer => DEVELOPERS.contains(&&userid.to_string()[..]),
    }
}

pub async fn highest_unlocked_perm(
    server_id: String,
    userid: u64,
    roles: Vec<RoleId>,
) -> PermissionLevel {
    if DEVELOPERS.contains(&&userid.to_string()[..]) {
        return PermissionLevel::Developer;
    }

    #[cfg(feature = "legacy-staff")]
    if config::read_config()
        .await
        .servers
        .get(&server_id)
        .unwrap()
        .staff
        .contains(&userid)
    {
        return PermissionLevel::Staff;
    }

    let staff_roles = match config::staff::list_staff_roles(server_id.clone()).await {
        Some(staff_roles) => staff_roles,
        None => return PermissionLevel::User,
    };

    for role in roles {
        if staff_roles.clone().contains(role.as_u64()) {
            return PermissionLevel::Staff;
        }
    }

    PermissionLevel::User
}
