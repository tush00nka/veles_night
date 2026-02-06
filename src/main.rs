use raylib::{
    ffi::{GetCurrentMonitor, GetMonitorHeight, GetMonitorWidth},
    prelude::*,
};

use crate::{
    dialogue::DialogueHandler,
    enemy_spirit::EnemiesHandler,
    gameover_handler::GameOverHandler,
    hotkey_handler::{HotkeyCategory, HotkeyHandler, HotkeyLoaderStruct},
    level_selection::LevelSelector,
    level_transition::LevelTransition,
    main_menu::MainMenuHandler,
    map::{Level, TILE_SIZE_PX},
    metadata_handler::MetadataHandler,
    music_handler::MusicHandler,
    order::OrderHandler,
    particle::Particle,
    save_handler::SaveHandler,
    scene::{Scene, SceneHandler},
    settings::SettingsHandler,
    settings_menu::SettingsMenuHandler,
    spirit::Spirit,
    spirits_handler::SpiritsHandler,
    texture_handler::TextureHandler,
    ui::UIHandler,
};

// mod light;

mod dialogue;
mod enemy_spirit;
mod gameover_handler;
mod hotkey_handler;
mod level_selection;
mod level_transition;
mod main_menu;
mod settings_menu;

mod map;
mod map_loader;
mod metadata_handler;
mod music_handler;
mod order;
mod particle;
mod save_handler;
mod scene;
mod settings;
mod spirit;
mod spirits_handler;
mod texture_handler;
mod ui;

mod color;
pub const FIRST_LEVEL: u8 = 0;

const SCREEN_WIDTH: i32 = 320; //256;
const SCREEN_HEIGHT: i32 = 180; //144

