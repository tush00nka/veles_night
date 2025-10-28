use std::fs;

use raylib::prelude::*;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, map::TILE_SCALE};

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

    pub fn draw(&mut self, rl: &mut RaylibDrawHandle) {
        rl.clear_background(Color::from_hex("0eaf9b").unwrap());

        for button in self.buttons.iter_mut() {
            let offset;

            if button
                .rec
                .check_collision_point_rec(rl.get_mouse_position())
            {
                offset = 10.;
            } else {
                offset = 0.;
            }

            fn lerp(a: f32, b: f32, f: f32) -> f32 {
                a * (1. - f) + (b * f)
            }

            button.offset = lerp(button.offset, offset, 10. * rl.get_frame_time());

            rl.draw_rectangle_rec(button.rec, Color::from_hex("0b8a8f").unwrap());

            rl.draw_rectangle_rec(
                Rectangle::new(
                    button.rec.x,
                    button.rec.y - button.offset,
                    button.rec.width,
                    button.rec.height,
                ),
                Color::from_hex("30e1b9").unwrap(),
            );
        }
    }
}
