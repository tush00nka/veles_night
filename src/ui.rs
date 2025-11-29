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
    ) {
        let mut intent: HotkeyCategory;
        for (title, button) in self.build_buttons.iter_mut() {
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
    ) -> bool {
        if hotkey_h.check_pressed(rl, HotkeyCategory::Exit) {
            self.quitting = !self.quitting;
        }

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if dialogue_h.current_phrase + 1 < dialogue_h.dialogue.len() {
                dialogue_h.current_phrase += 1;
                dialogue_h.dialogue_accumulator = String::new();
                dialogue_h.dialogue_counter = 0;
			} else if dialogue_h.current_phrase == dialogue_h.dialogue.len() - 1 {
                dialogue_h.current_phrase += 1;
			}
        }

        if !self.quitting {
            return false;
        }

        if unsafe {
            CheckCollisionPointRec(
                (rl.get_mouse_position()
                    - Vector2::new(
                        rl.get_screen_width() as f32 / 2. - SCREEN_WIDTH as f32 / 2.,
                        rl.get_screen_height() as f32 / 2. - SCREEN_HEIGHT as f32 / 2.,
                    ))
                .into(),
                QUIT_BUTTON.into(),
            )
        } && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
        {
            scene_h.set(Scene::MainMenu);
            self.quitting = false;
            return true;
        };
        return false;
    }

    // const DIALOGUE: [&'a str; 3] = [
    //     "Приветствую тебя, путник! Никак заблудился в этом лесу, да ещё и накануне ночи,\nчто в честь меня кличут?",
    //     "Ты либо храбрец, либо глупец, а может, и то и другое...\nНаказываю тебе помочь этим духам, что, как и ты, заблудились в лесу.",
    //     "Помоги им вернуться в мир нави, а я, так уж и быть, выведу тебя из леса.",
    // ];

    pub fn draw(
        &mut self,
        texture_handler: &TextureHandler,
        dialogue_h: &mut DialogueHandler,
        level: &mut Level,
        level_number: usize,
        font: &Font,
        rl: &mut RaylibDrawHandle,
    ) {
        for (tex_name, button) in self.build_buttons.iter() {
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

        rl.draw_rectangle(
            5,
            5,
            64 * TILE_SCALE,
            18 * TILE_SCALE,
            Color::BLACK.alpha(0.5),
        );

        rl.draw_text_ex(
            font,
            format!(
                "Духов проведено: {}/{}\nДревесина: {}",
                level.survived,
                level.survive,
                level.get_wood()
            )
            .as_str(),
            Vector2::one() * 10.,
            8. * TILE_SCALE as f32,
            1.0,
            Color::RAYWHITE,
        );

        let hint_text = if level_number == 0 {
            "Перетащите духа\nна дерево"
        } else if level_number == 1 {
            "Установите костёр,\nперетащив\nпиктограмму"
        } else {
            "R для перезапуска"
        };

        let height = hint_text.chars().filter(|&c| c == '\n').count();

        rl.draw_rectangle(
            5,
            20 * TILE_SCALE,
            64 * TILE_SCALE,
            9 * (height + 1) as i32 * TILE_SCALE,
            Color::BLACK.alpha(0.5),
        );

        rl.draw_text_ex(
            font,
            hint_text,
            Vector2::new(10., 18. * TILE_SCALE as f32 + 18.),
            8. * TILE_SCALE as f32,
            1.0,
            Color::RAYWHITE,
        );

        if dialogue_h.current_phrase < dialogue_h.dialogue.len() {
            rl.draw_texture_ex(
                texture_handler.get_safe("veles"),
                Vector2::new(0., SCREEN_HEIGHT as f32 - 48. * TILE_SCALE as f32),
                0.0,
                TILE_SCALE as f32,
                Color::WHITE,
            );

            rl.draw_rectangle(
                32 * TILE_SCALE + 16,
                SCREEN_HEIGHT - 3 * 8 * TILE_SCALE - 20,
                SCREEN_WIDTH - 32 * TILE_SCALE - 32,
                3 * 8 * TILE_SCALE + 10,
                Color::BLACK.alpha(0.5),
            );

            if dialogue_h.dialogue_counter
                < dialogue_h.dialogue[dialogue_h.current_phrase]
                    .1
                    .chars()
                    .count()
            {
                dialogue_h.dialogue_counter += 1;
			}

			let line = &mut dialogue_h.dialogue[dialogue_h.current_phrase].1.chars().rev();
			let line_len = dialogue_h.dialogue[dialogue_h.current_phrase].1.chars().count();
			for _ in 0..line_len-dialogue_h.dialogue_counter {
				line.next();
			}

            rl.draw_text_ex(
                font,
                &line.rev().collect::<String>(),
                Vector2::new(
                    32. * TILE_SCALE as f32 + 32.,
                    SCREEN_HEIGHT as f32 - 3. * 8. * TILE_SCALE as f32 - 20.,
                ),
                8. * TILE_SCALE as f32,
                0.,
                Color::RAYWHITE,
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

        rl.draw_rectangle(
            SCREEN_WIDTH / 3,
            SCREEN_HEIGHT / 3,
            SCREEN_WIDTH / 3,
            SCREEN_HEIGHT / 3,
            Color::BLACK.alpha(0.5),
        );

        rl.draw_text_ex(
            font,
            "Выйти в меню?",
            Vector2::new(
                SCREEN_WIDTH as f32 / 3. + 8. * 13.,
                SCREEN_HEIGHT as f32 / 3. + 10.,
            ),
            64.,
            2.,
            Color::RAYWHITE,
        );

        let mouse_over = unsafe {
            CheckCollisionPointRec(
                (rl.get_mouse_position()
                    - Vector2::new(
                        rl.get_screen_width() as f32 / 2. - SCREEN_WIDTH as f32 / 2.,
                        rl.get_screen_height() as f32 / 2. - SCREEN_HEIGHT as f32 / 2.,
                    ))
                .into(),
                QUIT_BUTTON.into(),
            )
        };

        rl.draw_rectangle_rec(
            QUIT_BUTTON,
            if mouse_over {
                Color::RAYWHITE
            } else {
                Color::BLACK.alpha(0.5)
            },
        );

        rl.draw_text_ex(
            font,
            "Да",
            Vector2::new(
                QUIT_BUTTON.x + QUIT_BUTTON.width / 2. - 32.,
                QUIT_BUTTON.y + 16.,
            ),
            64.,
            2.,
            if mouse_over {
                Color::BLACK
            } else {
                Color::RAYWHITE
            },
        );
    }
}
