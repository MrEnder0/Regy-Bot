use base64::{engine::general_purpose, Engine};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use reqwest::blocking::get;
use ron::{self, de::from_reader};
use scorched::{log_this, LogData, LogExpect, LogImportance};
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Serialize, Deserialize)]
pub struct MetaData {
    pub version: u32,
}

#[derive(Serialize, Deserialize, Clone)]
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

    let search_phrase = input_phrase.to_lowercase();
    let mut return_vec = Vec::new();

    for rti_object in rti_packages.packages {
        if matcher
            .fuzzy_match(&rti_object.description, &search_phrase)
            .is_some()
        {
            let decoded_regex = String::from_utf8(
                general_purpose::STANDARD_NO_PAD
                    .decode(rti_object.phrase.as_bytes())
                    .log_expect(LogImportance::Warning, "Unable to decode regex phrase"),
            )
            .unwrap();

            let decoded_rti_object = RtiObject {
                uuid: rti_object.uuid,
                phrase: decoded_regex,
                description: rti_object.description,
                version: rti_object.version,
            };

            return_vec.push(decoded_rti_object);
        }
    }

    if return_vec.len() == 0 {
        return None;
    }

    Some(return_vec)
}
