use raylib::prelude::*;

use crate::{enemy_spirit::EnemiesHandler, level_transition::{self, LevelTransition}, map::Level, map_loader::MapLoader, metadata_handler::{self, MetadataHandler}, scene::{Scene, SceneHandler}, spirits_handler::{self, SpiritsHandler}, ui::UIHandler};

pub struct SaveHandler{
    save_level: bool,
    pub should_load: bool,
}

impl SaveHandler{
    pub fn new() -> Self{
        Self{
            save_level: false,
            should_load: false,
        }
    }
    
    pub fn save_level(&mut self){
        self.save_level = true;
    }
    
    pub fn clear(&mut self){
        self.save_level = false;
        self.should_load = false;
    }
    
    pub fn set_to_load(&mut self){
        self.should_load = true;
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
        *level_number = 1; 
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
            self.save_level = false;
    }
}
