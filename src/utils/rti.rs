use reqwest::blocking::get;
use ron::{self, de::from_reader};
use scorched::{LogExpect, LogImportance, log_this, LogData};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

#[derive(Serialize, Deserialize)]
struct RtiObjectList {
    rti_objects: Vec<RtiObject>,
}

#[derive(Serialize, Deserialize)]
pub struct RtiObject {
    pub uuid: String,
    pub version: u32,
    pub phrase: String,
    pub description: String,
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

fn read_rti() -> RtiObjectList {
    let rti_packages_file =
        File::open("rti_packages.ron").log_expect(LogImportance::Error, "RTI file not found");
    let rti_packages: RtiObjectList = match from_reader(rti_packages_file) {
        Ok(x) => x,
        Err(e) => {
            log_this(LogData {
                importance: LogImportance::Warning,
                message: format!("Unable to read rti packages file:\n{}", e),
            });

            RtiObjectList {
                rti_objects: Vec::new(),
            }
        }
    };

    rti_packages
}

pub fn fuzzy_search_rti(input_phrase: String) -> Vec<RtiObject> {
    let rti_objects = read_rti();

    //find the 3 most relevant results from the description
    let matcher = SkimMatcherV2::default();

    let query = format!("{} ", input_phrase);

    let mut return_vec = Vec::new();

    for rti_object in rti_objects.rti_objects {
        if matcher.fuzzy_match(&rti_object.description, &query).is_some() {
            return_vec.push(rti_object);
        }
    }

    return_vec
}