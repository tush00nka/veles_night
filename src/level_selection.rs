use std::fs;

use raylib::prelude::*;

use crate::{
    SCREEN_HEIGHT, SCREEN_WIDTH,
    dialogue::DialogueHandler,
    enemy_spirit::EnemiesHandler,
    hotkey_handler::{HotkeyCategory, HotkeyHandler},
    level_transition::LevelTransition,
    map::Level,
    metadata_handler::MetadataHandler,
    save_handler::SaveHandler,
    scene::{Scene, SceneHandler},
    settings::SettingsHandler,
    spirits_handler::SpiritsHandler,
    texture_handler::TextureHandler,
    ui::{UIHandler, get_text_size},
};

const LEVEL_DIR: &str = "static/maps/";
const BACK_BUTTON_TEXTURE: &str = "main_menu_buttons";

const MENU_TEXT: &str = "Выбор уровня";
const BACK_BUTTON_TEXT: &str = "Назад в меню";

const BUTTON_SIZE: f32 = 16.;

const TEXT_SIZE: f32 = 12.;
const TEXT_SPACING: f32 = 1.05;
const MENU_NAME_Y_OFFSET: f32 = 20.;

const BUTTON_X_OFFSET: f32 = BUTTON_SIZE / 4.;
const BUTTON_Y_OFFSET: f32 = BUTTON_SIZE / 2.;

const COLUMNS_AMOUNT: usize = 10;

const BACK_BUTTON_SIZE_WIDTH: f32 = 64.;
const BACK_BUTTON_SIZE_HEIGHT: f32 = 16.;

const BACK_BUTTON_Y_OFFSET: f32 = BACK_BUTTON_SIZE_HEIGHT * 2.;

struct Button {
    rec: Rectangle,
    offset: f32,
}

pub struct LevelSelector {
    buttons: Vec<Button>,
    back_button_rect: Rectangle,
}

impl LevelSelector {
    #[profiling::function]
    pub fn new(pixel_scale: i32) -> Self {
        let dir = fs::read_dir(LEVEL_DIR);

        let level_count;
        match dir {
            Ok(d) => level_count = d.count(),
            Err(e) => panic!("Failed to read level dir: {}", e),
        }

        let mut buttons = vec![];

        for _ in 0..level_count {
            buttons.push(Button {
                rec: Rectangle::default(),
                offset: 0.0,
            });
        }

        let mut back_button = Rectangle::default();
        Self::set_to_default(&mut buttons, &mut back_button, pixel_scale as f32);

        Self {
            buttons,
            back_button_rect: back_button,
        }
    }

    pub fn rescale_ui(&mut self, scale: f32) {
        Self::set_to_default(&mut self.buttons, &mut self.back_button_rect, scale);
    }

    fn set_to_default(buttons: &mut Vec<Button>, rect: &mut Rectangle, scale: f32) {
        let row_count = ((buttons.len() + COLUMNS_AMOUNT / 2 - 1) as f32 / COLUMNS_AMOUNT as f32)
            .round() as f32;

        const WIDTH_PX: f32 =
            (BUTTON_SIZE + BUTTON_X_OFFSET) * COLUMNS_AMOUNT as f32 - BUTTON_X_OFFSET;
        let height_px = (BUTTON_SIZE + BUTTON_Y_OFFSET) * row_count - BUTTON_Y_OFFSET;

        for i in 0..buttons.len() {
            let x = ((SCREEN_WIDTH as f32 - WIDTH_PX) / 2.
                + (i % COLUMNS_AMOUNT) as f32 * (BUTTON_SIZE + BUTTON_X_OFFSET))
                * scale;

            let y = ((SCREEN_HEIGHT as f32 - height_px) / 2.
                + (i / COLUMNS_AMOUNT) as f32 * (BUTTON_SIZE + BUTTON_Y_OFFSET))
                * scale;

            buttons[i].rec.x = x;
            buttons[i].rec.y = y;
            buttons[i].rec.width = BUTTON_SIZE * scale;
            buttons[i].rec.height = BUTTON_SIZE * scale;

            rect.x = SCREEN_WIDTH as f32 / 2. * scale - BACK_BUTTON_SIZE_WIDTH / 2. * scale as f32;
            rect.y = SCREEN_HEIGHT as f32 * scale - BACK_BUTTON_Y_OFFSET * scale as f32;
            rect.width = BACK_BUTTON_SIZE_WIDTH * scale as f32;
            rect.height = BACK_BUTTON_SIZE_HEIGHT * scale as f32;
        }
    }

