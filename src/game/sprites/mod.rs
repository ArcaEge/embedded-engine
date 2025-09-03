use crate::engine::{Sprite, SpritePixel};

pub const PLAYER_SPRITE: Sprite = Sprite {
    width: 1,
    height: 1,
    pixels: &[
        // 0
        SpritePixel::Transparent,
    ],
};
