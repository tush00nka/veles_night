use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File, remove_file},
    io::BufWriter,
};

use crate::{map::TILE_SIZE, save_handler::SAVE_PATH, spirits_handler::SpiritsHandler};

const METADATA_PATH: &str = "static/metadata/";
const METADATA_EXTENSION: &str = ".json";

#[derive(Deserialize, Clone, Serialize)]
pub struct SpiritMetadata {
    pub position: [u8; 2],
    pub amount: u8,
    pub direction: [i8; 2],
}

#[derive(Deserialize, Clone, Serialize)]
pub struct EnemyMetadata {
    pub position: [u8; 2],
}

#[derive(Deserialize, Clone, Serialize)]
pub struct BonfireMetadata {
    pub position: [u8; 2],
    pub active: bool,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct SwampsMetadata {
    pub swamp: [u8; 2],
    pub teleport: [u8; 2],
}

#[derive(Deserialize, Clone, Serialize)]
pub struct MetadataHandler {
    pub survive: usize,
    pub spirits: Vec<SpiritMetadata>,
    pub swamps: Vec<SwampsMetadata>,
    pub enemies: Vec<EnemyMetadata>,
    pub bonfires: Vec<BonfireMetadata>,
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
        self.enemies = level_metadata.enemies;
        self.bonfires = level_metadata.bonfires;
    }

    pub fn get_survive(&self) -> usize {
        self.survive
    }
    pub fn load_save(&mut self) {
        let filenames = fs::read_dir(SAVE_PATH).unwrap();
        let mut path = "-1".to_string();
        for filename in filenames {
            let file = match filename {
                Ok(f) => f,
                Err(e) => panic!("COULDN'T LOAD SAVE METADATA - {e}"),
            };
            path = file
                .file_name()
                .into_string()
                .unwrap()
                .split('.')
                .next()
                .unwrap()
                .to_string();
            //could be error there, maybe don't need to add anything there
        }

        let path_to_save = if path == "-1" {
            panic!("ERROR DURING SAVE LOAD METADATA");
        } else {
            SAVE_PATH.to_string() + &path + &METADATA_EXTENSION.to_string()
        };

        let Ok(string_json) = fs::read_to_string(path_to_save) else {
            panic!("COULDN'T LOAD JSON FOR SAVE");
        };

        let Ok(level_metadata) = serde_json::from_str::<MetadataHandler>(&string_json) else {
            panic!("COULDN'T PARSE METADATA FOR JSON SAVE");
        };

        self.spirits = level_metadata.spirits;
        self.survive = level_metadata.survive;
        self.swamps = level_metadata.swamps;
        self.enemies = level_metadata.enemies;
        self.bonfires = level_metadata.bonfires;
    }

    pub fn save(&self, level_number: u8) {
        let Ok(filenames) = fs::read_dir(SAVE_PATH) else {
            panic!("COULDN'T EMPTY THE SAVE FOLDER TO SAVE")
        };

        for filename in filenames {
            let file = match filename {
                Ok(f) => f,
                Err(e) => panic!("COULDN'T GET FILENAME - {e}"),
            };

            // костыль, если есть расширешие файла - значит json
            if file.file_name().to_str().unwrap().split('.').count() > 1 {
                match remove_file(file.path()) {
                    Ok(_) => (),
                    Err(e) => panic!("ERROR OCCURED DURING CLEARING LAST SAVE - {e}"),
                };
            }
        }

        let path_to_save =
            SAVE_PATH.to_string() + &level_number.to_string() + &METADATA_EXTENSION.to_string();

        let Ok(file) = File::create(path_to_save) else {
            panic!("COULDN'T SAVE LAST LEVEL");
        };
        let writer = BufWriter::new(file);

        match serde_json::to_writer(writer, &self) {
            Ok(_) => (),
            Err(e) => panic!("COULDN'T SAVE CURRENT LEVEL METADATA - {e}"),
        };
    }

    pub fn change_spirits(&mut self, spirits_handler: &SpiritsHandler) {
        self.spirits = Vec::new();

        for spirit in spirits_handler.spirits.values() {
            if spirit.get_dead() {
                continue;
            }

            let position = [
                (spirit.get_position().x / TILE_SIZE as f32).floor() as u8,
                (spirit.get_position().y / TILE_SIZE as f32).floor() as u8,
            ];
            let direction = [
                spirit.get_direction().x.floor() as i8,
                spirit.get_direction().y.floor() as i8,
            ];

            self.spirits.push(SpiritMetadata {
                position,
                amount: 1,
                direction,
            });
        }
    }

    pub fn change_bonfires(
        &mut self,
        fire_td: HashMap<[u8; 2], bool>,
        fire_lr: HashMap<[u8; 2], bool>,
        fire_stop: HashMap<[u8; 2], bool>,
    ) {
        self.bonfires = Vec::new();
        for (position, active) in fire_td {
            self.bonfires.push(BonfireMetadata { position, active });
        }

        for (position, active) in fire_lr {
            self.bonfires.push(BonfireMetadata { position, active });
        }

        for (position, active) in fire_stop {
            self.bonfires.push(BonfireMetadata { position, active });
        }
    }
}