    #[profiling::function]
    pub fn update(
        &self,
        level_number: &mut u8,
        metadata_handler: &mut MetadataHandler,
        level: &mut Level,
        spirits_handler: &mut SpiritsHandler,
        enemies_handler: &mut EnemiesHandler,
        ui_handler: &mut UIHandler,
        level_transition: &mut LevelTransition,
        scene_handler: &mut SceneHandler,
        dialogue_handler: &mut DialogueHandler,
        rl: &mut RaylibHandle,
        settings_handler: &mut SettingsHandler,
        hotkey_handler: &mut HotkeyHandler,
    ) {
        let mouse_pos = rl.get_mouse_position()
            - Vector2::new(
                rl.get_screen_width() as f32 / 2.
                    - SCREEN_WIDTH as f32 / 2. * settings_handler.settings.pixel_scale as f32,
                rl.get_screen_height() as f32 / 2.
                    - SCREEN_HEIGHT as f32 / 2. * settings_handler.settings.pixel_scale as f32,
            );

        if hotkey_handler.check_pressed(rl, HotkeyCategory::PickButton1)
            || hotkey_handler.check_pressed(rl, HotkeyCategory::Exit)
            || (self.back_button_rect.check_collision_point_rec(mouse_pos)
                && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT))
        {
            scene_handler.set(Scene::MainMenu);
        }

