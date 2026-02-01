use std::fs;

use raylib::prelude::*;

use crate::{
    dialogue::DialogueHandler,
    enemy_spirit::EnemiesHandler,
    level_transition::LevelTransition,
    map::Level,
    map_loader::MapLoader,
    metadata_handler::MetadataHandler,
    scene::{Scene, SceneHandler},
    settings::SettingsHandler,
    spirits_handler::SpiritsHandler,
    ui::UIHandler,
};

pub const SAVE_PATH: &str = "dynamic/save/";

pub struct SaveHandler {
    pub should_save: bool,
    pub should_load: bool,
    pub is_there_saves: bool,
}

impl SaveHandler {
    #[profiling::function]
    pub fn new() -> Self {
        Self {
            should_save: false,
            should_load: false,
            is_there_saves: false,
        }
    }

    #[profiling::function]
    pub fn set_to_save(&mut self) {
        self.should_save = true;
    }

    #[profiling::function]
    pub fn set_to_load(&mut self) {
        self.should_load = true;
    }

    #[profiling::function]
    pub fn check_saves(&mut self) {
        let save_p = &SAVE_PATH.to_string();

        let mut dir;
        match fs::read_dir(save_p) {
            Ok(d) => dir = d,
            Err(_) => {
                fs::create_dir(save_p).expect("Couldn't create a dir :(");
                dir = fs::read_dir(save_p).unwrap();
            }
        }

        self.is_there_saves = dir.next().is_some();
    }

    #[profiling::function]
    pub fn get_level_number() -> u8 {
        let filenames = fs::read_dir(SAVE_PATH).unwrap();

        for filename in filenames {
            let file = match filename {
                Ok(f) => f,
                Err(e) => panic!("COULDN'T LOAD MAP FOR GETTING LEVEL NUMBER - {e}"),
            };

            let name = file
                .file_name()
                .into_string()
                .unwrap()
                .split('.')
                .next()
                .unwrap()
                .to_string();

            return name.parse().unwrap();
        }
        panic!("COULDN'T PARSE LEVEL_NUMBER");
    }

    #[profiling::function]
    pub fn load_save(
        &mut self,
        metadata_handler: &mut MetadataHandler,
        level: &mut Level,
        spirits_handler: &mut SpiritsHandler,
        enemies_handler: &mut EnemiesHandler,
        ui_handler: &mut UIHandler,
        level_number: &mut u8,
        level_transition: &mut LevelTransition,
        rl: &mut RaylibHandle,
        scene_handler: &mut SceneHandler,
        dialogue_handler: &mut DialogueHandler,
        settings_handler: &mut SettingsHandler,
    ) {
        metadata_handler.load_save();

        *level_number = SaveHandler::get_level_number();
        level_transition.set_cards(*level_number as usize);
        level.load_save(level_number.clone(), metadata_handler, rl); //need other function call
        spirits_handler.spawn_spirits(metadata_handler, settings_handler);
        enemies_handler.spawn_enemies(metadata_handler, settings_handler);
        scene_handler.set(Scene::Level);
        *ui_handler = UIHandler::new(
            level_number.clone() as usize,
            settings_handler.settings.pixel_scale as f32,
        );
        dialogue_handler.load_dialogue(&format!("level_{}", *level_number + 1));
        self.should_load = false;
    }

    #[profiling::function]
    pub fn create_save_file(
        &mut self,
        metadata_handler: &mut MetadataHandler,
        level: &mut Level,
        spirits_handler: &mut SpiritsHandler,
        level_number: &mut u8,
        settings_handler: &mut SettingsHandler,
    ) {
        MapLoader::save_map(*level_number, level, metadata_handler);
        metadata_handler.change_spirits(spirits_handler, settings_handler);
        metadata_handler.save(*level_number);
        self.should_save = false;
    }
}
