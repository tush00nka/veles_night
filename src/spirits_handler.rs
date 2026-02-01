use crate::{
    Spirit, map::TILE_SIZE_PX, metadata_handler::MetadataHandler, settings::SettingsHandler,
};
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
    pub fn rescale_ui(&mut self, prev_scale: f32, new_scale: f32) {
        for (_, spirit) in self.spirits.iter_mut() {
            spirit.rescale(prev_scale, new_scale);
        }
    }
    #[profiling::function]
    pub fn spawn_spirits(
        &mut self,
        metadata_handler: &mut MetadataHandler,
        settings_handler: &SettingsHandler,
    ) {
        self.spirits = HashMap::new();
        for spirits_list in 0..metadata_handler.spirits.len() {
            for i in 0..metadata_handler.spirits[spirits_list].amount {
                self.spirits.insert(
                    i as usize + spirits_list * 4,
                    Spirit::new(
                        Vector2::new(
                            metadata_handler.spirits[spirits_list].position[0] as f32
                                * (TILE_SIZE_PX * settings_handler.settings.pixel_scale as i32)
                                    as f32,
                            metadata_handler.spirits[spirits_list].position[1] as f32
                                * (TILE_SIZE_PX * settings_handler.settings.pixel_scale as i32)
                                    as f32,
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
