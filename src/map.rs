use raylib::{color::Color, prelude::*};

use crate::texture_handler::TextureHandler;

pub const LEVEL_WIDTH_TILES: usize = 16;
pub const LEVEL_HEIGHT_TILES: usize = 9;
const TILE_SIZE_PX: i32 = 16;
const TILE_SCALE: i32 = 4;
pub const TILE_SIZE: i32 = TILE_SIZE_PX * TILE_SCALE;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Air,
    Fire,
    Tree,
}

pub struct LevelMap {
    pub tiles: [[TileType; LEVEL_HEIGHT_TILES]; LEVEL_WIDTH_TILES],
}

impl LevelMap {
    pub fn new() -> Self {
        Self {
            tiles: [[TileType::Air; LEVEL_HEIGHT_TILES]; LEVEL_WIDTH_TILES],
        }
    }

    pub fn draw(&self, rl: &mut RaylibDrawHandle, texture_handler: &TextureHandler) {
        for x in 0..LEVEL_WIDTH_TILES {
            for y in 0..LEVEL_HEIGHT_TILES {
                match self.tiles[x][y] {
                    TileType::Air => {}
                    TileType::Fire => {
                        rl.draw_circle(
                            x as i32 * TILE_SIZE + TILE_SIZE / 2,
                            y as i32 * TILE_SIZE + TILE_SIZE / 2,
                            (TILE_SIZE * 3) as f32
                                + TILE_SIZE as f32 / 8. * (rl.get_time() * 2.).sin() as f32,
                            Color::ORANGE.alpha(0.25),
                        );
                        rl.draw_rectangle(
                            x as i32 * TILE_SIZE,
                            y as i32 * TILE_SIZE,
                            TILE_SIZE,
                            TILE_SIZE,
                            Color::ORANGERED,
                        );
                    }
                    TileType::Tree => {
                        rl.draw_texture_pro(
                            texture_handler.get_safe("tree"),
                            Rectangle::new(0., 0., 16., 16.),
                            Rectangle::new(
                                (x as i32 * TILE_SIZE) as f32,
                                (y as i32 * TILE_SIZE) as f32,
                                TILE_SIZE as f32,
                                TILE_SIZE as f32,
                            ),
                            Vector2::zero(),
                            0.0,
                            Color::WHITE,
                        );
                        // rl.draw_rectangle(
                        //     x as i32 * TILE_SIZE,
                        //     y as i32 * TILE_SIZE,
                        //     TILE_SIZE,
                        //     TILE_SIZE,
                        //     Color::DARKGREEN,
                        // );
                    }
                }
            }
        }
    }
}
