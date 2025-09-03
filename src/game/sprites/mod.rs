use crate::engine::{Sprite, SpritePixel, alloc::Vec};

pub fn player_sprite() -> Sprite {
    Sprite {
        width: 1,
        height: 1,
        pixels: Vec::from([
            // 0
            SpritePixel::Transparent,
        ]),
    }
}
