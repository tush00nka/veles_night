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
    level1.tiles[2][2] = TileType::Tree;
    level1.tiles[3][2] = TileType::Tree;
    level1.tiles[4][2] = TileType::Tree;
    level1.tiles[5][2] = TileType::Tree;
    level1.tiles[6][2] = TileType::Tree;

    level1.tiles[10][3] = TileType::Tree;
    level1.tiles[10][4] = TileType::Tree;
    level1.tiles[10][5] = TileType::Tree;
    level1.tiles[10][6] = TileType::Tree;

    while !rl.window_should_close() {
        // update stuff
        player.update_position(&level1, &mut rl);
        player.put_campfire(&mut level1, &mut rl);

        // draw stuff
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);

        player.draw(&mut d);
        level1.draw(&mut d);
    }
}
