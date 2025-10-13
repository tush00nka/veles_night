const MAP_PATH: &str = "static/maps/";
use crate::{map, metadata_handler::MetadataHandler};
use std::fs;
pub struct MapLoader;

impl MapLoader{
    pub fn get_map(
        level_number: u8
        , level_map: &mut map::LevelMap
        , metadata_handler: MetadataHandler
        ){
        let level_path = MAP_PATH.to_string() + &level_number.to_string();
        
        let Ok(level_str) = fs::read_to_string(level_path) else{
            panic!("CAN'T LOAD LEVEL");
        };

        //need to load json there and get metadata structure
        let mut x: usize = 0;
        let mut y: usize = 0;

        for tile in level_str.chars().into_iter(){
            // debug
            // print!("{tile}");
            match tile{
                '#' => level_map.tiles[x][y] = map::TileType::Tree,
                '.' => (), 
                '\n' => (),
                other =>{panic!("NOT DEFINED CHARACTER TO LOAD -{other}")}
            };
            
            x += 1;
            y += x / metadata_handler.level_metadata.map_size[0] as usize;
            x %= metadata_handler.level_metadata.map_size[0] as usize;
        }
    }  
}
