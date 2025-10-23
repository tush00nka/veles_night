use raylib::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

const HOTKEYS_PATH: &str = "dynamic/hotkeys.json";

#[derive(Deserialize, Serialize, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HotkeyCategory {
    Exit = 0,
    Continue = 1,
    Reset = 2,
    PickNearest = 3,
    PickBuilding1 = 4,
    PickBuilding2 = 5,
    PickBuilding3 = 6,
    Skip = 7,
    VolumeUp = 8,
    VolumeDown = 9,
    ERROR = 255,
}

#[derive(Deserialize, Serialize)]
pub enum KeyboardKeyString {
    KeyEnter,
    KeyEsc,
    KeySpace,
    KeyS,
    KeyQ,
    KeyR,
    KeyP,
    Key1,
    Key2,
    Key3,
    KeyU,
    KeyI,
    KeyPlus,
    KeyMinus
}
impl HotkeyCategory {
    pub fn from_bonfire(value: &str) -> HotkeyCategory {
        match value {
            x if x == "fire_td" => HotkeyCategory::PickBuilding1,
            x if x == "fire_lr" => HotkeyCategory::PickBuilding2,
            x if x == "fire_stop" => HotkeyCategory::PickBuilding3,
            _ => HotkeyCategory::ERROR,
        }
    }

    #[allow(unused)]
    pub fn from_u8(value: u8) -> HotkeyCategory {
        match value {
            x if x == HotkeyCategory::Exit as u8 => HotkeyCategory::Exit,
            x if x == HotkeyCategory::Reset as u8 => HotkeyCategory::Reset,
            x if x == HotkeyCategory::Continue as u8 => HotkeyCategory::Continue,
            x if x == HotkeyCategory::PickNearest as u8 => HotkeyCategory::PickNearest,
            x if x == HotkeyCategory::PickBuilding1 as u8 => HotkeyCategory::PickBuilding1,
            x if x == HotkeyCategory::PickBuilding2 as u8 => HotkeyCategory::PickBuilding2,
            x if x == HotkeyCategory::PickBuilding3 as u8 => HotkeyCategory::PickBuilding3,
            x if x == HotkeyCategory::Skip as u8 => HotkeyCategory::Skip,
            x if x == HotkeyCategory::VolumeUp as u8 => HotkeyCategory::VolumeUp,
            x if x == HotkeyCategory::VolumeDown as u8 => HotkeyCategory::VolumeDown,
            _ => HotkeyCategory::ERROR,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct HotkeyLoaderStruct {
    hotkeys: HashMap<HotkeyCategory, Vec<KeyboardKeyString>>,
}

pub struct HotkeyHandler {
    hotkeys: HashMap<HotkeyCategory, Vec<KeyboardKey>>,
    last_pressed_hotkey: Option<KeyboardKey>,
}

impl HotkeyLoaderStruct {
    pub fn new() -> Self {
        let path = HOTKEYS_PATH.to_string();
        let Ok(string_hotkeys) = fs::read_to_string(path) else {
            panic!("COULDN'T LOAD HOTKEYS");
        };
        println!("{}", string_hotkeys);
        let Ok(hotkeys) = serde_json::from_str(&string_hotkeys) else {
            panic!("COULDN'T PARSE HOTKEYS JSON");
        };

        return hotkeys;
    }
}

impl HotkeyHandler {
    pub fn new(hotkeys_raw: HotkeyLoaderStruct) -> Self {
        let mut hotkeys = HashMap::new();

        for (target, key_type) in hotkeys_raw.hotkeys.iter() {
            let mut vec = vec![];
            for i in key_type {
                let key = match i {
                    KeyboardKeyString::KeyEsc => KeyboardKey::KEY_ESCAPE,
                    KeyboardKeyString::KeyEnter => KeyboardKey::KEY_ENTER,
                    KeyboardKeyString::KeySpace => KeyboardKey::KEY_SPACE,
                    KeyboardKeyString::KeyQ => KeyboardKey::KEY_Q,
                    KeyboardKeyString::KeyR => KeyboardKey::KEY_R,
                    KeyboardKeyString::KeyP => KeyboardKey::KEY_P,
                    KeyboardKeyString::Key1 => KeyboardKey::KEY_ONE,
                    KeyboardKeyString::Key2 => KeyboardKey::KEY_TWO,
                    KeyboardKeyString::Key3 => KeyboardKey::KEY_THREE,
                    KeyboardKeyString::KeyS => KeyboardKey::KEY_S,
                    KeyboardKeyString::KeyMinus => KeyboardKey::KEY_MINUS,
                    KeyboardKeyString::KeyPlus => KeyboardKey::KEY_EQUAL,
                    KeyboardKeyString::KeyU => KeyboardKey::KEY_U,
                    KeyboardKeyString::KeyI => KeyboardKey::KEY_I,
                };
                vec.push(key);
            }
            hotkeys.insert(target.clone(), vec.clone());
        }

        Self {
            hotkeys: hotkeys,
            last_pressed_hotkey: None,
        }
    }
    pub fn get_last_key(&self) -> KeyboardKey {
        return self
            .last_pressed_hotkey
            .unwrap_or(KeyboardKey::KEY_NUM_LOCK);
    }

    pub fn clear_last(&mut self) {
        self.last_pressed_hotkey = None;
    }

    pub fn check_down(&mut self, rl: &RaylibHandle, target_intent: HotkeyCategory) -> bool {
        for (intent, keys) in self.hotkeys.iter() {
            if *intent != target_intent {
                continue;
            }

            for key in keys.iter() {
                if rl.is_key_down(*key) {
                    self.last_pressed_hotkey = Some(*key);
                    return true;
                }
            }
        }
        return false;
    }

    pub fn check_pressed(&mut self, rl: &RaylibHandle, target_intent: HotkeyCategory) -> bool {
        for (intent, keys) in self.hotkeys.iter() {
            if *intent != target_intent {
                continue;
            }

            for key in keys.iter() {
                if rl.is_key_pressed(*key) {
                    self.last_pressed_hotkey = Some(*key);
                    return true;
                }
            }
        }
        return false;
    }
}
