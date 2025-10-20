use raylib::prelude::*;

use crate::{
    gameover_handler::GameOverHandler, hotkey_handler::{HotkeyCategory, HotkeyHandler, HotkeyLoaderStruct}, level_transition::LevelTransition, main_menu::MainMenuHandler, map::{Level, TILE_SIZE}, metadata_handler::MetadataHandler, music_handler::MusicHandler, order::OrderHandler, particle::Particle, scene::{Scene, SceneHandler}, spirit::Spirit, spirits_handler::SpiritsHandler, texture_handler::TextureHandler, ui::UIHandler
};

// mod light;

mod gameover_handler;
mod hotkey_handler;
mod level_transition;
mod map;
mod map_loader;
mod metadata_handler;
mod music_handler;
mod order;
mod particle;
mod scene;
mod spirit;
mod spirits_handler;
mod swamp;
mod texture_handler;
mod ui;
mod main_menu;

pub const FIRST_LEVEL: u8 = 0;

const SCREEN_WIDTH: i32 = 16 * 16 * 4;
const SCREEN_HEIGHT: i32 = 16 * 9 * 4;
const MAX_LEVEL: u8 = 5; //ЗАТЫЧКА, ПЕРЕДЕЛАТЬ
//
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .resizable()
        .title("Велесова Ночь")
        .build();

    rl.set_target_fps(60);

    rl.set_exit_key(None);

    let mut rl_audio = RaylibAudio::init_audio_device().unwrap();
    let music_handler = MusicHandler::new(&mut rl_audio);
    music_handler.music_play();

    //let audio = raylib::core::audio::RaylibAudio::init_audio_device().unwrap();

    // let death_sound = audio.new_sound("static/audio/death.ogg").unwrap();
    // death_sound.play();

    let font = rl
        .load_font_ex(
            &thread,
            "static/fonts/nizhegorodsky.ttf",
            400,
            Some("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzАБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯабвгдеёжзийклмнопрстуфхцчшщъыьэюя0123456789+-%[](),.:!?/"),
        )
        .expect("no font???");

    let mut scene_handler = SceneHandler::new();

    let hotkey_loader_struct = HotkeyLoaderStruct::new();
    let mut hotkey_handler = HotkeyHandler::new(hotkey_loader_struct);

    let texture_handler = TextureHandler::new(&mut rl, &thread);
    // there's a safe variation - get_safe
    // also a common one - get

    let mut main_menu = MainMenuHandler::new();

    let args: Vec<String> = std::env::args().collect();

    let level_num = if args.len() > 1 {
        let Ok(level_num) = args[1].parse::<u8>() else {
            panic!("wrong cmd arg")
        };

        level_num
    } else {
        FIRST_LEVEL
    };

    let mut level_number = level_num;

    let mut level = Level::new();
    let mut metadata_handler = MetadataHandler::new(level_number);
    level.load(level_number, &mut metadata_handler, &mut rl);

    let mut spirits_handler = SpiritsHandler::new();
    spirits_handler.spawn_spirits(&mut metadata_handler);

    let mut order_handler = OrderHandler::new();
    let mut ui_handler = UIHandler::new(level_number as usize);
    let mut gameover_handler = GameOverHandler::new(gameover_handler::GameOverHandlerType::Level);

    let mut should_close = false;

    let mut gameend_handler = GameOverHandler::new(gameover_handler::GameOverHandlerType::Game);

    let mut level_transition = LevelTransition::new();

    let mut particles: Vec<Particle> = vec![];

    let mut shader = rl.load_shader(&thread, None, Some("static/shaders/bloom.fs"));

    while !rl.window_should_close() && !should_close {
        // update stuff

        music_handler.music_update();

        particles.retain(|particle| !particle.done);

        for particle in particles.iter_mut() {
            particle.update(&mut rl);
        }

        match scene_handler.get_current() {
            Scene::MainMenu => {
                rl.set_window_title(&thread, "Велесова Ночь");
                main_menu.update(&mut scene_handler, &mut should_close, &mut rl)
            }
            Scene::GameEnd => {
                rl.set_window_title(&thread, "Велесова Ночь - Победа");

                gameend_handler.update_gameover(
                    &mut level_number,
                    &mut rl,
                    &mut scene_handler,
                    &music_handler,
                    &mut hotkey_handler,
                    &mut should_close,
                );
            }
            Scene::GameOver => {
                music_handler.music_pause();

                rl.set_window_title(&thread, "Велесова Ночь - Поражение");

                if gameover_handler.update_gameover(
                    &mut level_number,
                    &mut rl,
                    &mut scene_handler,
                    &music_handler,
                    &mut hotkey_handler,
                    &mut should_close,
                ) {
                    music_handler.music_resume();
                    reload_procedure(
                        level_number as u8,
                        &mut level,
                        &mut metadata_handler,
                        &mut spirits_handler,
                        &mut rl,
                    );
                }
            }
            Scene::Level => {
                rl.set_window_title(
                    &thread,
                    format!("Велесова Ночь - Уровень {}", level_number + 1).as_str(),
                );

                if update_level(
                    &mut spirits_handler,
                    &mut particles,
                    &mut level,
                    &mut order_handler,
                    &mut ui_handler,
                    &mut scene_handler,
                    &music_handler,
                    &mut rl,
                    &mut hotkey_handler,
                ) {
                    reload_procedure(
                        level_number,
                        &mut level,
                        &mut metadata_handler,
                        &mut spirits_handler,
                        &mut rl,
                    );
                }
            }
            Scene::Transition => update_transition(
                &mut level_transition,
                &mut level_number,
                &mut metadata_handler,
                &mut level,
                &mut scene_handler,
                &mut spirits_handler,
                &mut rl,
                &mut hotkey_handler,
                &mut ui_handler,
            ),
        }

        // {
        //     let mut tm = rl.begin_texture_mode(&thread, &mut target);
        //     let mut t = tm.begin_drawing(&thread);

        //     // rl.begin_texture_mode(&thread, &mut target)
        //     // .draw(&thread, |mut t| {

        //     // });
        // }
        // draw stuff
        let mut d = rl.begin_drawing(&thread);

        {
            let mut s = d.begin_shader_mode(&mut shader);
            match scene_handler.get_current() {
                // Scene::MainMenu => draw_main_menu(&font, &texture_handler, &mut s),
                // Scene::GameEnd => gameend_handler.draw_gameover(&font, &mut s),
                // Scene::GameOver => gameover_handler.draw_gameover(&font, &mut s),
                Scene::Level => draw_level(
                    &mut level,
                    &texture_handler,
                    &mut spirits_handler,
                    &mut order_handler,
                    &mut s,
                ),
                // Scene::Transition => {
                    // draw_transition(&texture_handler, &font, &mut level_transition, &mut s)
                // }
                _ => {}
            }

            for particle in particles.iter_mut() {
                particle.draw(&mut s);
            }
        }

        match scene_handler.get_current() {
            Scene::MainMenu => main_menu.draw(&font, &texture_handler, &mut d),
            Scene::GameEnd => gameend_handler.draw_gameover(&font, &mut d),
            Scene::GameOver => gameover_handler.draw_gameover(&font, &mut d),
            Scene::Level => {
                draw_level_ui(&mut level, &texture_handler, &mut ui_handler, &font, &mut d)
            }
            Scene::Transition => {
                draw_transition(&texture_handler, &font, &mut level_transition, &mut d)
            }
        }

        d.draw_fps(0, 0);

        // match scene_handler.get_current() {
        //     Scene::MainMenu => draw_main_menu(&font, &texture_handler, &mut d),
        //     Scene::GameEnd => gameend_handler.draw_gameover(&font, &mut d),
        //     Scene::GameOver => gameover_handler.draw_gameover(&font, &mut d),
        //     Scene::Level => draw_level(
        //         &mut level,
        //         &texture_handler,
        //         &mut spirits_handler,
        //         &mut order_handler,
        //         &mut ui_handler,
        //         &font,
        //         &mut d,
        //     ),
        //     Scene::Transition => {
        //         draw_transition(&texture_handler, &font, &mut level_transition, &mut d)
        //     }
        // }

        // for particle in particles.iter_mut() {
        //     particle.draw(&mut d);
        // }
    }
}

