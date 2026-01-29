use std::cmp::min;

use raylib::prelude::*;

use crate::{
    SCREEN_HEIGHT, SCREEN_WIDTH,
    color::CustomColor,
    enemy_spirit::EnemiesHandler,
    level_transition::LevelTransition,
    map::{Level, TILE_SCALE_DEFAULT},
    metadata_handler::MetadataHandler,
    save_handler::SaveHandler,
    scene::{Scene, SceneHandler},
    settings::{MAXIMUM_PIXEL_SCALE, Settings, SettingsHandler},
    spirits_handler::SpiritsHandler,
    texture_handler::TextureHandler,
    ui::{Button, UIHandler, get_text_size},
};

#[derive(PartialEq, Clone, Copy)]
pub enum SliderStyle {
    Volume,
    Ruler,
}

#[derive(PartialEq, Clone, Copy)]
pub enum SettingsOptions {
    Shader,
    Fullscreen,
    MusicVolume,
    SoundVolume,
    Resolution,
}

const SLIDER_WIDTH_PX: u8 = 48;
const SLIDER_HEIGHT_PX: u8 = 16;
const RULER_WIDTH_PX: u8 = 48;
const RULER_PICKER_WIDTH_PX: u8 = 16;
const SLIDER_TEXTURE_OFFSET: u8 = 16;
const BUTTON_TEXTURE_WIDTH: f32 = 16.;
const UI_X_OFFSET: f32 = 128.;
const UI_Y_OFFSET: f32 = 14.;
const UI_SHIFT_SIZE: f32 = 20.;
const UI_Y_TOP_OFFSET: f32 = 1.;
const TEXT_X_OFFSET: f32 = 32.;
const TEXT_SIZE: f32 = 12.;
const TEXT_SPACING: f32 = 1.05;

const UI_UTILITY_X_OFFSET: u8 = 112;
const UI_UTILITY_Y_OFFSET: u8 = 128;
const UI_UTILITY_WIDTH: u8 = 64;
const UI_UTILITY_HEIGHT: u8 = 16;

const UI_WARNING_X_OFFSET: u8 = 80;
const UI_WARNING_Y_OFFSET: u8 = 105;
const UI_WARNING_X_SHIFT: u8 = 61;
const UI_WARNING_WIDTH: u8 = 32;
const UI_WARNING_HEIGHT: u8 = 16;
//64. 26.

const BACKGROUND_COLOR_HEX: &str = "0b8a8f";
const BACKGROUND_IMAGE: &str = "settings_bg";
const SETTINGS_BUTTON_TEXTURE: &str = "settings_button";
const SETTINGS_UI_TEXTURE: &str = "pause_menu";

const BUTTONS_SETTINGS: [&str; 2] = ["Шейдер", "Полный экран"];
const SLIDERS_SETTINGS: [&str; 3] = ["Громкость музыки", "Громкость звуков", "Разрешение"];

const WARNING_TEXT: [&str; 3] = ["Вы точно хотите выйти", "без сохранения", "настроек?"];
const WARNING_BUTTONS_TEXT: [&str; 2] = ["Да", "Еще подумаю"];

const UTILITY_BUTTONS: [&str; 2] = ["Выйти", "Сохранить настройки"];
const UTILITY_BUTTONS_TEXTURE: &str = "main_menu_buttons";

const SETTINGS_OPTIONS: [SettingsOptions; BUTTONS_SETTINGS.len() + SLIDERS_SETTINGS.len()] = [
    SettingsOptions::Shader,
    SettingsOptions::Fullscreen,
    SettingsOptions::MusicVolume,
    SettingsOptions::SoundVolume,
    SettingsOptions::Resolution,
];

const PIXEL_SCALE_TO_SLIDER_VALUE: f32 = 100. / MAXIMUM_PIXEL_SCALE as f32;

const SLIDER_TYPES: [SliderStyle; 3] =
    [SliderStyle::Volume, SliderStyle::Volume, SliderStyle::Ruler];

fn find_nearest(values: Vec<usize>, value: usize) -> usize {
    let mut nearest = values[0];

    for i in 0..values.len() {
        let temp = value.abs_diff(values[i]);

        if temp < value.abs_diff(nearest) {
            nearest = values[i];
        }
    }
    return nearest;
}

