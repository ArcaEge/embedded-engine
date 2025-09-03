use crate::engine::{Sprite, SpritePixel};

pub fn player_sprite() -> Sprite {
    Sprite {
        width: 2,
        height: 1,
        pixels: [
            // 0
            SpritePixel::White,
            SpritePixel::White,
        ]
        .to_vec(),
    }
}
