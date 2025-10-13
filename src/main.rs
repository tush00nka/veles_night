use std::collections::HashMap;

use raylib::prelude::*;

use crate::{
    map::{LevelMap, TILE_SIZE, TileType},
    order::OrderHandler,
    player::Player,
    spirit::Spirit,
    texture_handler::TextureHandler,
};

// mod light;
mod map;
mod order;
mod player;
mod spirit;
mod texture_handler;

const SCREEN_WIDTH: i32 = 16 * 16 * 4;
const SCREEN_HEIGHT: i32 = 16 * 9 * 4;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .resizable()
        .title("Велесова Ночь")
        .build();

    rl.set_target_fps(60);

    let texture_handler = TextureHandler::new(&mut rl, &thread);
    //there're safe variants - get_safe/get_mut_safe
    //also common ones - get and get_mut

    let mut player = Player::new();

    let mut level1 = LevelMap::new();
    // todo: remove this hardcoded mess
    level1.tiles[2][2] = TileType::Tree;
    level1.tiles[3][2] = TileType::Tree;
    level1.tiles[4][2] = TileType::Tree;
    level1.tiles[5][2] = TileType::Tree;
    // level1.tiles[6][2] = TileType::Tree;

    level1.tiles[10][2] = TileType::Tree;
    level1.tiles[10][3] = TileType::Tree;
    level1.tiles[10][4] = TileType::Tree;
    level1.tiles[10][5] = TileType::Tree;
    level1.tiles[10][6] = TileType::Tree;

    let mut spirits: HashMap<usize, Spirit> = HashMap::new();

    let mut order_handler = OrderHandler::new();

    let mut timer = 0.0;

    for i in 0..1 {
        spirits.insert(
            i,
            Spirit::new(Vector2::new(
                (7. + (i % 3) as f32) * TILE_SIZE as f32 - i as f32 * 10.,
                2. * TILE_SIZE as f32,
            )),
        );
    }

    while !rl.window_should_close() {
        // update stuff
        // player.update_position(&level1, &mut rl);
        player.put_campfire(&mut level1, &mut rl);

        // this is such a cool function fr fr tbh lowkey
        spirits.retain(|_, spirit| !spirit.get_dead());

        for spirit in spirits.values_mut() {
            spirit.update_behaviour(&mut level1, &mut timer, &mut order_handler, &mut rl);
        }

        order_handler.select_spirit(&mut spirits, &mut level1, &rl);
        order_handler.update_line(&level1, &rl);

        // draw stuff
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::from_hex("0b8a8f").unwrap());

        // player.draw(&mut d);
        // player.draw_line(&mut d);
        level1.draw(&mut d, &texture_handler);
        for spirit in spirits.values() {
            spirit.draw(&mut d, &texture_handler);
        }
        order_handler.draw(&spirits, &mut d);
        order_handler.draw_ui(&mut d);
    }
}
