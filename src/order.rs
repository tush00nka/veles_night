use raylib::prelude::*;

use crate::{
    hotkey_handler::HotkeyCategory, HotkeyHandler,
    map::{LEVEL_HEIGHT_TILES, LEVEL_WIDTH_TILES, Level, TILE_SIZE, TileType},
    spirit::SpiritState,
    spirits_handler::SpiritsHandler,
};

pub struct OrderHandler {
    spirit: Option<usize>,
    line_end: Option<Vector2>,
}

impl OrderHandler {
    pub fn new() -> Self {
        Self {
            spirit: None,
            line_end: None,
        }
    }

    pub fn select_spirit(
        &mut self,
        spirits_handler: &mut SpiritsHandler,
        level: &Level,
        rl: &RaylibHandle,
        hotkey_handler: &mut HotkeyHandler,
    ) {
        let if_mouse = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
        if if_mouse || hotkey_handler.check_pressed(rl, HotkeyCategory::PickNearest) {
            for (key, spirit) in spirits_handler.spirits.iter() {
                if spirit.get_position().distance_to(rl.get_mouse_position()) <= TILE_SIZE as f32 {
                    self.spirit = Some(*key);
                    break;
                }
            }
        }

        if if_mouse {
            hotkey_handler.clear_last();
        }

        let keyboard_last = hotkey_handler.get_last_key();
        if !rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT)
            && keyboard_last == KeyboardKey::KEY_NUM_LOCK
        {
            return;
        }

        if keyboard_last != KeyboardKey::KEY_NUM_LOCK && !rl.is_key_released(keyboard_last) {
            return;
        }

        let mouse_pos = rl.get_mouse_position();
        let tile_pos = mouse_pos / TILE_SIZE as f32;
        let (mut tile_x, mut tile_y) = (tile_pos.x.floor() as usize, tile_pos.y.floor() as usize);

        if tile_x >= LEVEL_WIDTH_TILES {
            tile_x = LEVEL_WIDTH_TILES - 1;
        }

        if tile_y >= LEVEL_HEIGHT_TILES {
            tile_y = LEVEL_HEIGHT_TILES - 1;
        }

        match level.tiles[tile_x][tile_y] {
            TileType::FireTD { active }
            | TileType::FireLR { active }
            | TileType::FireStop { active } => {
                if !active {
                    if let Some(key) = self.spirit {
                        if let Some(spirit) = spirits_handler.spirits.get_mut(&key) {
                            spirit.set_state(SpiritState::LightFire(tile_x, tile_y));
                        }
                    }
                }
            }
            TileType::Tree(_) => {
                if let Some(key) = self.spirit {
                    if let Some(spirit) = spirits_handler.spirits.get_mut(&key) {
                        spirit.set_state(SpiritState::ChopTree(tile_x, tile_y));
                    }
                }
            }
            _ => {}
        }

        match level.tiles[tile_x][tile_y] {
            TileType::Tree(_) => {
                if let Some(key) = self.spirit {
                    if let Some(spirit) = spirits_handler.spirits.get_mut(&key) {
                        spirit.set_state(SpiritState::ChopTree(tile_x, tile_y));
                    }
                }
            }
            _ => {}
        }

        self.spirit = None;
    }

    pub fn update_line(
        &mut self,
        level: &Level,
        rl: &RaylibHandle,
        hotkey_handler: &mut HotkeyHandler,
    ) {
        let mouse_pos = rl.get_mouse_position();
        let tile_pos = mouse_pos / TILE_SIZE as f32;
        let (mut tile_x, mut tile_y) = (tile_pos.x.floor() as usize, tile_pos.y.floor() as usize);

        if tile_x >= LEVEL_WIDTH_TILES {
            tile_x = LEVEL_WIDTH_TILES - 1;
        }

        if tile_y >= LEVEL_HEIGHT_TILES {
            tile_y = LEVEL_HEIGHT_TILES - 1;
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT)
            || hotkey_handler.check_down(rl, HotkeyCategory::PickNearest)
        {
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

    pub fn draw(&self, spirits_handler: &SpiritsHandler, rl: &mut RaylibDrawHandle) {
        let Some(key) = self.spirit else {
            return;
        };

        let Some(spirit) = spirits_handler.spirits.get(&key) else {
            return;
        };

        let Some(line_end) = self.line_end else {
            return;
        };

        rl.draw_line_ex(
            spirit.get_draw_position() + Vector2::one() * (TILE_SIZE / 2) as f32,
            line_end,
            16.,
            Color::LIGHTBLUE,
        );
        rl.draw_circle_v(
            spirit.get_draw_position() + Vector2::one() * (TILE_SIZE / 2) as f32,
            8.,
            Color::LIGHTBLUE,
        );
        rl.draw_circle_v(line_end, 8., Color::LIGHTBLUE);
    }
}
