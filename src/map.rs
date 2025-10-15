use raylib::{color::Color, prelude::*};

use crate::{metadata_handler::{self, MetadataHandler}, texture_handler::TextureHandler};

pub const LEVEL_WIDTH_TILES: usize = 16;
pub const LEVEL_HEIGHT_TILES: usize = 9;
const TILE_SIZE_PX: i32 = 16;
const TILE_SCALE: i32 = 4;
pub const TILE_SIZE: i32 = TILE_SIZE_PX * TILE_SCALE;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Air,
    FireTD { active: bool },
    FireLR { active: bool },
    FireStop { active: bool },
    Tree,
    Swamp{teleport_position: Vector2},
    Exit(char),
}

pub struct LevelMap {
    pub tiles: [[TileType; LEVEL_HEIGHT_TILES]; LEVEL_WIDTH_TILES],
}

impl LevelMap {
    pub fn new() -> Self {
        Self {
            tiles: [[TileType::Air; LEVEL_HEIGHT_TILES]; LEVEL_WIDTH_TILES],
        }
    }
    pub fn connect_swamps(&mut self, metadata_handler: MetadataHandler){
        for i in metadata_handler.swamps.iter(){
            match self.tiles[i.swamp[0] as usize][i.swamp[1] as usize]{
                TileType::Swamp{teleport_position: _} => {
                    self.tiles[i.swamp[0] as usize][i.swamp[1] as usize] = TileType::Swamp{
                        teleport_position: Vector2::new(
                            i.teleport[0] as f32,
                            i.teleport[1] as f32
                        ),
                    };
                    println!("teleport position - {} {} {} {}",i.swamp[0], i.swamp[1], i.teleport[0], i.teleport[1]);
                }
                _ =>{
                    println!("{} - {} - {} - {}", i.swamp[0], i.swamp[1], i.teleport[0], i.teleport[1]);
                    panic!("COULDN'T PAIR METADATA WITH LOADED MAP");
                }
            } 
        }
    }

    pub fn draw(&self, rl: &mut RaylibDrawHandle, texture_handler: &TextureHandler) {
        for x in 0..LEVEL_WIDTH_TILES {
            for y in 0..LEVEL_HEIGHT_TILES {
                let source = Rectangle::new(((x + y) % 3) as f32 * 16., 0., 16., 16.);

                rl.draw_texture_pro(
                    texture_handler.get_safe("grass"),
                    source,
                    Rectangle::new(
                        (x as i32 * TILE_SIZE) as f32,
                        (y as i32 * TILE_SIZE) as f32,
                        TILE_SIZE as f32,
                        TILE_SIZE as f32,
                    ),
                    Vector2::zero(),
                    0.0,
                    Color::WHITE,
                );

                match self.tiles[x][y] {
                    TileType::FireStop { active } => {
                        let source = if active {
                            Rectangle::new(
                                ((rl.get_time() * 4.) % 4.).floor() as f32 * 16.,
                                16.,
                                16.,
                                16.,
                            )
                        } else {
                            Rectangle::new(0., 0., 16., 16.)
                        };

                        rl.draw_texture_pro(
                            texture_handler.get_safe("fire_stop"),
                            source,
                            Rectangle::new(
                                (x as i32 * TILE_SIZE) as f32,
                                (y as i32 * TILE_SIZE) as f32,
                                TILE_SIZE as f32,
                                TILE_SIZE as f32,
                            ),
                            Vector2::zero(),
                            0.0,
                            Color::WHITE,
                        );
                    }
                    TileType::Tree => {
                        rl.draw_texture_pro(
                            texture_handler.get_safe("tree"),
                            Rectangle::new(0., 0., 16., 16.),
                            Rectangle::new(
                                (x as i32 * TILE_SIZE) as f32,
                                (y as i32 * TILE_SIZE) as f32,
                                TILE_SIZE as f32,
                                TILE_SIZE as f32,
                            ),
                            Vector2::zero(),
                            0.0,
                            Color::WHITE,
                        );
                    }
                    TileType::FireTD { active } => {
                        let source = if active {
                            Rectangle::new(
                                ((rl.get_time() * 4.) % 4.).floor() as f32 * 16.,
                                16.,
                                16.,
                                16.,
                            )
                        } else {
                            Rectangle::new(0., 0., 16., 16.)
                        };

                        rl.draw_texture_pro(
                            texture_handler.get_safe("fire_td"),
                            source,
                            Rectangle::new(
                                (x as i32 * TILE_SIZE) as f32,
                                (y as i32 * TILE_SIZE) as f32,
                                TILE_SIZE as f32,
                                TILE_SIZE as f32,
                            ),
                            Vector2::zero(),
                            0.0,
                            Color::WHITE,
                        );
                    }
                    TileType::FireLR { active } => {
                        let source = if active {
                            Rectangle::new(
                                ((rl.get_time() * 4.) % 4.).floor() as f32 * 16.,
                                16.,
                                16.,
                                16.,
                            )
                        } else {
                            Rectangle::new(0., 0., 16., 16.)
                        };

                        rl.draw_texture_pro(
                            texture_handler.get_safe("fire_lr"),
                            source,
                            Rectangle::new(
                                (x as i32 * TILE_SIZE) as f32,
                                (y as i32 * TILE_SIZE) as f32,
                                TILE_SIZE as f32,
                                TILE_SIZE as f32,
                            ),
                            Vector2::zero(),
                            0.0,
                            Color::WHITE,
                        );
                    }
                    TileType::Exit(rotation) => {
                        let offset = match rotation {
                            '^' => TILE_SIZE_PX as f32,
                            'v' => TILE_SIZE_PX as f32 * 3.,
                            '<' => 0.0,
                            '>' => TILE_SIZE_PX as f32 * 2.,
                            _ => {
                                panic!("impossible exit rotation")
                            }
                        };

                        let source = Rectangle::new(offset, 0., 16., 16.);

                        rl.draw_texture_pro(
                            texture_handler.get_safe("exit"),
                            source,
                            Rectangle::new(
                                (x as i32 * TILE_SIZE) as f32,
                                (y as i32 * TILE_SIZE) as f32,
                                TILE_SIZE as f32,
                                TILE_SIZE as f32,
                            ),
                            Vector2::zero(),
                            0.0,
                            Color::WHITE,
                        );
                    }
                    TileType::Swamp{teleport_position: _} =>{
                        rl.draw_texture_pro(
                            texture_handler.get_safe("swamp"),
                            Rectangle::new(0., 0., 16., 16.),    
                            Rectangle::new(
                                (x as i32 * TILE_SIZE) as f32,
                                (y as i32 * TILE_SIZE) as f32,
                                TILE_SIZE as f32,
                                TILE_SIZE as f32,
                            ),
                            Vector2::zero(),
                            0.0,
                            Color::WHITE,
                        );
                    }
                    _ => {}
                }
            }
        }
    }
}
