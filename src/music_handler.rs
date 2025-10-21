use raylib::prelude::*;
use std::collections::HashMap;
use std::fs;

const MUSIC_PATH: &str = "static/audio/";

pub struct MusicHandler<'a> {
    sounds: HashMap<String, Sound<'a>>,
    music: Music<'a>,
}

#[allow(unused)]
impl<'a> MusicHandler<'a> {
    pub fn new(rl_audio: &'a RaylibAudio) -> Self {
        let mut sounds = HashMap::new();
        let filenames = fs::read_dir(MUSIC_PATH).unwrap();

        for filename in filenames {
            let file = match filename {
                Ok(f) => f,
                Err(e) => panic!("COULDN'T LOAD SOUND - {e}"),
            };

            let name = file
                .file_name()
                .into_string()
                .unwrap()
                .split('.')
                .next()
                .unwrap()
                .to_string();

            let sound = rl_audio.new_sound(file.path().to_str().unwrap()).unwrap();
            sounds.insert(name, sound);
        }

        Self {
            sounds: sounds,
            music: rl_audio
                .new_music("static/music/forest_river_spirits.ogg")
                .unwrap(),
        }
    }

    pub fn music_play(&self) {
        self.music.play_stream();
    }

    pub fn music_update(&self) {
        self.music.update_stream();
    }

    pub fn music_pause(&self) {
        self.music.pause_stream();
    }

    pub fn music_resume(&self) {
        self.music.resume_stream();
    }

    pub fn play(&self, music_name: &str) {
        // for (i, sound) in self.sounds.iter(){
        //     println!("At least there's");
        //     sound.play();
        // }
        self.sounds.get(music_name).unwrap().play();
    }

    pub fn stop(&self, music_name: &str) {
        self.sounds.get(music_name).unwrap().stop();
    }
}
