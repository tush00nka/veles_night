use raylib::prelude::*;
use std::collections::HashMap;

use crate::{
    FIRST_LEVEL, SCREEN_HEIGHT, SCREEN_WIDTH,
    enemy_spirit::EnemiesHandler,
    level_transition::LevelTransition,
    map::Level,
    metadata_handler::MetadataHandler,
    save_handler::SaveHandler,
    scene::{Scene, SceneHandler},
    settings::SettingsHandler,
    settings_menu::SettingsMenuHandler,
    spirits_handler::SpiritsHandler,
    texture_handler::TextureHandler,
    ui::{Button, UIHandler, get_text_size},
};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct MainMenuHandler {
    buttons: HashMap<u8, Button>,
    labels: Vec<&'static str>,
}

impl MainMenuHandler {
    #[profiling::function]
    pub fn new(scale: f32) -> Self {
        let mut buttons = HashMap::new();
        for i in 0..4 {
            buttons.insert(
                i,
                Button {
                    selected: false,
                    rect: Rectangle::new(
                        ((SCREEN_WIDTH * scale as i32) / 2) as f32 - 32. * scale,
                        ((SCREEN_HEIGHT * scale as i32) / 2) as f32
                            + 16. * (i as f32 * scale) as f32,
                        64. * scale,
                        16. * scale,
                    ),
                    offset: 0.,
                    recoil: None,
                },
            );
        }

        let labels = vec!["Продолжить", "Начать", "Настройки", "Выйти", "Уровни"];
        assert_eq!(labels.len(), buttons.len() + 1);

        Self { buttons, labels }
    }

    #[profiling::function]
    pub fn update(
        &self,
        scene_handler: &mut SceneHandler,
        should_close: &mut bool,
        rl: &mut RaylibHandle,
        save_handler: &mut SaveHandler,
        settings_handler: &SettingsHandler,
        level_number: &mut u8,
        metadata_handler: &mut MetadataHandler,
        level: &mut Level,
        spirits_handler: &mut SpiritsHandler,
        enemies_handler: &mut EnemiesHandler,
        ui_handler: &mut UIHandler,
        level_transition: &mut LevelTransition,
        settings_menu: &mut SettingsMenuHandler,
    ) {
        for (key, button) in self.buttons.iter() {
            if button.rect.check_collision_point_rec(
                rl.get_mouse_position()
                    - Vector2::new(
                        rl.get_screen_width() as f32 / 2.
                            - (SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32) as f32
                                / 2.,
                        rl.get_screen_height() as f32 / 2.
                            - (SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32) as f32
                                / 2.,
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
                        spirits_handler.spawn_spirits(metadata_handler, settings_handler);
                        enemies_handler.spawn_enemies(metadata_handler, settings_handler);
                        *ui_handler = UIHandler::new(
                            FIRST_LEVEL as usize,
                            settings_handler.settings.pixel_scale as f32,
                        );
                        *level_transition = LevelTransition::new();
                        scene_handler.set(Scene::Level);
                    }
                    2 => {
                        settings_menu.set_scene(Scene::MainMenu);
                        settings_menu.align_settings(settings_handler.get_settings());
                        scene_handler.set(Scene::Settings);
                    }
                    3 => {
                        *should_close = true;
                    }
                    _ => {
                        panic!("Not implemented yet!");
                    }
                }
            }
        }
    }

    #[profiling::function]
    pub fn draw(
        &mut self,
        font: &Font,
        save_handler: &SaveHandler,
        texture_handler: &TextureHandler,
        rl: &mut RaylibDrawHandle,
        settings_handler: &SettingsHandler,
    ) {
        rl.clear_background(Color::from_hex("0b8a8f").unwrap());

        let mut maxlen: usize = 0;

        for text in self.labels.iter() {
            if text.len() > maxlen {
                maxlen = text.len();
            }
        }

        let logo_width: f32 = 96. * settings_handler.settings.pixel_scale as f32;
        let logo_height: f32 = 64. * settings_handler.settings.pixel_scale as f32;

        rl.draw_texture_ex(
            texture_handler.get_safe("main_menu_bg"),
            Vector2::zero(),
            0.0,
            settings_handler.settings.pixel_scale as f32,
            Color::WHITE,
        );

        rl.draw_texture_ex(
            texture_handler.get_safe("logo"),
            Vector2::new(
                ((SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32) / 2) as f32
                    - logo_width / 2.,
                ((SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32) / 4) as f32
                    - logo_height / 2.,
            ),
            0.0,
            settings_handler.settings.pixel_scale as f32,
            Color::WHITE,
        );

        for i in 0..self.labels.len() - 1 {
            if i == 0 && !save_handler.is_there_saves {
                continue;
            }

            let mut label_num = i;
            let button = self.buttons.get(&(i as u8)).unwrap();

            let text_offset_y = if button.rect.check_collision_point_rec(
                rl.get_mouse_position()
                    - Vector2::new(
                        rl.get_screen_width() as f32 / 2.
                            - (SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32) as f32
                                / 2.,
                        rl.get_screen_height() as f32 / 2.
                            - (SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32) as f32
                                / 2.,
                    ),
            ) {
                settings_handler.settings.pixel_scale as f32
            } else {
                0.
            };

            if i == 1 && save_handler.is_there_saves {
                label_num = self.labels.len() - 1;
            }

            let text_dimensions = get_text_size(
                font,
                self.labels[label_num],
                12. * settings_handler.settings.pixel_scale as f32,
                1.25 * settings_handler.settings.pixel_scale as f32,
            );

            let texture_offset = if text_offset_y == 0. { 16. } else { 0. };

            button.draw_with_text_middle(
                rl,
                self.labels[label_num],
                font,
                texture_handler.get_safe("main_menu_buttons"),
                &Rectangle::new(0., texture_offset, 64., 16.),
                text_dimensions,
                &Color::RAYWHITE,
                12. * settings_handler.settings.pixel_scale as f32,
                1.25 * settings_handler.settings.pixel_scale as f32,
                Vector2::new(0., -text_offset_y),
                Vector2::new(0., 0.),
            );
        }

        rl.draw_text_ex(
            font,
            format!("Версия {}", VERSION).as_str(),
            Vector2::one() * settings_handler.settings.pixel_scale as f32,
            6. * settings_handler.settings.pixel_scale as f32,
            0.,
            Color::RAYWHITE,
        );
    }
}
