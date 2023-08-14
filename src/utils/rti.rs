use std::{collections::BTreeMap, sync::Mutex};

static RTI: Mutex<BTreeMap<String, String>> = Mutex::new(BTreeMap::new());

pub struct RtiStruct {}

impl RtiStruct {
    pub fn load() {
        let binding = &RTI;
        let mut guard = binding.lock().unwrap();

        //TODO: Fetch rti (regex template index) file from github and parse it into the BTreeMap
    }

    pub fn reload() {
        RTI.lock().unwrap().clear();
        RtiStruct::load();
    }
}
