pub mod dev;

// User
pub mod fun;
pub mod general;

// Staff
pub mod infractions;
pub mod moderation;
pub mod rti;

// Administrator
pub mod management;
pub mod regex;

type Error = Box<dyn std::error::Error + Send + Sync>;

// Link all commands into vec
pub fn commands() -> Vec<poise::Command<super::Data, Error>> {
    let mut commands = vec![
        general::help::help(),
        general::permission_level::permission_level(),
        fun::about::about(),
        fun::why_rust::why_rust(),
        fun::what_is_regex::what_is_regex(),
        fun::skid::skid(),
        fun::random_word::random_word(),
        infractions::my_infractions::my_infractions(),
        infractions::add_infraction::add_infraction(),
        infractions::dismiss_infraction::dismiss_infraction(),
        infractions::list_infractions::list_infractions(),
        moderation::grab_pfp::grab_pfp(),
        moderation::nuke::nuke(),
        regex::add_regex::add_regex(),
        regex::remove_regex::remove_regex(),
        regex::list_regex::list_regex(),
        management::add_staff::add_staff(),
        management::remove_staff::remove_staff(),
        management::list_staff::list_staff(),
        management::config_setup::config_setup(),
        management::config_clone::config_clone_regex(),
        rti::search_rti::search_rti(),
        rti::update_rti::update_rti(),
        rti::reload_rti::reload_rti(),
    ];

    #[cfg(feature = "developer-commands")]
    for command in dev::dev_commands() {
        commands.push(command);
    }

    commands
}
