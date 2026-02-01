use std::{cmp::min, ffi::CString};

use raylib::{
    ffi::{CheckCollisionPointRec, MeasureTextEx},
    prelude::*,
};

use crate::{
    SCREEN_HEIGHT, SCREEN_WIDTH,
    color::CustomColor,
    dialogue::DialogueHandler,
    hotkey_handler::{HotkeyCategory, HotkeyHandler},
    map::{Level, TILE_SIZE_PX, TileType},
    scene::{Scene, SceneHandler},
    settings::SettingsHandler,
    texture_handler::TextureHandler,
};

const STATISTICS_BAR_TEXTURE: &str = "stat_bar";
const STATISTICS_TEXT_X_OFFSET: f32 = 16.;
const STATISTICS_TEXT_Y_OFFSET: f32 = 1.5;
const STATISTICS_TEXT_SHIFT: f32 = 12.;

const DIALOGUE_BOX_TEXTURE: &str = "dialogue_box";

const BAR_TEXT_SIZE: f32 = 8.;
const BAR_TEXT_SPACING: f32 = 1.;
const BAR_X_OFFSET: f32 = 6.;
const BAR_Y_OFFSET: f32 = 5.;

const SPIRIT_ICON_TEXTURE: &str = "spirit_icon";
const WOOD_ICON_TEXTURE: &str = "wood_icon";
const WOOD_ICON_OFFSET_Y: f32 = 12.;

const BUTTON_LABELS: [&str; 3] = ["fire_td", "fire_lr", "fire_stop"];
const PAUSE_BUTTON_LABELS: [&str; 3] = ["Заново", "Настройки", "Выйти"];

const BUTTONS_X_OFFSET: f32 = BUTTON_TEXTURE_WIDTH / 2.;
const BUTTONS_Y_OFFSET: f32 = -BUTTON_TEXTURE_HEIGHT / 4.;
const BUTTON_TEXTURE_WIDTH: f32 = 16.;
const BUTTON_TEXTURE_HEIGHT: f32 = 16.;

const PAUSE_BUTTON_TEXTURE: &str = "game_buttons";
const PAUSE_BUTTON_Y_OFFSET: f32 = 3.;
const PAUSE_BUTTON_TEXTURE_WIDTH: f32 = 64.;
const PAUSE_BUTTON_TEXTURE_HEIGHT: f32 = 16.;

const PAUSE_TEXT_SIZE: f32 = 12.;
const PAUSE_TEXT_SPACING: f32 = 1.05;

const PANEL_TEXTURE: &str = "pause_menu";
const PANEL_WIDTH: f32 = 128.;
const PANEL_HEIGHT: f32 = 96.;
const PANEL_TEXT_Y_OFFSET: f32 = 7.;

pub fn get_text_size(
    font: &Font,
    text: &str,
    text_size_basic: f32,
    text_space_basic: f32,
) -> raylib::ffi::Vector2 {
    let ctext = CString::new(text).unwrap();
    return unsafe { MeasureTextEx(**font, ctext.as_ptr(), text_size_basic, text_space_basic) };
}

pub struct Button {
    pub rect: Rectangle,
    pub offset: f32,
    pub selected: bool,
}
impl Default for Button {
    fn default() -> Self {
        Self {
            rect: Rectangle::default(),
            offset: 0.,
            selected: false,
        }
    }
}
impl Button {
    pub fn draw_with_text_middle(
        &self,
        rl: &mut RaylibDrawHandle,
        text: &str,
        font: &Font,
        texture: &Texture2D,
        texture_rectangle: &Rectangle,
        text_dimensions: raylib::ffi::Vector2,
        color: &Color,
        text_size: f32,
        text_spacing: f32,
        text_offset_pressed: Vector2,
        texture_offset: Vector2,
    ) {
        rl.draw_texture_pro(
            texture,
            texture_rectangle,
            self.rect,
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );

        rl.draw_text_pro(
            font,
            text,
            Vector2::new(
                self.rect.x
                    + (self.rect.width - text_dimensions.x) / 2.
                    + text_offset_pressed.x
                    + texture_offset.x,
                self.rect.y
                    + (self.rect.height - text_dimensions.y) / 2.
                    + text_offset_pressed.y
                    + texture_offset.y,
            ),
            Vector2::zero(),
            0.,
            text_size,
            text_spacing,
            color,
        );
    }
}
pub struct UIHandler {
    build_buttons: Vec<Button>,
    pause_buttons: Vec<Button>,
    quitting: bool,
}

