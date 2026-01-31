use std::fs;

use raylib::prelude::*;

use crate::{
    SCREEN_HEIGHT, SCREEN_WIDTH,
    dialogue::DialogueHandler,
    enemy_spirit::EnemiesHandler,
    level_transition::LevelTransition,
    map::Level,
    metadata_handler::MetadataHandler,
    save_handler::SaveHandler,
    scene::{Scene, SceneHandler},
    settings::SettingsHandler,
    spirits_handler::SpiritsHandler,
    texture_handler::TextureHandler,
    ui::UIHandler,
};

const LEVEL_DIR: &str = "static/maps/";
const BUTTON_SIZE: f32 = 16.;

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

        let button_size_with_gap = BUTTON_SIZE + BUTTON_SIZE / 4.;
        let offset = SCREEN_WIDTH as f32 / 2. * pixel_scale as f32
            - button_size_with_gap / 2. * pixel_scale as f32 * 10.;

        for i in 0..level_count {
            buttons.push(Button {
                rec: Rectangle::new(
                    (i % 10) as f32 * button_size_with_gap * pixel_scale as f32 + offset,
                    SCREEN_HEIGHT as f32 / 2. * pixel_scale as f32
                        + (i / 10) as f32 * BUTTON_SIZE * pixel_scale as f32 * 1.25
                        - BUTTON_SIZE * pixel_scale as f32,
                    BUTTON_SIZE * pixel_scale as f32,
                    BUTTON_SIZE * pixel_scale as f32,
                ),
                offset: 0.0,
            });
        }

        Self {
            buttons,
            back_button_rect: Rectangle::new(
                (SCREEN_WIDTH / 2 * pixel_scale) as f32 - 32. * pixel_scale as f32,
                (SCREEN_HEIGHT * pixel_scale) as f32 - 32. * pixel_scale as f32,
                64. * pixel_scale as f32,
                16. * pixel_scale as f32,
            ),
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
    ) {
        let mouse_pos = rl.get_mouse_position()
            - Vector2::new(
                rl.get_screen_width() as f32 / 2.
                    - SCREEN_WIDTH as f32 / 2. * settings_handler.settings.pixel_scale as f32,
                rl.get_screen_height() as f32 / 2.
                    - SCREEN_HEIGHT as f32 / 2. * settings_handler.settings.pixel_scale as f32,
            );

        if self.back_button_rect.check_collision_point_rec(mouse_pos)
            && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
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

        rl.draw_text_pro(
            font,
            "Выбор уровня",
            Vector2::new(
                SCREEN_WIDTH as f32 / 2. * settings_handler.settings.pixel_scale as f32
                    - 24. * settings_handler.settings.pixel_scale as f32,
                20. * settings_handler.settings.pixel_scale as f32,
            ),
            Vector2::zero(),
            0.0,
            12. * settings_handler.settings.pixel_scale as f32,
            2.,
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
            let offset;

            let button = &mut self.buttons[i];

            if button.rec.check_collision_point_rec(mouse_pos)
                && i <= SaveHandler::get_level_number().into()
            {
                offset = 16.;
            } else {
                offset = 0.;
            }

            button.offset = lerp(button.offset, offset, 10. * rl.get_frame_time());

            let color = if i < 10 {
                Color::from_hex("0b8a8f").unwrap()
            } else {
                Color::from_hex("b33831").unwrap()
            };

            rl.draw_rectangle_rec(button.rec, color);

            let color = if i < 10 {
                let c = if i <= SaveHandler::get_level_number().into() {
                    Color::from_hex("30e1b9").unwrap()
                } else {
                    Color::from_hex("0b8a8f").unwrap()
                };

                c
            } else if i < 20 {
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

            rl.draw_text_pro(
                font,
                format!("{}", i + 1).as_str(),
                Vector2::new(
                    button.rec.x + 6. * settings_handler.settings.pixel_scale as f32,
                    button.rec.y - button.offset
                        + 3. * settings_handler.settings.pixel_scale as f32,
                ),
                Vector2::zero(),
                0.0,
                12. * settings_handler.settings.pixel_scale as f32,
                0.0,
                Color::WHITE,
            );
        }

        let offset;
        let text_offset;
        if self.back_button_rect.check_collision_point_rec(mouse_pos) {
            offset = 0.;
            text_offset = settings_handler.settings.pixel_scale as f32;
        } else {
            offset = 16.;
            text_offset = 0.;
        };

        rl.draw_texture_pro(
            texture_handler.get("main_menu_buttons"),
            Rectangle::new(0.0, offset, 64., 16.),
            self.back_button_rect,
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );

        rl.draw_text_pro(
            font,
            "Назад",
            Vector2::new(
                self.back_button_rect.x + self.back_button_rect.width / 2.
                    - 6. * 2. * settings_handler.settings.pixel_scale as f32,
                self.back_button_rect.y + self.back_button_rect.height / 2.
                    - 6. * settings_handler.settings.pixel_scale as f32
                    - text_offset,
            ),
            Vector2::zero(),
            0.0,
            12. * settings_handler.settings.pixel_scale as f32,
            2.,
            Color::RAYWHITE,
        );
    }
}
