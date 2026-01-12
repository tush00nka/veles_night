use std::{cmp::min, collections::HashMap};

use raylib::{ffi::CheckCollisionPointRec, prelude::*};

use crate::{
    SCREEN_HEIGHT, SCREEN_WIDTH,
    dialogue::DialogueHandler,
    hotkey_handler::{HotkeyCategory, HotkeyHandler},
    map::{Level, TILE_SCALE, TILE_SIZE, TileType},
    scene::{Scene, SceneHandler},
    texture_handler::TextureHandler,
};

pub struct Button {
    pub rect: Rectangle,
    pub selected: bool,
}

const TEXT_COLOR: Color = Color::new(46, 34, 47, 255);

const RESTART_BUTTON: Rectangle = Rectangle::new(
    SCREEN_WIDTH as f32 / 3. + SCREEN_WIDTH as f32 / 6. - 100.,
    SCREEN_HEIGHT as f32 / 3. + 30.,
    200.,
    100.,
);

const QUIT_BUTTON: Rectangle = Rectangle::new(
    SCREEN_WIDTH as f32 / 3. + SCREEN_WIDTH as f32 / 6. - 100.,
    SCREEN_HEIGHT as f32 / 3. + 150.,
    200.,
    100.,
);

pub struct UIHandler {
    build_buttons: HashMap<String, Button>,
    quitting: bool,
    // dialogue_accumulator: String,
    // dialogure_iterator: Option<Chars<'a>>,
    // current_dialogue: usize,
}

impl UIHandler {
    pub fn new(level_number: usize) -> Self {
        let mut buttons = HashMap::new();

        let labels = ["fire_td", "fire_lr", "fire_stop"];

        for i in 0..min(labels.len(), level_number) {
            buttons.insert(
                labels[i].to_string(),
                Button {
                    rect: Rectangle::new(
                        i as f32 * 20. * TILE_SCALE as f32 + (SCREEN_WIDTH / 2) as f32
                            - 10. * TILE_SCALE as f32 * min(labels.len(), level_number) as f32,
                        SCREEN_HEIGHT as f32 - 10. - 16. * TILE_SCALE as f32,
                        16. * TILE_SCALE as f32,
                        16. * TILE_SCALE as f32,
                    ),
                    selected: false,
                },
            );
        }

        Self {
            build_buttons: buttons,
            quitting: false,
            // dialogue_accumulator: String::new(),
            // dialogure_iterator: Some(Self::DIALOGUE[0].chars()),
            // current_dialogue: 0,
        }
    }

    pub fn build(
        &mut self,
        level: &mut Level,
        rl: &mut RaylibHandle,
        hotkey_h: &mut HotkeyHandler,
        dialogue_h: &mut DialogueHandler,
    ) {
        let dialoging = dialogue_h.current_phrase < dialogue_h.dialogue.len();

        let mut intent: HotkeyCategory;
        for (title, button) in self.build_buttons.iter_mut() {
            if dialoging {
                break;
            }

            intent = HotkeyCategory::from_bonfire(title);

            if hotkey_h.check_pressed(rl, intent) {
                button.selected = true;
            }

            if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                if unsafe {
                    CheckCollisionPointRec(
                        (rl.get_mouse_position()
                            - Vector2::new(
                                rl.get_screen_width() as f32 / 2. - SCREEN_WIDTH as f32 / 2.,
                                rl.get_screen_height() as f32 / 2. - SCREEN_HEIGHT as f32 / 2.,
                            ))
                        .into(),
                        button.rect.into(),
                    )
                } {
                    button.selected = true;
                }
                hotkey_h.clear_last();
            }

            let keyboard_last = hotkey_h.get_last_key();

            if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
                button.selected = false;
                continue;
            }

