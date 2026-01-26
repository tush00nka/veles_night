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
    spirits_handler::SpiritsHandler,
    texture_handler::TextureHandler,
    ui::{Button, UIHandler},
};

#[derive(PartialEq, Clone, Copy)]
pub enum SliderStyle {
    Volume,
    Ruler,
}

pub const SLIDER_WIDTH_PX: u8 = 48;
pub const SLIDER_HEIGHT_PX: u8 = 16;
pub const SLIDER_TEXTURE_OFFSET: u8 = 16;

impl SliderStyle {
    pub fn is_snap(slider_style: &SliderStyle) -> bool {
        return *slider_style == SliderStyle::Ruler;
    }

    fn get_outline_rect(slider_style: &SliderStyle) -> usize {
        return match *slider_style {
            SliderStyle::Volume => 1,
            SliderStyle::Ruler => 0,
        };
    }

    fn get_texture_name(slider_style: &SliderStyle) -> &str {
        return match *slider_style {
            SliderStyle::Volume => "volume_slider",
            SliderStyle::Ruler => "ruler_slider",
        };
    }

    fn get_sprite_parts_amount(slider_style: &SliderStyle) -> u8 {
        return match *slider_style {
            SliderStyle::Volume => 3,
            SliderStyle::Ruler => 2,
        };
    }

