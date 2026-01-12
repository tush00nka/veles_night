use raylib::core::text::RaylibFont;
use raylib::prelude::*;
use std::collections::HashMap;

use crate::{
    FIRST_LEVEL, SCREEN_HEIGHT, SCREEN_WIDTH,
    enemy_spirit::EnemiesHandler,
    level_transition::LevelTransition,
    map::{Level, TILE_SCALE_DEFAULT},
    metadata_handler::MetadataHandler,
    save_handler::SaveHandler,
    scene::{Scene, SceneHandler},
    spirits_handler::SpiritsHandler,
    texture_handler::TextureHandler,
    ui::{Button, UIHandler},
};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct MainMenuHandler {
    buttons: HashMap<u8, Button>,
    labels: Vec<&'static str>,
    labels_offsets: Vec<f32>,
}

impl MainMenuHandler {
    pub fn new() -> Self {
        let mut buttons = HashMap::new();

        buttons.insert(
            0,
            Button {
                selected: false,
                rect: Rectangle::new(
                    (SCREEN_WIDTH / 2) as f32 - 32. * TILE_SCALE_DEFAULT as f32,
                    (SCREEN_HEIGHT / 2) as f32,
                    64. * TILE_SCALE_DEFAULT as f32,
                    16. * TILE_SCALE_DEFAULT as f32,
                ),
				offset: 0.,
            },
        );

        buttons.insert(
            1,
            Button {
                selected: false,
                rect: Rectangle::new(
                    (SCREEN_WIDTH / 2) as f32 - 32. * TILE_SCALE_DEFAULT as f32,
                    (SCREEN_HEIGHT / 2) as f32 + 16. * TILE_SCALE_DEFAULT as f32,
                    64. * TILE_SCALE_DEFAULT as f32,
                    16. * TILE_SCALE_DEFAULT as f32,
                ),
				offset: 0.,
            },
        );

        buttons.insert(
            2,
            Button {
                selected: false,
                rect: Rectangle::new(
                    (SCREEN_WIDTH / 2) as f32 - 32. * TILE_SCALE_DEFAULT as f32,
                    (SCREEN_HEIGHT / 2) as f32 + 32. * TILE_SCALE_DEFAULT as f32,
                    64. * TILE_SCALE_DEFAULT as f32,
                    16. * TILE_SCALE_DEFAULT as f32,
                ),
				offset: 0.,
            },
        );

        let labels = vec!["Продолжить", "Начать", "Выйти", "Уровни"];
        let labels_offsets = vec![0., 4.25, 3.5, 4.];
        Self {
            buttons,
            labels,
            labels_offsets,
        }
    }

