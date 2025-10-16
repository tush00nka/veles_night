use std::collections::HashMap;

use raylib::{ffi::CheckCollisionPointRec, prelude::*};
use crate::{scene::{self, SceneHandler}, ui::Button, FIRST_LEVEL, SCREEN_HEIGHT, SCREEN_WIDTH};


pub struct GameOverHandler{
    restart_buttons: HashMap<String, Button>,
    restart_text: HashMap<String, Vector2> //position
}
const LABELS_RESTART: [&str; 2] = ["МЕНЮ", "РЕСТАРТ"];

impl GameOverHandler{
    pub fn new() -> Self {
        let mut restart_buttons = HashMap::new();
        let mut restart_text = HashMap::new();

                for i in 0..LABELS_RESTART.len(){
            restart_buttons.insert(
                LABELS_RESTART[i].to_string()
                , Button{
                    rect: Rectangle::new(
                              (SCREEN_WIDTH as f32/ 1.1) as f32, 
                              256. + (SCREEN_HEIGHT / 2) as f32 + i as f32 * 256.,
                              256.,
                              128.,
                          ),
                          selected: false,
                }
            );
            restart_text.insert(
                LABELS_RESTART[i].to_string()
                , Vector2::new(
                    SCREEN_WIDTH as f32 / 1.1,
                    256. + (SCREEN_HEIGHT / 2) as f32 + i as f32 * 256., 
                    ),
            );
        }

        Self {
            restart_buttons,
            restart_text
        }
    }

    pub fn draw_gameover(&self, font: &Font, rl: &mut RaylibDrawHandle){
        rl.clear_background(Color::from_hex("0b8a8f").unwrap());
        for (name, button) in self.restart_buttons.iter() {
            let color = if unsafe {
                CheckCollisionPointRec(rl.get_mouse_position().into(), button.rect.into())
            } {
                Color::WHITE
            } else {
                Color::BLACK.alpha(0.5)
            };
            
            let color_text = if color == Color::WHITE{
                Color::BLACK                
            }else{
                Color::WHITE
            };

            rl.draw_text_pro(
                font
                , "ТЫ НЕ ДОСТОИН УПРАВЛЯТЬ ЭТИМИ НЕСЧАСТНЫМИ ДУШАМИ!"
                , Vector2::new(
                    (SCREEN_WIDTH / 2) as f32,
                    (SCREEN_HEIGHT / 2) as f32,
                )
                , Vector2::zero()
                , 0.0
                , 48.
                , 2.0
                , Color::RAYWHITE
            );

            rl.draw_rectangle_rec(button.rect, color);
            rl.draw_text_pro(
                font
                , name
                , self.restart_text.get(name).unwrap()
                , Vector2::zero()
                , 0.0
                , 48. 
                , 2.0
                , color_text
            );

        }
    }
    pub fn update_gameover(&mut self, level_number: &mut u8, rl: &mut RaylibHandle, scene_handler: &mut SceneHandler) -> bool{
       //проверка на коллизию и на хоткей - esc/enter
       //
        for (title, button) in self.restart_buttons.iter_mut(){
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT){
                if unsafe{
                    CheckCollisionPointRec(rl.get_mouse_position().into(), button.rect.into())
                }{
                    button.selected = true;
                }
            }
            if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT){
                if button.selected{
                    let scene = match title.as_str(){
                        "МЕНЮ" => {
                           *level_number = FIRST_LEVEL;
                            crate::scene::Scene::MainMenu
                            },
                        "РЕСТАРТ" => crate::Scene::Level,
                        _ => {
                            panic!("NOT EXISITNG BUTTON");
                        }
                    };
                    button.selected = false;
                    scene_handler.set(scene);
                    return true
                }
            }
        }
        return false
    } 
}
