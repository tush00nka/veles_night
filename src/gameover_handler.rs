use std::collections::HashMap;

use crate::{
    FIRST_LEVEL, SCREEN_HEIGHT, SCREEN_WIDTH,
    hotkey_handler::{HotkeyCategory, HotkeyHandler},
    scene::SceneHandler,
    ui::Button,
};
use raylib::{ffi::CheckCollisionPointRec, prelude::*};

pub struct GameOverHandler {
    restart_buttons: HashMap<String, Button>,
    restart_text: HashMap<String, Vector2>, //position
}
const LABELS_RESTART: [&str; 2] = ["СДАТЬСЯ", "ЕЩЁ РАЗ"];

impl GameOverHandler {
    pub fn new() -> Self {
        let mut restart_buttons = HashMap::new();
        let mut restart_text = HashMap::new();

        for i in 0..LABELS_RESTART.len() {
            restart_buttons.insert(
                LABELS_RESTART[i].to_string(),
                Button {
                    rect: Rectangle::new(
                        SCREEN_WIDTH as f32 / 2. - 75.,
                        (SCREEN_HEIGHT / 2 + 64) as f32 + i as f32 * 96.,
                        150.,
                        64.,
                    ),
                    selected: false,
                },
            );
            restart_text.insert(
                LABELS_RESTART[i].to_string(),
                Vector2::new(
                    SCREEN_WIDTH as f32 / 2. - 70.,
                    (SCREEN_HEIGHT / 2 + 64) as f32 + i as f32 * 96. + 5.,
                ),
            );
        }

        Self {
            restart_buttons,
            restart_text,
        }
    }

    pub fn draw_gameover(&self, font: &Font, rl: &mut RaylibDrawHandle) {
        rl.clear_background(Color::from_hex("0b8a8f").unwrap());
        for (name, button) in self.restart_buttons.iter() {
            let color = if unsafe {
                CheckCollisionPointRec(rl.get_mouse_position().into(), button.rect.into())
            } {
                Color::WHITE
            } else {
                Color::BLACK.alpha(0.5)
            };

            let color_text = if color == Color::WHITE {
                Color::BLACK
            } else {
                Color::WHITE
            };

            rl.draw_text_pro(
                font,
                "мы в canned meat studios хотим поблагодарить\nвас за игру в велесову ночь. Нам очень жаль,\nчто вы не добились успехов и надеемся,\nчто вы справитесь лучше в следующий раз.\nудачи!",
                Vector2::new((128.) as f32, (SCREEN_HEIGHT / 6) as f32),
                Vector2::zero(),
                0.0,
                48.,
                2.0,
                Color::RAYWHITE,
            );

            rl.draw_rectangle_rec(button.rect, color);
            rl.draw_text_pro(
                font,
                name,
                self.restart_text.get(name).unwrap(),
                Vector2::zero(),
                0.0,
                48.,
                2.0,
                color_text,
            );
        }
    }
    pub fn update_gameover(
        &mut self,
        level_number: &mut u8,
        rl: &mut RaylibHandle,
        scene_handler: &mut SceneHandler,
        hotkeys: &mut HotkeyHandler,
    ) -> bool {
        let mut scene = crate::scene::Scene::Level;
        let mut check = false; 
        
        if hotkeys.check_pressed(rl, HotkeyCategory::Exit){
            scene = crate::scene::Scene::MainMenu;
            *level_number = FIRST_LEVEL; 
            check = true;
        }

        if hotkeys.check_pressed(rl, HotkeyCategory::Continue){
            check = true;
        }

        if check {
            scene_handler.set(scene);
            return true
        }

        for (title, button) in self.restart_buttons.iter_mut() {
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                if unsafe {
                    CheckCollisionPointRec(rl.get_mouse_position().into(), button.rect.into())
                } {
                    button.selected = true;
                }
            }
            if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) && button.selected {
                let scene = match title.as_str() {
                    "СДАТЬСЯ" => {
                        *level_number = FIRST_LEVEL;
                        crate::scene::Scene::MainMenu
                    }
                    "ЕЩЁ РАЗ" => crate::Scene::Level,
                    _ => {
                        panic!("NOT EXISITNG BUTTON");
                    }
                };
                button.selected = false;
                scene_handler.set(scene);
                return true;
            }
        }
        return false;
    }
}