fn main() {
    profiling::scope!("Initialization");
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .resizable()
        .title("Велесова Ночь")
        .build();

    rl.set_target_fps(get_monitor_refresh_rate(get_current_monitor() as i32) as u32);

    rl.set_exit_key(None);

    let rl_audio = RaylibAudio::init_audio_device().unwrap();
    let music_handler = MusicHandler::new(&rl_audio);
    music_handler.music_play();
    let mut settings_handler = SettingsHandler::new();
    settings_handler.save();

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

    let mut main_menu = MainMenuHandler::new(settings_handler.settings.pixel_scale as f32);
    let mut settings_menu = SettingsMenuHandler::new(settings_handler.settings.pixel_scale as f32);

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
    spirits_handler.spawn_spirits(&mut metadata_handler, &mut settings_handler);

    let mut enemies_handler = EnemiesHandler::new();
    enemies_handler.spawn_enemies(&mut metadata_handler, &settings_handler);

    let mut order_handler = OrderHandler::new();
    let mut ui_handler = UIHandler::new(
        level_number as usize,
        settings_handler.settings.pixel_scale as f32,
    );
    let mut gameover_handler = GameOverHandler::new(
        gameover_handler::GameOverHandlerType::Level,
        settings_handler.settings.pixel_scale as f32,
    );
    let mut gameend_handler = GameOverHandler::new(
        gameover_handler::GameOverHandlerType::Game,
        settings_handler.settings.pixel_scale as f32,
    );
    let mut level_transition = LevelTransition::new();
    let mut level_selector = LevelSelector::new(settings_handler.settings.pixel_scale as i32);

    let mut should_close = false;

    let mut particles: Vec<Particle> = vec![];

    let mut shader = rl.load_shader(&thread, None, Some("static/shaders/bloom.fs"));

    let mut target = rl
        .load_render_texture(
            &thread,
            SCREEN_WIDTH as u32 * settings_handler.settings.pixel_scale as u32,
            SCREEN_HEIGHT as u32 * settings_handler.settings.pixel_scale as u32,
        )
        .expect("Couldn't load render texture");

    let mut save_handler = SaveHandler::new();

    let monitor_width = unsafe { GetMonitorWidth(GetCurrentMonitor()) };
    let monitor_height = unsafe { GetMonitorHeight(GetCurrentMonitor()) };

    let mut dialogue_handler = DialogueHandler::new();
    dialogue_handler.load_dialogue(&format!("level_{level_number}"));

    rl.set_window_size(
        SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32,
        SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32,
    );

    let mut fullscreen = settings_handler.settings.fullscreen;
    let mut prev_scale = settings_handler.settings.pixel_scale;

    if fullscreen {
        rl.toggle_fullscreen();
    }

    while !rl.window_should_close() && !should_close {
        profiling::scope!("Game frame");
        if settings_menu.should_remade {
            settings_menu.should_remade = false;

            settings_menu.rescale_ui(settings_handler.settings.pixel_scale as f32);
            main_menu.rescale_ui(settings_handler.settings.pixel_scale as f32);
            ui_handler.rescale_ui(settings_handler.settings.pixel_scale as f32);
            level_selector.rescale_ui(settings_handler.settings.pixel_scale as f32);

            spirits_handler.rescale_ui(
                prev_scale as f32,
                settings_handler.settings.pixel_scale as f32,
            );
            enemies_handler.rescale_ui(
                prev_scale as f32,
                settings_handler.settings.pixel_scale as f32,
            );

            prev_scale = settings_handler.settings.pixel_scale;

            rl.set_window_size(
                SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32,
                SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32,
            );

            target = rl
                .load_render_texture(
                    &thread,
                    SCREEN_WIDTH as u32 * settings_handler.settings.pixel_scale as u32,
                    SCREEN_HEIGHT as u32 * settings_handler.settings.pixel_scale as u32,
                )
                .expect("Couldn't load render texture");
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F) {
            settings_handler.settings.fullscreen = !settings_handler.settings.fullscreen;
            settings_menu.set_inner_setting(
                settings_handler.settings.fullscreen as u8,
                settings_menu::SettingsOptions::Fullscreen,
            );
        }
        if settings_handler.settings.fullscreen != fullscreen {
            fullscreen = settings_handler.settings.fullscreen;
            rl.toggle_fullscreen();
            rl.set_window_size(
                SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32,
                SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32,
            );
        }

        music_handler.music_update(&settings_handler.get_settings());
        save_handler.check_saves();

        if save_handler.should_save {
            save_handler.create_save_file(
                &mut metadata_handler,
                &mut level,
                &mut spirits_handler,
                &mut level_number,
                &mut settings_handler,
            );
        }

        if save_handler.should_load {
            save_handler.load_save(
                &mut metadata_handler,
                &mut level,
                &mut spirits_handler,
                &mut enemies_handler,
                &mut ui_handler,
                &mut level_number,
                &mut level_transition,
                &mut rl,
                &mut scene_handler,
                &mut dialogue_handler,
                &mut settings_handler,
            );
        }
        // update stuff
        particles.retain(|particle| !particle.done);

        for particle in particles.iter_mut() {
            particle.update(&mut rl);
        }

        scene_handler.update(&mut rl);
        if hotkey_handler.check_pressed(&rl, HotkeyCategory::Skip)
            && rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL)
            && rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT)
        {
            if scene_handler.get_current() == Scene::GameEnd {
                level_number = level_num;
            }

            let scene = scene_handler.get_next();
            scene_handler.set(scene);
        }

        if hotkey_handler.check_down(&rl, HotkeyCategory::VolumeUp) {
            settings_handler.settings.general_audio += 1.;
            if settings_handler.settings.general_audio > 100. {
                settings_handler.settings.general_audio = 100.;
            }

            rl_audio.set_master_volume(settings_handler.settings.general_audio / 100.);
        }

        if hotkey_handler.check_down(&rl, HotkeyCategory::VolumeDown) {
            settings_handler.settings.general_audio -= 1.;
            if settings_handler.settings.general_audio <= 0. {
                settings_handler.settings.general_audio = 0.;
            }

            rl_audio.set_master_volume(settings_handler.settings.general_audio / 100.);
        }

        match scene_handler.get_current() {
            Scene::GameOver => (),
            _ => music_handler.music_resume(),
        };

        match scene_handler.get_current() {
            Scene::MainMenu => {
                rl.set_window_title(&thread, "Велесова Ночь");
                if rl.is_key_pressed(KeyboardKey::KEY_L) {
                    scene_handler.set(Scene::LevelSelection);
                }
                main_menu.update(
                    &mut scene_handler,
                    &mut should_close,
                    &mut rl,
                    &mut save_handler,
                    &settings_handler,
                    &mut level_number,
                    &mut metadata_handler,
                    &mut level,
                    &mut spirits_handler,
                    &mut enemies_handler,
                    &mut ui_handler,
                    &mut level_transition,
                    &mut settings_menu,
                    &mut hotkey_handler,
                );
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
                    &mut settings_handler,
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
                    &mut settings_handler,
                ) {
                    music_handler.music_resume();
                    reload_procedure(
                        level_number as u8,
                        &mut level,
                        &mut metadata_handler,
                        &mut enemies_handler,
                        &mut spirits_handler,
                        &mut rl,
                        &mut settings_handler,
                    );
                }
            }
            Scene::Level => {
                if cfg!(debug_assertions) {
                    rl.set_window_title(
                        &thread,
                        format!(
                            "[{}] Велесова Ночь - Уровень {}",
                            rl.get_fps(),
                            level_number + 1
                        )
                        .as_str(),
                    );
                } else {
                    rl.set_window_title(
                        &thread,
                        format!("Велесова Ночь - Уровень {}", level_number + 1).as_str(),
                    );
                }

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
                    &mut enemies_handler,
                    &mut save_handler,
                    &mut dialogue_handler,
                    &mut settings_menu,
                    &mut settings_handler,
                ) {
                    reload_procedure(
                        level_number,
                        &mut level,
                        &mut metadata_handler,
                        &mut enemies_handler,
                        &mut spirits_handler,
                        &mut rl,
                        &mut settings_handler,
                    );
                }

                if settings_menu.check_scene() {
                    settings_menu.set_scene(Scene::Level);
                    settings_menu.align_settings(&settings_handler.get_settings());
                    scene_handler.set(Scene::Settings);
                }
            }
            Scene::Transition => update_transition(
                &mut level_transition,
                &mut level_number,
                &mut metadata_handler,
                &mut level,
                &mut scene_handler,
                &mut spirits_handler,
                &mut enemies_handler,
                &mut rl,
                &mut hotkey_handler,
                &mut ui_handler,
                &mut dialogue_handler,
                &mut settings_handler,
            ),
            Scene::LevelSelection => {
                level_selector.update(
                    &mut level_number,
                    &mut metadata_handler,
                    &mut level,
                    &mut spirits_handler,
                    &mut enemies_handler,
                    &mut ui_handler,
                    &mut level_transition,
                    &mut scene_handler,
                    &mut dialogue_handler,
                    &mut rl,
                    &mut settings_handler,
                    &mut hotkey_handler,
                );
            }
            Scene::Settings => {
                settings_menu.update(
                    &mut scene_handler,
                    &mut rl,
                    &mut settings_handler,
                    &mut hotkey_handler,
                );
            }
        }
        profiling::scope!("Drawing");
        // draw stuff
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // we draw to the texture
        {
            let mut t = d.begin_texture_mode(&thread, &mut target);
            if !settings_handler.settings.shader && Scene::Level == scene_handler.get_current() {
                level_texture(
                    &mut level,
                    level_number,
                    &texture_handler,
                    &mut spirits_handler,
                    &mut enemies_handler,
                    &mut order_handler,
                    &mut t,
                    &mut particles,
                    &mut settings_handler,
                );
            } else {
                t.draw_shader_mode(&mut shader, |mut s| match scene_handler.get_current() {
                    Scene::Level => level_texture(
                        &mut level,
                        level_number,
                        &texture_handler,
                        &mut spirits_handler,
                        &mut enemies_handler,
                        &mut order_handler,
                        &mut s,
                        &mut particles,
                        &mut settings_handler,
                    ),
                    _ => {}
                });
            }

            match scene_handler.get_current() {
                Scene::MainMenu => {
                    main_menu.draw(
                        &font,
                        &save_handler,
                        &texture_handler,
                        &mut t,
                        &settings_handler,
                    );
                }
                Scene::Settings => {
                    settings_menu.draw(
                        &font,
                        &texture_handler,
                        &mut t,
                        &mut settings_handler,
                        &mut hotkey_handler,
                    );
                }
                Scene::GameEnd => {
                    gameend_handler.draw_gameover(&font, &mut t, &mut settings_handler)
                }
                Scene::GameOver => {
                    gameover_handler.draw_gameover(&font, &mut t, &mut settings_handler)
                }
                Scene::Level => draw_level_ui(
                    &mut level,
                    &texture_handler,
                    &mut ui_handler,
                    &mut dialogue_handler,
                    &font,
                    &mut t,
                    &mut settings_handler,
                ),
                Scene::Transition => {
                    level_transition.draw(&texture_handler, &font, &mut t, &settings_handler);
                }
                Scene::LevelSelection => {
                    level_selector.draw(&font, &texture_handler, &mut t, &mut settings_handler);
                }
            }

            scene_handler.draw(&mut t, &mut settings_handler);
        }

        let dest_rec = if d.is_window_fullscreen() {
            Rectangle::new(
                monitor_width as f32 / 2.
                    - (SCREEN_WIDTH as f32 * settings_handler.settings.pixel_scale as f32) / 2.,
                monitor_height as f32 / 2.
                    - (SCREEN_HEIGHT as f32 * settings_handler.settings.pixel_scale as f32) / 2.,
                SCREEN_WIDTH as f32 * settings_handler.settings.pixel_scale as f32,
                SCREEN_HEIGHT as f32 * settings_handler.settings.pixel_scale as f32,
            )
        } else {
            Rectangle::new(
                d.get_screen_width() as f32 / 2.
                    - (SCREEN_WIDTH as f32 * settings_handler.settings.pixel_scale as f32) / 2.,
                d.get_screen_height() as f32 / 2.
                    - (SCREEN_HEIGHT as f32 * settings_handler.settings.pixel_scale as f32) / 2.,
                SCREEN_WIDTH as f32 * settings_handler.settings.pixel_scale as f32,
                SCREEN_HEIGHT as f32 * settings_handler.settings.pixel_scale as f32,
            )
        };

        // we draw the texture in the middle of the screen
        d.draw_texture_pro(
            &target,
            Rectangle::new(
                0.,
                0.,
                SCREEN_WIDTH as f32 * settings_handler.settings.pixel_scale as f32,
                -SCREEN_HEIGHT as f32 * settings_handler.settings.pixel_scale as f32,
            ),
            dest_rec,
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );
        profiling::finish_frame!();
    }

    match scene_handler.get_current() {
        Scene::Transition => {
            preparation_to_save(
                &mut (level_number + 1),
                &mut metadata_handler,
                &mut level,
                &mut spirits_handler,
                &mut rl,
                &mut settings_handler,
            );

            save_handler.create_save_file(
                &mut metadata_handler,
                &mut level,
                &mut spirits_handler,
                &mut level_number,
                &mut settings_handler,
            );
        }
        Scene::GameEnd => {
            preparation_to_save(
                &mut level_number,
                &mut metadata_handler,
                &mut level,
                &mut spirits_handler,
                &mut rl,
                &mut settings_handler,
            );

            save_handler.create_save_file(
                &mut metadata_handler,
                &mut level,
                &mut spirits_handler,
                &mut level_number,
                &mut settings_handler,
            );
        }
        Scene::Level => save_handler.create_save_file(
            &mut metadata_handler,
            &mut level,
            &mut spirits_handler,
            &mut level_number,
            &mut settings_handler,
        ),
        _ => (),
    };
}

