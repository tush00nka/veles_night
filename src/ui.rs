use std::{cmp::min, collections::HashMap};

use raylib::{ffi::CheckCollisionPointRec, prelude::*};

use crate::{
    SCREEN_HEIGHT, SCREEN_WIDTH,
    hotkey_handler::{HotkeyCategory, HotkeyHandler},
    map::{Level, TILE_SIZE, TileType},
    scene::{Scene, SceneHandler},
    texture_handler::TextureHandler,
};

pub struct Button {
    pub rect: Rectangle,
    pub selected: bool,
}

const QUIT_BUTTON: Rectangle = Rectangle::new(
    SCREEN_WIDTH as f32 / 3. + SCREEN_WIDTH as f32 / 6. - 100.,
    SCREEN_HEIGHT as f32 / 3. + 150.,
    200.,
    100.,
);

pub struct UIHandler {
    build_buttons: HashMap<String, Button>,
    quitting: bool,
}

impl UIHandler {
    pub fn new(level_number: usize) -> Self {
        let mut buttons = HashMap::new();

        let labels = ["fire_td", "fire_lr", "fire_stop"];

        for i in 0..min(labels.len(), level_number) {
            buttons.insert(
                labels[i].to_string(),
                Button {
                    rect: Rectangle::new(
                        i as f32 * 80. + (SCREEN_WIDTH / 2) as f32 - 40. * 3., // todo: pohui
                        5.,
                        64.,
                        64.,
                    ),
                    selected: false,
                },
            );
        }

        Self {
            build_buttons: buttons,
            quitting: false,
        }
    }

    pub fn build(
        &mut self,
        level: &mut Level,
        rl: &mut RaylibHandle,
        hotkey_h: &mut HotkeyHandler,
    ) {
        let mut intent: HotkeyCategory;
        for (title, button) in self.build_buttons.iter_mut() {
            intent = HotkeyCategory::from_bonfire(title);

            if hotkey_h.check_pressed(rl, intent) {
                button.selected = true;
            }

            if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                if unsafe {
                    CheckCollisionPointRec(rl.get_mouse_position().into(), button.rect.into())
                } {
                    button.selected = true;
                }
                hotkey_h.clear_last();
            }

            let keyboard_last = hotkey_h.get_last_key();

            if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                button.selected = false;
                continue;
            }

            if !rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT)
                && keyboard_last == KeyboardKey::KEY_NUM_LOCK
            {
                continue;
            }

            if keyboard_last != KeyboardKey::KEY_NUM_LOCK && !rl.is_key_released(keyboard_last) {
                continue;
            }

            if button.selected && level.get_wood() > 0 {
                let tile = match title.as_str() {
                    "fire_td" => TileType::FireTD { active: false },
                    "fire_lr" => TileType::FireLR { active: false },
                    "fire_stop" => TileType::FireStop { active: false },
                    _ => {
                        panic!("wait how")
                    }
                };

                let pos = rl.get_mouse_position() / (Vector2::one() * TILE_SIZE as f32);
                let (x, y) = (pos.x as usize, pos.y as usize);

                level.tiles[x][y] = tile;
                level.remove_wood();
            }
            button.selected = false;
        }
    }

    pub fn update(
        &mut self,
        hotkey_h: &mut HotkeyHandler,
        scene_h: &mut SceneHandler,
        rl: &mut RaylibHandle,
    ) -> bool{
        if hotkey_h.check_pressed(rl, HotkeyCategory::Exit) {
            self.quitting = !self.quitting;
        }

        if !self.quitting {
            return false;
        }

        if unsafe { CheckCollisionPointRec(rl.get_mouse_position().into(), QUIT_BUTTON.into()) }
        && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
        {
            scene_h.set(Scene::MainMenu);
            return true;
        };
        return false;
    }

    pub fn draw(
        &self,
        texture_handler: &TextureHandler,
        level: &mut Level,
        font: &Font,
        rl: &mut RaylibDrawHandle,
    ) {
        for (tex_name, button) in self.build_buttons.iter() {
            let color = if unsafe {
                CheckCollisionPointRec(rl.get_mouse_position().into(), button.rect.into())
            } {
                Color::WHITE
            } else {
                Color::BLACK.alpha(0.5)
            };

            rl.draw_rectangle_rec(button.rect, color);

            if !button.selected {
                rl.draw_texture_pro(
                    texture_handler.get_safe(tex_name),
                    Rectangle::new(
                        ((rl.get_time() * 8.) % 4.).floor() as f32 * 16.,
                        16.,
                        16.,
                        16.,
                    ),
                    button.rect,
                    Vector2::zero(),
                    0.0,
                    Color::WHITE,
                );
            } else {
                rl.draw_texture_pro(
                    texture_handler.get_safe(tex_name),
                    Rectangle::new(
                        ((rl.get_time() * 8.) % 4.).floor() as f32 * 16.,
                        16.,
                        16.,
                        16.,
                    ),
                    Rectangle::new(
                        rl.get_mouse_position().x - (TILE_SIZE / 2) as f32,
                        rl.get_mouse_position().y - (TILE_SIZE / 2) as f32,
                        TILE_SIZE as f32,
                        TILE_SIZE as f32,
                    ),
                    Vector2::zero(),
                    0.0,
                    Color::WHITE,
                );
            }
        }

        rl.draw_rectangle(5, 5, 256, 74, Color::BLACK.alpha(0.5));

        rl.draw_text_ex(
            font,
            format!(
                "Духов проведено: {}/{}\nДревесина: {}",
                level.survived,
                level.survive,
                level.get_wood()
            )
            .as_str(),
            Vector2::one() * 10.,
            32.,
            1.0,
            Color::RAYWHITE,
        );

        rl.draw_rectangle(SCREEN_WIDTH - 260, 5, 256, 40, Color::BLACK.alpha(0.5));
        rl.draw_text_ex(
            font,
            "R для перезапуска",
            Vector2::new((SCREEN_WIDTH - 250) as f32, 10.),
            32.,
            1.0,
            Color::RAYWHITE,
        );

        if self.quitting {
            rl.draw_rectangle(
                SCREEN_WIDTH / 3,
                SCREEN_HEIGHT / 3,
                SCREEN_WIDTH / 3,
                SCREEN_HEIGHT / 3,
                Color::BLACK.alpha(0.5),
            );

            rl.draw_text_ex(
                font,
                "Выйти в меню?",
                Vector2::new(
                    SCREEN_WIDTH as f32 / 3. + 8. * 13.,
                    SCREEN_HEIGHT as f32 / 3. + 10.,
                ),
                64.,
                2.,
                Color::RAYWHITE,
            );

            let mouse_over = unsafe {
                CheckCollisionPointRec(rl.get_mouse_position().into(), QUIT_BUTTON.into())
            };

            rl.draw_rectangle_rec(
                QUIT_BUTTON,
                if mouse_over {
                    Color::RAYWHITE
                } else {
                    Color::BLACK.alpha(0.5)
                },
            );

            rl.draw_text_ex(
                font,
                "Да",
                Vector2::new(
                    QUIT_BUTTON.x + QUIT_BUTTON.width / 2. - 32.,
                    QUIT_BUTTON.y + 16.,
                ),
                64.,
                2.,
                if mouse_over {
                    Color::BLACK
                } else {
                    Color::RAYWHITE
                },
            );
        }
    }
}
