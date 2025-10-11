use raylib::prelude::*;

use crate::{
    SCREEN_HEIGHT, SCREEN_WIDTH,
    map::{LEVEL_HEIGHT_TILES, LEVEL_WIDTH_TILES, LevelMap, TILE_SIZE, TileType},
};

pub struct Player {
    position: Vector2,
    speed: f32,
}

impl Player {
    pub fn new() -> Self {
        return Self {
            position: Vector2::new((SCREEN_WIDTH / 2) as f32, (SCREEN_HEIGHT / 2) as f32),
            speed: 5.,
        };
    }

    pub fn update_position(&mut self, level: &LevelMap, rl: &mut RaylibHandle) {
        let mut dir = Vector2::zero();

        if rl.is_key_down(KeyboardKey::KEY_D) {
            dir.x += 1.;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            dir.x -= 1.;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            dir.y += 1.;
        }
        if rl.is_key_down(KeyboardKey::KEY_W) {
            dir.y -= 1.;
        }

        // constraints, so we can't go off map
        self.position.x = ((TILE_SIZE / 2) as f32).max(
            self.position
                .x
                .min((TILE_SIZE as usize * LEVEL_WIDTH_TILES - TILE_SIZE as usize / 2) as f32),
        );
        self.position.y = ((TILE_SIZE / 2) as f32).max(
            self.position
                .y
                .min((TILE_SIZE as usize * LEVEL_HEIGHT_TILES - TILE_SIZE as usize / 2) as f32),
        );

        let next_x = ((self.position.x + dir.x * self.speed) / TILE_SIZE as f32).floor() as usize;
        let next_y = ((self.position.y + dir.y * self.speed) / TILE_SIZE as f32).floor() as usize;

        let pos = Vector2::new(
            (self.position.x / TILE_SIZE as f32).floor(),
            (self.position.y / TILE_SIZE as f32).floor(),
        );

        if level.tiles[next_x][pos.y as usize] == TileType::Tree {
            self.position.y += dir.y * self.speed;
            return;
        } else if level.tiles[pos.x as usize][next_y] == TileType::Tree {
            self.position.x += dir.x * self.speed;
            return;
        }

        self.position += dir.normalized() * self.speed;
    }

    pub fn put_campfire(&self, level: &mut LevelMap, rl: &mut RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            let pos = Vector2::new(
                (self.position.x / TILE_SIZE as f32).floor(),
                (self.position.y / TILE_SIZE as f32).floor(),
            );

            level.tiles[pos.x as usize][pos.y as usize] = TileType::Fire;
        }
    }

    pub fn draw(&self, rl: &mut RaylibDrawHandle) {
        rl.draw_circle_v(self.position, 25., Color::GREEN);
    }
}
