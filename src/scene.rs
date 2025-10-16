// use raylib::prelude::*;

#[derive(Clone, Copy)]
pub enum Scene {
    MainMenu,
    Level,
    Transition,
    GameOver,
}

pub struct SceneHandler {
    current: Scene
}

impl SceneHandler {
    pub fn new() -> Self {
        Self {
            current: Scene::MainMenu,
        }
    }

    pub fn set(&mut self, scene: Scene) {
        self.current = scene;
    }

    pub fn get_current(&self) -> Scene {
        self.current
    }
}
