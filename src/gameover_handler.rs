use std::collections::HashMap;

use raylib::{ffi::CheckCollisionPointRec, prelude::*};
use crate::{ui::Button, SCREEN_HEIGHT, SCREEN_WIDTH};


pub struct GameOverHandler{
    restart_buttons: HashMap<String, Button>,
}


impl GameOverHandler{
    pub fn new() -> Self {
        let mut restart_buttons = HashMap::new();

        let labels_restart = ["menu", "restart"];

        for i in 0..labels_restart.len(){
            restart_buttons.insert(
                labels_restart[i].to_string()
                , Button{
                    rect: Rectangle::new(
                              (SCREEN_WIDTH / 2) as f32 - 40. * 2., 
                              i as f32 * 80. + (SCREEN_HEIGHT / 2) as f32 - 40. * 3.,
                              64.,
                              64.,
                          ),
                          selected: false,
                }
            ); 
        }

        Self {
            restart_buttons: restart_buttons,
        }
    }

    pub fn draw_gameover(&self, font: &Font, rl: &mut RaylibDrawHandle){
        rl.clear_background(Color::from_hex("0b8a8f").unwrap());
        for (_, button) in self.restart_buttons.iter() {
            let color = if unsafe {
                CheckCollisionPointRec(rl.get_mouse_position().into(), button.rect.into())
            } {
                Color::WHITE
            } else {
                Color::BLACK.alpha(0.5)
            };


            rl.draw_text_pro(
                font
                , "ТЫ НЕ ДОСТОИН УПРАВЛЯТЬ ЭТИМИ НЕСЧАСТНЫМИ ДУШАМИ!"
                , Vector2::new(
                    (SCREEN_WIDTH / 2) as f32 - 10.,
                    (SCREEN_HEIGHT / 2) as f32 + 32.,
                )
                , Vector2::zero()
                , 0.0
                , 48.
                , 2.0
                , Color::RAYWHITE
            );
            rl.draw_rectangle_rec(button.rect, color);
        }
    }
    pub fn update_gameover(&self, rl: &mut RaylibHandle){
        
    } 
}
