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

const BUTTON_TEXTURE_WIDTH: f32 = 64.;
const BUTTON_TEXTURE_HEIGHT: f32 = 16.;
const BUTTON_OFFSET_Y: f32 = 16.;
const LOGO_WIDTH: f32 = 96.;
const LOGO_HEIGHT: f32 = 64.;

const BACKGROUND_IMAGE_NAME: &str = "main_menu_bg";
const LOGO_NAME: &str = "logo";

const BUTTON_LABELS: [&str; 5] = ["Продолжить", "Начать", "Настройки", "Выйти", "Уровни"];

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
                        (SCREEN_WIDTH as f32 - BUTTON_TEXTURE_WIDTH) / 2. * scale,
                        ((SCREEN_HEIGHT * scale as i32) / 2) as f32
                            + BUTTON_OFFSET_Y * (i as f32 * scale) as f32,
                        BUTTON_TEXTURE_WIDTH * scale,
                        BUTTON_TEXTURE_HEIGHT * scale,
                    ),
                    offset: 0.,
                },
            );
        }

        let labels: Vec<&str> = BUTTON_LABELS.into();
        assert_eq!(labels.len(), buttons.len() + 1);

        Self { buttons, labels }
    }
    pub fn rescale_ui(&mut self, new_scale: f32) {
        for i in 0..BUTTON_LABELS.len() - 1 {
            let button = self.buttons.get_mut(&(i as u8)).unwrap();
            button.rect.x = (SCREEN_WIDTH as f32 - BUTTON_TEXTURE_WIDTH) / 2. * new_scale;
            button.rect.y = ((SCREEN_HEIGHT * new_scale as i32) / 2) as f32
                + BUTTON_OFFSET_Y * (i as f32 * new_scale) as f32;
            button.rect.width = BUTTON_TEXTURE_WIDTH * new_scale;
            button.rect.height = BUTTON_TEXTURE_HEIGHT * new_scale;
        }
    }
    #[profiling::function]
    pub fn update(
        &mut self,
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
        for (key, button) in self.buttons.iter_mut() {
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

        let logo_width: f32 = LOGO_WIDTH * settings_handler.settings.pixel_scale as f32;
        let logo_height: f32 = LOGO_HEIGHT * settings_handler.settings.pixel_scale as f32;

        rl.draw_texture_ex(
            texture_handler.get_safe(BACKGROUND_IMAGE_NAME),
            Vector2::zero(),
            0.0,
            settings_handler.settings.pixel_scale as f32,
            Color::WHITE,
        );

        rl.draw_texture_ex(
            texture_handler.get_safe(LOGO_NAME),
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
            let button = self.buttons.get_mut(&(i as u8)).unwrap();

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
            ) && rl.is_mouse_button_up(MouseButton::MOUSE_BUTTON_LEFT)
            {
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
