use raylib::prelude::*;
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
pub struct LevelMetadata{
    pub percent: f32,
    pub map_size: Vec<u8>, 
    pub spirits: Vec<SpiritMetadata>,
}

#[derive(Deserialize, Clone)]
pub struct MetadataHandler{
    pub level_metadata: LevelMetadata,
}

impl MetadataHandler{
    pub fn new(level_number: u8) -> Self{
        let path = METADATA_PATH.to_string() 
            + &level_number.to_string() 
            + &METADATA_EXTENSION.to_string();
        println!("{path}");
        let Ok(string_json) = fs::read_to_string(path) else{
            panic!("COULDN'T LOAD JSON FOR LEVEL {level_number}");
        };
        
        let level_metadata: LevelMetadata = serde_json::from_str(&string_json).unwrap();    
        return Self{level_metadata};
    }
}
