use std::{collections::HashMap, fs};
use serde::{Serialize, Deserialize};
use raylib::prelude::*;

const HOTKEYS_PATH: &str = "dynamic/hotkeys.json";

#[derive(Deserialize,Serialize, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HotkeyCategory {
    Exit,
    Continue,
    Reset,
    PickNearest,
}

#[derive(Deserialize, Serialize)]
pub enum KeyboardKeyString{
    KeyEnter,
    KeyEsc,
    KeySpace,
    KeyQ,
    KeyR,
    KeyP,
}

#[derive(Deserialize, Serialize)]
pub struct HotkeyLoaderStruct{
    hotkeys: HashMap<HotkeyCategory, Vec<KeyboardKeyString>>
}

pub struct HotkeyHandler{
    hotkeys: HashMap<HotkeyCategory, Vec<KeyboardKey>>,
    last_pressed_hotkey: Option<KeyboardKey>,   
}

impl HotkeyLoaderStruct{
    pub fn new() -> Self{
        let path = HOTKEYS_PATH.to_string();
        let Ok(string_hotkeys) = fs::read_to_string(path) else{
            panic!("COULDN'T LOAD HOTKEYS");
        };
        println!("{}",string_hotkeys);
        let Ok(hotkeys) = serde_json::from_str(&string_hotkeys) else{
            panic!("COULDN'T PARSE HOTKEYS JSON");
        };

        return hotkeys;
    } 
}

impl HotkeyHandler{
    pub fn new(hotkeys_raw: HotkeyLoaderStruct) -> Self{
        let mut hotkeys = HashMap::new();

        for (target, key_type) in hotkeys_raw.hotkeys.iter(){
            let mut vec = vec![];
            for i in key_type{
                let key = match i{
                    KeyboardKeyString::KeyEsc => KeyboardKey::KEY_ESCAPE,
                    KeyboardKeyString::KeyEnter => KeyboardKey::KEY_ENTER,
                    KeyboardKeyString::KeySpace => KeyboardKey::KEY_SPACE,
                    KeyboardKeyString::KeyQ => KeyboardKey::KEY_Q,
                    KeyboardKeyString::KeyR => KeyboardKey::KEY_R,
                    KeyboardKeyString::KeyP => KeyboardKey::KEY_P
                };
                vec.push(key);
                hotkeys.insert(target.clone(), vec.clone());
            }
        }

        Self{
            hotkeys: hotkeys,
            last_pressed_hotkey: None,
        }
    }
    pub fn get_last_key(&self) -> KeyboardKey{
        return self.last_pressed_hotkey.unwrap_or(KeyboardKey::KEY_NUM_LOCK);
    } 
    
    pub fn clear_last(&mut self){
        self.last_pressed_hotkey = None;
    }

    pub fn check_down(&mut self, rl: &RaylibHandle, target_intent: HotkeyCategory) -> bool{
        for (intent, keys) in self.hotkeys.iter(){
            if *intent != target_intent{
                continue;
            }

            for key in keys.iter(){
                if rl.is_key_down(*key){
                    self.last_pressed_hotkey = Some(*key);
                    return true;
                }
            }
        }
        return false;
    }

    pub fn check_pressed(&mut self, rl: &RaylibHandle, target_intent: HotkeyCategory) -> bool{
        for (intent, keys) in self.hotkeys.iter(){
            if *intent != target_intent{
                continue;
            }

            for key in keys.iter(){
                if rl.is_key_pressed(*key){
                    self.last_pressed_hotkey = Some(*key);
                    return true;
                }
            }
        }
        return false;
    }
}
