use raylib::{
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

const LEVEL_WIDTH_TILES: usize = 16;
const LEVEL_HEIGHT_TILES: usize = 9;
const TILE_SIZE_PX: i32 = 16;
const TILE_SCALE: i32 = 4;
const TILE_SIZE: i32 = TILE_SIZE_PX * TILE_SCALE;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Air,
    LeftFire,
    RightFire,
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

    pub fn draw(&self, rl: &mut RaylibDrawHandle) {
        for x in 0..LEVEL_WIDTH_TILES {
            for y in 0..LEVEL_HEIGHT_TILES {
                if self.tiles[x][y] != TileType::Air {
                    rl.draw_rectangle(
                        x as i32 * TILE_SIZE,
                        y as i32 * TILE_SIZE,
                        TILE_SIZE,
                        TILE_SIZE,
                        Color::ORANGERED,
                    );
                }
            }
        }
    }
}