    fn get_rects(slider_style: &SliderStyle, multiplier: u8) -> Vec<Rectangle> {
        let mut vector: Vec<Rectangle> = vec![];
        for _ in 0..Self::get_sprite_parts_amount(slider_style) {
            vector.push(Rectangle {
                x: 0.,
                y: 0.,
                width: (SLIDER_WIDTH_PX * multiplier) as f32,
                height: (SLIDER_HEIGHT_PX * multiplier) as f32,
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
    pub fn new(slider_style: SliderStyle) -> Self {
        let rects = SliderStyle::get_rects(&slider_style, TILE_SCALE_DEFAULT as u8);

        Self {
            slider_value: 50,
            snap: SliderStyle::is_snap(&slider_style),
            slider_style,
            rects,
        }
    }
    fn change_position(&mut self, position: Vector2) {
        for rectangle in self.rects.iter_mut() {
            rectangle.x = position.x;
            rectangle.y = position.y;
        }
    }

    fn change_size(&mut self, dimensions: Vector2, index: usize) {
        {
            self.rects[index].width = dimensions.x;
            self.rects[index].height = dimensions.y;
        }
    }
}

pub struct SettingsMenuHandler {
    previous_scene: Option<Scene>,
    picked_element: Option<usize>,
    buttons: Vec<Button>,
    sliders: Vec<Slider>,
}

const BUTTONS_SETTINGS: [&str; 2] = ["Шейдер", "Текст"];
const SLIDER_NUMBER: usize = BUTTONS_SETTINGS.len();

const SLIDERS_SETTINGS: [&str; 3] = ["Громкость музыки", "Громкость звуков", "Разрешение"];

impl SettingsMenuHandler {
    pub fn new() -> Self {
        let mut buttons = Vec::new();
        let mut sliders = Vec::new();

        for index in 0..BUTTONS_SETTINGS.len() {
            buttons.push(Button {
                selected: false,
                rect: Rectangle {
                    x: (192 * TILE_SCALE_DEFAULT as usize) as f32,
                    y: ((index + 1) * 24 * TILE_SCALE_DEFAULT as usize) as f32,
                    width: 32. * TILE_SCALE_DEFAULT as f32 / 2.,
                    height: 32. * TILE_SCALE_DEFAULT as f32 / 2.,
                },
                offset: 0.,
            });
        }

        let mut slider_type = SliderStyle::Volume;

        for index in 0..SLIDERS_SETTINGS.len() {
            if index == 2 {
                slider_type = SliderStyle::Ruler;
            };

            sliders.push(Slider::new(slider_type));
        }

        for (index, slider) in sliders.iter_mut().enumerate() {
            slider.change_position(Vector2::new(
                (192 * TILE_SCALE_DEFAULT) as f32,
                ((3 + index) * 24 * TILE_SCALE_DEFAULT as usize) as f32,
            ));

            for i in 0..slider.rects.len() {
                let dimensions = if slider.slider_style == SliderStyle::Ruler && i == 1 {
                    Vector2::new(
                        16. * TILE_SCALE_DEFAULT as f32,
                        16. * TILE_SCALE_DEFAULT as f32,
                    )
                } else {
                    Vector2::new(
                        48. * TILE_SCALE_DEFAULT as f32,
                        16. * TILE_SCALE_DEFAULT as f32,
                    )
                };
                slider.change_size(dimensions, i);
            }
        }

        return Self {
            picked_element: None,
            previous_scene: None,
            buttons,
            sliders,
        };
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
    ) {
        let mut index = 0;
        for button in self.buttons.iter_mut() {
            if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) && self.previous_scene.is_some() {
                scene_handler.set(self.previous_scene.unwrap());
                self.previous_scene = None;
            }
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
                }
            }
            index += 1;
        }
        if self.picked_element.is_some()
            && rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT)
        {
            self.picked_element = None;
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
                println!("what");
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
        rl.clear_background(Color::from_hex("0b8a8f").unwrap());

        rl.draw_texture_ex(
            texture_handler.get_safe("main_menu_bg"),
            Vector2::zero(),
            0.0,
            TILE_SCALE_DEFAULT as f32,
            Color::WHITE,
        );

        let mut index = 0;

        for (button_num, button) in self.buttons.iter_mut().enumerate() {
            let mouse_down = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
            let texture_offset = if button.rect.check_collision_point_rec(
                rl.get_mouse_position()
                    - Vector2::new(
                        rl.get_screen_width() as f32 / 2. - SCREEN_WIDTH as f32 / 2.,
                        rl.get_screen_height() as f32 / 2. - SCREEN_HEIGHT as f32 / 2.,
                    ),
            ) && mouse_down
                && self.picked_element.is_none()
            {
                self.picked_element = Some(index);
                32.
            } else {
                if self.picked_element.is_some_and(|b| b == index) && mouse_down {
                    32.
                } else {
                    0.
                }
            };

            let button_state = if button.selected { 32. } else { 0. };
            rl.draw_texture_pro(
                texture_handler.get_safe("settings_button"),
                Rectangle::new(texture_offset, button_state, 32., 32.),
                button.rect,
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );

            rl.draw_text_pro(
                font,
                BUTTONS_SETTINGS[button_num],
                Vector2::new(
                    32. * TILE_SCALE_DEFAULT as f32,
                    (index + 1) as f32 * 24. * TILE_SCALE_DEFAULT as f32,
                ),
                Vector2::zero(),
                0.,
                12. * TILE_SCALE_DEFAULT as f32,
                1.05 * TILE_SCALE_DEFAULT as f32,
                CustomColor::BLACK_TEXT,
            );

            index += 1;
        }
        for (slider_num, slider) in self.sliders.iter_mut().enumerate() {
            for i in 0..slider.rects.len() {
                let mut width = SLIDER_WIDTH_PX;
                let mut height = SLIDER_HEIGHT_PX;
                let x = 0;
                let y = i as u8 * SLIDER_TEXTURE_OFFSET;

                match slider.slider_style {
                    SliderStyle::Ruler => {
                        if i == slider.rects.len() - 1 {
                            height = 16;
                            width = 16;
                            let new_val = if slider.slider_value as f32 > 97. {
                                95.
                            } else {
                                slider.slider_value as f32
                            };
                            slider.rects[i].x =
                                slider.rects[SliderStyle::get_outline_rect(&SliderStyle::Ruler)].x
                                    + (SLIDER_WIDTH_PX as f32 * new_val / 100.).floor()
                                        * TILE_SCALE_DEFAULT as f32;
                        }
                    }
                    SliderStyle::Volume => {
                        if i == 0 {
                            width = (SLIDER_WIDTH_PX as f32 * slider.slider_value as f32 / 100.)
                                .floor() as u8;

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
                    32. * TILE_SCALE_DEFAULT as f32,
                    (index + 1) as f32 * 24. * TILE_SCALE_DEFAULT as f32,
                ),
                Vector2::zero(),
                0.,
                12. * TILE_SCALE_DEFAULT as f32,
                1.05 * TILE_SCALE_DEFAULT as f32,
                CustomColor::BLACK_TEXT,
            );

            index += 1;
        }
    }
}
