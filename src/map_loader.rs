const MAP_PATH: &str = "static/maps/";
use raylib::prelude::*;
use crate::{map::{self, TileType, LEVEL_WIDTH_TILES}, metadata_handler};
use std::fs;
pub struct MapLoader;

impl MapLoader {
    pub fn connect_swamps(level_map: &mut map::LevelMap, swamps_data: metadata_handler::MetadataHandler){
        for i in swamps_data.swamps.iter(){
            match level_map.tiles[i.swamp[0] as usize][i.swamp[1] as usize]{
                TileType::Swamp{teleport_position: _} => (), 
                _ =>{
                    println!("{} - {} - {} - {}", i.swamp[0], i.swamp[1], i.teleport[0], i.teleport[1]);
                    panic!("COULDN'T PAIR METADATA WITH LOADED MAP");
                }
            }
            level_map.tiles[i.swamp[0] as usize][i.swamp[1] as usize] = TileType::Swamp{
                teleport_position: Vector2::new(
                                       i.teleport[0] as f32,
                                       i.teleport[1] as f32
                                   )
            };
        }
    }
        
    pub fn get_map(level_number: u8, level_map: &mut map::LevelMap) {
        let level_path = MAP_PATH.to_string() + &level_number.to_string();

        let Ok(level_str) = fs::read_to_string(level_path) else {
            panic!("CAN'T LOAD LEVEL");
        };

        //need to load json there and get metadata structure
        let mut x: usize = 0;
        let mut y: usize = 0;

        for tile in level_str.chars().into_iter() {
            // debug
            // print!("{tile}");
            match tile {
                '#' => {
                    level_map.tiles[x][y] = map::TileType::Tree;
                }
                '^' => {
                    level_map.tiles[x][y] = map::TileType::Exit('^');
                }
                '<' => {
                    level_map.tiles[x][y] = map::TileType::Exit('<');
                }
                '>' => {
                    level_map.tiles[x][y] = map::TileType::Exit('>');
                }
                'v' => {
                    level_map.tiles[x][y] = map::TileType::Exit('v');
                }
                '.' => {}
                's' => {
                    println!("{} - {}", x, y);
                    level_map.tiles[x][y] = map::TileType::Swamp{teleport_position: Vector2::zero()};
                }
                '\n' => continue,
                other => {
                    panic!("NOT DEFINED CHARACTER TO LOAD -{other}")
                }
            };

            x += 1;
            y += x / LEVEL_WIDTH_TILES as usize;
            x %= LEVEL_WIDTH_TILES as usize;
        }
    }
}
