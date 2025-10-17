use std::f32::consts::PI;

use raylib::prelude::*;

pub struct Particle {
    start_position: Vector2,
    positions: Vec<Vector2>,
    amount: usize,
    radius: f32,
    speed: f32,
    pub done: bool
}

impl Particle {
    pub fn new(position: Vector2, amount: usize, radius: f32, speed: f32) -> Self {
        let positions = vec![position; amount];
        Self {
            start_position: position,
            positions,
            amount,
            radius,
            speed,
            done: false,
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        for i in 0..self.positions.len() {
            let position = self.positions.get_mut(i).unwrap();

            let angle = 2. * PI / self.amount as f32 * i as f32;
            let destination = Vector2::new(
                angle.cos() * self.radius,
                angle.sin() * self.radius,
            );

            *position = position.lerp(self.start_position + destination, self.speed * rl.get_frame_time());
            self.done = position.distance_to(self.start_position + destination) <= 1.;
        }        
    }

    pub fn draw(&self, rl: &mut RaylibDrawHandle) {
        for position in self.positions.iter() {
            rl.draw_circle_v(position, 4. + (position.x as i32 % 6) as f32, Color::WHITE);
        }
    }
}