fn preparation_to_save(
    level_number: &mut u8,
    metadata_handler: &mut MetadataHandler,
    level: &mut Level,
    spirits_handler: &mut SpiritsHandler,
    rl: &mut RaylibHandle,
    settings_handler: &mut SettingsHandler,
) {
    metadata_handler.load(*level_number);
    level.load(*level_number, metadata_handler, rl);
    spirits_handler.spawn_spirits(metadata_handler, settings_handler);
}

fn update_level<'a>(
    spirits_handler: &mut SpiritsHandler,
    particles: &mut Vec<Particle>,
    level: &mut Level,
    order_handler: &mut OrderHandler,
    ui_handler: &mut UIHandler,
    scene_handler: &mut SceneHandler,
    music_handler: &MusicHandler,
    rl: &mut RaylibHandle,
    hotkey_handler: &mut HotkeyHandler,
    enemies_handler: &mut EnemiesHandler,
    save_handler: &mut SaveHandler,
    dialogue_handler: &mut DialogueHandler,
    settings_menu: &mut SettingsMenuHandler,
    settings_handler: &mut SettingsHandler,
) -> bool {
    let (level_quit, level_restart, level_settings) = ui_handler.update(
        hotkey_handler,
        scene_handler,
        dialogue_handler,
        rl,
        settings_handler,
    );

    if level_quit {
        save_handler.set_to_save();
    };

    level.update(
        scene_handler,
        spirits_handler.spirits.len() as u8,
        music_handler,
        settings_handler,
    );

    if hotkey_handler.check_pressed(rl, HotkeyCategory::Reset) || level_restart {
        return true;
    }

    if level_settings {
        settings_menu.set_scene(Scene::Level);
    }

    if ui_handler.is_pause() {
        return false;
    }

    order_handler.select_spirit(spirits_handler, level, rl, hotkey_handler, settings_handler);
    order_handler.update_line(level, rl, hotkey_handler, settings_handler);

    ui_handler.build(
        level,
        rl,
        hotkey_handler,
        dialogue_handler,
        settings_handler,
    );

    for spirit in spirits_handler.spirits.values() {
        if spirit.get_dead() {
            particles.push(Particle::new(
                Vector2::new(
                    spirit.get_position().x
                        + (settings_handler.settings.pixel_scale as i32
                            * (TILE_SIZE_PX * settings_handler.settings.pixel_scale as i32))
                            as f32
                            / 2.,
                    spirit.get_draw_position().y
                        + (settings_handler.settings.pixel_scale as i32
                            * (TILE_SIZE_PX * settings_handler.settings.pixel_scale as i32))
                            as f32
                            / 2.,
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

    // better than iter_mut() with (_, enemy)
    for enemy in enemies_handler.enemies.values_mut() {
        enemy.collide_check(spirits_handler);
    }

    for spirit in spirits_handler.spirits.values_mut() {
        spirit.update_behaviour(level, music_handler, rl, settings_handler);
    }

    return false;
}

fn draw_level(
    level: &mut Level,
    level_number: u8,
    texture_handler: &TextureHandler,
    spirits_handler: &mut SpiritsHandler,
    enemies_handler: &mut EnemiesHandler,
    order_handler: &mut OrderHandler,
    rl: &mut RaylibDrawHandle,
    settings_handler: &mut SettingsHandler,
) {
    rl.clear_background(Color::from_hex("0b8a8f").unwrap());

    level.draw(rl, texture_handler, level_number, settings_handler);
    for spirit in spirits_handler.spirits.values() {
        spirit.draw(rl, texture_handler, settings_handler);
    }
    for enemy in enemies_handler.enemies.values() {
        enemy.draw(rl, texture_handler, settings_handler);
    }

    order_handler.draw(spirits_handler, texture_handler, rl, settings_handler);
}

fn draw_level_ui<'a>(
    level: &mut Level,
    texture_handler: &TextureHandler,
    ui_handler: &mut UIHandler,
    dialogue_handler: &mut DialogueHandler,
    font: &Font,
    rl: &mut RaylibDrawHandle,
    settings_handler: &mut SettingsHandler,
) {
    ui_handler.draw(
        texture_handler,
        dialogue_handler,
        level,
        font,
        rl,
        settings_handler,
    );
}

fn update_transition(
    level_transition: &mut LevelTransition,
    level_number: &mut u8,
    metadata_handler: &mut MetadataHandler,
    level: &mut Level,
    scene_handler: &mut SceneHandler,
    spirits_handler: &mut SpiritsHandler,
    enemies_handler: &mut EnemiesHandler,
    rl: &mut RaylibHandle,
    hotkey_handler: &mut HotkeyHandler,
    ui_handler: &mut UIHandler,
    dialogue_handler: &mut DialogueHandler,
    settings_handler: &mut SettingsHandler,
) {
    if !hotkey_handler.check_pressed(rl, HotkeyCategory::Continue)
        && !rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
    {
        return;
    }

    *level_number += 1;
    if *level_number >= level_transition.max_level {
        scene_handler.set(Scene::GameEnd);
        return;
    }

    level_transition.set_cards(*level_number as usize);
    metadata_handler.load(*level_number);
    level.load(*level_number, metadata_handler, rl);
    spirits_handler.spawn_spirits(metadata_handler, settings_handler);
    enemies_handler.spawn_enemies(metadata_handler, settings_handler);
    scene_handler.set(Scene::Level);
    dialogue_handler.load_dialogue(&format!("level_{}", *level_number + 1));
    *ui_handler = UIHandler::new(
        level_number.clone() as usize,
        settings_handler.settings.pixel_scale as f32,
    );
}

fn reload_procedure(
    current_level: u8,
    level: &mut Level,
    metadata_handler: &mut MetadataHandler,
    enemies_handler: &mut EnemiesHandler,
    spirits_handler: &mut SpiritsHandler,
    rl: &mut RaylibHandle,
    settings_handler: &mut SettingsHandler,
) {
    *level = Level::new();
    *metadata_handler = MetadataHandler::new(current_level);
    level.load(current_level, metadata_handler, rl);

    *spirits_handler = SpiritsHandler::new();
    spirits_handler.spawn_spirits(metadata_handler, settings_handler);
    enemies_handler.spawn_enemies(metadata_handler, settings_handler);
}

fn level_texture(
    level: &mut Level,
    level_number: u8,
    texture_handler: &TextureHandler,
    spirits_handler: &mut SpiritsHandler,
    enemies_handler: &mut EnemiesHandler,
    order_handler: &mut OrderHandler,
    rl: &mut RaylibDrawHandle,
    particles: &mut Vec<Particle>,
    settings_handler: &mut SettingsHandler,
) {
    draw_level(
        level,
        level_number,
        &texture_handler,
        spirits_handler,
        enemies_handler,
        order_handler,
        rl,
        settings_handler,
    );
    for particle in particles.iter_mut() {
        particle.draw(rl);
    }
}
