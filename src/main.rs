use raylib::prelude::*;

use crate::light::{setup_light, LightInfo, MAX_LIGHTS};

mod light;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

pub struct Player {
    position: Vector2,
    speed: f32,
}

impl Player {
    fn new() -> Self {
        return Self {
            position: Vector2::zero(),
            speed: 5.
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .resizable()
        .title("Велесова Ночь")
        .build();

    rl.set_target_fps(60);

    let mut player = Player::new();

    // let mut lights: [LightInfo; MAX_LIGHTS] = [LightInfo::default(); MAX_LIGHTS];

    // setup_light(&mut lights, 0, 600., 400., 300., &mut d, &thread);

    while !rl.window_should_close() {

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

        player.position += dir.normalized() * player.speed;

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);
        // d.draw_rectangle_v(player.position, Vector2::one() * 50., Color::GREEN);
        d.draw_circle_v(player.position, 25., Color::GREEN);
    }
}