impl SliderStyle {
    pub fn get_sprite_offset(slider_style: &SliderStyle) -> Vector2 {
        return match *slider_style {
            SliderStyle::Ruler => Vector2::new(0., 0.),
            SliderStyle::Volume => Vector2::new(-1., -2.),
        };
    }
    fn get_snap_points(slider_style: &SliderStyle) -> Vec<usize> {
        return match *slider_style {
            SliderStyle::Ruler => vec![0, 46, 100],
            _ => panic!("not implemented yet!"),
        };
    }
    fn get_snap(slider_style: &SliderStyle) -> bool {
        return match *slider_style {
            SliderStyle::Ruler => true,
            _ => false,
        };
    }
    fn special_size_picker(slider_style: &SliderStyle) -> bool {
        return match *slider_style {
            SliderStyle::Ruler => true,
            _ => false,
        };
    }
    fn get_picker_size(slider_style: &SliderStyle) -> (usize, usize) {
        return match *slider_style {
            SliderStyle::Ruler => (RULER_PICKER_WIDTH_PX as usize, SLIDER_HEIGHT_PX as usize),
            _ => unimplemented!("Not implemented yet"),
        };
    }

    fn get_picker_pressed_texture(slider_style: &SliderStyle) -> (u8, u8) {
        return match *slider_style {
            SliderStyle::Ruler => (16, 16),
            _ => unimplemented!("Not implemented yet"),
        };
    }

    fn get_picker_offset_according_to_slider(slider_style: &SliderStyle) -> f32 {
        return match *slider_style {
            SliderStyle::Ruler => -6.,
            SliderStyle::Volume => 0.,
            //_ => unimplemented!("Not implemented yet"),
        };
    }
    fn get_picker_offset_according_to_slider_pressed(slider_style: &SliderStyle) -> f32 {
        return match *slider_style {
            SliderStyle::Ruler => -7.,
            SliderStyle::Volume => 1.,
            //_ => unimplemented!("Not implemented yet"),
        };
    }

    fn get_picker_rect(slider_style: &SliderStyle) -> usize {
        return match *slider_style {
            SliderStyle::Volume => 1,
            SliderStyle::Ruler => 1,
        };
    }
    fn get_outline_rect(slider_style: &SliderStyle) -> usize {
        return match *slider_style {
            SliderStyle::Volume => 0,
            SliderStyle::Ruler => 0,
        };
    }

    fn get_texture_name(slider_style: &SliderStyle) -> &str {
        return match *slider_style {
            SliderStyle::Volume => "volume_slider",
            SliderStyle::Ruler => "ruler_slider",
        };
    }
    fn get_dimensions(slider_style: &SliderStyle) -> (usize, usize) {
        return match *slider_style {
            SliderStyle::Volume => (SLIDER_WIDTH_PX as usize, SLIDER_HEIGHT_PX as usize),
            SliderStyle::Ruler => (RULER_WIDTH_PX as usize, SLIDER_HEIGHT_PX as usize),
        };
    }
    fn get_sprite_parts_amount(slider_style: &SliderStyle) -> u8 {
        return match *slider_style {
            SliderStyle::Volume => 2,
            SliderStyle::Ruler => 2,
        };
    }

    fn get_rects(
        slider_style: &SliderStyle,
        multiplier: usize,
        initial_position: Vector2,
    ) -> Vec<Rectangle> {
        let mut vector: Vec<Rectangle> = vec![];
        let need_special_size = SliderStyle::special_size_picker(slider_style);

        for index in 0..Self::get_sprite_parts_amount(slider_style) {
            let (width, height) = if need_special_size
                && SliderStyle::get_picker_rect(slider_style) == index as usize
            {
                SliderStyle::get_picker_size(slider_style)
            } else {
                SliderStyle::get_dimensions(slider_style)
            };
            vector.push(Rectangle {
                x: initial_position.x,
                y: initial_position.y,
                width: (width * multiplier) as f32,
                height: (height * multiplier) as f32,
            });
        }

        return vector;
    }
}

pub struct Slider {
    slider_value: u8,
    snap: bool,
    slider_style: SliderStyle,
    rects: Vec<Rectangle>,
}