    pub fn update(
        &self,
        scene_handler: &mut SceneHandler,
        should_close: &mut bool,
        rl: &mut RaylibHandle,
        save_handler: &mut SaveHandler,
        level_number: &mut u8,
        metadata_handler: &mut MetadataHandler,
        level: &mut Level,
        spirits_handler: &mut SpiritsHandler,
        enemies_handler: &mut EnemiesHandler,
        ui_handler: &mut UIHandler,
        level_transition: &mut LevelTransition,
    ) {
        for (key, button) in self.buttons.iter() {
            if button.rect.check_collision_point_rec(
                rl.get_mouse_position()
                    - Vector2::new(
                        rl.get_screen_width() as f32 / 2. - SCREEN_WIDTH as f32 / 2.,
                        rl.get_screen_height() as f32 / 2. - SCREEN_HEIGHT as f32 / 2.,
                    ),
            ) && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
            {
                match key {
                    0 => {
                        if save_handler.is_there_saves {
                            save_handler.set_to_load();
                        }
                    }
                    1 => {
                        if save_handler.is_there_saves {
                            scene_handler.set(Scene::LevelSelection);
                            return;
                        }
                        *level_number = FIRST_LEVEL;
                        metadata_handler.load(*level_number);
                        level.load(*level_number, metadata_handler, rl);
                        spirits_handler.spawn_spirits(metadata_handler);
                        enemies_handler.spawn_enemies(metadata_handler);
                        *ui_handler = UIHandler::new(FIRST_LEVEL as usize);
                        *level_transition = LevelTransition::new();
                        scene_handler.set(Scene::Level);
                    }
                    2 => {
                        *should_close = true;
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn draw(
        &mut self,
        font: &Font,
        save_handler: &SaveHandler,
        texture_handler: &TextureHandler,
        rl: &mut RaylibDrawHandle,
    ) {
        rl.clear_background(Color::from_hex("0b8a8f").unwrap());

        let mut maxlen: usize = 0;

        for text in self.labels.iter() {
            if text.len() > maxlen {
                maxlen = text.len();
            }
        }

        const LOGO_WIDTH: f32 = 96. * TILE_SCALE_DEFAULT as f32;
        const LOGO_HEIGHT: f32 = 64. * TILE_SCALE_DEFAULT as f32;

        rl.draw_texture_ex(
            texture_handler.get_safe("main_menu_bg"),
            Vector2::zero(),
            0.0,
            TILE_SCALE_DEFAULT as f32,
            Color::WHITE,
        );

        rl.draw_texture_ex(
            texture_handler.get_safe("logo"),
            Vector2::new(
                (SCREEN_WIDTH / 2) as f32 - LOGO_WIDTH / 2.,
                (SCREEN_HEIGHT / 4) as f32 - LOGO_HEIGHT / 2.,
            ),
            0.0,
            TILE_SCALE_DEFAULT as f32,
            Color::WHITE,
        );

        for (key, button) in self.buttons.iter_mut() {
            if *key == 0 && !save_handler.is_there_saves {
                continue;
            }

            let texture_offset = if button.rect.check_collision_point_rec(
                rl.get_mouse_position()
                    - Vector2::new(
                        rl.get_screen_width() as f32 / 2. - SCREEN_WIDTH as f32 / 2.,
                        rl.get_screen_height() as f32 / 2. - SCREEN_HEIGHT as f32 / 2.,
                    ),
            ) {
                0.
            } else {
                16.
            };

            rl.draw_texture_pro(
                texture_handler.get_safe("main_menu_buttons"),
                Rectangle::new(0., texture_offset, 64., 16.),
                button.rect,
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
        }

        for i in 0..self.labels.len() - 1 {
            if i == 0 && !save_handler.is_there_saves {
                continue;
            }

            let mut label_num = i;
            let button = self.buttons.get(&(i as u8)).unwrap();

            let text_offset_y = if button.rect.check_collision_point_rec(
                rl.get_mouse_position()
                    - Vector2::new(
                        rl.get_screen_width() as f32 / 2. - SCREEN_WIDTH as f32 / 2.,
                        rl.get_screen_height() as f32 / 2. - SCREEN_HEIGHT as f32 / 2.,
                    ),
            ) {
                TILE_SCALE_DEFAULT as f32
            } else {
                0.
            };

            if i == 1 && save_handler.is_there_saves {
                label_num = self.labels.len() - 1;
            }

            rl.draw_text_pro(
                font,
                self.labels[label_num],
                Vector2::new(
                    button.rect.x
                        + 5.5 * TILE_SCALE_DEFAULT as f32
                        + ((maxlen - self.labels[label_num].len()) * TILE_SCALE_DEFAULT as usize)
                            as f32
                        + self.labels_offsets[label_num] * TILE_SCALE_DEFAULT as f32,
                    button.rect.y - text_offset_y + 1.5 * TILE_SCALE_DEFAULT as f32,
                ),
                Vector2::zero(),
                0.0,
                12. * TILE_SCALE_DEFAULT as f32,
                1.25 * TILE_SCALE_DEFAULT as f32,
                Color::RAYWHITE,
            );

            // rl.draw_texture_pro(
            //     texture_handler.get_safe("main_menu_buttons"),
            //     Rectangle::new(64., 16. * i as f32, 64., 16.),
            //     self.buttons.get(&(i as u8)).unwrap().rect,
            //     Vector2::zero(),
            //     0.0,
            //     Color::WHITE,
            // );
        }

        rl.draw_text_ex(
            font,
            format!("Версия {}", VERSION).as_str(),
            Vector2::one() * TILE_SCALE_DEFAULT as f32,
            12. * TILE_SCALE_DEFAULT as f32,
            2.,
            Color::RAYWHITE,
        );
    }
}
