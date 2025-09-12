use crate::engine::PreciseOffset;

/// Types of collisions on an actor
#[derive(Clone, PartialEq)]
pub enum CollisionTypes {
    None,
    /// Bounding box collisions, (top left, bottom right exclusive)
    BoundingBox(PreciseOffset, PreciseOffset),
}
