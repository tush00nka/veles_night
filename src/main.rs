use raylib::prelude::*;

use crate::{
    map::LevelMap,
    order::OrderHandler,
    scene::{Scene, SceneHandler},
    spirit::Spirit,
    spirits_handler::SpiritsHandler,
    texture_handler::TextureHandler,
    ui::UIHandler,
};

// mod light;

mod light;
mod map;
mod map_loader;
mod metadata_handler;
mod order;
mod scene;
mod spirit;
mod spirits_handler;
mod texture_handler;
mod ui;

const SCREEN_WIDTH: i32 = 16 * 16 * 4;
const SCREEN_HEIGHT: i32 = 16 * 9 * 4;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .resizable()
        .title("Велесова Ночь")
        .build();

    rl.set_target_fps(60);

    let mut scene_handler = SceneHandler::new();

    let texture_handler = TextureHandler::new(&mut rl, &thread);
    //there're safe variants - get_safe/get_mut_safe
    //also common ones - get and get_mut

    let level_number = 0;

    let metadata_handler = metadata_handler::MetadataHandler::load(level_number);
    let mut level = LevelMap::new();

    map_loader::MapLoader::get_map(level_number, &mut level);

    let mut spirits_handler = SpiritsHandler::new();
    spirits_handler.spawn_spirits(metadata_handler);

    let mut order_handler = OrderHandler::new();
    let mut ui_handler = UIHandler::new();

    while !rl.window_should_close() {
        // update stuff

        match scene_handler.get_current() {
            Scene::MainMenu => update_main_menu(&mut scene_handler, &mut rl),
            Scene::Level => update_level(
                &mut spirits_handler,
                &mut level,
                &mut order_handler,
                &mut ui_handler,
                &mut rl,
            ),
            Scene::Transition => update_transition(),
        }

        // draw stuff
        let mut d = rl.begin_drawing(&thread);

        match scene_handler.get_current() {
            Scene::MainMenu => draw_main_menu(&mut d),
            Scene::Level => draw_level(
                &mut level,
                &texture_handler,
                &mut spirits_handler,
                &mut order_handler,
                &mut ui_handler,
                &mut d,
            ),
            Scene::Transition => todo!("can't trasition between levels yet!"),
        }
    }
}

fn update_main_menu(scene_handler: &mut SceneHandler, rl: &mut RaylibHandle) {
    if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
        scene_handler.set(scene::Scene::Level);
    }
}

fn draw_main_menu(rl: &mut RaylibDrawHandle) {
    rl.clear_background(Color::from_hex("0b8a8f").unwrap());
    rl.draw_text(
        "VELES NIGHT\npress ENTER to begin",
        10,
        10,
        28,
        Color::RAYWHITE,
    );
}

fn update_level(
    spirits_handler: &mut SpiritsHandler,
    level: &mut LevelMap,
    order_handler: &mut OrderHandler,
    ui_handler: &mut UIHandler,
    rl: &mut RaylibHandle,
) {
    // this is such a cool function fr fr tbh lowkey
    spirits_handler
        .spirits
        .retain(|_, spirit| !spirit.get_dead());

    for spirit in spirits_handler.spirits.values_mut() {
        spirit.update_behaviour(level, order_handler, rl);
    }

    order_handler.select_spirit(spirits_handler, level, rl);
    order_handler.update_line(level, rl);

    ui_handler.build(order_handler, level, rl);
}

fn draw_level(
    level: &mut LevelMap,
    texture_handler: &TextureHandler,
    spirits_handler: &mut SpiritsHandler,
    order_handler: &mut OrderHandler,
    ui_handler: &mut UIHandler,
    rl: &mut RaylibDrawHandle,
) {
    rl.clear_background(Color::from_hex("0b8a8f").unwrap());

    level.draw(rl, texture_handler);
    for spirit in spirits_handler.spirits.values() {
        spirit.draw(rl, texture_handler);
    }
    order_handler.draw(spirits_handler, rl);
    order_handler.draw_ui(rl);
    ui_handler.draw(texture_handler, rl);
}

fn update_transition() {
    todo!("no level transition behaviour")
}
