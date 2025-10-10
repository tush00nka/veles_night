use raylib::prelude::*;

use crate::{
    map::{LevelMap, TileType},
    player::Player,
};

mod light;
mod map;
mod player;

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .resizable()
        .title("Велесова Ночь")
        .build();

    rl.set_target_fps(60);

    let mut player = Player::new();

    let mut level1 = LevelMap::new();

    while !rl.window_should_close() {
        // update stuff
        player.update_position(&mut rl);
        player.put_campfire(&mut level1, &mut rl);

        // draw stuff
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);

        player.draw(&mut d);
        level1.draw(&mut d);
    }
}
