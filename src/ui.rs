use std::collections::HashMap;

use raylib::{ffi::CheckCollisionPointRec, prelude::*};

use crate::{
    map::{Level, TileType, TILE_SIZE}, texture_handler::TextureHandler, SCREEN_WIDTH
};

struct Button {
    rect: Rectangle,
    selected: bool,
}

pub struct UIHandler {
    build_buttons: HashMap<String, Button>,
}

impl UIHandler {
    pub fn new() -> Self {
        let mut buttons = HashMap::new();

        let labels = ["fire_td", "fire_lr", "fire_stop"];

        for i in 0..3 {
            buttons.insert(
                labels[i].to_string(),
                Button {
                    rect: Rectangle::new(
                        i as f32 * 80. + (SCREEN_WIDTH / 2) as f32 - 40. * 3., // todo: pohui
                        16.,
                        64.,
                        64.,
                    ),
                    selected: false,
                },
            );
        }

        Self {
            build_buttons: buttons,
        }
    }

    pub fn build(&mut self, level: &mut Level, rl: &mut RaylibHandle) {
        for (title, button) in self.build_buttons.iter_mut() {
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                if unsafe {
                    CheckCollisionPointRec(rl.get_mouse_position().into(), button.rect.into())
                } {
                    button.selected = true;
                }
            }

            if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                if button.selected && level.get_wood() > 0 {
                    let tile = match title.as_str() {
                        "fire_td" => TileType::FireTD { active: false },
                        "fire_lr" => TileType::FireLR { active: false },
                        "fire_stop" => TileType::FireStop { active: false },
                        _ => {
                            panic!("wait how")
                        }
                    };

                    let pos = rl.get_mouse_position() / (Vector2::one() * TILE_SIZE as f32);
                    let (x, y) = (pos.x as usize, pos.y as usize);

                    level.tiles[x][y] = tile;
                    level.remove_wood();
                }

                button.selected = false;
            }
        }
    }

    pub fn draw(&self, texture_handler: &TextureHandler, rl: &mut RaylibDrawHandle) {
        for (tex_name, button) in self.build_buttons.iter() {
            let color = if unsafe {
                CheckCollisionPointRec(rl.get_mouse_position().into(), button.rect.into())
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
                        ((rl.get_time() * 4.) % 4.).floor() as f32 * 16.,
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
                        ((rl.get_time() * 4.) % 4.).floor() as f32 * 16.,
                        16.,
                        16.,
                        16.,
                    ),
                    Rectangle::new(
                        rl.get_mouse_position().x - (TILE_SIZE / 2) as f32,
                        rl.get_mouse_position().y - (TILE_SIZE / 2) as f32,
                        TILE_SIZE as f32,
                        TILE_SIZE as f32,
                    ),
                    Vector2::zero(),
                    0.0,
                    Color::WHITE,
                );
            }
        }
    }
}
