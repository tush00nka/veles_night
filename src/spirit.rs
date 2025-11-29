use raylib::prelude::*;

use crate::{
    map::{LEVEL_HEIGHT_TILES, LEVEL_WIDTH_TILES, Level, TILE_SIZE, TileType},
    music_handler::MusicHandler,
    texture_handler::TextureHandler,
};

const SPIRIT_SPEED: f32 = 5.;

pub enum SpiritState {
    Patrol,
    ChopTree(usize, usize),
    LightFire(usize, usize),
}

pub struct Spirit {
    position: Vector2,
    draw_position: Vector2,
    timer: f32,
    direction: Vector2,
    state: SpiritState,
    dead: bool,
    teleported: u8,
}

impl Spirit {
    #[allow(unused)]
    pub fn default(pos: Vector2) -> Self {
        Self {
            position: pos,
            draw_position: pos,
            timer: 0.0,
            direction: Vector2::new(1., 0.),
            state: SpiritState::Patrol,
            dead: false,
            teleported: 0,
        }
    }

    pub fn new(pos: Vector2, dir: Vector2) -> Self {
        Self {
            position: pos,
            draw_position: pos,
            timer: 0.0,
            direction: dir,
            state: SpiritState::Patrol,
            dead: false,
            teleported: 0,
        }
    }

    #[allow(unused)]
    pub fn get_direction(&self) -> Vector2 {
        self.direction
    }

    pub fn get_position(&self) -> Vector2 {
        self.position //+ Vector2::one() * (TILE_SIZE / 2) as f32
    }

    #[allow(unused)]
    pub fn get_teleported(&self) -> u8 {
        self.teleported
    }
    pub fn get_draw_position(&self) -> Vector2 {
        // let x = (self.draw_position.x / TILE_SCALE as f32).floor() * TILE_SCALE as f32;
        // let y = (self.draw_position.y / TILE_SCALE as f32).floor() * TILE_SCALE as f32;

        // Vector2::new(x, y)
        self.draw_position
    }

    pub fn get_dead(&self) -> bool {
        self.dead
    }

    pub fn set_state(&mut self, state: SpiritState) {
        self.state = state;
    }

    pub fn update_behaviour(
        &mut self,
        level: &mut Level,
        music_handler: &MusicHandler,
        rl: &mut RaylibHandle,
    ) {
        match self.state {
            SpiritState::Patrol => {
                if self.timer >= 0.5 {
                    self.patrol(music_handler, level);
                    self.timer = 0.0;
                } else {
                    self.timer += rl.get_frame_time();
                    self.update_position_smoothly(rl);
                }
            }
            SpiritState::ChopTree(x, y) => self.chop_tree(x, y, level, music_handler, rl),
            SpiritState::LightFire(x, y) => self.light_fire(x, y, level, music_handler, rl),
        }
    }

    pub fn kill_spirit(&mut self) {
        self.dead = true;
    }

    pub fn update_position_smoothly(&mut self, rl: &mut RaylibHandle) {
        self.draw_position = self
            .draw_position
            .lerp(self.position, SPIRIT_SPEED * rl.get_frame_time());
    }

    fn patrol(&mut self, music_handler: &MusicHandler, level: &mut Level) {
        let (tile_x, tile_y) = (
            (self.get_position().x / TILE_SIZE as f32).floor() as usize,
            (self.get_position().y / TILE_SIZE as f32).floor() as usize,
        );

        let mut next = self.get_position() + self.direction * TILE_SIZE as f32;

        let (next_x, next_y) = (
            (next.x / TILE_SIZE as f32).round() as usize,
            (next.y / TILE_SIZE as f32).round() as usize,
        );

        if self.teleported != 0 {
            self.teleported -= 1;
        }

        // step on tile to activate
        match level.tiles[tile_x][tile_y] {
            TileType::FireTD {
                active,
                selected: _,
            } => {
                if active && self.direction.y == 0. {
                    self.direction = Vector2::new(0., 1.);
                    return;
                }
            }
            TileType::FireLR {
                active,
                selected: _,
            } => {
                if active && self.direction.x == 0. {
                    self.direction = Vector2::new(1., 0.);
                    return;
                }
            }
            TileType::Exit(_) => {
                self.dead = true;
                music_handler.play("foom");
                level.survive();
                return;
            }

            TileType::Swamp { teleport_position } => {
                if self.teleported == 0 {
                    self.teleported = 2;
                    next = teleport_position * TILE_SIZE as f32;
                }
            }
            _ => {}
        }

        if self.teleported <= 1
            && (next_x >= LEVEL_WIDTH_TILES
                || next_y >= LEVEL_HEIGHT_TILES
                || tile_x <= 0
                || tile_y <= 0)
        {
            self.dead = true;
            music_handler.play("foom");
            return;
        }
        if self.teleported <= 1 {
            // activate before tile
            match level.tiles[next_x][next_y] {
                TileType::Tree {
                    chance: _,
                    selected: _,
                } => {
                    self.direction *= -1.;
                    return;
                }
                TileType::FireStop {
                    active,
                    selected: _,
                } => {
                    if active {
                        self.direction *= -1.;
                        return;
                    }
                }
                _ => {}
            }
        }

        // if level.tiles[next_x][next_y] == TileType::Tree {
        //     self.direction *= -1.;
        // }

        self.position = next;
    }

