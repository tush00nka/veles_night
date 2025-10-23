use std::collections::HashMap;

use raylib::{ffi::CheckCollisionPointRec, prelude::*};

use crate::{
    FIRST_LEVEL, SCREEN_HEIGHT, SCREEN_WIDTH,
    enemy_spirit::EnemiesHandler,
    map::{Level, TILE_SCALE},
    metadata_handler::{self, MetadataHandler},
    save_handler::SaveHandler,
    scene::{Scene, SceneHandler},
    spirits_handler::{self, SpiritsHandler},
    texture_handler::TextureHandler,
    ui::Button,
};

pub struct MainMenuHandler {
    buttons: HashMap<u8, Button>,
    labels: Vec<&'static str>,
}

impl MainMenuHandler {
    pub fn new() -> Self {
        let mut buttons = HashMap::new();

        buttons.insert(
            0,
            Button {
                selected: false,
                rect: Rectangle::new(
                    (SCREEN_WIDTH / 2) as f32 - 98.,
                    (SCREEN_HEIGHT / 2) as f32 + 32.,
                    216.,
                    64.,
                ),
            },
        );

        buttons.insert(
            1,
            Button {
                selected: false,
                rect: Rectangle::new(
                    (SCREEN_WIDTH / 2) as f32 - 98.,
                    (SCREEN_HEIGHT / 2) as f32 + 100.,
                    216.,
                    64.,
                ),
            },
        );

        buttons.insert(
            2,
            Button {
                selected: false,
                rect: Rectangle::new(
                    (SCREEN_WIDTH / 2) as f32 - 98.,
                    (SCREEN_HEIGHT / 2) as f32 + 168.,
                    216.,
                    64.,
                ),
            },
        );

        let labels = vec!["Продолжить", "Начать", "Закончить"];

        Self { buttons, labels }
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
    ) {
        for (key, button) in self.buttons.iter() {
            if unsafe { CheckCollisionPointRec(rl.get_mouse_position().into(), button.rect.into()) }
                && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
            {
                match key {
                    0 => {
                        if save_handler.is_there_saves {
                            save_handler.set_to_load();
                        }
                    }
                    1 => {
                        *level_number = FIRST_LEVEL;
                        metadata_handler.load(*level_number);
                        level.load(*level_number, metadata_handler, rl);
                        spirits_handler.spawn_spirits(metadata_handler);
                        enemies_handler.spawn_enemies(metadata_handler);
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

        const LOGO_WIDTH: f32 = 96. * TILE_SCALE as f32;
        const LOGO_HEIGHT: f32 = 64. * TILE_SCALE as f32;

        rl.draw_texture_ex(
            texture_handler.get_safe("main_menu_bg"),
            Vector2::zero(),
            0.0,
            TILE_SCALE as f32,
            Color::WHITE,
        );

        rl.draw_texture_ex(
            texture_handler.get_safe("logo"),
            Vector2::new(
                (SCREEN_WIDTH / 2) as f32 - LOGO_WIDTH / 2.,
                (SCREEN_HEIGHT / 4) as f32 - LOGO_HEIGHT / 2.,
            ),
            0.0,
            TILE_SCALE as f32,
            Color::WHITE,
        );

        let mut button_selected = 128;

        for (key, button) in self.buttons.iter_mut() {
            if *key == 0 && !save_handler.is_there_saves {
                continue;
            }

            let color;
            if unsafe { CheckCollisionPointRec(rl.get_mouse_position().into(), button.rect.into()) }
            {
                color = Color::WHITE;

                button_selected = *key;
            } else {
                color = Color::BLACK.alpha(0.5);
            };

            rl.draw_rectangle_rec(button.rect, color);
        }

        for i in 0..self.labels.len() {
            if i == 0 && !save_handler.is_there_saves {
                continue;
            }

            let color;
            if i == button_selected as usize {
                color = Color::BLACK;
            } else {
                color = Color::RAYWHITE;
            }
            rl.draw_text_pro(
                font,
                self.labels[i],
                Vector2::new(
                    (SCREEN_WIDTH / 2) as f32 - 70.,
                    (SCREEN_HEIGHT / 2) as f32 + 40. + (66. * i as f32),
                ),
                Vector2::zero(),
                0.0,
                48.,
                2.0,
                color,
            );
        }
    }
}
