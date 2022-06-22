use bevy::prelude::Component;

use super::NUM_TILES_X;

#[derive(Component, Clone)]
pub struct TilePosition {
    pub x: i32,
    pub y: i32,
}

pub fn tile_index(x: i32, y: i32) -> usize {
    ((NUM_TILES_X as i32 * y) + x) as usize
}
