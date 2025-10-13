use std::{collections::HashMap, fs};

use raylib::prelude::*;

pub struct TextureHandler{
    textures: HashMap<String, Texture2D>,
    default_texture: Texture2D
}
const DEFAULT_TEXTURE: &str = "static/textures/tree.png";

impl TextureHandler{
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self{
        let mut textures = HashMap::new();

        let filenames = fs::read_dir("static/textures/").unwrap();

        for filename in filenames{
            let file = match filename{
                Ok(f) => f,
                Err(e) => panic!("Couldn't load this texture - {e}"),
            };

            let name = file
                .file_name()
                .into_string()
                .unwrap()
                .split('.')
                .next()
                .unwrap()
                .to_string();

            let texture = rl
                .load_texture(&thread, file.path().to_str().unwrap()).unwrap();

            textures.insert(name, texture);
        }

        let default_texture = rl.load_texture(&thread, DEFAULT_TEXTURE).unwrap();
        Self {textures, default_texture}
    }
    pub fn get_mut(&mut self, str: &str) -> &mut Texture2D {
        return self.textures.get_mut(str).unwrap();   
    }
    
    pub fn get(&self, str: &str) -> &Texture2D{
        return self.textures.get(str).unwrap();
    }
    
    pub fn get_mut_safe(&mut self, str: &str) -> &mut Texture2D{
        let texture = match self.textures.get_mut(str){
            Some(f) => f,
            _ => {
                println!("COULDN'T LOAD PROPER TEXTURE! USING DEFAULT - {DEFAULT_TEXTURE}");
                &mut self.default_texture
            }
        };
        return texture;
    }

    pub fn get_safe(&self, str: &str) -> &Texture2D{
        let texture = match self.textures.get(str){
            Some(f) => f,
            _ => {
                println!("COULDN'T LOAD PROPER TEXTURE! USING DEFAULT - {DEFAULT_TEXTURE}");
                &self.default_texture
            }
        };
        return texture;
    }
}