fn update_main_menu(
    scene_handler: &mut SceneHandler,
    rl: &mut RaylibHandle,
    hotkey_handler: &mut HotkeyHandler,
) {
    if hotkey_handler.check_pressed(rl, HotkeyCategory::Continue)
        || rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
    {
        scene_handler.set(scene::Scene::Level);
    }
}

fn update_level(
    spirits_handler: &mut SpiritsHandler,
    particles: &mut Vec<Particle>,
    level: &mut Level,
    order_handler: &mut OrderHandler,
    ui_handler: &mut UIHandler,
    scene_handler: &mut SceneHandler,
    music_handler: &MusicHandler,
    rl: &mut RaylibHandle,
    hotkey_handler: &mut HotkeyHandler,
) -> bool {
    for spirit in spirits_handler.spirits.values() {
        if spirit.get_dead() {
            particles.push(Particle::new(
                Vector2::new(
                    spirit.get_position().x + TILE_SIZE as f32 / 2.,
                    spirit.get_draw_position().y + TILE_SIZE as f32 / 2.,
                ),
                16,
                32.,
                5.,
            ));
        }
    }

    // this is such a cool function fr fr tbh lowkey
    spirits_handler
        .spirits
        .retain(|_, spirit| !spirit.get_dead());

    for spirit in spirits_handler.spirits.values_mut() {
        spirit.update_behaviour(level, music_handler, rl);
    }

    order_handler.select_spirit(spirits_handler, level, rl, hotkey_handler);
    order_handler.update_line(level, rl, hotkey_handler);

    ui_handler.build(level, rl, hotkey_handler);

    level.update(
        scene_handler,
        spirits_handler.spirits.len() as u8,
        music_handler,
    );

    if hotkey_handler.check_pressed(rl, HotkeyCategory::Reset) {
        return true;
    }
    return false;
}

