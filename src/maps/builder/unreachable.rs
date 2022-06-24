use crate::maps::{TileType, NUM_TILES_X, NUM_TILES_Y};
use bracket_pathfinding::prelude::{Algorithm2D, BaseMap, DijkstraMap, Point, SmallVec};

pub fn unreachable(tiles: &[TileType], features: &[TileType], starts: &[usize]) -> Vec<usize> {
    let map = MinimumTileMap {
        tiles: tiles
            .iter()
            .zip(features.iter())
            .map(|(t, f)| f.can_player_enter() && t.can_player_enter())
            .collect(),
    };
    let mut dm = DijkstraMap::new(NUM_TILES_X, NUM_TILES_Y, starts, &map, 200.0);
    DijkstraMap::build(&mut dm, starts, &map);
    dm.map
        .iter()
        .enumerate()
        .filter(|(_, cost)| **cost > 2000.0)
        .map(|(idx, _)| idx)
        .collect()
}

struct MinimumTileMap {
    tiles: Vec<bool>,
}

impl MinimumTileMap {
    fn try_exit(&self, location: Point, delta: Point) -> Option<usize> {
        let destination = location + delta;
        if self.in_bounds(destination) && self.tiles[self.point2d_to_index(destination)] {
            Some(self.point2d_to_index(destination))
        } else {
            None
        }
    }
}

impl Algorithm2D for MinimumTileMap {
    fn dimensions(&self) -> Point {
        Point::new(NUM_TILES_X, NUM_TILES_Y)
    }
}

impl BaseMap for MinimumTileMap {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);
        if let Some(idx) = self.try_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0));
        }
        if let Some(idx) = self.try_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0));
        }
        if let Some(idx) = self.try_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0));
        }
        if let Some(idx) = self.try_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0));
        }
        exits
    }
}
