use crate::engine::{Sprite, SpritePixel};

pub const PLAYER_SPRITE: Sprite = Sprite {
    width: 3,
    height: 3,
    pixels: &[
        // 0
        SpritePixel::White,
        SpritePixel::White,
        SpritePixel::White,
        // 1
        SpritePixel::White,
        SpritePixel::Black,
        SpritePixel::White,
        // 2
        SpritePixel::White,
        SpritePixel::White,
        SpritePixel::White,
    ],
};
