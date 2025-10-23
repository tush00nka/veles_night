use std::fs;

use raylib::prelude::*;

use crate::{enemy_spirit::EnemiesHandler, level_transition::{self, LevelTransition}, map::Level, map_loader::{MapLoader, MAP_SAVE_PATH}, metadata_handler::{self, MetadataHandler, SAVE_METADATA_PATH}, scene::{Scene, SceneHandler}, spirits_handler::{self, SpiritsHandler}, ui::UIHandler};

pub struct SaveHandler{
    pub should_save: bool,
    pub should_load: bool,
    pub is_there_saves: bool,
}

impl SaveHandler{
    pub fn new() -> Self{
        Self{
            should_save: false,
            should_load: false,
            is_there_saves: false,
        }
    }
    
    pub fn set_to_save(&mut self){
        self.should_save = true;
    }
   
    pub fn set_to_load(&mut self){
        self.should_load = true;
    }

    pub fn check_saves(&mut self){
        let map_p = &MAP_SAVE_PATH.to_string();
        let metadata_p = &SAVE_METADATA_PATH.to_string();

        self.is_there_saves = fs::read_dir(map_p).unwrap().next().is_some() && fs::read_dir(metadata_p).unwrap().next().is_some();
    }

    fn get_level_number() -> u8{
        let filenames = fs::read_dir(MAP_SAVE_PATH).unwrap();

        for filename in filenames{
            let file = match  filename{
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
    ){
        metadata_handler.load_save();
        
        *level_number = SaveHandler::get_level_number(); 
        level_transition.set_cards(*level_number as usize);
        level.load_save(level_number.clone(), metadata_handler, rl); //need other function call
        spirits_handler.spawn_spirits(metadata_handler);
        enemies_handler.spawn_enemies(metadata_handler);
        scene_handler.set(Scene::Level);
        *ui_handler = UIHandler::new(level_number.clone() as usize);
        self.should_load = false;
    }

    pub fn create_save_file(
        &mut self, 
        metadata_handler: &mut MetadataHandler,
        level: &mut Level,
        spirits_handler: &mut SpiritsHandler,
        level_number: &mut u8,
        ){
            MapLoader::save_map(*level_number, level, metadata_handler);         
            metadata_handler.change_spirits(spirits_handler);
            metadata_handler.save(*level_number);
            self.should_save = false;
    }
}
