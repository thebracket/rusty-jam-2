use std::collections::HashSet;

use crate::maps::{RegionMap, TilePosition};
use bevy::prelude::*;
use bracket_pathfinding::prelude::{field_of_view_set, Point};

#[derive(Component)]
pub struct FieldOfView {
    pub range: i32,
    pub fov_set: HashSet<Point>,
}

impl FieldOfView {
    pub fn new(range: i32) -> Self {
        Self {
            range,
            fov_set: HashSet::new(),
        }
    }
}

pub fn update_field_of_view(
    mut fov_query: Query<
        (&mut FieldOfView, &TilePosition),
        Or<(Changed<TilePosition>, Added<TilePosition>)>,
    >,
    map: Res<RegionMap>,
) {
    for (mut fov, pos) in fov_query.iter_mut() {
        fov.fov_set = field_of_view_set(Point::new(pos.x, pos.y), fov.range, &*map);
    }
}
