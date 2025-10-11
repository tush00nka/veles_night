use raylib::prelude::*;

use crate::map::{LevelMap, TileType, TILE_SIZE};

const SPIRIT_SPEED: f32 = 2.;

pub struct Spirit {
    position: Vector2,
    direction: Vector2,
}

impl Spirit {
    pub fn new(pos: Vector2) -> Self {
        Self {
            position: pos,
            direction: Vector2::new(1., 0.),
        }
    }

    pub fn patrol(&mut self, level: &LevelMap) {
        let (tile_x, tile_y) = (
            (self.position.x / TILE_SIZE as f32).floor() as usize,
            (self.position.y / TILE_SIZE as f32).floor() as usize,
        );

        let (dir_x, dir_y) = (self.direction.x as usize, self.direction.y as usize);

        let (next_x, next_y) = (tile_x + dir_x, tile_y + dir_y);

        if level.tiles[next_x][next_y] == TileType::Tree {
            self.direction *= -1.;
        }

        self.position += self.direction * SPIRIT_SPEED;
    }

    pub fn draw(&self, rl: &mut RaylibDrawHandle) {
        rl.draw_circle_v(
            self.position + Vector2::new(TILE_SIZE as f32 / 2., TILE_SIZE as f32 / 2.),
            (TILE_SIZE / 2) as f32,
            Color::LIGHTBLUE.alpha(0.75),
        );
    }
}
