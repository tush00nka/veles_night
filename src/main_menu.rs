use std::collections::HashMap;

use raylib::{ffi::CheckCollisionPointRec, prelude::*};

use crate::{
    SCREEN_HEIGHT, SCREEN_WIDTH,
    scene::{Scene, SceneHandler},
    texture_handler::TextureHandler,
    ui::Button,
};

pub struct MainMenuHandler {
    buttons: HashMap<u8, Button>,
    labels: Vec<&'static str>,
}

impl MainMenuHandler {
    pub fn new() -> Self {
        let mut buttons = HashMap::new();

        buttons.insert(
            0,
            Button {
                selected: false,
                rect: Rectangle::new(
                    (SCREEN_WIDTH / 2) as f32 - 98.,
                    (SCREEN_HEIGHT / 2) as f32 + 32.,
                    196.,
                    64.,
                ),
            },
        );

        buttons.insert(
            1,
            Button {
                selected: false,
                rect: Rectangle::new(
                    (SCREEN_WIDTH / 2) as f32 - 98.,
                    (SCREEN_HEIGHT / 2) as f32 + 100.,
                    196.,
                    64.,
                ),
            },
        );

        let labels = vec!["Начать", "Закончить"];

        Self { buttons, labels }
    }

    pub fn update(&self, scene_handler: &mut SceneHandler, should_close: &mut bool, rl: &mut RaylibHandle) {
        for (key, button) in self.buttons.iter() {
            if unsafe { CheckCollisionPointRec(rl.get_mouse_position().into(), button.rect.into()) }
                && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
            {
                match key{
                    0 => {
                        scene_handler.set(Scene::Level);
                    }
                    1 => {
                        *should_close = true;
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn draw(
        &mut self,
        font: &Font,
        texture_handler: &TextureHandler,
        rl: &mut RaylibDrawHandle,
    ) {
        rl.clear_background(Color::from_hex("0b8a8f").unwrap());

        const LOGO_WIDTH: f32 = 96. * 4.;
        const LOGO_HEIGHT: f32 = 64. * 4.;

        rl.draw_texture_ex(
            texture_handler.get_safe("main_menu_bg"),
            Vector2::zero(),
            0.0,
            4.,
            Color::WHITE,
        );

        rl.draw_texture_ex(
            texture_handler.get_safe("logo"),
            Vector2::new(
                (SCREEN_WIDTH / 2) as f32 - LOGO_WIDTH / 2.,
                (SCREEN_HEIGHT / 4) as f32 - LOGO_HEIGHT / 2.,
            ),
            0.0,
            4.,
            Color::WHITE,
        );

        let mut button_selected = 128;

        for (name, button) in self.buttons.iter_mut() {
            
            let color;
            if unsafe {
                CheckCollisionPointRec(rl.get_mouse_position().into(), button.rect.into())
            } {
                color = Color::WHITE;

                button_selected = *name; 
            } else {
                color = Color::BLACK.alpha(0.5); 
            };

            rl.draw_rectangle_rec(button.rect, color);
        }

        for i in 0..self.labels.len() {
            let color;
            if i == button_selected as usize {
                color = Color::BLACK;
            } else {
                color = Color::RAYWHITE;
            }
            rl.draw_text_pro(
                font,
                self.labels[i],
                Vector2::new(
                    (SCREEN_WIDTH / 2) as f32 - 70.,
                    (SCREEN_HEIGHT / 2) as f32 + 40. + (66. * i as f32),
                ),
                Vector2::zero(),
                0.0,
                48.,
                2.0,
                color,
            );
        }
    }
}
