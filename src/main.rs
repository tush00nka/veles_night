use std::collections::HashMap;

use raylib::prelude::*;

use crate::{
    map::{LevelMap, TileType, TILE_SIZE}, order::OrderHandler, spirit::Spirit, spirits_handler::SpiritsHandler, texture_handler::TextureHandler, ui::UIHandler
};

// mod light;

mod map_loader;
mod light;
mod map;
mod order;
mod spirit;
mod texture_handler;
mod ui;
mod metadata_handler;
mod spirits_handler;

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
    
    let metadata_handler = metadata_handler::MetadataHandler::load(1);

    let mut level1 = LevelMap::new();
       
    map_loader::MapLoader::get_map(1, &mut level1, metadata_handler.clone());

    let mut spirits_handler = SpiritsHandler::new();
    spirits_handler.spawn_spirits(metadata_handler);
    
    let mut order_handler = OrderHandler::new();
    let mut ui_handler = UIHandler::new();
    
    while !rl.window_should_close() {
        // update stuff

        // this is such a cool function fr fr tbh lowkey
        spirits_handler.spirits.retain(|_, spirit| !spirit.get_dead());

        for spirit in spirits_handler.spirits.values_mut() {
            spirit.update_behaviour(&mut level1, &mut order_handler, &mut rl);
        }

        order_handler.select_spirit(&mut spirits_handler.spirits, &mut level1, &rl);
        order_handler.update_line(&level1, &rl);

        ui_handler.build(&mut order_handler, &mut level1, &mut rl);

        // draw stuff
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::from_hex("0b8a8f").unwrap());

        level1.draw(&mut d, &texture_handler);
        for spirit in spirits_handler.spirits.values() {
            spirit.draw(&mut d, &texture_handler);
        }
        order_handler.draw(&spirits_handler.spirits, &mut d);
        order_handler.draw_ui(&mut d);
        ui_handler.draw(&texture_handler, &mut d);
    }
}
