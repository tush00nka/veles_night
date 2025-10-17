use std::fs;

use raylib::prelude::*;
use serde::Deserialize;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, map::TILE_SIZE_PX, texture_handler::TextureHandler};

pub enum CardContentType {
    Image(String),
    Text(String),
}

pub struct TransitionCard {
    pub stage: usize,
    pub content: CardContentType,
}

impl TransitionCard {
    fn new(content: CardContentType) -> Self {
        Self {
            stage: 0,
            content: content,
        }
    }
}

#[derive(Deserialize)]
struct LevelUnlock {
    texture: String,
    name: String,
    description: String,
}

#[derive(Deserialize)]
struct UnlockWrapper {
    unlocks: Vec<LevelUnlock>,
}

pub struct LevelTransition {
    unlock_wrapper: UnlockWrapper,
    pub cards: [TransitionCard; 3],
}

impl LevelTransition {
    pub fn new() -> Self {
        let path = "static/unlocks.json";
        let Ok(string_json) = fs::read_to_string(path) else {
            panic!("COULDN'T LOAD JSON FOR UNLOCKS");
        };

        let Ok(unlock_wrapper) = serde_json::from_str(&string_json) else {
            panic!("COULDN'T PARSE JSON FOR UNLOCKS");
        };

        let mut new_transition = Self {
            unlock_wrapper,
            cards: [
                TransitionCard::new(CardContentType::Image("".to_string())),
                TransitionCard::new(CardContentType::Text("".to_string())),
                TransitionCard::new(CardContentType::Text("".to_string())),
            ],
        };

        // ensure we have something to show
        new_transition.set_cards(0);

        new_transition
    }

    pub fn set_cards(&mut self, level_completed: usize) {
        for card in self.cards.iter_mut() {
            card.stage = 0;
        }

        self.cards[0].content =
            CardContentType::Image(self.unlock_wrapper.unlocks[level_completed].texture.clone());
        self.cards[1].content =
            CardContentType::Text(self.unlock_wrapper.unlocks[level_completed].name.clone());
        self.cards[2].content = CardContentType::Text(
            self.unlock_wrapper.unlocks[level_completed]
                .description
                .clone(),
        );
    }

    const CARD_SIZE: i32 = 256;

    pub fn draw(
        &mut self,
        texture_handler: &TextureHandler,
        font: &Font,
        rl: &mut RaylibDrawHandle,
    ) {
        rl.clear_background(Color::BROWN);

        let cards = [
            Rectangle::new(
                (SCREEN_WIDTH / 2 - Self::CARD_SIZE / 2 - Self::CARD_SIZE - 20) as f32,
                (SCREEN_HEIGHT / 2 - Self::CARD_SIZE / 2) as f32,
                Self::CARD_SIZE as f32,
                Self::CARD_SIZE as f32,
            ),
            Rectangle::new(
                (SCREEN_WIDTH / 2 - Self::CARD_SIZE / 2) as f32,
                (SCREEN_HEIGHT / 2 - Self::CARD_SIZE / 2) as f32,
                Self::CARD_SIZE as f32,
                Self::CARD_SIZE as f32,
            ),
            Rectangle::new(
                (SCREEN_WIDTH / 2 - Self::CARD_SIZE / 2 + Self::CARD_SIZE + 20) as f32,
                (SCREEN_HEIGHT / 2 - Self::CARD_SIZE / 2) as f32,
                Self::CARD_SIZE as f32,
                Self::CARD_SIZE as f32,
            ),
        ];

        for i in 0..3 {
            let offset;
            if self.cards[i].stage >= 5 {
                offset = 64. * 5.;
            } else {
                offset = self.cards[i].stage as f32 * 64.;
                self.cards[i].stage = ((rl.get_time() * 8.) % 6.).floor() as usize;
            };

            rl.draw_texture_pro(
                texture_handler.get_safe("card"),
                Rectangle::new(offset, 0.0, 64., 64.),
                cards[i],
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );

            if self.cards[i].stage < 5 {
                continue;
            }

            match &self.cards[i].content {
                CardContentType::Image(img) => rl.draw_texture_pro(
                    texture_handler.get_safe(img.as_str()),
                    Rectangle::new(
                        ((rl.get_time() * 8.) % 4.).floor() as f32 * TILE_SIZE_PX as f32,
                        TILE_SIZE_PX as f32,
                        TILE_SIZE_PX as f32,
                        TILE_SIZE_PX as f32,
                    ),
                    Rectangle::new(cards[i].x + 128. - 32., cards[i].y + 128. - 32., 64., 64.),
                    Vector2::zero(),
                    0.0,
                    Color::WHITE,
                ),
                CardContentType::Text(text) => {
                    let line_count = text.chars().filter(|&c| c == '\n').count() as f32;

                    rl.draw_text_pro(
                        font,
                        text.as_str(),
                        Vector2::new(
                            cards[i].x + 32.,
                            cards[i].y + 128. - (line_count + 1.) * 14.,
                        ),
                        Vector2::zero(),
                        0.0,
                        28.,
                        0.0,
                        Color::BLACK,
                    )
                }
            }
        }
    }
}
