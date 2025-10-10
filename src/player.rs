use raylib::prelude::*;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct Player {
    position: Vector2,
    speed: f32,
}

impl Player {
    pub fn new() -> Self {
        return Self {
            position: Vector2::new((SCREEN_WIDTH / 2) as f32, (SCREEN_HEIGHT / 2) as f32),
            speed: 5.,
        };
    }

    pub fn update_position(&mut self, rl: &mut RaylibHandle) {
        let mut dir = Vector2::zero();

        if rl.is_key_down(KeyboardKey::KEY_D) {
            dir.x += 1.;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            dir.x -= 1.;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            dir.y += 1.;
        }
        if rl.is_key_down(KeyboardKey::KEY_W) {
            dir.y -= 1.;
        }

        self.position += dir.normalized() * self.speed;
    }

    pub fn draw(&self, rl: &mut RaylibDrawHandle) {
        rl.draw_circle_v(self.position, 25., Color::GREEN);
    }
}