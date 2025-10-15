use crate::{metadata_handler::{MetadataHandler}, Spirit, map::TILE_SIZE};
use std::collections::HashMap;
use raylib::prelude::*;

pub struct SpiritsHandler{
    pub spirits: HashMap<usize, Spirit>, 
}

impl SpiritsHandler{
    pub fn new() -> Self{
        Self {
            spirits: HashMap::new(),
        }
    }
    pub fn spawn_spirits(&mut self, metadata_handler: &mut MetadataHandler){
        for spirits_list in 0..metadata_handler.spirits.len(){
            for i in 0..metadata_handler.spirits[spirits_list].amount{
                self.spirits.insert(
                    i as usize + spirits_list * 4,
                    Spirit::new(
                        Vector2::new(
                            metadata_handler.spirits[spirits_list].position[0]  as f32 * TILE_SIZE as f32,
                            metadata_handler.spirits[spirits_list].position[1]  as f32 * TILE_SIZE as f32, 
                    ),
                        Vector2::new(
                            metadata_handler.spirits[spirits_list].direction[0] as f32,
                            metadata_handler.spirits[spirits_list].direction[1] as f32)
                    ),
                );

            }
        }
    }
}
