use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    fullscreen: bool,
    pixel_scale: u8,
    music: f32,
    sound: f32,
    language: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            fullscreen: false,
            pixel_scale: 4,
            music: 100.0,
            sound: 100.0,
            language: "ru".to_string(),
        }
    }
}

pub struct SettingsHandler {
    settings: Settings,
}

const SETTINGS_PATH: &str = "dynamic/settings.json";

impl SettingsHandler {
    pub fn new() -> Self {
        let path = SETTINGS_PATH.to_string();

		if !std::fs::exists(&path).expect("COULDN'T CHECK IF SETTINGS FILE EXISTS") {
			return Self {settings: Settings::default() };
		}

        let Ok(s) = std::fs::read_to_string(path) else {
            panic!("COULDN'T LOAD SETTINGS");
        };
        let Ok(settings) = serde_json::from_str(&s) else {
            panic!("COULDN'T PARSE SETTINGS JSON");
        };

        Self { settings }
    }

    pub fn save(&self) {
        let Ok(s) = serde_json::to_string_pretty(&self.settings) else {
            panic!("COULDN'T SERIALIZE SETTINGS TO JSON");
        };

        std::fs::write(SETTINGS_PATH, s).expect("COULDN'T WRITE SETTINGS TO FILE");
    }
}
