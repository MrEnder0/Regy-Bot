mod clean_logs;
mod echo;
mod get_ipm;
mod reset_ipm;
mod shutdown;
mod upload_logs;
mod build_info;

#[cfg(target_os = "windows")]
pub mod local_update;

type Error = Box<dyn std::error::Error + Send + Sync>;

// Link all dev commands into vec
pub fn dev_commands() -> Vec<poise::Command<super::super::Data, Error>> {
    vec![
        clean_logs::clean_logs(),
        echo::echo(),
        get_ipm::get_ipm(),
        reset_ipm::reset_ipm(),
        shutdown::shutdown(),
        upload_logs::upload_logs(),
        build_info::build_info(),
        #[cfg(target_os = "windows")]
        local_update::update(),
    ]
}
