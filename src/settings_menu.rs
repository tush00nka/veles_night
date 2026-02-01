use raylib::prelude::*;

use crate::{
    SCREEN_HEIGHT, SCREEN_WIDTH,
    color::CustomColor,
    scene::{Scene, SceneHandler},
    settings::{MAXIMUM_PIXEL_SCALE, Settings, SettingsHandler},
    texture_handler::TextureHandler,
    ui::{Button, get_text_size},
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
    GeneralAudio,
    Resolution,
}

const SLIDER_WIDTH_PX: u8 = 48;
const SLIDER_HEIGHT_PX: u8 = 16;
const RULER_WIDTH_PX: u8 = 48;
const RULER_PICKER_WIDTH_PX: u8 = 16;
const SLIDER_TEXTURE_OFFSET: u8 = 16;
const BUTTON_TEXTURE_WIDTH: f32 = 16.;
const BUTTON_TEXTURE_HEIGHT: f32 = 16.;

const UI_X_OFFSET: f32 = 128.;
const UI_Y_OFFSET: f32 = 14.;
const UI_SHIFT_SIZE: f32 = 18.;
const UI_Y_TOP_OFFSET: f32 = 1.;
const TEXT_X_OFFSET: f32 = 32.;
const TEXT_SIZE: f32 = 12.;
const TEXT_SPACING: f32 = 1.25;

const UI_UTILITY_Y_OFFSET_SAVE_BUTTON: u8 = 124;
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
const SLIDERS_SETTINGS: [&str; 4] = [
    "Общая громкость",
    "Громкость музыки",
    "Громкость звуков",
    "Разрешение",
];

const WARNING_TEXT: [&str; 3] = ["Вы хотите выйти", "без сохранения", "настроек?"];
const WARNING_BUTTONS_TEXT: [&str; 2] = ["Да", "Нет"];

const UTILITY_BUTTONS: [&str; 2] = ["Назад", "Сохранить"];
const UTILITY_BUTTONS_TEXTURE: &str = "main_menu_buttons";

const SETTINGS_OPTIONS: [SettingsOptions; BUTTONS_SETTINGS.len() + SLIDERS_SETTINGS.len()] = [
    SettingsOptions::Shader,
    SettingsOptions::Fullscreen,
    SettingsOptions::GeneralAudio,
    SettingsOptions::MusicVolume,
    SettingsOptions::SoundVolume,
    SettingsOptions::Resolution,
];

const PIXEL_SCALE_TO_SLIDER_VALUE: f32 = 100. / MAXIMUM_PIXEL_SCALE as f32;

const SLIDER_TYPES: [SliderStyle; 4] = [
    SliderStyle::Volume,
    SliderStyle::Volume,
    SliderStyle::Volume,
    SliderStyle::Ruler,
];

