use std::collections::HashMap;

use raylib::{ffi::CheckCollisionPointRec, prelude::*};

use crate::{SCREEN_WIDTH, texture_handler::TextureHandler};

pub struct UIHandler {
    build_buttons: HashMap<String, Rectangle>,
}

impl UIHandler {
    pub fn new() -> Self {
        let mut buttons = HashMap::new();

        let labels = ["fire_td", "fire_lr", "fire_stop"];

        for i in 0..3 {
            buttons.insert(
                labels[i].to_string(),
                Rectangle::new(
                    i as f32 * 80. + (SCREEN_WIDTH / 2) as f32 - 40. * 3., // todo: pohui
                    16.,
                    64.,
                    64.,
                ),
            );
        }

        Self {
            build_buttons: buttons,
        }
    }

    pub fn draw(&self, texture_handler: &TextureHandler, rl: &mut RaylibDrawHandle) {
        for (tex_name, button) in self.build_buttons.iter() {
            let color =
                if unsafe { CheckCollisionPointRec(rl.get_mouse_position().into(), button.into()) }
                {
                    Color::WHITE
                } else {
                    Color::BLACK.alpha(0.5)
                };

            rl.draw_rectangle_rec(button, color);
            rl.draw_texture_pro(
                texture_handler.get_safe(tex_name),
                Rectangle::new(
                    ((rl.get_time() * 4.) % 4.).floor() as f32 * 16.,
                    16.,
                    16.,
                    16.,
                ),
                button,
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
        }
    }
}
