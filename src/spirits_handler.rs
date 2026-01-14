use crate::{Spirit, map::TILE_SIZE, metadata_handler::MetadataHandler};
use raylib::prelude::*;
use std::collections::HashMap;

pub struct SpiritsHandler {
    pub spirits: HashMap<usize, Spirit>,
}

impl SpiritsHandler {
    #[profiling::function]
    pub fn new() -> Self {
        Self {
            spirits: HashMap::new(),
        }
    }
    #[profiling::function]
    pub fn spawn_spirits(&mut self, metadata_handler: &mut MetadataHandler) {
        self.spirits = HashMap::new();
        for spirits_list in 0..metadata_handler.spirits.len() {
            for i in 0..metadata_handler.spirits[spirits_list].amount {
                self.spirits.insert(
                    i as usize + spirits_list * 4,
                    Spirit::new(
                        Vector2::new(
                            metadata_handler.spirits[spirits_list].position[0] as f32
                                * TILE_SIZE as f32,
                            metadata_handler.spirits[spirits_list].position[1] as f32
                                * TILE_SIZE as f32,
                        ),
                        Vector2::new(
                            metadata_handler.spirits[spirits_list].direction[0] as f32,
                            metadata_handler.spirits[spirits_list].direction[1] as f32,
                        ),
                    ),
                );
            }
        }
    }
}