impl Slider {
    pub fn new(slider_style: SliderStyle, start_position: Vector2) -> Self {
        let rects =
            SliderStyle::get_rects(&slider_style, TILE_SCALE_DEFAULT as usize, start_position);

        Self {
            slider_value: 50,
            snap: SliderStyle::get_snap(&slider_style),
            slider_style,
            rects,
        }
    }
}

pub struct SettingsMenuHandler {
    previous_scene: Option<Scene>,
    picked_element: Option<usize>,
    buttons: Vec<Button>,
    sliders: Vec<Slider>,
    ui_buttons: Vec<Button>,
    in_menu_settings: Settings,
    draw_warning: bool,
}
impl SettingsMenuHandler {
    pub fn new() -> Self {
        let mut buttons = Vec::new();
        let mut sliders = Vec::new();
        let mut buttons_utility = Vec::new();

        for index in 0..BUTTONS_SETTINGS.len() {
            buttons.push(Button {
                selected: false,
                rect: Rectangle {
                    x: TILE_SCALE_DEFAULT as f32 * UI_X_OFFSET,
                    y: (UI_Y_OFFSET + UI_Y_TOP_OFFSET + index as f32 * UI_SHIFT_SIZE)
                        * TILE_SCALE_DEFAULT as f32,
                    width: BUTTON_TEXTURE_WIDTH * TILE_SCALE_DEFAULT as f32,
                    height: BUTTON_TEXTURE_WIDTH * TILE_SCALE_DEFAULT as f32,
                },
                offset: 0.,
            });
        }

        for index in 0..UTILITY_BUTTONS.len() {
            buttons_utility.push(Button {
                selected: false,
                rect: Rectangle {
                    x: (TILE_SCALE_DEFAULT * UI_UTILITY_X_OFFSET as i32) as f32 * index as f32,
                    y: index as f32 * (UI_UTILITY_Y_OFFSET as i32 * TILE_SCALE_DEFAULT) as f32,
                    width: (UI_UTILITY_WIDTH as i32 * TILE_SCALE_DEFAULT) as f32,
                    height: (UI_UTILITY_HEIGHT as i32 * TILE_SCALE_DEFAULT) as f32,
                },
                offset: 0.,
            });
        }
        for index in 0..WARNING_BUTTONS_TEXT.len() {
            buttons_utility.push(Button {
                selected: false,
                rect: Rectangle {
                    x: (TILE_SCALE_DEFAULT * UI_WARNING_X_OFFSET as i32) as f32
                        + (UI_WARNING_X_SHIFT as usize * TILE_SCALE_DEFAULT as usize * index)
                            as f32,
                    y: (TILE_SCALE_DEFAULT * UI_WARNING_Y_OFFSET as i32) as f32,
                    width: (UI_WARNING_WIDTH as i32 * TILE_SCALE_DEFAULT) as f32,
                    height: (UI_WARNING_HEIGHT as i32 * TILE_SCALE_DEFAULT) as f32,
                },
                offset: 0.,
            });
        }

        for (index, slider_type) in SLIDER_TYPES.iter().enumerate() {
            sliders.push(Slider::new(
                *slider_type,
                Vector2::new(
                    UI_X_OFFSET * TILE_SCALE_DEFAULT as f32,
                    (UI_Y_OFFSET
                        + UI_Y_TOP_OFFSET
                        + (index + BUTTONS_SETTINGS.len()) as f32 * UI_SHIFT_SIZE)
                        * TILE_SCALE_DEFAULT as f32,
                ) + SliderStyle::get_sprite_offset(slider_type)
                    * Vector2::new(TILE_SCALE_DEFAULT as f32, TILE_SCALE_DEFAULT as f32),
            ));
        }

        return Self {
            picked_element: None,
            previous_scene: None,
            buttons,
            ui_buttons: buttons_utility,
            sliders,
            in_menu_settings: Settings::default(),
            draw_warning: false,
        };
    }

    fn set_setting_button(settings: &mut Settings, settings_option: SettingsOptions, value: bool) {
        match settings_option {
            SettingsOptions::Fullscreen => settings.fullscreen = value,
            SettingsOptions::Shader => settings.shader = value,
            _ => panic!("Not implemented yet!"),
        };
    }