            if !rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT)
                && keyboard_last == KeyboardKey::KEY_NUM_LOCK
            {
                continue;
            }

            if keyboard_last != KeyboardKey::KEY_NUM_LOCK && !rl.is_key_released(keyboard_last) {
                continue;
            }

            if button.selected && level.get_wood() > 0 {
                let pos = (rl.get_mouse_position()
                    - Vector2::new(
                        rl.get_screen_width() as f32 / 2. - SCREEN_WIDTH as f32 / 2.,
                        rl.get_screen_height() as f32 / 2. - SCREEN_HEIGHT as f32 / 2.,
                    ))
                    / (Vector2::one() * TILE_SIZE as f32);
                let (x, y) = (pos.x as usize, pos.y as usize);

                if level.tiles[x][y] != TileType::Air {
                    button.selected = false;
                    continue;
                }

                let tile = match title.as_str() {
                    "fire_td" => TileType::FireTD {
                        active: false,
                        selected: false,
                    },
                    "fire_lr" => TileType::FireLR {
                        active: false,
                        selected: false,
                    },
                    "fire_stop" => TileType::FireStop {
                        active: false,
                        selected: false,
                    },
                    _ => {
                        panic!("wait how")
                    }
                };

                level.tiles[x][y] = tile;
                level.remove_wood();
            }
            button.selected = false;
        }
    }

    pub fn update(
        &mut self,
        hotkey_h: &mut HotkeyHandler,
        scene_h: &mut SceneHandler,
        dialogue_h: &mut DialogueHandler,
        rl: &mut RaylibHandle,
    ) -> (bool, bool) {
        if hotkey_h.check_pressed(rl, HotkeyCategory::Exit) {
            self.quitting = !self.quitting;
        }

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if dialogue_h.dialogue.len() <= 0 {
                dialogue_h.current_phrase += 1;
            } else if dialogue_h.current_phrase + 1 < dialogue_h.dialogue.len() {
                dialogue_h.current_phrase += 1;
                dialogue_h.dialogue_accumulator = String::new();
                dialogue_h.dialogue_counter = 0;
            } else if dialogue_h.current_phrase == dialogue_h.dialogue.len() - 1 {
                dialogue_h.current_phrase += 1;
            }
        }

        if !self.quitting {
            return (false, false);
        }

        if QUIT_BUTTON.check_collision_point_rec(
            rl.get_mouse_position()
                - Vector2::new(
                    rl.get_screen_width() as f32 / 2. - SCREEN_WIDTH as f32 / 2.,
                    rl.get_screen_height() as f32 / 2. - SCREEN_HEIGHT as f32 / 2.,
                ),
        ) && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
        {
            scene_h.set(Scene::MainMenu);
            self.quitting = false;
            return (true, false);
        };

        if RESTART_BUTTON.check_collision_point_rec(
            rl.get_mouse_position()
                - Vector2::new(
                    rl.get_screen_width() as f32 / 2. - SCREEN_WIDTH as f32 / 2.,
                    rl.get_screen_height() as f32 / 2. - SCREEN_HEIGHT as f32 / 2.,
                ),
        ) && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
        {
            self.quitting = false;
            return (false, true);
        };

        return (false, false);
    }

    pub fn draw(
        &mut self,
        texture_handler: &TextureHandler,
        dialogue_h: &mut DialogueHandler,
        level: &mut Level,
        _level_number: usize,
        font: &Font,
        rl: &mut RaylibDrawHandle,
    ) {
        let dialoging = dialogue_h.current_phrase < dialogue_h.dialogue.len();

        for (tex_name, button) in self.build_buttons.iter() {
            if dialoging {
                break;
            }

            let color = if unsafe {
                CheckCollisionPointRec(
                    (rl.get_mouse_position()
                        - Vector2::new(
                            rl.get_screen_width() as f32 / 2. - SCREEN_WIDTH as f32 / 2.,
                            rl.get_screen_height() as f32 / 2. - SCREEN_HEIGHT as f32 / 2.,
                        ))
                    .into(),
                    button.rect.into(),
                )
            } {
                Color::WHITE
            } else {
                Color::BLACK.alpha(0.5)
            };

            rl.draw_rectangle_rec(button.rect, color);

            if !button.selected {
                rl.draw_texture_pro(
                    texture_handler.get_safe(tex_name),
                    Rectangle::new(
                        ((rl.get_time() * 8.) % 4.).floor() as f32 * 16.,
                        16.,
                        16.,
                        16.,
                    ),
                    button.rect,
                    Vector2::zero(),
                    0.0,
                    Color::WHITE,
                );
            } else {
                rl.draw_texture_pro(
                    texture_handler.get_safe(tex_name),
                    Rectangle::new(
                        ((rl.get_time() * 8.) % 4.).floor() as f32 * 16.,
                        16.,
                        16.,
                        16.,
                    ),
                    Rectangle::new(
                        (rl.get_mouse_position()
                            - Vector2::new(
                                rl.get_screen_width() as f32 / 2. - SCREEN_WIDTH as f32 / 2.,
                                rl.get_screen_height() as f32 / 2. - SCREEN_HEIGHT as f32 / 2.,
                            ))
                        .x - (TILE_SIZE / 2) as f32,
                        (rl.get_mouse_position()
                            - Vector2::new(
                                rl.get_screen_width() as f32 / 2. - SCREEN_WIDTH as f32 / 2.,
                                rl.get_screen_height() as f32 / 2. - SCREEN_HEIGHT as f32 / 2.,
                            ))
                        .y - (TILE_SIZE / 2) as f32,
                        TILE_SIZE as f32,
                        TILE_SIZE as f32,
                    ),
                    Vector2::zero(),
                    0.0,
                    Color::WHITE,
                );
            }
        }

        // rl.draw_rectangle(
        //     5,
        //     5,
        //     64 * TILE_SCALE,
        //     18 * TILE_SCALE,
        //     Color::BLACK.alpha(0.5),
        // );

        rl.draw_texture_ex(
            texture_handler.get("stat_bar"),
            Vector2::one() * TILE_SCALE as f32,
            0.0,
            TILE_SCALE as f32,
            Color::WHITE,
        );

        let bar_offset = Vector2::new(6. * TILE_SCALE as f32, 5. * TILE_SCALE as f32);

        rl.draw_texture_ex(
            texture_handler.get("spirit_icon"),
            bar_offset,
            0.0,
            TILE_SCALE as f32,
            Color::WHITE,
        );

        rl.draw_text_ex(
            font,
            format!("{}/{}", level.survived, level.survive).as_str(),
            bar_offset + Vector2::new(16. * TILE_SCALE as f32, 10.),
            8. * TILE_SCALE as f32,
            1.0,
            TEXT_COLOR,
        );

        rl.draw_texture_ex(
            texture_handler.get("wood_icon"),
            bar_offset + Vector2::new(0., 12. * TILE_SCALE as f32),
            0.0,
            TILE_SCALE as f32,
            Color::WHITE,
        );

        rl.draw_text_ex(
            font,
            format!("{}", level.get_wood()).as_str(),
            bar_offset + Vector2::new(16. * TILE_SCALE as f32, 10. + 12. * TILE_SCALE as f32),
            8. * TILE_SCALE as f32,
            1.0,
            TEXT_COLOR,
        );

        // let hint_text = if level_number == 0 {
        //     "Перетащите духа\nна дерево"
        // } else if level_number == 1 {
        //     "Установите костёр,\nперетащив\nпиктограмму"
        // } else {
        //     "R для перезапуска"
        // };

        // let height = hint_text.chars().filter(|&c| c == '\n').count();

        // rl.draw_rectangle(
        //     5,
        //     30 * TILE_SCALE,
        //     64 * TILE_SCALE,
        //     9 * (height + 1) as i32 * TILE_SCALE,
        //     Color::BLACK.alpha(0.5),
        // );

        // rl.draw_text_ex(
        //     font,
        //     hint_text,
        //     Vector2::new(10., 28. * TILE_SCALE as f32 + 18.),
        //     8. * TILE_SCALE as f32,
        //     1.0,
        //     Color::RAYWHITE,
        // );

        if dialoging {
            rl.draw_texture_ex(
                texture_handler.get_safe("yarilo"),
                Vector2::new(0., SCREEN_HEIGHT as f32 - 48. * TILE_SCALE as f32),
                0.0,
                TILE_SCALE as f32,
                Color::WHITE,
            );

            rl.draw_texture_ex(
                texture_handler.get("dialogue_box"),
                Vector2::new(
                    32. * TILE_SCALE as f32,
                    SCREEN_HEIGHT as f32 - 48. * TILE_SCALE as f32,
                ),
                0.0,
                TILE_SCALE as f32,
                Color::WHITE,
            );

            if dialogue_h.dialogue_counter
                < dialogue_h.dialogue[dialogue_h.current_phrase]
                    .1
                    .chars()
                    .count()
            {
                dialogue_h.dialogue_counter += 1;
            }

            let line = &mut dialogue_h.dialogue[dialogue_h.current_phrase]
                .1
                .chars()
                .rev();
            let line_len = dialogue_h.dialogue[dialogue_h.current_phrase]
                .1
                .chars()
                .count();
            for _ in 0..line_len - dialogue_h.dialogue_counter {
                line.next();
            }

            rl.draw_text_ex(
                font,
                &line.rev().collect::<String>(),
                Vector2::new(
                    32. * TILE_SCALE as f32 + 64.,
                    SCREEN_HEIGHT as f32 - 3. * 8. * TILE_SCALE as f32 - 20.,
                ),
                8. * TILE_SCALE as f32,
                0.,
                TEXT_COLOR,
            );

            if dialogue_h.dialogue_accumulator.chars().count()
                >= dialogue_h.dialogue[dialogue_h.current_phrase]
                    .1
                    .chars()
                    .count()
            {
                rl.draw_text_ex(
                    font,
                    "Далее...",
                    Vector2::new(
                        SCREEN_WIDTH as f32 - 32. * TILE_SCALE as f32,
                        SCREEN_HEIGHT as f32 - 12. * TILE_SCALE as f32,
                    ),
                    8. * TILE_SCALE as f32,
                    0.,
                    Color::RAYWHITE.alpha((rl.get_time() * 2.).sin().abs() as f32),
                )
            }
        }

        if !self.quitting {
            return;
        }

		let screen_width = rl.get_screen_width();
		let screen_height = rl.get_screen_height();

        // panel
        rl.draw_rectangle(
            screen_width / 3,
            screen_height / 4,
            screen_width / 3,
            screen_height / 2,
            Color::from_hex("0b5e65").unwrap(),
        );

        rl.draw_text_ex(
            font,
            "Меню",
            Vector2::new(
                screen_width as f32 / 2. - 50.,
                screen_height as f32 / 3. - 50.,
            ),
            64.,
            2.,
            Color::RAYWHITE,
        );

        let mouse_over = RESTART_BUTTON.check_collision_point_rec(
            rl.get_mouse_position()
                - Vector2::new(
                    rl.get_screen_width() as f32 / 2. - SCREEN_WIDTH as f32 / 2.,
                    rl.get_screen_height() as f32 / 2. - SCREEN_HEIGHT as f32 / 2.,
                ),
        );

        rl.draw_rectangle_rec(
            RESTART_BUTTON,
            if mouse_over {
                Color::from_hex("30e1b9").unwrap()
            } else {
				Color::from_hex("0b8a8f").unwrap()
            },
        );

        rl.draw_text_ex(
            font,
            "Заново",
            Vector2::new(RESTART_BUTTON.x + 16., RESTART_BUTTON.y + 16.),
            64.,
            2.,
			Color::RAYWHITE
        );

        let mouse_over = QUIT_BUTTON.check_collision_point_rec(
            rl.get_mouse_position()
                - Vector2::new(
                    rl.get_screen_width() as f32 / 2. - SCREEN_WIDTH as f32 / 2.,
                    rl.get_screen_height() as f32 / 2. - SCREEN_HEIGHT as f32 / 2.,
                ),
        );

        rl.draw_rectangle_rec(
            QUIT_BUTTON,
            if mouse_over {
                Color::from_hex("30e1b9").unwrap()
            } else {
				Color::from_hex("0b8a8f").unwrap()
            },
        );

        rl.draw_text_ex(
            font,
            "Выйти",
            Vector2::new(QUIT_BUTTON.x + 16., QUIT_BUTTON.y + 16.),
            64.,
            2.,
			Color::RAYWHITE
        );
    }
}