    fn light_fire(
        &mut self,
        x: usize,
        y: usize,
        level: &mut Level,
        music_handler: &MusicHandler,
        rl: &RaylibHandle,
    ) {
        match level.tiles[x][y] {
            TileType::FireTD {
                active,
                selected: _,
            }
            | TileType::FireLR {
                active,
                selected: _,
            }
            | TileType::FireStop {
                active,
                selected: _,
            } => {
                if active {
                    self.state = SpiritState::Patrol;
                    return;
                }
            }
            _ => {
                self.state = SpiritState::Patrol;
                return;
            }
        }

        let target = Vector2::new(x as f32 * TILE_SIZE as f32, y as f32 * TILE_SIZE as f32);

        if self.position.distance_to(target) <= (TILE_SIZE / 10) as f32 {
            let tile = level.tiles.get_mut(x).unwrap().get_mut(y).unwrap();
            match tile {
                TileType::FireTD {
                    active,
                    selected: _,
                }
                | TileType::FireLR {
                    active,
                    selected: _,
                }
                | TileType::FireStop {
                    active,
                    selected: _,
                } => *active = true,
                _ => {
                    panic!("no such tile bruh")
                }
            }
            self.dead = true;
            music_handler.play("foom");
        }

        self.position = self
            .position
            .lerp(target, SPIRIT_SPEED * rl.get_frame_time());

        self.draw_position = self.position;
    }

    fn chop_tree(
        &mut self,
        x: usize,
        y: usize,
        level: &mut Level,
        music_handler: &MusicHandler,
        rl: &RaylibHandle,
    ) {
        match level.tiles[x][y] {
            TileType::Tree {
                chance: _,
                selected: _,
            } => {}
            _ => {
                self.state = SpiritState::Patrol;
                return;
            }
        };

        let target = Vector2::new(x as f32 * TILE_SIZE as f32, y as f32 * TILE_SIZE as f32);

        if self.position.distance_to(target) <= (TILE_SIZE / 10) as f32 {
            level.tiles[x][y] = TileType::Air;
            level.add_wood();
            self.dead = true;
            music_handler.play("foom");
        }

        self.position = self
            .position
            .lerp(target, SPIRIT_SPEED * rl.get_frame_time());

        self.draw_position = self.position;
    }

    pub fn draw(&self, rl: &mut RaylibDrawHandle, texture_handler: &TextureHandler) {
        let source = Rectangle::new(
            ((rl.get_time() * 8.) % 4.).floor() as f32 * 16.,
            16.,
            16.,
            16.,
        );

        rl.draw_texture_pro(
            texture_handler.get_safe("spirit"),
            source,
            Rectangle::new(
                self.get_draw_position().x,
                self.get_draw_position().y,
                TILE_SIZE as f32,
                TILE_SIZE as f32,
            ),
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );

        // let (x, y) = (
        //     (self.get_position().x / TILE_SIZE as f32).floor(),
        //     (self.get_position().y / TILE_SIZE as f32).floor(),
        // );

        // rl.draw_rectangle_lines(
        //     x as i32 * TILE_SIZE,
        //     y as i32 * TILE_SIZE,
        //     TILE_SIZE,
        //     TILE_SIZE,
        //     Color::WHITE,
        // );

        // let (next_x, next_y) = (x + self.direction.x, y + self.direction.y);
        // rl.draw_rectangle_lines(
        //     next_x as i32 * TILE_SIZE,
        //     next_y as i32 * TILE_SIZE,
        //     TILE_SIZE,
        //     TILE_SIZE,
        //     Color::GRAY,
        // );
    }
}
