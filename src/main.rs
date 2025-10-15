use raylib::prelude::*;

use crate::{
    level_transition::LevelTransition,
    map::Level,
    metadata_handler::MetadataHandler,
    order::OrderHandler,
    scene::{Scene, SceneHandler},
    spirit::Spirit,
    spirits_handler::SpiritsHandler,
    texture_handler::TextureHandler,
    ui::UIHandler,
};

// mod light;

mod level_transition;
mod map;
mod map_loader;
mod metadata_handler;
mod order;
mod scene;
mod spirit;
mod spirits_handler;
mod swamp;
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

    let font = rl
        .load_font_ex(
            &thread,
            "static/fonts/nizhegorodsky.ttf",
            400,
            Some("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzАБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯабвгдеёжзийклмнопрстуфхцчшщъыьэюя0123456789+-%[](),.:!?/"),
        )
        .expect("no font???");

    let mut scene_handler = SceneHandler::new();

    let texture_handler = TextureHandler::new(&mut rl, &thread);
    // there's a safe variation - get_safe
    // also a common one - get

    let mut level_number = 0;

    let mut level = Level::new();
    let mut metadata_handler = MetadataHandler::new(level_number);
    level.load(level_number, &mut metadata_handler);

    let mut spirits_handler = SpiritsHandler::new();
    spirits_handler.spawn_spirits(&mut metadata_handler);

    let mut order_handler = OrderHandler::new();
    let mut ui_handler = UIHandler::new();

    let mut level_transition = LevelTransition::new();

    while !rl.window_should_close() {
        // update stuff

        match scene_handler.get_current() {
            Scene::MainMenu => update_main_menu(&mut scene_handler, &mut rl),
            Scene::Level => update_level(
                &mut spirits_handler,
                &mut level,
                &mut order_handler,
                &mut ui_handler,
                &mut scene_handler,
                &mut rl,
            ),
            Scene::Transition => update_transition(
                &mut level_transition,
                &mut level_number,
                &mut metadata_handler,
                &mut level,
                &mut scene_handler,
                &mut spirits_handler,
                &mut rl,
            ),
        }

        // draw stuff
        let mut d = rl.begin_drawing(&thread);

        match scene_handler.get_current() {
            Scene::MainMenu => draw_main_menu(&font, &mut d),
            Scene::Level => draw_level(
                &mut level,
                &texture_handler,
                &mut spirits_handler,
                &mut order_handler,
                &mut ui_handler,
                &font,
                &mut d,
            ),
            Scene::Transition => {
                draw_transition(&texture_handler, &font, &mut level_transition, &mut d)
            }
        }
    }
}

fn update_main_menu(scene_handler: &mut SceneHandler, rl: &mut RaylibHandle) {
    if rl.is_key_pressed(KeyboardKey::KEY_ENTER)
        || rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
    {
        scene_handler.set(scene::Scene::Level);
    }
}

fn draw_main_menu(font: &Font, rl: &mut RaylibDrawHandle) {
    rl.clear_background(Color::from_hex("0b8a8f").unwrap());

    rl.draw_text_pro(
        font,
        "ВЕЛЕСОВА НОЧЬ",
        Vector2::new(
            (SCREEN_WIDTH / 2) as f32 - 15. * 10.,
            (SCREEN_HEIGHT / 2) as f32 - 32.,
        ),
        Vector2::zero(),
        0.0,
        64.,
        2.0,
        Color::RAYWHITE,
    );
    rl.draw_text_pro(
        font,
        "ENTER чтобы начать",
        Vector2::new(
            (SCREEN_WIDTH / 2) as f32 - 155.,
            (SCREEN_HEIGHT / 2) as f32 + 32.,
        ),
        Vector2::zero(),
        0.0,
        48.,
        2.0,
        Color::RAYWHITE,
    );
}

fn update_level(
    spirits_handler: &mut SpiritsHandler,
    level: &mut Level,
    order_handler: &mut OrderHandler,
    ui_handler: &mut UIHandler,
    scene_handler: &mut SceneHandler,
    rl: &mut RaylibHandle,
) {
    // this is such a cool function fr fr tbh lowkey
    spirits_handler
        .spirits
        .retain(|_, spirit| !spirit.get_dead());

    for spirit in spirits_handler.spirits.values_mut() {
        spirit.update_behaviour(level, rl);
    }

    order_handler.select_spirit(spirits_handler, level, rl);
    order_handler.update_line(level, rl);

    ui_handler.build(level, rl);

    level.update(scene_handler);
}

fn draw_level(
    level: &mut Level,
    texture_handler: &TextureHandler,
    spirits_handler: &mut SpiritsHandler,
    order_handler: &mut OrderHandler,
    ui_handler: &mut UIHandler,
    font: &Font,
    rl: &mut RaylibDrawHandle,
) {
    rl.clear_background(Color::from_hex("0b8a8f").unwrap());

    level.draw(rl, texture_handler);
    for spirit in spirits_handler.spirits.values() {
        spirit.draw(rl, texture_handler);
    }
    order_handler.draw(spirits_handler, rl);
    ui_handler.draw(texture_handler, level, font, rl);
}

fn update_transition(
    level_transition: &mut LevelTransition,
    level_number: &mut u8,
    metadata_handler: &mut MetadataHandler,
    level: &mut Level,
    scene_handler: &mut SceneHandler,
    spirits_handler: &mut SpiritsHandler,
    rl: &mut RaylibHandle,
) {
    if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
        *level_number += 1;
        level_transition.set_cards(*level_number as usize);
        metadata_handler.load(*level_number);
        level.load(*level_number, metadata_handler);
        spirits_handler.spawn_spirits(metadata_handler);
        scene_handler.set(Scene::Level);
    }
}

fn draw_transition(
    texture_handler: &TextureHandler,
    font: &Font,
    level_transition: &mut LevelTransition,
    rl: &mut RaylibDrawHandle,
) {
    level_transition.draw(texture_handler, font, rl);
    rl.draw_text_pro(
        font,
        "this is level transition scene",
        Vector2::zero(),
        Vector2::zero(),
        0.0,
        24.,
        0.0,
        Color::RAYWHITE,
    );
}
