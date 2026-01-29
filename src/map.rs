use raylib::{color::Color, prelude::*};

use crate::{
    map_loader,
    metadata_handler::MetadataHandler,
    music_handler::MusicHandler,
    scene::{Scene, SceneHandler},
    texture_handler::TextureHandler,
};

pub const TILE_SCALE_DEFAULT: i32 = 5;

pub const LEVEL_WIDTH_TILES: usize = 16;
pub const LEVEL_HEIGHT_TILES: usize = 9;
pub const TILE_SIZE_PX: i32 = 16;
pub const TILE_SIZE: i32 = TILE_SIZE_PX * TILE_SCALE_DEFAULT;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Air,
    FireTD { active: bool, selected: bool },
    FireLR { active: bool, selected: bool },
    FireStop { active: bool, selected: bool },
    Tree { chance: i32, selected: bool },
    Swamp { teleport_position: Vector2 },
    Exit(char),
}

pub struct Level {
    pub tiles: [[TileType; LEVEL_HEIGHT_TILES]; LEVEL_WIDTH_TILES],
    pub wood: usize,
    pub survived: usize,
    pub survive: usize,
}

impl Level {
    #[profiling::function]
    pub fn new() -> Self {
        Self {
            tiles: [[TileType::Air; LEVEL_HEIGHT_TILES]; LEVEL_WIDTH_TILES],
            wood: 0,
            survived: 0,
            survive: 0,
        }
    }

    fn set_load_data(&mut self, metadata_handler: &mut MetadataHandler) {
        self.survive = metadata_handler.get_survive();
        self.survived = 0;
        self.wood = 0;
        self.connect_swamps(metadata_handler);
        self.light_bonfires(metadata_handler);
    }

    #[profiling::function]
    pub fn load(
        &mut self,
        level_number: u8,
        metadata_handler: &mut MetadataHandler,
        rl: &mut RaylibHandle,
    ) {
        map_loader::MapLoader::get_map(level_number, self, rl);
        self.set_load_data(metadata_handler);
    }

    #[profiling::function]
    pub fn load_save(
        &mut self,
        level_number: u8,
        metadata_handler: &mut MetadataHandler,
        rl: &mut RaylibHandle,
    ) {
        map_loader::MapLoader::get_map_save(level_number, self, rl);
        self.set_load_data(metadata_handler);
    }

    pub fn completed(&self) -> bool {
        return self.survived >= self.survive;
    }

    pub fn get_wood(&self) -> usize {
        self.wood
    }

    pub fn add_wood(&mut self) {
        self.wood += 1;
    }

    pub fn remove_wood(&mut self) {
        self.wood -= 1;
    }

    pub fn survive(&mut self) {
        self.survived += 1;
    }

    #[profiling::function]
    pub fn connect_swamps(&mut self, metadata_handler: &mut MetadataHandler) {
        for i in metadata_handler.swamps.iter() {
            match self.tiles[i.swamp[0] as usize][i.swamp[1] as usize] {
                TileType::Swamp {
                    teleport_position: _,
                } => {
                    self.tiles[i.swamp[0] as usize][i.swamp[1] as usize] = TileType::Swamp {
                        teleport_position: Vector2::new(i.teleport[0] as f32, i.teleport[1] as f32),
                    };
                    // println!(
                    //     "teleport position - {} {} {} {}",
                    //     i.swamp[0], i.swamp[1], i.teleport[0], i.teleport[1]
                    // );
                }
                _ => {
                    //  println!(
                    //     "{} - {} - {} - {}",
                    //   i.swamp[0], i.swamp[1], i.teleport[0], i.teleport[1]
                    //);
                    panic!("COULDN'T PAIR METADATA WITH LOADED MAP");
                }
            }
        }
    }

