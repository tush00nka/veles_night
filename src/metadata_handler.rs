use std::fs;
use serde::Deserialize;

const METADATA_PATH: &str = "static/metadata/";
const METADATA_EXTENSION: &str = ".json";

#[derive(Deserialize, Clone)]
pub struct SpiritMetadata{
    pub position: Vec<u8>,
    pub amount: u8,
    pub direction: Vec<u8>,
}

#[derive(Deserialize, Clone)]
pub struct MetadataHandler{
    pub percent: f32,
    pub map_width: u8, 
    pub spirits: Vec<SpiritMetadata>,
}

impl MetadataHandler{
    pub fn load(level_number: u8) -> Self{
        let path = METADATA_PATH.to_string() 
            + &level_number.to_string() 
            + &METADATA_EXTENSION.to_string();
        println!("{path}");
        let Ok(string_json) = fs::read_to_string(path) else{
            panic!("COULDN'T LOAD JSON FOR LEVEL {level_number}");
        };
        
        let Ok(level_metadata) = serde_json::from_str(&string_json) else{
            panic!("COULDN'T LOAD METADATA FOR LEVEL {level_number}");
        };
        return level_metadata;
    }
    //todo add option to load by path
}
