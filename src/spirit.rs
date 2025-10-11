use raylib::prelude::*;

use crate::{map::{LevelMap, TileType, LEVEL_HEIGHT_TILES, LEVEL_WIDTH_TILES, TILE_SIZE}, order::OrderHandler};

const SPIRIT_SPEED: f32 = 2.;

pub enum SpiritState {
    Patrol,
    ChopTree(usize, usize),
}

pub struct Spirit {
    position: Vector2,
    direction: Vector2,
    state: SpiritState,
    dead: bool,
}

impl Spirit {
    pub fn new(pos: Vector2) -> Self {
        Self {
            position: pos,
            direction: Vector2::new(1., 0.),
            state: SpiritState::Patrol,
            dead: false,
        }
    }

    pub fn get_position(&self) -> Vector2 {
        self.position + Vector2::one() * (TILE_SIZE / 2) as f32
    }

    pub fn get_dead(&self) -> bool {
        self.dead
    }

    pub fn set_state(&mut self, state: SpiritState) {
        self.state = state;
    }

    pub fn update_behaviour(&mut self, level: &mut LevelMap, order_handler: &mut OrderHandler, rl: &RaylibHandle) {
        match self.state {
            SpiritState::Patrol => self.patrol(level),
            SpiritState::ChopTree(x, y) => self.chop_tree(x, y, level, order_handler, rl),
        }
    }

    fn patrol(&mut self, level: &LevelMap) {
        let (tile_x, tile_y) = (
            (self.position.x / TILE_SIZE as f32).floor() as usize,
            (self.position.y / TILE_SIZE as f32).floor() as usize,
        );

        let (dir_x, dir_y) = (self.direction.x as usize, self.direction.y as usize);

        let (next_x, next_y) = (tile_x + dir_x, tile_y + dir_y);

        if next_x >= LEVEL_WIDTH_TILES - 1
            || next_y >= LEVEL_HEIGHT_TILES - 1
            || next_x <= 0
            || next_y <= 0
        {
            self.direction *= -1.;
        }

        if level.tiles[next_x][next_y] == TileType::Tree {
            self.direction *= -1.;
        }

        self.position += self.direction * SPIRIT_SPEED;
    }

    fn chop_tree(&mut self, x: usize, y: usize, level: &mut LevelMap, order_handler: &mut OrderHandler, rl: &RaylibHandle) {
        if level.tiles[x][y] != TileType::Tree {
            self.state = SpiritState::Patrol;
            return;
        }

        let target = Vector2::new(x as f32 * TILE_SIZE as f32, y as f32 * TILE_SIZE as f32);

        if self.position.distance_to(target) <= (TILE_SIZE / 10) as f32 {
            level.tiles[x][y] = TileType::Air;
            order_handler.add_wood();
            self.dead = true;
        }

        self.position = self
            .position
            .lerp(target, SPIRIT_SPEED * rl.get_frame_time())
    }

    pub fn draw(&self, rl: &mut RaylibDrawHandle) {
        rl.draw_circle_v(
            self.position + Vector2::new(TILE_SIZE as f32 / 2., TILE_SIZE as f32 / 2.),
            (TILE_SIZE / 2) as f32,
            Color::LIGHTBLUE.alpha(0.75),
        );
    }
}
