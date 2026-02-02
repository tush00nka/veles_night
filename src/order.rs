use raylib::prelude::*;

use crate::{
    HotkeyHandler, SCREEN_HEIGHT, SCREEN_WIDTH,
    hotkey_handler::HotkeyCategory,
    map::{LEVEL_HEIGHT_TILES, LEVEL_WIDTH_TILES, Level, TILE_SIZE_PX, TileType},
    settings::SettingsHandler,
    spirit::SpiritState,
    spirits_handler::SpiritsHandler,
    texture_handler::TextureHandler,
};

pub struct OrderHandler {
    spirit: Option<usize>,
    line_end: Option<Vector2>,
}

impl OrderHandler {
    #[profiling::function]
    pub fn new() -> Self {
        Self {
            spirit: None,
            line_end: None,
        }
    }

    #[profiling::function]
    pub fn select_spirit(
        &mut self,
        spirits_handler: &mut SpiritsHandler,
        level: &mut Level,
        rl: &RaylibHandle,
        hotkey_handler: &mut HotkeyHandler,
        settings_handler: &mut SettingsHandler,
    ) {
        let if_mouse = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
        if if_mouse || hotkey_handler.check_pressed(rl, HotkeyCategory::PickNearest) {
            let mouse_pos = rl.get_mouse_position()
                - Vector2::new(
                    rl.get_screen_width() as f32 / 2.
                        - (SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32) as f32 / 2.,
                    rl.get_screen_height() as f32 / 2.
                        - (SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32) as f32
                            / 2.,
                );

            let mut key_nearest = usize::MAX;
            let mut nearest_dist = f32::MAX;

            for (key, spirit) in spirits_handler.spirits.iter() {
                let dist = (spirit.get_draw_position()
                    + Vector2::new(
                        ((TILE_SIZE_PX * settings_handler.settings.pixel_scale as i32) / 2) as f32,
                        ((TILE_SIZE_PX * settings_handler.settings.pixel_scale as i32) / 2) as f32,
                    ))
                .distance_to(mouse_pos);
                if dist
                    <= (TILE_SIZE_PX * settings_handler.settings.pixel_scale as i32) as f32 * 1.1
                    && dist < nearest_dist
                {
                    key_nearest = *key;
                    nearest_dist = dist;
                }
            }
            if key_nearest != usize::MAX {
                self.spirit = Some(key_nearest);
            }
        }

        if if_mouse {
            hotkey_handler.clear_last();
        }

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
            self.spirit = None;
            return;
        }

