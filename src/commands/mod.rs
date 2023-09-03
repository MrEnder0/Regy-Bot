pub mod dev;

// User
pub mod general;
pub mod info;

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
    vec![
        general::help::help(),
        general::permission_level::permission_level(),
        info::about::about(),
        info::why_rust::why_rust(),
        info::what_is_regex::what_is_regex(),
        info::skid::skid(),
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
        dev::upload_logs::upload_logs(),
        dev::clean_logs::clean_logs(),
        dev::get_ipm::get_ipm(),
        dev::reset_ipm::reset_ipm(),
        dev::echo::echo(),
        dev::shutdown::shutdown(),
        dev::local_update::update(),
        rti::search_rti::search_rti(),
        rti::update_rti::update_rti(),
        rti::reload_rti::reload_rti(),
    ]
}
