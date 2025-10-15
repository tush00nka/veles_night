use serde::Deserialize;
use std::fs;

const METADATA_PATH: &str = "static/metadata/";
const METADATA_EXTENSION: &str = ".json";

#[derive(Deserialize, Clone)]
pub struct SpiritMetadata {
    pub position: [u8; 2],
    pub amount: u8,
    pub direction: [u8; 2],
}

#[derive(Deserialize, Clone)]
pub struct SwampsMetadata {
    pub swamp: [u8; 2],
    pub teleport: [u8; 2],
}

#[derive(Deserialize, Clone)]
pub struct MetadataHandler {
    pub survive: usize,
    pub spirits: Vec<SpiritMetadata>,
    pub swamps: Vec<SwampsMetadata>,
}

impl MetadataHandler {
    pub fn new(level_number: u8) -> Self {
        let path =
            METADATA_PATH.to_string() + &level_number.to_string() + &METADATA_EXTENSION.to_string();
        println!("{path}");
        let Ok(string_json) = fs::read_to_string(path) else {
            panic!("COULDN'T LOAD JSON FOR LEVEL {level_number}");
        };

        let Ok(level_metadata) = serde_json::from_str(&string_json) else {
            panic!("COULDN'T LOAD METADATA FOR LEVEL {level_number}");
        };
        return level_metadata;
    } //todo add option to load by path

    pub fn load(&mut self, level_number: u8) {
        let path =
            METADATA_PATH.to_string() + &level_number.to_string() + &METADATA_EXTENSION.to_string();
        println!("{path}");
        let Ok(string_json) = fs::read_to_string(path) else {
            panic!("COULDN'T LOAD JSON FOR LEVEL {level_number}");
        };

        let Ok(level_metadata) = serde_json::from_str::<MetadataHandler>(&string_json) else {
            panic!("COULDN'T LOAD METADATA FOR LEVEL {level_number}");
        };

        self.spirits = level_metadata.spirits;
        self.survive = level_metadata.survive;
        self.swamps = level_metadata.swamps;
    }

    pub fn get_survive(&self) -> usize {
        self.survive
    }
}