fn draw_level(
    level: &mut Level,
    texture_handler: &TextureHandler,
    spirits_handler: &mut SpiritsHandler,
    order_handler: &mut OrderHandler,
    rl: &mut RaylibDrawHandle,
) {
    rl.clear_background(Color::from_hex("0b8a8f").unwrap());

    level.draw(rl, texture_handler);
    for spirit in spirits_handler.spirits.values() {
        spirit.draw(rl, texture_handler);
    }
    order_handler.draw(spirits_handler, rl);
}

fn draw_level_ui(
    level: &mut Level,
    texture_handler: &TextureHandler,
    ui_handler: &mut UIHandler,
    font: &Font,
    rl: &mut RaylibDrawHandle,
) {
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
    hotkey_handler: &mut HotkeyHandler,
    ui_handler: &mut UIHandler,
) {
    if !hotkey_handler.check_pressed(rl, HotkeyCategory::Continue)
        && !rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT)
    {
        return;
    }

    *level_number += 1;
    if *level_number > MAX_LEVEL {
        scene_handler.set(Scene::GameEnd);
        return;
    }

    level_transition.set_cards(*level_number as usize);
    metadata_handler.load(*level_number);
    level.load(*level_number, metadata_handler, rl);
    spirits_handler.spawn_spirits(metadata_handler);
    scene_handler.set(Scene::Level);
    *ui_handler = UIHandler::new(level_number.clone() as usize);
}

fn draw_transition(
    texture_handler: &TextureHandler,
    font: &Font,
    level_transition: &mut LevelTransition,
    rl: &mut RaylibDrawHandle,
) {
    level_transition.draw(texture_handler, font, rl);
    // rl.draw_text_pro(
    //     font,
    //     "this is level transition scene",
    //     Vector2::zero(),
    //     Vector2::zero(),
    //     0.0,
    //     24.,
    //     0.0,
    //     Color::RAYWHITE,
    // );
}

fn reload_procedure(
    current_level: u8,
    level: &mut Level,
    metadata_handler: &mut MetadataHandler,
    spirits_handler: &mut SpiritsHandler,
    rl: &mut RaylibHandle,
) {
    *level = Level::new();
    *metadata_handler = MetadataHandler::new(current_level);
    level.load(current_level, metadata_handler, rl);

    *spirits_handler = SpiritsHandler::new();
    spirits_handler.spawn_spirits(metadata_handler);
}
