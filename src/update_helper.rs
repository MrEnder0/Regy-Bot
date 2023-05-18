use std::{
    process::Command,
    time::Duration,
    path::Path
};

fn main() {
    //Wait for Regy to fully shutdown
    std::thread::sleep(Duration::from_secs(3));

    let regy_bin = "regy_bot.exe";

    if !Path::new(regy_bin).exists() {
        println!("[Update Helper ERROR] Regy binary does not exist, shutting down.");
        return;
    }

    if !Path::new("updated").exists() {
        println!("[Update Helper WARNING] Regy has not been updated, shutting down.");
        return;
    }

    println!("[Update Helper INFO] Regy has finished updating restarting Regy.");
    std::fs::remove_file("updated").expect("Failed to remove updated file");
    std::thread::spawn(|| {
        Command::new("regy_bot.exe").spawn().expect("Failed to start regy_bot.exe");
    });

    println!("[Update Helper INFO] Update helper has finished, closing update helper.");
    std::process::exit(0);
}