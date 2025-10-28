use std::fs;

use raylib::prelude::*;

use crate::{
    SCREEN_HEIGHT, SCREEN_WIDTH,
    enemy_spirit::EnemiesHandler,
    level_transition::LevelTransition,
    map::{Level, TILE_SCALE},
    metadata_handler::MetadataHandler,
    scene::{Scene, SceneHandler},
    spirits_handler::SpiritsHandler,
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
}

impl LevelSelector {
    pub fn new() -> Self {
        let dir = fs::read_dir(LEVEL_DIR);

        let level_count;
        match dir {
            Ok(d) => level_count = d.count(),
            Err(e) => panic!("Failed to read level dir: {}", e),
        }

        let mut buttons = vec![];

        let button_size_with_gap = BUTTON_SIZE + BUTTON_SIZE / 4.;
        let offset = SCREEN_WIDTH as f32 / 2.
            - button_size_with_gap / 2. * TILE_SCALE as f32 * level_count as f32;

        for i in 0..level_count {
            buttons.push(Button {
                rec: Rectangle::new(
                    i as f32 * button_size_with_gap * TILE_SCALE as f32 + offset,
                    SCREEN_HEIGHT as f32 / 2. - BUTTON_SIZE * TILE_SCALE as f32 / 2.,
                    BUTTON_SIZE * TILE_SCALE as f32,
                    BUTTON_SIZE * TILE_SCALE as f32,
                ),
                offset: 0.0,
            });
        }

        Self { buttons }
    }

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
        rl: &mut RaylibHandle,
    ) {
        let mouse_pos = rl.get_mouse_position()
            - Vector2::new(
                rl.get_screen_width() as f32 / 2. - SCREEN_WIDTH as f32 / 2.,
                rl.get_screen_height() as f32 / 2. - SCREEN_HEIGHT as f32 / 2.,
            );

        for i in 0..self.buttons.len() {
            if self.buttons[i].rec.check_collision_point_rec(mouse_pos) {
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                    *level_number = i as u8;
                    metadata_handler.load(*level_number);
                    level.load(*level_number, metadata_handler, rl);
                    spirits_handler.spawn_spirits(metadata_handler);
                    enemies_handler.spawn_enemies(metadata_handler);
                    *ui_handler = UIHandler::new(i);
                    *level_transition = LevelTransition::new();
                    scene_handler.set(Scene::Level);
                }
            }
        }
    }

    pub fn draw(&mut self, font: &Font, rl: &mut RaylibDrawHandle) {
        rl.clear_background(Color::from_hex("0eaf9b").unwrap());

        let mouse_pos = rl.get_mouse_position()
            - Vector2::new(
                rl.get_screen_width() as f32 / 2. - SCREEN_WIDTH as f32 / 2.,
                rl.get_screen_height() as f32 / 2. - SCREEN_HEIGHT as f32 / 2.,
            );

        for i in 0..self.buttons.len() {
            let offset;

            let button = &mut self.buttons[i];

            if button.rec.check_collision_point_rec(mouse_pos) {
                offset = 16.;
            } else {
                offset = 0.;
            }

            fn lerp(a: f32, b: f32, f: f32) -> f32 {
                a * (1. - f) + (b * f)
            }

            button.offset = lerp(button.offset, offset, 10. * rl.get_frame_time());

            rl.draw_rectangle_rec(button.rec, Color::from_hex("0b8a8f").unwrap());

            // let pp =
            // ((button.rec.y - button.offset) / TILE_SCALE as f32).floor() * TILE_SCALE as f32;
            rl.draw_rectangle_rec(
                Rectangle::new(
                    button.rec.x,
                    button.rec.y - button.offset,
                    button.rec.width,
                    button.rec.height,
                ),
                Color::from_hex("30e1b9").unwrap(),
            );

            rl.draw_text_pro(
                font,
                format!("{}", i+1).as_str(),
                Vector2::new(button.rec.x + 6. * TILE_SCALE as f32, button.rec.y - button.offset + 3. * TILE_SCALE as f32),
                Vector2::zero(),
                0.0,
                12. * TILE_SCALE as f32,
                0.0,
                Color::RAYWHITE,
            );
        }
    }
}
