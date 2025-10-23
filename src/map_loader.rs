const MAP_PATH: &str = "static/maps/";
const MAP_SAVE_PATH: &str = "dynamic/save/map/";

use raylib::prelude::*;
use crate::{map::{self, TileType, LEVEL_HEIGHT_TILES, LEVEL_WIDTH_TILES}, metadata_handler::{self, BonfireMetadata, MetadataHandler}};
use std::{collections::HashMap, fs::{self, remove_file, write, File}, io::{BufWriter, Write}};
pub struct MapLoader;

impl MapLoader {
    pub fn get_map_save(level_number: u8, level_map: &mut map::Level, rl: &mut RaylibHandle){
        let level_path = MAP_SAVE_PATH.to_string() + &level_number.to_string();
        MapLoader::map_loading(level_path, level_map, rl);
    }

    fn map_loading(level_path: String, level_map: &mut map::Level, rl: &mut RaylibHandle){
        let Ok(mut level_str) = fs::read_to_string(level_path) else {
            panic!("CAN'T LOAD LEVEL");
        };

        level_str = level_str.replace('\n', "");
        if level_str.len() != LEVEL_WIDTH_TILES * LEVEL_HEIGHT_TILES{
            panic!("MAP IS NOT OF PROPER SIZE, IT SHOULD BE {}x{}, BUT IT IS {}", LEVEL_WIDTH_TILES, LEVEL_HEIGHT_TILES, level_str.len());
        }
        //need to load json there and get metadata structure
        let mut x: usize = 0;
        let mut y: usize = 0;

        for tile in level_str.chars().into_iter() {
            // clear map beforehand
            level_map.tiles[x][y] = TileType::Air;
            // debug
            // print!("{tile}");
            match tile {
                '#' => {
                    level_map.tiles[x][y] = TileType::Tree(rl.get_random_value(0..100));
                }
                '^' => {
                    level_map.tiles[x][y] = TileType::Exit('^');
                }
                '<' => {
                    level_map.tiles[x][y] = TileType::Exit('<');
                }
                '>' => {
                    level_map.tiles[x][y] = TileType::Exit('>');
                }
                'v' => {
                    level_map.tiles[x][y] = TileType::Exit('v');
                }
                '.' => {}
                '1' => {
                    level_map.tiles[x][y] = TileType::FireTD { active: false };
                }
                '2' => {
                    level_map.tiles[x][y] = TileType::FireLR { active: false };
                }
                '3' => {
                    level_map.tiles[x][y] = TileType::FireStop { active: false };
                }
                's' => {
                    level_map.tiles[x][y] = TileType::Swamp {
                        teleport_position: Vector2::zero(),
                    };
                }
                other => {
                    panic!("NOT DEFINED CHARACTER TO LOAD -{other}")
                }
            };

            x += 1;
            y += x / LEVEL_WIDTH_TILES as usize;
            x %= LEVEL_WIDTH_TILES as usize;
        }
    }

    pub fn save_map(level_number:u8, level_map: &mut map::Level, metadata_handler: &mut MetadataHandler){
        let mut map = "".to_string(); 
        let mut x = 0;
        let mut y = 0;

        let mut fire_td: HashMap<[u8;2], bool> = HashMap::new(); 
        let mut fire_lr: HashMap<[u8;2], bool> = HashMap::new(); 
        let mut fire_stop: HashMap<[u8;2], bool> = HashMap::new(); 

        for tile_line in level_map.tiles{
            for tile in tile_line{
                match tile {
                    TileType::Air => map += ".",
                    TileType::Tree(_) => map += "#",
                    TileType::FireTD { active } => {
                        fire_td.insert([x,y], active);
                        map += "1";
                    },
                    TileType::FireLR { active } => {
                        fire_lr.insert([x,y], active);
                        map += "2";
                    },
                    TileType::FireStop { active } => {
                        fire_stop.insert([x,y],active);
                        map += "3";
                    },
                    TileType::Swamp { teleport_position: _ } => map += "s",
                    TileType::Exit(val) => map += &val.to_string(),
                };
                x += 1;
            }
            y += 1;
            x = 0;
            map += "\n";
        }
        metadata_handler.change_bonfires(fire_td, fire_lr, fire_stop);
        
        let path = MAP_SAVE_PATH.to_string() + &level_number.to_string();
        
        let filenames = fs::read_dir(MAP_SAVE_PATH).unwrap();

        for filename in filenames{
            let file = match filename {
                Ok(f) => f,
                Err(e) => panic!("COULDN'T READ FILE IN SAVE MAP- {e}"),
            };
            
            match remove_file(file.path()){
                Ok(_) => (),
                Err(e) => panic!("ERROR OCCURED DURING CLEARING LAST SAVE - {e}"),
            };
        }
        let Ok(mut file) = File::create(path) else{
            panic!("COULDN'T SAVE LAST LEVEL MAP");
        };
        
        write!(file, "{}", map);
    }

    pub fn get_map(level_number: u8, level_map: &mut map::Level, rl: &mut RaylibHandle) {
        let level_path = MAP_PATH.to_string() + &level_number.to_string();
        MapLoader::map_loading(level_path, level_map, rl);
    }
}