    #[profiling::function]
    pub fn light_bonfires(&mut self, metadata_handler: &mut MetadataHandler) {
        for bonfire in metadata_handler.bonfires.iter_mut() {
            match self.tiles[bonfire.position[0] as usize][bonfire.position[1] as usize] {
                TileType::FireLR {
                    active: _,
                    selected: _,
                } => {
                    self.tiles[bonfire.position[0] as usize][bonfire.position[1] as usize] =
                        TileType::FireLR {
                            active: bonfire.active,
                            selected: false,
                        };
                }
                TileType::FireTD {
                    active: _,
                    selected: _,
                } => {
                    self.tiles[bonfire.position[0] as usize][bonfire.position[1] as usize] =
                        TileType::FireTD {
                            active: bonfire.active,
                            selected: false,
                        };
                }
                TileType::FireStop {
                    active: _,
                    selected: _,
                } => {
                    self.tiles[bonfire.position[0] as usize][bonfire.position[1] as usize] =
                        TileType::FireStop {
                            active: bonfire.active,
                            selected: false,
                        };
                }
                _ => panic!(
                    "ERROR WITH BONFIRES BINDING, METADATA POSITION - {} {}",
                    bonfire.position[0], bonfire.position[1]
                ),
            };
        }
    }

    #[profiling::function]
    pub fn update(
        &self,
        scene_handler: &mut SceneHandler,
        left_amount: u8,
        music_handler: &MusicHandler,
    ) {
        if self.completed() && left_amount == 0 {
            scene_handler.set(Scene::Transition);
        } else if left_amount == 0 {
            music_handler.play("death");
            scene_handler.set(Scene::GameOver);
        }
    }

    #[profiling::function]
    pub fn draw(
        &self,
        rl: &mut RaylibDrawHandle,
        texture_handler: &TextureHandler,
        level_number: u8,
    ) {
        for x in 0..LEVEL_WIDTH_TILES {
            for y in 0..LEVEL_HEIGHT_TILES {
                let stage_offset = if level_number < 10 {
                    0.
                } else if level_number < 20 {
                    TILE_SIZE_PX as f32
                } else {
                    TILE_SIZE_PX as f32 * 2.
                };

                // let source = Rectangle::new(((x + y) % 3) as f32 * 16., 0., 16., 16.);
                let source = Rectangle::new(((x + y) % 3) as f32 * 16., stage_offset, 16., 16.);

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
                    TileType::FireStop { active, selected } => {
                        let source = if active {
                            Rectangle::new(
                                ((rl.get_time() * 8.) % 4.).floor() as f32 * 16.,
                                16.,
                                16.,
                                16.,
                            )
                        } else if selected {
                            Rectangle::new(16., 0., 16., 16.)
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
                    TileType::Tree { chance, selected } => {
                        let offset = if chance >= 50 {
                            0
                        } else if chance >= 25 {
                            1
                        } else {
                            2
                        };

                        let selected_offset = if selected {
                            stage_offset * 2. + TILE_SIZE_PX as f32
                        } else {
                            stage_offset * 2.
                        };

                        let source = Rectangle::new(
                            offset as f32 * TILE_SIZE_PX as f32,
                            selected_offset,
                            TILE_SIZE_PX as f32,
                            TILE_SIZE_PX as f32,
                        );

                        rl.draw_texture_pro(
                            texture_handler.get_safe("trees"),
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
                    TileType::FireTD { active, selected } => {
                        let source = if active {
                            Rectangle::new(
                                ((rl.get_time() * 8.) % 4.).floor() as f32 * 16.,
                                16.,
                                16.,
                                16.,
                            )
                        } else if selected {
                            Rectangle::new(16., 0., 16., 16.)
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
                    TileType::FireLR { active, selected } => {
                        let source = if active {
                            Rectangle::new(
                                ((rl.get_time() * 8.) % 4.).floor() as f32 * 16.,
                                16.,
                                16.,
                                16.,
                            )
                        } else if selected {
                            Rectangle::new(16., 0., 16., 16.)
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

                        let source = Rectangle::new(
                            offset,
                            ((rl.get_time() * 8.) % 4.).floor() as f32 * 16.,
                            16.,
                            16.,
                        );

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
                    TileType::Swamp {
                        teleport_position: _,
                    } => {
                        let source = Rectangle::new(
                            ((rl.get_time() * 2.) % 2.).floor() as f32 * 16.,
                            16.,
                            16.,
                            16.,
                        );

                        rl.draw_texture_pro(
                            texture_handler.get_safe("swamp"),
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
                    _ => {}
                }
            }
        }
    }
}