        let keyboard_last = hotkey_handler.get_last_key();
        if !rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT)
            && keyboard_last == KeyboardKey::KEY_NUM_LOCK
        {
            for y in 0..LEVEL_HEIGHT_TILES {
                for x in 0..LEVEL_WIDTH_TILES {
                    match &mut level.tiles[x][y] {
                        TileType::FireTD {
                            active: _,
                            selected,
                        }
                        | TileType::FireLR {
                            active: _,
                            selected,
                        }
                        | TileType::FireStop {
                            active: _,
                            selected,
                        }
                        | TileType::Tree {
                            chance: _,
                            selected,
                        } => *selected = false,
                        _ => {}
                    }
                }
            }

            if self.spirit.is_none() {
                return;
            }

            let mouse_pos = rl.get_mouse_position()
                - Vector2::new(
                    rl.get_screen_width() as f32 / 2.
                        - (SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32) as f32 / 2.,
                    rl.get_screen_height() as f32 / 2.
                        - (SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32) as f32
                            / 2.,
                );

            let tile_pos =
                mouse_pos / (TILE_SIZE_PX * settings_handler.settings.pixel_scale as i32) as f32;
            let (mut tile_x, mut tile_y) =
                (tile_pos.x.floor() as usize, tile_pos.y.floor() as usize);

            if tile_x >= LEVEL_WIDTH_TILES {
                tile_x = LEVEL_WIDTH_TILES - 1;
            }

            if tile_y >= LEVEL_HEIGHT_TILES {
                tile_y = LEVEL_HEIGHT_TILES - 1;
            }

            match &mut level.tiles[tile_x][tile_y] {
                TileType::FireTD {
                    active: _,
                    selected,
                }
                | TileType::FireLR {
                    active: _,
                    selected,
                }
                | TileType::FireStop {
                    active: _,
                    selected,
                }
                | TileType::Tree {
                    chance: _,
                    selected,
                } => {
                    *selected = true;
                }
                _ => {}
            }

            return;
        }

        if keyboard_last != KeyboardKey::KEY_NUM_LOCK && !rl.is_key_released(keyboard_last) {
            return;
        }

        let mouse_pos = rl.get_mouse_position()
            - Vector2::new(
                rl.get_screen_width() as f32 / 2.
                    - (SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32) as f32 / 2.,
                rl.get_screen_height() as f32 / 2.
                    - (SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32) as f32 / 2.,
            );
        let tile_pos =
            mouse_pos / (TILE_SIZE_PX * settings_handler.settings.pixel_scale as i32) as f32;
        let (mut tile_x, mut tile_y) = (tile_pos.x.floor() as usize, tile_pos.y.floor() as usize);

        if tile_x >= LEVEL_WIDTH_TILES {
            tile_x = LEVEL_WIDTH_TILES - 1;
        }

        if tile_y >= LEVEL_HEIGHT_TILES {
            tile_y = LEVEL_HEIGHT_TILES - 1;
        }

        match level
            .tiles
            .get_mut(tile_x)
            .unwrap()
            .get_mut(tile_y)
            .unwrap()
        {
            TileType::FireTD { active, selected }
            | TileType::FireLR { active, selected }
            | TileType::FireStop { active, selected } => {
                if !*active {
                    if let Some(key) = self.spirit {
                        if let Some(spirit) = spirits_handler.spirits.get_mut(&key) {
                            *selected = false;
                            spirit.set_state(SpiritState::LightFire(tile_x, tile_y));
                        }
                    }
                }
            }
            TileType::Tree {
                chance: _,
                selected,
            } => {
                if let Some(key) = self.spirit {
                    if let Some(spirit) = spirits_handler.spirits.get_mut(&key) {
                        *selected = false;
                        spirit.set_state(SpiritState::ChopTree(tile_x, tile_y));
                    }
                }
            }
            _ => {
                // println!("wht?");
            }
        }

        self.spirit = None;
    }

    #[profiling::function]
    pub fn update_line(
        &mut self,
        level: &Level,
        rl: &RaylibHandle,
        hotkey_handler: &mut HotkeyHandler,
        settings_handler: &mut SettingsHandler,
    ) {
        let mouse_pos = rl.get_mouse_position()
            - Vector2::new(
                rl.get_screen_width() as f32 / 2.
                    - (SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32) as f32 / 2.,
                rl.get_screen_height() as f32 / 2.
                    - (SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32) as f32 / 2.,
            );
        let tile_pos =
            mouse_pos / (TILE_SIZE_PX * settings_handler.settings.pixel_scale as i32) as f32;
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
                TileType::Air { selected: _ }
                | TileType::Swamp {
                    teleport_position: _,
                }
                | TileType::Exit(_) => {}
                _ => {
                    self.line_end = Some(
                        Vector2::new(tile_x as f32, tile_y as f32)
                            * (TILE_SIZE_PX * settings_handler.settings.pixel_scale as i32) as f32,
                    );
                    return;
                }
            }

            self.line_end = Some(
                mouse_pos
                    - Vector2::one()
                        * (TILE_SIZE_PX * settings_handler.settings.pixel_scale as i32) as f32
                        / 2.,
            );
        } else {
            self.line_end = None;
        }
    }

    #[profiling::function]
    pub fn draw(
        &self,
        spirits_handler: &SpiritsHandler,
        texture_handler: &TextureHandler,
        rl: &mut RaylibDrawHandle,
        settings_handler: &mut SettingsHandler,
    ) {
        let Some(key) = self.spirit else {
            return;
        };

        let Some(spirit) = spirits_handler.spirits.get(&key) else {
            return;
        };

        let Some(line_end) = self.line_end else {
            return;
        };

        let direction = line_end - spirit.get_draw_position();

        let length = (direction.length()
            / (TILE_SIZE_PX * settings_handler.settings.pixel_scale as i32) as f32)
            .floor()
            * 2.
            + 1.;

        for i in 0..=length as usize {
            let position = spirit.get_draw_position() + direction / length * i as f32;

            // pixel-perfect effect (may be a bit extra)
            let position = Vector2::new(
                (position.x / settings_handler.settings.pixel_scale as f32).floor()
                    * settings_handler.settings.pixel_scale as f32,
                (position.y / settings_handler.settings.pixel_scale as f32).floor()
                    * settings_handler.settings.pixel_scale as f32,
            );

            rl.draw_texture_ex(
                texture_handler.get_safe("dot"),
                position,
                0.0,
                settings_handler.settings.pixel_scale as f32,
                Color::RAYWHITE,
            );
        }
    }
}
