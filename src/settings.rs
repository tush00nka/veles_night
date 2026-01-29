use serde::{Deserialize, Serialize};

use crate::map::TILE_SCALE_DEFAULT;
pub const MAXIMUM_PIXEL_SCALE: u8 = 3;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Settings {
    pub language: String,
    pub music: f32,
    pub sound: f32,
    pub pixel_scale: u8,
    pub shader: bool,
    pub fullscreen: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            pixel_scale: TILE_SCALE_DEFAULT as u8,
            music: 100.0,
            sound: 100.0,
            language: "ru".to_string(),
            shader: false,
            fullscreen: false,
        }
    }
}

pub struct SettingsHandler {
    settings: Settings,
}

const SETTINGS_PATH: &str = "dynamic/settings.json";

impl SettingsHandler {
    #[profiling::function]
    pub fn new() -> Self {
        let path = SETTINGS_PATH.to_string();

        if !std::fs::exists(&path).expect("COULDN'T CHECK IF SETTINGS FILE EXISTS") {
            return Self {
                settings: Settings::default(),
            };
        }

        let Ok(s) = std::fs::read_to_string(path) else {
            return Self {
                settings: Settings::default(),
            };
        };
        let Ok(settings) = serde_json::from_str(&s) else {
            return Self {
                settings: Settings::default(),
            };
        };

        Self { settings }
    }

    #[profiling::function]
    pub fn save(&self) {
        let Ok(s) = serde_json::to_string_pretty(&self.settings) else {
            panic!("COULDN'T SERIALIZE SETTINGS TO JSON");
        };

        std::fs::write(SETTINGS_PATH, s).expect("COULDN'T WRITE SETTINGS TO FILE");
    }
    pub fn get_settings(&self) -> &Settings {
        return &self.settings;
    }
    pub fn set_settings(&mut self, settings: &Settings) {
        self.settings = settings.clone();
    }
}