impl UIHandler {
    #[profiling::function]
    pub fn new(level_number: usize, scale: f32) -> Self {
        let mut build_buttons = Vec::new();

        let len = min(BUTTON_LABELS.len(), level_number);
        for _ in 0..len {
            build_buttons.push(Button::default());
        }

        let mut pause_buttons: Vec<Button> = Vec::new();
        for _ in 0..PAUSE_BUTTON_LABELS.len() {
            pause_buttons.push(Button::default());
        }

        Self::set_default(&mut build_buttons, &mut pause_buttons, scale);
        Self {
            build_buttons,
            quitting: false,
            pause_buttons,
        }
    }
    pub fn rescale_ui(&mut self, new_scale: f32) {
        Self::set_default(&mut self.build_buttons, &mut self.pause_buttons, new_scale);
    }
    fn set_default(build_buttons: &mut Vec<Button>, pause_buttons: &mut Vec<Button>, scale: f32) {
        let len = build_buttons.len();
        let is_len_odd = len % 2 != 0;

        for i in 0..len {
            let x = if is_len_odd {
                (SCREEN_WIDTH as f32 - BUTTON_TEXTURE_WIDTH) / 2. * scale
                    + (i as isize - (len - 1) as isize / 2) as f32
                        * scale
                        * (BUTTON_TEXTURE_WIDTH + BUTTONS_X_OFFSET)
            } else {
                let offset_from_center = if i < len / 2 {
                    -BUTTONS_X_OFFSET
                } else {
                    BUTTONS_X_OFFSET
                };
                (SCREEN_WIDTH as f32 / 2. + offset_from_center) * scale
                    + (i as isize - len as isize / 2) as f32 * scale * BUTTON_TEXTURE_WIDTH
            };

            build_buttons[i].rect.x = x;
            build_buttons[i].rect.y =
                (SCREEN_HEIGHT as f32 - BUTTON_TEXTURE_HEIGHT + BUTTONS_Y_OFFSET) * scale;
            build_buttons[i].rect.width = BUTTON_TEXTURE_WIDTH * scale;
            build_buttons[i].rect.height = BUTTON_TEXTURE_HEIGHT * scale;
        }

        for i in 0..pause_buttons.len() {
            pause_buttons[i].rect.x =
                (SCREEN_WIDTH as f32 - PAUSE_BUTTON_TEXTURE_WIDTH) / 2. * scale;
            pause_buttons[i].rect.y = (SCREEN_HEIGHT as f32 / 2.
                + (PAUSE_BUTTON_Y_OFFSET + BUTTON_TEXTURE_HEIGHT) * (i as f32 - 1.))
                * scale;
            pause_buttons[i].rect.width = PAUSE_BUTTON_TEXTURE_WIDTH * scale;
            pause_buttons[i].rect.height = PAUSE_BUTTON_TEXTURE_HEIGHT * scale;
        }
    }

