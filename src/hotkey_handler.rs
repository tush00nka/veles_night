use std::{collections::HashMap, fs};
use serde::{Serialize, Deserialize};
use raylib::prelude::*;

const HOTKEYS_PATH: &str = "dynamic/hotkeys.json";

#[derive(Deserialize,Serialize, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HotkeyCategory {
    Exit,
    Continue,
}

#[derive(Deserialize, Serialize)]
pub enum KeyboardKeyString{
    KeyEnter,
    KeyEsc,
    KeySpace,
    KeyQ
}

#[derive(Deserialize, Serialize)]
pub struct HotkeyLoaderStruct{
   hotkeys: HashMap<HotkeyCategory, Vec<KeyboardKeyString>>
}

pub struct HotkeyHandler{
    pub hotkeys: HashMap<HotkeyCategory, Vec<KeyboardKey>>
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
                };
                vec.push(key);
                hotkeys.insert(target.clone(), vec.clone());
            }
        }

        Self{
            hotkeys
        }
    }
}
