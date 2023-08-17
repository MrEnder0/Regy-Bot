use reqwest::blocking::get;
use ron::{self, de::from_reader};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;

#[derive(Serialize, Deserialize)]

pub struct RtiObject {
    uuid: String,
    version: u32,
    phrase: String,
    description: String,
}

pub async fn download_rti() {
    let url = "https://raw.githubusercontent.com/MrEnder0/Regy-Bot/rti_packages/rti_packages.ron";
    let mut response = get(url).unwrap();

    let mut file = File::create("rti_packages.ron").unwrap();
    response.copy_to(&mut file).unwrap();
}

pub fn load_rti() -> Result<Vec<RtiObject>, Box<dyn Error>> {
    let mut return_vec = Vec::new();
    let rti: Vec<RtiObject> = from_reader(File::open("rti_packages.ron")?)?;

    for rti_object in rti {
        return_vec.push(rti_object);
    }

    Ok(return_vec)
}