    #[profiling::function]
    pub fn build(
        &mut self,
        level: &mut Level,
        rl: &mut RaylibHandle,
        hotkey_h: &mut HotkeyHandler,
        dialogue_h: &mut DialogueHandler,
        settings_handler: &mut SettingsHandler,
    ) {
        let dialoging = dialogue_h.current_phrase < dialogue_h.dialogue.len();

        let mut intent: HotkeyCategory;
        for (label_index, button) in self.build_buttons.iter_mut().enumerate() {
            if dialoging {
                break;
            }

            intent = HotkeyCategory::from_bonfire(BUTTON_LABELS[label_index]);

            if hotkey_h.check_pressed(rl, intent) {
                button.selected = true;
            }

            if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                if unsafe {
                    CheckCollisionPointRec(
                        (rl.get_mouse_position()
                            - Vector2::new(
                                rl.get_screen_width() as f32 / 2.
                                    - (SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32)
                                        as f32
                                        / 2.,
                                rl.get_screen_height() as f32 / 2.
                                    - (SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32)
                                        as f32
                                        / 2.,
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
                        rl.get_screen_width() as f32 / 2.
                            - (SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32) as f32
                                / 2.,
                        rl.get_screen_height() as f32 / 2.
                            - (SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32) as f32
                                / 2.,
                    ))
                    / (Vector2::one()
                        * (TILE_SIZE_PX * settings_handler.settings.pixel_scale as i32) as f32);
                let (x, y) = (pos.x as usize, pos.y as usize);

                if level.tiles[x][y] != TileType::Air {
                    button.selected = false;
                    continue;
                }

                let tile = match BUTTON_LABELS[label_index] {
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

    #[profiling::function]
    pub fn update(
        &mut self,
        hotkey_h: &mut HotkeyHandler,
        scene_h: &mut SceneHandler,
        dialogue_h: &mut DialogueHandler,
        rl: &mut RaylibHandle,
        settings_handler: &mut SettingsHandler,
    ) -> (bool, bool, bool) {
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
            return (false, false, false);
        }

        if self.pause_buttons[2].rect.check_collision_point_rec(
            rl.get_mouse_position()
                - Vector2::new(
                    rl.get_screen_width() as f32 / 2.
                        - (SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32) as f32 / 2.,
                    rl.get_screen_height() as f32 / 2.
                        - (SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32) as f32
                            / 2.,
                ),
        ) && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
        {
            scene_h.set(Scene::MainMenu);
            self.quitting = false;
            return (true, false, false);
        };

        if self.pause_buttons[0].rect.check_collision_point_rec(
            rl.get_mouse_position()
                - Vector2::new(
                    rl.get_screen_width() as f32 / 2.
                        - (SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32) as f32 / 2.,
                    rl.get_screen_height() as f32 / 2.
                        - (SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32) as f32
                            / 2.,
                ),
        ) && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
        {
            self.quitting = false;
            return (false, true, false);
        };

        if self.pause_buttons[1].rect.check_collision_point_rec(
            rl.get_mouse_position()
                - Vector2::new(
                    rl.get_screen_width() as f32 / 2.
                        - (SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32) as f32 / 2.,
                    rl.get_screen_height() as f32 / 2.
                        - (SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32) as f32
                            / 2.,
                ),
        ) && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
        {
            self.quitting = false;
            return (false, false, true);
        };

        return (false, false, false);
    }

    #[profiling::function]
    pub fn draw(
        &mut self,
        texture_handler: &TextureHandler,
        dialogue_h: &mut DialogueHandler,
        level: &mut Level,
        font: &Font,
        rl: &mut RaylibDrawHandle,
        settings_handler: &mut SettingsHandler,
    ) {
        let dialoging = dialogue_h.current_phrase < dialogue_h.dialogue.len();

        for (label_index, button) in self.build_buttons.iter_mut().enumerate() {
            if dialoging {
                break;
            }

            let target_offset = if button.rect.check_collision_point_rec(
                rl.get_mouse_position()
                    - Vector2::new(
                        rl.get_screen_width() as f32 / 2.
                            - (SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32) as f32
                                / 2.,
                        rl.get_screen_height() as f32 / 2.
                            - (SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32) as f32
                                / 2.,
                    ),
            ) {
                6. * settings_handler.settings.pixel_scale as f32
            } else {
                2. * settings_handler.settings.pixel_scale as f32
            };

            button.offset = lerp(button.offset, target_offset, 10. * rl.get_frame_time());

            rl.draw_texture_ex(
                texture_handler.get("pedestal"),
                Vector2::new(button.rect.x, button.rect.y),
                0.0,
                settings_handler.settings.pixel_scale as f32,
                Color::WHITE,
            );

            if !button.selected {
                let mut offset_rect = button.rect;
                offset_rect.y -= button.offset;

                rl.draw_texture_pro(
                    texture_handler.get(BUTTON_LABELS[label_index]),
                    Rectangle::new(
                        ((rl.get_time() * 8.) % 4.).floor() as f32 * BUTTON_TEXTURE_WIDTH,
                        BUTTON_TEXTURE_HEIGHT,
                        BUTTON_TEXTURE_WIDTH,
                        BUTTON_TEXTURE_HEIGHT,
                    ),
                    offset_rect,
                    Vector2::zero(),
                    0.0,
                    Color::WHITE,
                );
            } else {
                rl.draw_texture_pro(
                    texture_handler.get_safe(BUTTON_LABELS[label_index]),
                    Rectangle::new(
                        ((rl.get_time() * 8.) % 4.).floor() as f32 * BUTTON_TEXTURE_WIDTH,
                        BUTTON_TEXTURE_HEIGHT,
                        BUTTON_TEXTURE_WIDTH,
                        BUTTON_TEXTURE_HEIGHT,
                    ),
                    Rectangle::new(
                        (rl.get_mouse_position()
                            - Vector2::new(
                                rl.get_screen_width() as f32 / 2.
                                    - (SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32)
                                        as f32
                                        / 2.,
                                rl.get_screen_height() as f32 / 2.
                                    - (SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32)
                                        as f32
                                        / 2.,
                            ))
                        .x - ((TILE_SIZE_PX * settings_handler.settings.pixel_scale as i32) / 2)
                            as f32,
                        (rl.get_mouse_position()
                            - Vector2::new(
                                rl.get_screen_width() as f32 / 2.
                                    - (SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32)
                                        as f32
                                        / 2.,
                                rl.get_screen_height() as f32 / 2.
                                    - (SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32)
                                        as f32
                                        / 2.,
                            ))
                        .y - ((TILE_SIZE_PX * settings_handler.settings.pixel_scale as i32) / 2)
                            as f32,
                        (TILE_SIZE_PX * settings_handler.settings.pixel_scale as i32) as f32,
                        (TILE_SIZE_PX * settings_handler.settings.pixel_scale as i32) as f32,
                    ),
                    Vector2::zero(),
                    0.0,
                    Color::WHITE,
                );
            }
        }

        rl.draw_texture_ex(
            texture_handler.get(STATISTICS_BAR_TEXTURE),
            Vector2::one() * settings_handler.settings.pixel_scale as f32,
            0.0,
            settings_handler.settings.pixel_scale as f32,
            Color::WHITE,
        );

        let bar_offset = Vector2::new(
            BAR_X_OFFSET * settings_handler.settings.pixel_scale as f32,
            BAR_Y_OFFSET * settings_handler.settings.pixel_scale as f32,
        );

        rl.draw_texture_ex(
            texture_handler.get(SPIRIT_ICON_TEXTURE),
            bar_offset,
            0.0,
            settings_handler.settings.pixel_scale as f32,
            Color::WHITE,
        );

        rl.draw_text_ex(
            font,
            format!("{}/{}", level.survived, level.survive).as_str(),
            bar_offset
                + Vector2::new(
                    STATISTICS_TEXT_X_OFFSET * settings_handler.settings.pixel_scale as f32,
                    STATISTICS_TEXT_Y_OFFSET * settings_handler.settings.pixel_scale as f32,
                ),
            BAR_TEXT_SIZE * settings_handler.settings.pixel_scale as f32,
            BAR_TEXT_SPACING * settings_handler.settings.pixel_scale as f32,
            CustomColor::BLACK_TEXT,
        );

        rl.draw_texture_ex(
            texture_handler.get(WOOD_ICON_TEXTURE),
            bar_offset
                + Vector2::new(
                    0.,
                    WOOD_ICON_OFFSET_Y * settings_handler.settings.pixel_scale as f32,
                ),
            0.0,
            settings_handler.settings.pixel_scale as f32,
            Color::WHITE,
        );

        rl.draw_text_ex(
            font,
            format!("{}", level.get_wood()).as_str(),
            bar_offset
                + Vector2::new(
                    STATISTICS_TEXT_X_OFFSET * settings_handler.settings.pixel_scale as f32,
                    STATISTICS_TEXT_Y_OFFSET * settings_handler.settings.pixel_scale as f32
                        + STATISTICS_TEXT_SHIFT * settings_handler.settings.pixel_scale as f32,
                ),
            BAR_TEXT_SIZE * settings_handler.settings.pixel_scale as f32,
            BAR_TEXT_SPACING * settings_handler.settings.pixel_scale as f32,
            CustomColor::BLACK_TEXT,
        );

        if dialoging {
            let (speaker, line) = &mut dialogue_h.dialogue[dialogue_h.current_phrase];

            rl.draw_texture_ex(
                texture_handler.get_safe(speaker),
                Vector2::new(
                    0.,
                    (SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32) as f32
                        - 48. * settings_handler.settings.pixel_scale as f32,
                ),
                0.0,
                settings_handler.settings.pixel_scale as f32,
                Color::WHITE,
            );

            rl.draw_texture_ex(
                texture_handler.get(DIALOGUE_BOX_TEXTURE),
                Vector2::new(
                    32. * settings_handler.settings.pixel_scale as f32,
                    (SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32) as f32
                        - 48. * settings_handler.settings.pixel_scale as f32,
                ),
                0.0,
                settings_handler.settings.pixel_scale as f32,
                Color::WHITE,
            );

            if dialogue_h.dialogue_counter < line.chars().count() {
                dialogue_h.dialogue_counter += 1;
            }

            let mut temp_line = line.chars().rev();
            let line_len = line.chars().count();
            for _ in 0..line_len - dialogue_h.dialogue_counter {
                temp_line.next();
            }

            rl.draw_text_ex(
                font,
                &temp_line.rev().collect::<String>(),
                Vector2::new(
                    32. * settings_handler.settings.pixel_scale as f32 + 64.,
                    (SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32) as f32
                        - 3. * 8. * settings_handler.settings.pixel_scale as f32
                        - 20.,
                ),
                8. * settings_handler.settings.pixel_scale as f32,
                0.,
                CustomColor::BLACK_TEXT,
            );

            if dialogue_h.dialogue_accumulator.chars().count() >= line.chars().count() {
                rl.draw_text_ex(
                    font,
                    "Далее...",
                    Vector2::new(
                        (SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32) as f32
                            - 32. * settings_handler.settings.pixel_scale as f32,
                        (SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32) as f32
                            - 12. * settings_handler.settings.pixel_scale as f32,
                    ),
                    8. * settings_handler.settings.pixel_scale as f32,
                    0.,
                    Color::RAYWHITE.alpha((rl.get_time() * 2.).sin().abs() as f32),
                )
            }
        }

        if !self.quitting {
            return;
        }

        let panel_position = Vector2::new(
            (SCREEN_WIDTH as f32 - PANEL_WIDTH) * settings_handler.settings.pixel_scale as f32 / 2.,
            (SCREEN_HEIGHT as f32 - PANEL_HEIGHT) * settings_handler.settings.pixel_scale as f32
                / 2.,
        );

        rl.draw_texture_ex(
            texture_handler.get(PANEL_TEXTURE),
            panel_position,
            0.0,
            settings_handler.settings.pixel_scale as f32,
            Color::WHITE,
        );

        let text_size = get_text_size(
            font,
            "Меню",
            PAUSE_TEXT_SIZE * settings_handler.settings.pixel_scale as f32,
            PAUSE_TEXT_SPACING * settings_handler.settings.pixel_scale as f32,
        );

        rl.draw_text_ex(
            font,
            "Меню",
            panel_position
                + Vector2::new(
                    settings_handler.settings.pixel_scale as f32 * PAUSE_BUTTON_TEXTURE_WIDTH
                        - text_size.x / 2.,
                    settings_handler.settings.pixel_scale as f32 * PANEL_TEXT_Y_OFFSET
                        - text_size.y / 2.,
                ),
            settings_handler.settings.pixel_scale as f32 * 12.,
            PAUSE_TEXT_SPACING * settings_handler.settings.pixel_scale as f32,
            CustomColor::BLACK_TEXT,
        );

        for (index, button_rect) in self.pause_buttons.iter().enumerate() {
            let mouse_over = button_rect.rect.check_collision_point_rec(
                rl.get_mouse_position()
                    - Vector2::new(
                        rl.get_screen_width() as f32 / 2.
                            - (SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32) as f32
                                / 2.,
                        rl.get_screen_height() as f32 / 2.
                            - (SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32) as f32
                                / 2.,
                    ),
            );

            let (texture_offset, text_offset) =
                if mouse_over && rl.is_mouse_button_up(MouseButton::MOUSE_BUTTON_LEFT) {
                    (PAUSE_BUTTON_TEXTURE_HEIGHT, -2.)
                } else {
                    (0., 0.)
                };

            let source = Rectangle::new(
                0.,
                texture_offset,
                PAUSE_BUTTON_TEXTURE_WIDTH,
                PAUSE_BUTTON_TEXTURE_HEIGHT,
            );

            let text_size_button = get_text_size(
                font,
                PAUSE_BUTTON_LABELS[index],
                PAUSE_TEXT_SIZE * settings_handler.settings.pixel_scale as f32,
                PAUSE_TEXT_SPACING * settings_handler.settings.pixel_scale as f32,
            );

            button_rect.draw_with_text_middle(
                rl,
                PAUSE_BUTTON_LABELS[index],
                font,
                texture_handler.get_safe(PAUSE_BUTTON_TEXTURE),
                &source,
                text_size_button,
                &CustomColor::BLACK_TEXT,
                PAUSE_TEXT_SIZE * settings_handler.settings.pixel_scale as f32,
                PAUSE_TEXT_SPACING * settings_handler.settings.pixel_scale as f32,
                Vector2::new(
                    0.,
                    text_offset * settings_handler.settings.pixel_scale as f32,
                ),
                Vector2::zero(),
            );
        }
    }
}
