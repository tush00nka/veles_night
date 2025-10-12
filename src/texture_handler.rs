use std::{collections::HashMap, fs};

use raylib::prelude::*;

pub struct TextureHandler{
    textures: HashMap<String, Texture2D>,
}

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

        Self {textures}
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
            _ => panic!("Couldn't get this texture - {str}")
        };
        return texture;
    }

    pub fn get_safe(&self, str: &str) -> &Texture2D{
        let texture = match self.textures.get(str){
            Some(f) => f,
            _ => panic!("Couldn't get this texture - {str}")
        };
        return texture;
    }
}