fn find_nearest(values: Vec<usize>, value: usize) -> usize {
    let mut nearest = 0;

    for i in 0..values.len() {
        let temp = value.abs_diff(values[i]);

        if temp < value.abs_diff(values[nearest]) {
            nearest = i;
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
            SliderStyle::Volume => 4.,
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
    pub fn new(slider_style: SliderStyle, start_position: Vector2, pixel_scale: usize) -> Self {
        let rects = SliderStyle::get_rects(&slider_style, pixel_scale, start_position);

        Self {
            slider_value: 50,
            snap: SliderStyle::get_snap(&slider_style),
            slider_style,
            rects,
        }
    }

    fn back_to_default(&mut self, scale: f32, position: Vector2) {
        let need_special_size = SliderStyle::special_size_picker(&self.slider_style);

        for index in 0..SliderStyle::get_sprite_parts_amount(&self.slider_style) {
            let (width, height) = if need_special_size
                && SliderStyle::get_picker_rect(&self.slider_style) == index as usize
            {
                SliderStyle::get_picker_size(&self.slider_style)
            } else {
                SliderStyle::get_dimensions(&self.slider_style)
            };
            self.rects[index as usize].x = position.x;
            self.rects[index as usize].y = position.y;
            self.rects[index as usize].width = width as f32 * scale;
            self.rects[index as usize].height = height as f32 * scale;
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
    pub should_remade: bool,
}
impl SettingsMenuHandler {
    pub fn new(scale: f32) -> Self {
        let mut buttons = Vec::new();
        let mut sliders = Vec::new();
        let mut buttons_utility = Vec::new();

        for _ in 0..BUTTONS_SETTINGS.len() {
            buttons.push(Button {
                selected: false,
                rect: Rectangle::default(),
                offset: 0.,
            });
        }

        for _ in 0..UTILITY_BUTTONS.len() {
            buttons_utility.push(Button {
                selected: false,
                rect: Rectangle::default(),
                offset: 0.,
            });
        }
        for _ in 0..WARNING_BUTTONS_TEXT.len() {
            buttons_utility.push(Button {
                selected: false,
                rect: Rectangle::default(),
                offset: 0.,
            });
        }

        for (index, slider_type) in SLIDER_TYPES.iter().enumerate() {
            sliders.push(Slider::new(
                *slider_type,
                Vector2::new(
                    UI_X_OFFSET * scale as f32,
                    (UI_Y_OFFSET
                        + UI_Y_TOP_OFFSET
                        + (index + BUTTONS_SETTINGS.len()) as f32 * UI_SHIFT_SIZE)
                        * scale as f32,
                ) + SliderStyle::get_sprite_offset(slider_type)
                    * Vector2::new(scale as f32, scale as f32),
                scale as usize,
            ));
        }
        Self::set_ui_to_default(&mut buttons, &mut buttons_utility, &mut sliders, scale);
        return Self {
            picked_element: None,
            previous_scene: None,
            buttons,
            ui_buttons: buttons_utility,
            sliders,
            in_menu_settings: Settings::default(),
            draw_warning: false,
            should_remade: false,
        };
    }
    pub fn rescale_ui(&mut self, new_scale: f32) {
        Self::set_ui_to_default(
            &mut self.buttons,
            &mut self.ui_buttons,
            &mut self.sliders,
            new_scale,
        );
    }

    fn set_ui_to_default(
        common_buttons: &mut Vec<Button>,
        utility_buttons: &mut Vec<Button>,
        sliders: &mut Vec<Slider>,
        new_scale: f32,
    ) {
        for (index, button) in common_buttons.iter_mut().enumerate() {
            button.rect.x = new_scale * UI_X_OFFSET;
            button.rect.y =
                (UI_Y_OFFSET + UI_Y_TOP_OFFSET + index as f32 * UI_SHIFT_SIZE) * new_scale;
            button.rect.width = BUTTON_TEXTURE_WIDTH * new_scale;
            button.rect.height = BUTTON_TEXTURE_HEIGHT * new_scale;
        }
        for index in 0..UTILITY_BUTTONS.len() {
            let additional_offset = Vector2::new(1., 1.) * new_scale;
            let additional_multiplier = if index == 0 { 1. } else { 0. };

            utility_buttons[index].rect.x =
                new_scale * (SCREEN_WIDTH - UI_UTILITY_WIDTH as i32) as f32 * index as f32 / 2.
                    + additional_multiplier * additional_offset.x;
            utility_buttons[index].rect.y =
                index as f32 * UI_UTILITY_Y_OFFSET_SAVE_BUTTON as f32 * new_scale
                    + additional_multiplier * additional_offset.y;
            utility_buttons[index].rect.width = UI_UTILITY_WIDTH as f32 * new_scale;
            utility_buttons[index].rect.height = UI_UTILITY_HEIGHT as f32 * new_scale;
        }
        for index in UTILITY_BUTTONS.len()..UTILITY_BUTTONS.len() + WARNING_BUTTONS_TEXT.len() {
            utility_buttons[index].rect.x = new_scale * UI_WARNING_X_OFFSET as f32
                + UI_WARNING_X_SHIFT as f32 * new_scale * (index - UTILITY_BUTTONS.len()) as f32;
            utility_buttons[index].rect.y = new_scale * UI_WARNING_Y_OFFSET as f32;
            utility_buttons[index].rect.width = UI_WARNING_WIDTH as f32 * new_scale;
            utility_buttons[index].rect.height = UI_WARNING_HEIGHT as f32 * new_scale;
        }

        for (index, slider) in sliders.iter_mut().enumerate() {
            slider.back_to_default(
                new_scale,
                Vector2::new(
                    UI_X_OFFSET * new_scale,
                    (UI_Y_OFFSET
                        + UI_Y_TOP_OFFSET
                        + (index + BUTTONS_SETTINGS.len()) as f32 * UI_SHIFT_SIZE)
                        * new_scale,
                ) + SliderStyle::get_sprite_offset(&slider.slider_style)
                    * Vector2::new(new_scale, new_scale),
            );
        }
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
            SettingsOptions::Resolution => settings.pixel_scale = value + 1,
            SettingsOptions::GeneralAudio => settings.general_audio = value as f32,
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
                    let snap_points = SliderStyle::get_snap_points(&slider.slider_style);
                    slider.slider_value = snap_points[find_nearest(
                        SliderStyle::get_snap_points(&slider.slider_style),
                        (settings.pixel_scale as f32 * PIXEL_SCALE_TO_SLIDER_VALUE) as usize,
                    ) as usize] as u8;
                }
                SettingsOptions::GeneralAudio => {
                    self.in_menu_settings.general_audio = settings.general_audio;
                    slider.slider_value = settings.general_audio as u8;
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
        rl: &mut RaylibHandle,
        settings_handler: &mut SettingsHandler,
    ) {
        for (i, button) in self.ui_buttons.iter_mut().enumerate() {
            if button.selected
                && rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT)
                && button.rect.check_collision_point_rec(
                    rl.get_mouse_position()
                        - Vector2::new(
                            rl.get_screen_width() as f32 / 2.
                                - (SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32)
                                    as f32
                                    / 2.,
                            rl.get_screen_height() as f32 / 2.
                                - (SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32)
                                    as f32
                                    / 2.,
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
                        if self.previous_scene.is_some() {
                            scene_handler.set(self.previous_scene.unwrap());
                            self.previous_scene = None;
                        }
                    }
                    1 => {
                        if warning {
                            continue;
                        }

                        self.should_remade = true;
                        rl.set_window_size(
                            SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32,
                            SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32,
                        );
                        settings_handler.set_settings(&self.in_menu_settings);
                        settings_handler.save();
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

        for (index, button) in self.buttons.iter_mut().enumerate() {
            if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT)
                && self.picked_element.is_some_and(|b| b == index)
            {
                self.picked_element = None;
                if button.rect.check_collision_point_rec(
                    rl.get_mouse_position()
                        - Vector2::new(
                            rl.get_screen_width() as f32 / 2.
                                - (SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32)
                                    as f32
                                    / 2.,
                            rl.get_screen_height() as f32 / 2.
                                - (SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32)
                                    as f32
                                    / 2.,
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
        }

        if self.picked_element.is_some_and(|b| b >= self.buttons.len())
            && rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT)
        {
            let val = self.picked_element.unwrap();
            self.picked_element = None;

            if self.sliders[val - BUTTONS_SETTINGS.len()].snap {
                let snap_values = SliderStyle::get_snap_points(
                    &self.sliders[val - BUTTONS_SETTINGS.len()].slider_style,
                );

                self.sliders[val - BUTTONS_SETTINGS.len()].slider_value =
                    snap_values[find_nearest(
                        snap_values.clone(),
                        self.sliders[val - BUTTONS_SETTINGS.len()].slider_value as usize,
                    ) as usize] as u8;
            }
        };

        let buttons_len = self.buttons.len();

        for (index, slider) in self.sliders.iter_mut().enumerate() {
            let outline_rect_n = SliderStyle::get_outline_rect(&slider.slider_style);
            if slider.rects[outline_rect_n].check_collision_point_rec(
                rl.get_mouse_position()
                    - Vector2::new(
                        rl.get_screen_width() as f32 / 2.
                            - (SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32) as f32
                                / 2.,
                        rl.get_screen_height() as f32 / 2.
                            - (SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32) as f32
                                / 2.,
                    ),
            ) && self.picked_element.is_none()
                && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
            {
                self.picked_element = Some(index + buttons_len);
            }

            if self
                .picked_element
                .is_some_and(|b| b != index + buttons_len)
                || self.picked_element.is_none()
            {
                continue;
            };
            let mouse_cords = rl.get_mouse_position()
                - Vector2::new(
                    rl.get_screen_width() as f32 / 2.
                        - (SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32) as f32 / 2.,
                    rl.get_screen_height() as f32 / 2.
                        - (SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32) as f32
                            / 2.,
                );
            let maxlen = SLIDER_WIDTH_PX * settings_handler.settings.pixel_scale;
            let slider_value = if mouse_cords.x < slider.rects[outline_rect_n].x {
                0.
            } else {
                if (slider.rects[outline_rect_n].x + maxlen as f32) < mouse_cords.x {
                    100.
                } else {
                    (mouse_cords.x - slider.rects[outline_rect_n].x) * 100. / maxlen as f32
                }
            };
            let mut value_to_set = slider_value;
            slider.slider_value = if slider.snap {
                let snap_points = SliderStyle::get_snap_points(&slider.slider_style);
                value_to_set = find_nearest(snap_points.clone(), slider_value as usize) as f32;
                snap_points[value_to_set as usize] as u8
            } else {
                slider_value as u8
            };

            SettingsMenuHandler::set_setting_slider(
                &mut self.in_menu_settings,
                SETTINGS_OPTIONS[index + buttons_len],
                value_to_set as u8,
            );
        }
    }

    #[profiling::function]
    pub fn draw(
        &mut self,
        font: &Font,
        texture_handler: &TextureHandler,
        rl: &mut RaylibDrawHandle,
        settings_handler: &mut SettingsHandler,
    ) {
        rl.clear_background(Color::from_hex(BACKGROUND_COLOR_HEX).unwrap());

        rl.draw_texture_ex(
            texture_handler.get_safe(BACKGROUND_IMAGE),
            Vector2::zero(),
            0.0,
            settings_handler.settings.pixel_scale as f32,
            Color::WHITE,
        );

        for (button_num, button) in self.buttons.iter().enumerate() {
            let mouse_down = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);

            let texture_offset = if button.rect.check_collision_point_rec(
                rl.get_mouse_position()
                    - Vector2::new(
                        rl.get_screen_width() as f32 / 2.
                            - (SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32) as f32
                                / 2.,
                        rl.get_screen_height() as f32 / 2.
                            - (SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32) as f32
                                / 2.,
                    ),
            ) && mouse_down
                && self.picked_element.is_none()
                && !self.draw_warning
            {
                self.picked_element = Some(button_num);
                BUTTON_TEXTURE_WIDTH
            } else {
                if self.picked_element.is_some_and(|b| b == button_num)
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
                    TEXT_X_OFFSET * settings_handler.settings.pixel_scale as f32,
                    (UI_Y_OFFSET + UI_Y_TOP_OFFSET + button_num as f32 * UI_SHIFT_SIZE)
                        * settings_handler.settings.pixel_scale as f32,
                ),
                Vector2::zero(),
                0.,
                TEXT_SIZE * settings_handler.settings.pixel_scale as f32,
                TEXT_SPACING * settings_handler.settings.pixel_scale as f32,
                Color::RAYWHITE,
            );
        }

        for (slider_num, slider) in self.sliders.iter_mut().enumerate() {
            let picker = SliderStyle::get_picker_rect(&slider.slider_style);

            for i in 0..slider.rects.len() {
                let (mut width, mut height) = SliderStyle::get_dimensions(&slider.slider_style);
                let mut x = 0;
                let mut y = i as u8 * SLIDER_TEXTURE_OFFSET;
                let mut picker_offset =
                    SliderStyle::get_picker_offset_according_to_slider(&slider.slider_style);

                if SliderStyle::special_size_picker(&slider.slider_style) && picker == i {
                    (width, height) = SliderStyle::get_picker_size(&slider.slider_style);
                    let new_val = if slider.slider_value as f32 > 95. {
                        95.
                    } else {
                        slider.slider_value as f32
                    };

                    if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT)
                        && self
                            .picked_element
                            .is_some_and(|b| b == self.buttons.len() + slider_num)
                    {
                        picker_offset = SliderStyle::get_picker_offset_according_to_slider_pressed(
                            &slider.slider_style,
                        );

                        (x, y) = SliderStyle::get_picker_pressed_texture(&slider.slider_style);
                    }

                    slider.rects[i].x =
                        slider.rects[SliderStyle::get_outline_rect(&slider.slider_style)].x
                            + picker_offset * settings_handler.settings.pixel_scale as f32
                            + (SLIDER_WIDTH_PX as f32 * new_val / 100.).floor()
                                * settings_handler.settings.pixel_scale as f32;
                } else if i == picker {
                    width = ((SLIDER_WIDTH_PX as f32 * slider.slider_value as f32 / 100.).floor()
                        + picker_offset * settings_handler.settings.pixel_scale as f32)
                        as usize;

                    slider.rects[i].width =
                        width as f32 * settings_handler.settings.pixel_scale as f32;
                }

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
                    TEXT_X_OFFSET * settings_handler.settings.pixel_scale as f32,
                    (UI_Y_OFFSET
                        + UI_Y_TOP_OFFSET
                        + (slider_num + BUTTONS_SETTINGS.len()) as f32 * UI_SHIFT_SIZE)
                        * settings_handler.settings.pixel_scale as f32,
                ),
                Vector2::zero(),
                0.,
                TEXT_SIZE * settings_handler.settings.pixel_scale as f32,
                TEXT_SPACING * settings_handler.settings.pixel_scale as f32,
                Color::RAYWHITE,
            );
        }

        for i in 0..UTILITY_BUTTONS.len() {
            let (texture_offset, text_offset) =
                if self.ui_buttons[i].rect.check_collision_point_rec(
                    rl.get_mouse_position()
                        - Vector2::new(
                            rl.get_screen_width() as f32 / 2.
                                - (SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32)
                                    as f32
                                    / 2.,
                            rl.get_screen_height() as f32 / 2.
                                - (SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32)
                                    as f32
                                    / 2.,
                        ),
                ) && self.picked_element.is_none()
                    && !self.draw_warning
                    && rl.is_mouse_button_up(MouseButton::MOUSE_BUTTON_LEFT)
                {
                    self.ui_buttons[i].selected = true;
                    (0., 0.)
                } else {
                    (
                        UI_UTILITY_HEIGHT as f32,
                        settings_handler.settings.pixel_scale as f32,
                    )
                };

            let text_dimensions = get_text_size(
                font,
                UTILITY_BUTTONS[i],
                TEXT_SIZE * settings_handler.settings.pixel_scale as f32,
                TEXT_SPACING * settings_handler.settings.pixel_scale as f32,
            );

            self.ui_buttons[i].draw_with_text_middle(
                rl,
                UTILITY_BUTTONS[i],
                font,
                texture_handler.get_safe(UTILITY_BUTTONS_TEXTURE),
                &Rectangle::new(
                    0.,
                    texture_offset,
                    UI_UTILITY_WIDTH as f32,
                    UI_UTILITY_HEIGHT as f32,
                ),
                text_dimensions,
                &Color::RAYWHITE,
                TEXT_SIZE * settings_handler.settings.pixel_scale as f32,
                TEXT_SPACING * settings_handler.settings.pixel_scale as f32,
                Vector2::new(0., text_offset),
                Vector2::new(0., -(settings_handler.settings.pixel_scale as f32)),
            );
        }

        if !self.draw_warning {
            return;
        }

        rl.draw_texture_ex(
            texture_handler.get_safe(SETTINGS_UI_TEXTURE),
            Vector2::new(
                64. * settings_handler.settings.pixel_scale as f32,
                26. * settings_handler.settings.pixel_scale as f32,
            ),
            0.0,
            settings_handler.settings.pixel_scale as f32,
            Color::WHITE,
        );

        for i in 0..WARNING_TEXT.len() {
            let text_size = get_text_size(
                font,
                WARNING_TEXT[i],
                TEXT_SIZE * settings_handler.settings.pixel_scale as f32,
                TEXT_SPACING * settings_handler.settings.pixel_scale as f32,
            );

            rl.draw_text_pro(
                font,
                WARNING_TEXT[i],
                Vector2::new(
                    128. * settings_handler.settings.pixel_scale as f32 - text_size.x / 2.,
                    text_size.y
                        + 26. * settings_handler.settings.pixel_scale as f32
                        + text_size.y * i as f32,
                ),
                Vector2::zero(),
                0.,
                TEXT_SIZE * settings_handler.settings.pixel_scale as f32,
                TEXT_SPACING * settings_handler.settings.pixel_scale as f32,
                CustomColor::BLACK_TEXT,
            );
        }

        for i in 0..WARNING_BUTTONS_TEXT.len() {
            let (texture_offset, text_offset) = if self.ui_buttons[i + UTILITY_BUTTONS.len()]
                .rect
                .check_collision_point_rec(
                    rl.get_mouse_position()
                        - Vector2::new(
                            rl.get_screen_width() as f32 / 2.
                                - (SCREEN_WIDTH * settings_handler.settings.pixel_scale as i32)
                                    as f32
                                    / 2.,
                            rl.get_screen_height() as f32 / 2.
                                - (SCREEN_HEIGHT * settings_handler.settings.pixel_scale as i32)
                                    as f32
                                    / 2.,
                        ),
                )
                && self.picked_element.is_none()
                && rl.is_mouse_button_up(MouseButton::MOUSE_BUTTON_LEFT)
            {
                self.ui_buttons[i + UTILITY_BUTTONS.len()].selected = true;
                (0., 0.)
            } else {
                (
                    UI_UTILITY_HEIGHT as f32,
                    settings_handler.settings.pixel_scale as f32,
                )
            };

            let text_size = get_text_size(
                font,
                WARNING_BUTTONS_TEXT[i],
                TEXT_SIZE * settings_handler.settings.pixel_scale as f32,
                TEXT_SPACING * settings_handler.settings.pixel_scale as f32,
            );

            self.ui_buttons[i + WARNING_BUTTONS_TEXT.len()].draw_with_text_middle(
                rl,
                WARNING_BUTTONS_TEXT[i],
                font,
                texture_handler.get_safe(UTILITY_BUTTONS_TEXTURE),
                &Rectangle::new(
                    0.,
                    texture_offset,
                    UI_WARNING_WIDTH as f32,
                    UI_WARNING_HEIGHT as f32,
                ),
                text_size,
                &Color::RAYWHITE,
                TEXT_SIZE * settings_handler.settings.pixel_scale as f32,
                TEXT_SPACING * settings_handler.settings.pixel_scale as f32,
                Vector2::new(0., text_offset),
                Vector2::new(0., -(settings_handler.settings.pixel_scale as f32)),
            );
        }
    }
}