        for i in 0..self.buttons.len() {
            if i > SaveHandler::get_level_number().into() {
                return;
            }

            if self.buttons[i].rec.check_collision_point_rec(mouse_pos) {
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                    *level_number = i as u8;
                    metadata_handler.load(*level_number);
                    level.load(*level_number, metadata_handler, rl);
                    spirits_handler.spawn_spirits(metadata_handler, settings_handler);
                    enemies_handler.spawn_enemies(metadata_handler, settings_handler);
                    *ui_handler = UIHandler::new(i, settings_handler.settings.pixel_scale as f32);
                    *level_transition = LevelTransition::new();
                    scene_handler.set(Scene::Level);
                    dialogue_handler.load_dialogue(&format!("level_{}", *level_number + 1));
                }
            }
        }
    }

    #[profiling::function]
    pub fn draw(
        &mut self,
        font: &Font,
        texture_handler: &TextureHandler,
        rl: &mut RaylibDrawHandle,
        settings_handler: &mut SettingsHandler,
    ) {
        rl.clear_background(Color::from_hex("0b5e65").unwrap());

        let text_dimensions = get_text_size(
            font,
            MENU_TEXT,
            TEXT_SIZE * settings_handler.settings.pixel_scale as f32,
            TEXT_SPACING * settings_handler.settings.pixel_scale as f32,
        );

        rl.draw_text_pro(
            font,
            MENU_TEXT,
            Vector2::new(
                (SCREEN_WIDTH as f32 * settings_handler.settings.pixel_scale as f32
                    - text_dimensions.x)
                    / 2.,
                MENU_NAME_Y_OFFSET * settings_handler.settings.pixel_scale as f32,
            ),
            Vector2::zero(),
            0.0,
            TEXT_SIZE * settings_handler.settings.pixel_scale as f32,
            TEXT_SPACING * settings_handler.settings.pixel_scale as f32,
            Color::RAYWHITE,
        );

        let mouse_pos = rl.get_mouse_position()
            - Vector2::new(
                rl.get_screen_width() as f32 / 2.
                    - SCREEN_WIDTH as f32 / 2. * settings_handler.settings.pixel_scale as f32,
                rl.get_screen_height() as f32 / 2.
                    - SCREEN_HEIGHT as f32 / 2. * settings_handler.settings.pixel_scale as f32,
            );

        for i in 0..self.buttons.len() {
            let button = &mut self.buttons[i];

            let offset = if button.rec.check_collision_point_rec(mouse_pos)
                && i <= SaveHandler::get_level_number().into()
            {
                BUTTON_SIZE
            } else {
                0.
            };

            button.offset = lerp(button.offset, offset, 10. * rl.get_frame_time());

            let color = if i < COLUMNS_AMOUNT {
                Color::from_hex("0b8a8f").unwrap()
            } else {
                Color::from_hex("b33831").unwrap()
            };

            rl.draw_rectangle_rec(button.rec, color);

            let color = if i < COLUMNS_AMOUNT {
                let c = if i <= SaveHandler::get_level_number().into() {
                    Color::from_hex("30e1b9").unwrap()
                } else {
                    Color::from_hex("0b8a8f").unwrap()
                };

                c
            } else if i < COLUMNS_AMOUNT * 2 {
                let c = if i <= SaveHandler::get_level_number().into() {
                    Color::from_hex("f57d4a").unwrap()
                } else {
                    Color::from_hex("b33831").unwrap()
                };

                c
            } else {
                Color::from_hex("0b8a8f").unwrap()
            };

            rl.draw_rectangle_rec(
                Rectangle::new(
                    button.rec.x,
                    button.rec.y - button.offset,
                    button.rec.width,
                    button.rec.height,
                ),
                color,
            );

            let level_number_dimensions = get_text_size(
                font,
                format!("{}", i + 1).as_str(),
                TEXT_SIZE * settings_handler.settings.pixel_scale as f32,
                TEXT_SPACING * settings_handler.settings.pixel_scale as f32,
            );

            rl.draw_text_pro(
                font,
                format!("{}", i + 1).as_str(),
                Vector2::new(
                    button.rec.x + (button.rec.width - level_number_dimensions.x) / 2.,
                    button.rec.y - button.offset
                        + (button.rec.height - level_number_dimensions.y) / 2.,
                ),
                Vector2::zero(),
                0.0,
                TEXT_SIZE * settings_handler.settings.pixel_scale as f32,
                TEXT_SPACING * settings_handler.settings.pixel_scale as f32,
                Color::WHITE,
            );
        }

        let (offset, text_offset) = if self.back_button_rect.check_collision_point_rec(mouse_pos)
            && rl.is_mouse_button_up(MouseButton::MOUSE_BUTTON_LEFT)
        {
            (0., settings_handler.settings.pixel_scale as f32)
        } else {
            (BACK_BUTTON_SIZE_HEIGHT, 0.)
        };

        rl.draw_texture_pro(
            texture_handler.get(BACK_BUTTON_TEXTURE),
            Rectangle::new(0.0, offset, BACK_BUTTON_SIZE_WIDTH, BACK_BUTTON_SIZE_HEIGHT),
            self.back_button_rect,
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );

        let text_dimensions = get_text_size(
            font,
            BACK_BUTTON_TEXT,
            TEXT_SIZE * settings_handler.settings.pixel_scale as f32,
            TEXT_SPACING * settings_handler.settings.pixel_scale as f32,
        );

        rl.draw_text_pro(
            font,
            BACK_BUTTON_TEXT,
            Vector2::new(
                self.back_button_rect.x + (self.back_button_rect.width - text_dimensions.x) / 2.,
                self.back_button_rect.y + (self.back_button_rect.height - text_dimensions.y) / 2.
                    - text_offset,
            ),
            Vector2::zero(),
            0.0,
            TEXT_SIZE * settings_handler.settings.pixel_scale as f32,
            TEXT_SPACING * settings_handler.settings.pixel_scale as f32,
            Color::RAYWHITE,
        );
    }
}
