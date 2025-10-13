use std::collections::HashMap;

use raylib::prelude::*;

use crate::{
    map::{LEVEL_HEIGHT_TILES, LEVEL_WIDTH_TILES, LevelMap, TILE_SIZE, TileType},
    spirit::{Spirit, SpiritState},
};

pub struct OrderHandler {
    spirit: Option<usize>,
    line_end: Option<Vector2>,
    wood: usize,
}

impl OrderHandler {
    pub fn new() -> Self {
        Self {
            spirit: None,
            line_end: None,
            wood: 0,
        }
    }

    pub fn add_wood(&mut self) {
        self.wood += 1;
    }

    pub fn remove_wood(&mut self) {
        self.wood -= 1;
    }

    pub fn select_spirit(
        &mut self,
        spirits: &mut HashMap<usize, Spirit>,
        level: &LevelMap,
        rl: &RaylibHandle,
    ) {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            for (key, spirit) in spirits.iter() {
                if spirit.get_position().distance_to(rl.get_mouse_position()) <= TILE_SIZE as f32 {
                    self.spirit = Some(*key);
                    break;
                }
            }
        }

        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_pos = rl.get_mouse_position();
            let tile_pos = mouse_pos / TILE_SIZE as f32;
            let (tile_x, tile_y) = (tile_pos.x.floor() as usize, tile_pos.y.floor() as usize);

            match level.tiles[tile_x][tile_y] {
                TileType::FireTD { active }
                | TileType::FireLR { active }
                | TileType::FireStop { active } => {
                    if !active {
                        if let Some(key) = self.spirit {
                            if let Some(spirit) = spirits.get_mut(&key) {
                                spirit.set_state(SpiritState::LightFire(tile_x, tile_y));
                            }
                        }
                    }
                }
                TileType::Tree => {
                    if let Some(key) = self.spirit {
                        if let Some(spirit) = spirits.get_mut(&key) {
                            spirit.set_state(SpiritState::ChopTree(tile_x, tile_y));
                        }
                    }
                }
                _ => {}
            }

            if level.tiles[tile_x][tile_y] == TileType::Tree {
                if let Some(key) = self.spirit {
                    if let Some(spirit) = spirits.get_mut(&key) {
                        spirit.set_state(SpiritState::ChopTree(tile_x, tile_y));
                    }
                }
            }

            self.spirit = None;
        }
    }

    pub fn update_line(&mut self, level: &LevelMap, rl: &RaylibHandle) {
        let mouse_pos = rl.get_mouse_position();
        let tile_pos = mouse_pos / TILE_SIZE as f32;
        let (tile_x, tile_y) = (tile_pos.x.floor() as usize, tile_pos.y.floor() as usize);

        if tile_x >= LEVEL_WIDTH_TILES {
            return;
        }

        if tile_y >= LEVEL_HEIGHT_TILES {
            return;
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            match level.tiles[tile_x][tile_y] {
                TileType::Air => {}
                _ => {
                    self.line_end = Some(
                        Vector2::new(tile_x as f32, tile_y as f32) * TILE_SIZE as f32
                            + Vector2::one() * (TILE_SIZE / 2) as f32,
                    );
                    return;
                }
            }
            // if level.tiles[tile_x][tile_y] == TileType::Tree {
            //     self.line_end = Some(
            //         Vector2::new(tile_x as f32, tile_y as f32) * TILE_SIZE as f32
            //             + Vector2::one() * (TILE_SIZE / 2) as f32,
            //     );
            //     return;
            // }
            self.line_end = Some(mouse_pos);
        } else {
            self.line_end = None;
        }
    }

    pub fn draw(&self, spirits: &HashMap<usize, Spirit>, rl: &mut RaylibDrawHandle) {
        let Some(key) = self.spirit else {
            return;
        };

        let Some(spirit) = spirits.get(&key) else {
            return;
        };

        let Some(line_end) = self.line_end else {
            return;
        };

        rl.draw_line_ex(
            spirit.get_position() + Vector2::one() * (TILE_SIZE / 2) as f32,
            line_end,
            16.,
            Color::LIGHTBLUE,
        );
        rl.draw_circle_v(
            spirit.get_position() + Vector2::one() * (TILE_SIZE / 2) as f32,
            8.,
            Color::LIGHTBLUE,
        );
        rl.draw_circle_v(line_end, 8., Color::LIGHTBLUE);
    }

    pub fn draw_ui(&self, rl: &mut RaylibDrawHandle) {
        rl.draw_text(
            format!("wood: {}", self.wood).as_str(),
            10,
            10,
            28,
            Color::BROWN,
        );
    }
}
