use std::{
    process::Command,
    time::Duration
};

fn main() {
    println!("Regy has finished updating... Restarting now.");
    std::thread::sleep(Duration::from_secs(3));
    Command::new("regy_bot.exe").spawn().expect("Failed to start regy_bot.exe");
    std::process::exit(0);
}