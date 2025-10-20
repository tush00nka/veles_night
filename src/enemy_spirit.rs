use std::collections::HashMap;

use raylib::prelude::*;

use crate::{map::TILE_SIZE, metadata_handler::{self, MetadataHandler}, spirit::Spirit, spirits_handler::SpiritsHandler, texture_handler::{self, TextureHandler}};

pub struct EnemiesHandler{
    pub enemies: HashMap<u8, Enemy>,
}

impl EnemiesHandler{
    pub fn new() -> Self{
        Self{
            enemies: HashMap::new()
        }
    }

    pub fn spawn_enemies(&mut self, metadata_handler: &mut MetadataHandler){
        for i in 0..metadata_handler.enemies.len(){
            self.enemies.insert(i as u8, Enemy::new( 
                    Vector2::new(
                        metadata_handler.enemies[i].position[0] as f32 * TILE_SIZE as f32,
                        metadata_handler.enemies[i].position[1] as f32 * TILE_SIZE as f32,
                    )
            )
            );
        }
    }
}

pub struct Enemy{
    position: Vector2,
}

impl Enemy{
    pub fn new(position: Vector2) -> Self{
        Self{
            position
        }
    }
    
    fn get_position(&self) -> Vector2{
        return self.position
    }
    pub fn collide_check(&mut self, spirits: &mut SpiritsHandler) {
        
        let near_spirits = spirits.spirits.iter_mut().filter(|(_, spirit)| spirit.get_position() == self.get_position()); 
    
        for spirit in near_spirits{
            spirit.1.kill_spirit();
        }
    }
    pub fn draw(&self, rl: &mut RaylibDrawHandle, texture_handler: &TextureHandler){
        let source = Rectangle::new(
            ((rl.get_time() * 8.) % 4.).floor() as f32 * 16.,
            16.,
            16.,
            16.,
        );

        rl.draw_texture_pro(
            texture_handler.get("enemy_ghost"),
            source,
            Rectangle::new(
                self.position.x as f32,
                self.position.y as f32,
                TILE_SIZE as f32,
                TILE_SIZE as f32,
            ),
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );
    }
}
