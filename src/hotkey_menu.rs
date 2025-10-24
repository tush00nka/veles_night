use std::collections::HashMap;
use raylib::{ffi::CheckCollisionPointRec, prelude::*};

use crate::{hotkey_handler::{HotkeyCategory, HotkeyHandler}, save_handler::SaveHandler, scene::{Scene, SceneHandler}, texture_handler::TextureHandler, ui::Button, SCREEN_HEIGHT, SCREEN_WIDTH};

const KEYS_PATH: &str = "static/textures/keys";
const BUTTON_TEXT: &str = "Вернуться в меню";
pub struct HotkeyMenuHandler{
    labels: Vec<String>,
    button: Button
}

impl HotkeyMenuHandler{
    pub fn new(hotkey_handler: &mut HotkeyHandler) -> Self{
        let mut labels = Vec::new();
        
        for hotkey_category in hotkey_handler.hotkeys.keys(){
            match hotkey_category{
                HotkeyCategory::Skip => (),
                HotkeyCategory::ERROR => (),
                _ => labels.push(HotkeyCategory::to_string(*hotkey_category)),
            } 
        } 
        Self { 
            labels,
            button: Button { rect: Rectangle::new(0., 0., 300., 64.), selected: false } 
        }
    }

    pub fn update(
        &mut self,
        hotkey_handler: &mut HotkeyHandler,
        scene_handler: &mut SceneHandler,
        rl: &mut RaylibHandle,
        ){
        if hotkey_handler.check_pressed(rl, HotkeyCategory::Exit) {
            scene_handler.set(Scene::MainMenu);
            return;
        }

        if unsafe {CheckCollisionPointRec(rl.get_mouse_position().into(), self.button.rect.into())} 
        && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
        {
            scene_handler.set(Scene::MainMenu);
            return;
        }
    }
    
    pub fn draw(
        &mut self,
        font: &Font,
        save_handler: &SaveHandler,
        texture_handler: &TextureHandler,
        rl: &mut RaylibDrawHandle,
    ){
        rl.clear_background(Color::from_hex("0b8a8f").unwrap());

        self.button.selected = false;        
        let mut color;
        if unsafe { CheckCollisionPointRec(rl.get_mouse_position().into(), self.button.rect.into()) }
        {
            color = Color::WHITE;
            self.button.selected = true;
        } else {
            color = Color::BLACK.alpha(0.5);
        };
        
        rl.draw_rectangle_rec(self.button.rect, color);
       
        if self.button.selected{
            color = Color::BLACK;
        }else{
            color = Color::RAYWHITE;
        }

        rl.draw_text_pro(
            font,
            BUTTON_TEXT,
            Vector2::new(0.,0.),
            Vector2::zero(),
            0.0,
            48.,
            2.0,
            color,
        );


    }
}
