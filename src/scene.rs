use raylib::prelude::*;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scene {
    MainMenu,
    Level,
    Transition,
    GameOver,
    GameEnd,
    LevelSelection,
}

pub struct SceneHandler {
    current: Scene,
    next: Option<Scene>,
    progress: f32,
    fade_in: bool,
    initiated: bool,
}

impl SceneHandler {
    pub fn new() -> Self {
        Self {
            current: Scene::MainMenu,
            next: None,
            progress: 0.,
            fade_in: true,
            initiated: false,
        }
    }

    pub fn get_current(&self) -> Scene {
        self.current
    }

    pub fn set(&mut self, scene: Scene) {
        if self.initiated {
            return;
        }
        self.initiated = true;
        self.fade_in = true;
        self.progress = 0.;
        self.next = Some(scene);
    }

    pub fn get_next(&self) -> Scene {
        match self.current {
            Scene::Level => Scene::Transition,
            Scene::Transition => Scene::Level,
            Scene::MainMenu => Scene::Level,
            Scene::GameOver => Scene::GameOver,
            _ => Scene::MainMenu,
        }
    }

    const FADE_SPEED: f32 = 4.;

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        if self.fade_in {
            if let Some(next) = self.next {
                self.progress += Self::FADE_SPEED * rl.get_frame_time();

                if self.progress >= 1.0 {
                    self.fade_in = false;
                    self.current = next;
                    self.next = None;
                    self.initiated = false;
                }
            }
        } else {
            self.progress -= Self::FADE_SPEED * rl.get_frame_time();

            if self.progress <= 0.0 {
                self.fade_in = true;
            }
        }
    }

    pub fn draw(&self, rl: &mut RaylibDrawHandle) {
        rl.draw_rectangle_v(
            Vector2::zero(),
            Vector2::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32),
            Color::BLACK.alpha(self.progress),
        );
    }
}
