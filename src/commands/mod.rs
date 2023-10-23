mod dev;

// User
mod fun;
mod general;

// Staff
mod infractions;
mod moderation;
mod rti;

// Administrator
mod management;
mod regex;

type Error = Box<dyn std::error::Error + Send + Sync>;

// Link all commands into vec
pub fn commands() -> Vec<poise::Command<super::Data, Error>> {
    let mut commands = vec![
        general::help::help(),
        general::permission_level::permission_level(),
        fun::about::about(),
        fun::why_rust::why_rust(),
        fun::what_is_regex::what_is_regex(),
        fun::what_are_dead_zones::what_are_dead_zones(),
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
        #[cfg(feature = "legacy-staff")]
        management::staff::add_staff::add_staff(),
        #[cfg(feature = "legacy-staff")]
        management::staff::remove_staff::remove_staff(),
        #[cfg(feature = "legacy-staff")]
        management::staff::list_staff::list_staff(),
        management::staff::add_staff_role::add_staff_role(),
        management::staff::remove_staff_role::remove_staff_role(),
        management::staff::list_staff_roles::list_staff_roles(),
        management::config_setup::config_setup(),
        management::config_clone::config_clone_regex(),
        management::add_dead_zone::add_dead_zone(),
        management::remove_dead_zone::remove_dead_zone(),
        management::list_dead_zones::list_dead_zones(),
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
