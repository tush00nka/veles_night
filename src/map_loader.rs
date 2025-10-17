const MAP_PATH: &str = "static/maps/";
use raylib::prelude::*;
use crate::{map::{self, TileType, LEVEL_HEIGHT_TILES, LEVEL_WIDTH_TILES}};
use std::fs;
pub struct MapLoader;

impl MapLoader {
    pub fn get_map(level_number: u8, level_map: &mut map::Level, rl: &mut RaylibHandle) {
        let level_path = MAP_PATH.to_string() + &level_number.to_string();

        let Ok(mut level_str) = fs::read_to_string(level_path) else {
            panic!("CAN'T LOAD LEVEL");
        };

        level_str = level_str.replace('\n', "");
        if level_str.len() != LEVEL_WIDTH_TILES * LEVEL_HEIGHT_TILES{
            panic!("MAP IS NOT PROPPER SIZE, IT SHOULD BE - {}, AND IT'S - {}", LEVEL_WIDTH_TILES * LEVEL_HEIGHT_TILES, level_str.len());
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
                's' => {
                    println!("{} - {}", x, y);
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
}
