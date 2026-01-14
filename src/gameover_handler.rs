use std::collections::HashMap;

use crate::{
    FIRST_LEVEL, SCREEN_HEIGHT, SCREEN_WIDTH,
    hotkey_handler::{HotkeyCategory, HotkeyHandler},
    music_handler::MusicHandler,
    scene::{Scene, SceneHandler},
    ui::Button,
};
use raylib::{ffi::CheckCollisionPointRec, prelude::*};

pub struct GameOverHandler {
    restart_buttons: HashMap<String, Button>,
    restart_text: HashMap<String, Vector2>, //position
    gameover_type: GameOverHandlerType,
}

const LABELS_RESTART: [&str; 2] = ["СДАТЬСЯ", "ЕЩЁ РАЗ"];
const LABELS_ENDGAME: [&str; 2] = ["В МЕНЮ", "ВЫЙТИ"];

const LEVEL_LOSE: [&str; 1] = [
    "Мы в canned meat studios хотим поблагодарить\nвас за игру в велесову ночь. Нам очень жаль,\nчто вы не добились успехов и надеемся,\nчто вы справитесь лучше в следующий раз.\nудачи!",
];
const GAME_END_TEXT: [&str; 1] =
    ["Спасибо за игру в велесову ночь!\nБлагодаря Вам души предков обрели покой."];

#[derive(Clone, PartialEq, Eq)]
pub enum GameOverHandlerType {
    Level,
    Game,
}

impl GameOverHandler {
    pub fn new(window_type: GameOverHandlerType) -> Self {
        let mut restart_buttons = HashMap::new();
        let mut restart_text = HashMap::new();

        let text = match window_type {
            GameOverHandlerType::Level => LABELS_RESTART,
            GameOverHandlerType::Game => LABELS_ENDGAME,
        };

        for i in 0..text.len() {
            restart_buttons.insert(
                text[i].to_string(),
                Button {
                    rect: Rectangle::new(
                        SCREEN_WIDTH as f32 / 2. - 75.,
                        (SCREEN_HEIGHT / 2 + 64) as f32 + i as f32 * 96.,
                        150.,
                        64.,
                    ),
					offset: 0.,
                    selected: false,
                },
            );
            restart_text.insert(
                text[i].to_string(),
                Vector2::new(
                    SCREEN_WIDTH as f32 / 2. - 70.,
                    (SCREEN_HEIGHT / 2 + 64) as f32 + i as f32 * 96. + 5.,
                ),
            );
        }

        Self {
            restart_buttons,
            restart_text,
            gameover_type: window_type,
        }
    }

    #[profiling::function]
    pub fn draw_gameover(&self, font: &Font, rl: &mut RaylibDrawHandle) {
        rl.clear_background(Color::from_hex("0b8a8f").unwrap());
        for (name, button) in self.restart_buttons.iter() {
            let color = if unsafe {
                CheckCollisionPointRec(
                    (rl.get_mouse_position()
                        - Vector2::new(
                            rl.get_screen_width() as f32 / 2. - SCREEN_WIDTH as f32 / 2.,
                            rl.get_screen_height() as f32 / 2. - SCREEN_HEIGHT as f32 / 2.,
                        ))
                    .into(),
                    button.rect.into(),
                )
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

            let main_text = match self.gameover_type {
                GameOverHandlerType::Level => LEVEL_LOSE[0], //can add pseudo-random to pick random
                //slur to player
                GameOverHandlerType::Game => GAME_END_TEXT[0],
            };

            rl.draw_text_pro(
                font,
                main_text,
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

    #[profiling::function]
    pub fn update_gameover(
        &mut self,
        level_number: &mut u8,
        rl: &mut RaylibHandle,
        scene_handler: &mut SceneHandler,
        music_handler: &MusicHandler,
        hotkeys: &mut HotkeyHandler,
        should_close: &mut bool,
    ) -> bool {
        let mut scene = crate::scene::Scene::Level;
        let mut check = false;

        if self.gameover_type == GameOverHandlerType::Level {
            music_handler.music_pause();
        }

        if hotkeys.check_pressed(rl, HotkeyCategory::Exit) {
            scene = crate::scene::Scene::MainMenu;
            *level_number = FIRST_LEVEL;
            check = true;
        }

        if hotkeys.check_pressed(rl, HotkeyCategory::Continue) {
            if self.gameover_type == GameOverHandlerType::Game {
                scene = Scene::MainMenu;
            }
            check = true;
        }

        if check {
            scene_handler.set(scene);
            music_handler.stop("death");
            return true;
        }

        for (title, button) in self.restart_buttons.iter_mut() {
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                if unsafe {
                    CheckCollisionPointRec(
                        (rl.get_mouse_position()
                            - Vector2::new(
                                rl.get_screen_width() as f32 / 2. - SCREEN_WIDTH as f32 / 2.,
                                rl.get_screen_height() as f32 / 2. - SCREEN_HEIGHT as f32 / 2.,
                            ))
                        .into(),
                        button.rect.into(),
                    )
                } {
                    button.selected = true;
                }
            }
            if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) && button.selected {
                let scene = match title.as_str() {
                    "СДАТЬСЯ" | "В МЕНЮ" => {
                        *level_number = FIRST_LEVEL;
                        crate::scene::Scene::MainMenu
                    }
                    "ЕЩЁ РАЗ" => crate::Scene::Level,
                    "ВЫЙТИ" => {
                        *should_close = true;
                        return false;
                    }
                    _ => {
                        panic!("NOT EXISITNG BUTTON");
                    }
                };
                button.selected = false;
                scene_handler.set(scene);
                music_handler.stop("death");
                return true;
            }
        }
        return false;
    }
}