    fn set_setting_slider(settings: &mut Settings, settings_option: SettingsOptions, value: u8) {
        match settings_option {
            SettingsOptions::SoundVolume => settings.sound = value as f32,
            SettingsOptions::MusicVolume => settings.music = value as f32,
            SettingsOptions::Resolution => settings.pixel_scale = value,
            _ => panic!("Not implemented yet!"),
        };
    }

    pub fn align_settings(&mut self, settings: &Settings) {
        for (index, button) in self.buttons.iter_mut().enumerate() {
            match SETTINGS_OPTIONS[index] {
                SettingsOptions::Shader => {
                    self.in_menu_settings.shader = settings.shader;
                    button.selected = settings.shader;
                }
                SettingsOptions::Fullscreen => {
                    self.in_menu_settings.shader = settings.fullscreen;
                    button.selected = settings.fullscreen;
                }
                _ => panic!("Not implemented yet!"),
            };
        }

        for (index, slider) in self.sliders.iter_mut().enumerate() {
            match SETTINGS_OPTIONS[index + BUTTONS_SETTINGS.len()] {
                SettingsOptions::MusicVolume => {
                    self.in_menu_settings.music = settings.music;
                    slider.slider_value = settings.music as u8;
                }
                SettingsOptions::SoundVolume => {
                    self.in_menu_settings.sound = settings.sound;
                    slider.slider_value = settings.sound as u8;
                }
                SettingsOptions::Resolution => {
                    self.in_menu_settings.pixel_scale = settings.pixel_scale;
                    slider.slider_value = settings.pixel_scale * PIXEL_SCALE_TO_SLIDER_VALUE as u8;
                }
                _ => panic!("Not implemented yet!"),
            };
        }
    }

    pub fn set_scene(&mut self, scene: Scene) {
        self.previous_scene = Some(scene);
    }

    pub fn check_scene(&mut self) -> bool {
        return self.previous_scene.is_some();
    }

    #[profiling::function]
    pub fn update(
        &mut self,
        scene_handler: &mut SceneHandler,
        _should_close: &mut bool,
        rl: &mut RaylibHandle,
        _save_handler: &mut SaveHandler,
        _level_number: &mut u8,
        _metadata_handler: &mut MetadataHandler,
        _level: &mut Level,
        _spirits_handler: &mut SpiritsHandler,
        _enemies_handler: &mut EnemiesHandler,
        _ui_handler: &mut UIHandler,
        _level_transition: &mut LevelTransition,
        settings_handler: &mut SettingsHandler,
    ) {
        let mut index = 0;
        for (i, button) in self.ui_buttons.iter_mut().enumerate() {
            if button.selected
                && rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT)
                && button.rect.check_collision_point_rec(
                    rl.get_mouse_position()
                        - Vector2::new(
                            rl.get_screen_width() as f32 / 2. - SCREEN_WIDTH as f32 / 2.,
                            rl.get_screen_height() as f32 / 2. - SCREEN_HEIGHT as f32 / 2.,
                        ),
                )
            {
                let warning = self.draw_warning;
                button.selected = false;
                match i {
                    0 => {
                        if warning {
                            continue;
                        }
                        if *settings_handler.get_settings() != self.in_menu_settings {
                            self.draw_warning = true;
                            return;
                        }
                        scene_handler.set(self.previous_scene.unwrap());
                        self.previous_scene = None;
                    }
                    1 => {
                        if warning {
                            continue;
                        }
                        settings_handler.set_settings(&self.in_menu_settings);
                    }
                    2 => {
                        self.draw_warning = false;
                        scene_handler.set(self.previous_scene.unwrap());
                        self.previous_scene = None;
                    }
                    3 => {
                        self.draw_warning = false;
                    }
                    _ => break,
                };
            }
        }
        if self.draw_warning {
            return;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) && self.previous_scene.is_some() {
            scene_handler.set(self.previous_scene.unwrap());
            self.previous_scene = None;
        }

