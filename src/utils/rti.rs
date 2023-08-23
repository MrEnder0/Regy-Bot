use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use reqwest::blocking::get;
use ron::{self, de::from_reader};
use scorched::{log_this, LogData, LogExpect, LogImportance};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;

#[derive(Serialize, Deserialize)]
pub struct MetaData {
    pub version: u32,
}

#[derive(Serialize, Deserialize)]
pub struct RtiObject {
    pub uuid: String,
    pub phrase: String,
    pub description: String,
    pub version: u32,
}

#[derive(Serialize, Deserialize)]
pub struct RtiPackages {
    pub meta: MetaData,
    pub packages: Vec<RtiObject>,
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

fn read_rti() -> RtiPackages {
    let rti_packages_file =
        File::open("rti_packages.ron").log_expect(LogImportance::Warning, "RTI file not found");
    let rti_packages: RtiPackages = match from_reader(rti_packages_file) {
        Ok(x) => x,
        Err(e) => {
            log_this(LogData {
                importance: LogImportance::Warning,
                message: format!("Unable to read rti packages file:\n{}", e),
            });

            RtiPackages {
                meta: MetaData { version: 0 },
                packages: Vec::new(),
            }
        }
    };

    rti_packages
}

pub fn fuzzy_search_rti(input_phrase: String) -> Option<Vec<RtiObject>> {
    let rti_packages = read_rti();
    let matcher = SkimMatcherV2::default();

    let mut return_vec = Vec::new();

    for rti_object in rti_packages.packages {
        if matcher
            .fuzzy_match(&rti_object.description, &input_phrase)
            .is_some()
        {
            return_vec.push(rti_object);
        }
    }

    if return_vec.len() == 0 {
        return None;
    }

    Some(return_vec)
}