        for button in self.buttons.iter_mut() {
            if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT)
                && self.picked_element.is_some_and(|b| b == index)
            {
                self.picked_element = None;
                if button.rect.check_collision_point_rec(
                    rl.get_mouse_position()
                        - Vector2::new(
                            rl.get_screen_width() as f32 / 2. - SCREEN_WIDTH as f32 / 2.,
                            rl.get_screen_height() as f32 / 2. - SCREEN_HEIGHT as f32 / 2.,
                        ),
                ) {
                    button.selected = !button.selected;
                    SettingsMenuHandler::set_setting_button(
                        &mut self.in_menu_settings,
                        SETTINGS_OPTIONS[index],
                        button.selected,
                    );
                }
            }
            index += 1;
        }

        if self.picked_element.is_some_and(|b| b >= index)
            && rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT)
        {
            let val = self.picked_element.unwrap();
            self.picked_element = None;

            if self.sliders[val - BUTTONS_SETTINGS.len()].snap {
                self.sliders[val - BUTTONS_SETTINGS.len()].slider_value = find_nearest(
                    SliderStyle::get_snap_points(&SliderStyle::Ruler),
                    self.sliders[val - BUTTONS_SETTINGS.len()].slider_value as usize,
                ) as u8;
            }
        };

        for slider in self.sliders.iter_mut() {
            let outline_rect_n = SliderStyle::get_outline_rect(&slider.slider_style);
            if slider.rects[outline_rect_n].check_collision_point_rec(
                rl.get_mouse_position()
                    - Vector2::new(
                        rl.get_screen_width() as f32 / 2. - SCREEN_WIDTH as f32 / 2.,
                        rl.get_screen_height() as f32 / 2. - SCREEN_HEIGHT as f32 / 2.,
                    ),
            ) && self.picked_element.is_none()
                && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
            {
                self.picked_element = Some(index);
            }

            if self.picked_element.is_some_and(|b| b != index) || self.picked_element.is_none() {
                index += 1;
                continue;
            };
            let mouse_cords = rl.get_mouse_position()
                - Vector2::new(
                    rl.get_screen_width() as f32 / 2. - SCREEN_WIDTH as f32 / 2.,
                    rl.get_screen_height() as f32 / 2. - SCREEN_HEIGHT as f32 / 2.,
                );
            let maxlen = (SLIDER_WIDTH_PX as i32 * TILE_SCALE_DEFAULT) as f32;
            let slider_value = if mouse_cords.x < slider.rects[outline_rect_n].x {
                0.
            } else {
                if (slider.rects[outline_rect_n].x + maxlen) < mouse_cords.x {
                    100.
                } else {
                    (mouse_cords.x - slider.rects[outline_rect_n].x) * 100. / maxlen
                }
            };

            slider.slider_value = slider_value as u8;
            SettingsMenuHandler::set_setting_slider(
                &mut self.in_menu_settings,
                SETTINGS_OPTIONS[index],
                slider.slider_value,
            );
            index += 1;
        }
    }

    #[profiling::function]
    pub fn draw(
        &mut self,
        font: &Font,
        _save_handler: &SaveHandler,
        texture_handler: &TextureHandler,
        rl: &mut RaylibDrawHandle,
    ) {
        rl.clear_background(Color::from_hex(BACKGROUND_COLOR_HEX).unwrap());

        rl.draw_texture_ex(
            texture_handler.get_safe(BACKGROUND_IMAGE),
            Vector2::zero(),
            0.0,
            TILE_SCALE_DEFAULT as f32,
            Color::WHITE,
        );

        let mut index = 0;

        for (button_num, button) in self.buttons.iter().enumerate() {
            let mouse_down = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);

            let texture_offset = if button.rect.check_collision_point_rec(
                rl.get_mouse_position()
                    - Vector2::new(
                        rl.get_screen_width() as f32 / 2. - SCREEN_WIDTH as f32 / 2.,
                        rl.get_screen_height() as f32 / 2. - SCREEN_HEIGHT as f32 / 2.,
                    ),
            ) && mouse_down
                && self.picked_element.is_none()
                && !self.draw_warning
            {
                self.picked_element = Some(index);
                BUTTON_TEXTURE_WIDTH
            } else {
                if self.picked_element.is_some_and(|b| b == index)
                    && mouse_down
                    && !self.draw_warning
                {
                    BUTTON_TEXTURE_WIDTH
                } else {
                    0.
                }
            };

            let button_state = if button.selected {
                BUTTON_TEXTURE_WIDTH
            } else {
                0.
            };

            rl.draw_texture_pro(
                texture_handler.get_safe(SETTINGS_BUTTON_TEXTURE),
                Rectangle::new(
                    texture_offset,
                    button_state,
                    BUTTON_TEXTURE_WIDTH,
                    BUTTON_TEXTURE_WIDTH,
                ),
                button.rect,
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );

            rl.draw_text_pro(
                font,
                BUTTONS_SETTINGS[button_num],
                Vector2::new(
                    TEXT_X_OFFSET * TILE_SCALE_DEFAULT as f32,
                    (UI_Y_OFFSET + UI_Y_TOP_OFFSET + index as f32 * UI_SHIFT_SIZE)
                        * TILE_SCALE_DEFAULT as f32,
                ),
                Vector2::zero(),
                0.,
                TEXT_SIZE * TILE_SCALE_DEFAULT as f32,
                TEXT_SPACING * TILE_SCALE_DEFAULT as f32,
                Color::RAYWHITE,
            );

            index += 1;
        }
        for (slider_num, slider) in self.sliders.iter_mut().enumerate() {
            for i in 0..slider.rects.len() {
                let (mut width, mut height) = SliderStyle::get_dimensions(&slider.slider_style);
                let mut x = 0;
                let mut y = i as u8 * SLIDER_TEXTURE_OFFSET;
                let mut picker_offset =
                    SliderStyle::get_picker_offset_according_to_slider(&slider.slider_style);

                match slider.slider_style {
                    SliderStyle::Ruler => {
                        if i == SliderStyle::get_picker_rect(&SliderStyle::Ruler) {
                            (height, width) = SliderStyle::get_picker_size(&slider.slider_style);
                            let new_val = if slider.slider_value as f32 > 95. {
                                95.
                            } else {
                                slider.slider_value as f32
                            };

                            if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT)
                                && self.picked_element.is_some_and(|b| b == index + slider_num)
                            {
                                picker_offset =
                                    SliderStyle::get_picker_offset_according_to_slider_pressed(
                                        &SliderStyle::Ruler,
                                    );

                                (x, y) =
                                    SliderStyle::get_picker_pressed_texture(&SliderStyle::Ruler);
                            }
                            slider.rects[i].x =
                                slider.rects[SliderStyle::get_outline_rect(&SliderStyle::Ruler)].x
                                    + (picker_offset as i32 * TILE_SCALE_DEFAULT) as f32
                                    + (SLIDER_WIDTH_PX as f32 * new_val / 100.).floor()
                                        * TILE_SCALE_DEFAULT as f32;
                        }
                    }
                    SliderStyle::Volume => {
                        if i == SliderStyle::get_picker_rect(&slider.slider_style) {
                            width = (SLIDER_WIDTH_PX as f32 * slider.slider_value as f32 / 100.)
                                .floor() as usize;

                            slider.rects[i].width = width as f32 * TILE_SCALE_DEFAULT as f32;
                        }
                    }
                };

                rl.draw_texture_pro(
                    texture_handler.get_safe(SliderStyle::get_texture_name(&slider.slider_style)),
                    Rectangle::new(x as f32, y as f32, width as f32, height as f32),
                    slider.rects[i],
                    Vector2::zero(),
                    0.0,
                    Color::WHITE,
                );
            }
            rl.draw_text_pro(
                font,
                SLIDERS_SETTINGS[slider_num],
                Vector2::new(
                    TEXT_X_OFFSET * TILE_SCALE_DEFAULT as f32,
                    (UI_Y_OFFSET
                        + UI_Y_TOP_OFFSET
                        + (slider_num + BUTTONS_SETTINGS.len()) as f32 * UI_SHIFT_SIZE)
                        * TILE_SCALE_DEFAULT as f32,
                ),
                Vector2::zero(),
                0.,
                TEXT_SIZE * TILE_SCALE_DEFAULT as f32,
                TEXT_SPACING * TILE_SCALE_DEFAULT as f32,
                Color::RAYWHITE,
            );
        }

        for i in 0..UTILITY_BUTTONS.len() {
            let mouse_down = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
            let texture_offset = if self.ui_buttons[i].rect.check_collision_point_rec(
                rl.get_mouse_position()
                    - Vector2::new(
                        rl.get_screen_width() as f32 / 2. - SCREEN_WIDTH as f32 / 2.,
                        rl.get_screen_height() as f32 / 2. - SCREEN_HEIGHT as f32 / 2.,
                    ),
            ) && mouse_down
                && self.picked_element.is_none()
                && !self.draw_warning
            {
                self.ui_buttons[i].selected = true;
                UI_UTILITY_HEIGHT as f32
            } else {
                0.
            };

            rl.draw_texture_pro(
                texture_handler.get_safe(UTILITY_BUTTONS_TEXTURE),
                Rectangle::new(
                    0.,
                    texture_offset,
                    UI_UTILITY_WIDTH as f32,
                    UI_UTILITY_HEIGHT as f32,
                ),
                self.ui_buttons[i].rect,
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
        }

        if !self.draw_warning {
            return;
        }

        rl.draw_texture_ex(
            texture_handler.get_safe(SETTINGS_UI_TEXTURE),
            Vector2::new(
                64. * TILE_SCALE_DEFAULT as f32,
                26. * TILE_SCALE_DEFAULT as f32,
            ),
            0.0,
            TILE_SCALE_DEFAULT as f32,
            Color::WHITE,
        );

        for i in 0..WARNING_TEXT.len() {
            let text_size = get_text_size(font, WARNING_TEXT[i], TEXT_SIZE, TEXT_SPACING);
            rl.draw_text_pro(
                font,
                WARNING_TEXT[i],
                Vector2::new(
                    64. * TILE_SCALE_DEFAULT as f32 - text_size.x / 2.
                        + 32. * TILE_SCALE_DEFAULT as f32,
                    26. * TILE_SCALE_DEFAULT as f32 - text_size.y / 2.
                        + 8. * TILE_SCALE_DEFAULT as f32
                        + i as f32 * TILE_SCALE_DEFAULT as f32 * text_size.y,
                ),
                Vector2::zero(),
                0.,
                TEXT_SIZE * TILE_SCALE_DEFAULT as f32,
                TEXT_SPACING * TILE_SCALE_DEFAULT as f32,
                CustomColor::BLACK_TEXT,
            );
        }

        for i in 0..WARNING_BUTTONS_TEXT.len() {
            let mouse_down = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
            let texture_offset = if self.ui_buttons[i + UTILITY_BUTTONS.len()]
                .rect
                .check_collision_point_rec(
                    rl.get_mouse_position()
                        - Vector2::new(
                            rl.get_screen_width() as f32 / 2. - SCREEN_WIDTH as f32 / 2.,
                            rl.get_screen_height() as f32 / 2. - SCREEN_HEIGHT as f32 / 2.,
                        ),
                )
                && mouse_down
                && self.picked_element.is_none()
            {
                self.ui_buttons[i + UTILITY_BUTTONS.len()].selected = true;
                UI_UTILITY_HEIGHT as f32
            } else {
                0.
            };

            rl.draw_texture_pro(
                texture_handler.get_safe(UTILITY_BUTTONS_TEXTURE),
                Rectangle::new(
                    0.,
                    texture_offset,
                    UI_WARNING_WIDTH as f32,
                    UI_WARNING_HEIGHT as f32,
                ),
                self.ui_buttons[i + WARNING_BUTTONS_TEXT.len()].rect,
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );

            let text_size = get_text_size(font, WARNING_BUTTONS_TEXT[i], TEXT_SIZE, TEXT_SPACING);

            rl.draw_text_pro(
                font,
                WARNING_BUTTONS_TEXT[i],
                Vector2::new(
                    self.ui_buttons[i + WARNING_BUTTONS_TEXT.len()].rect.x - text_size.x / 2.
                        + 32. * TILE_SCALE_DEFAULT as f32,
                    self.ui_buttons[i + WARNING_BUTTONS_TEXT.len()].rect.y - text_size.y / 2.
                        + 8. * TILE_SCALE_DEFAULT as f32,
                ),
                Vector2::zero(),
                0.,
                TEXT_SIZE * TILE_SCALE_DEFAULT as f32,
                TEXT_SPACING * TILE_SCALE_DEFAULT as f32,
                CustomColor::BLACK_TEXT,
            );
        }
    }
}
