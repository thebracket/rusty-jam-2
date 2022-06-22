use bracket_pathfinding::prelude::{DistanceAlg, Point};

use super::MapTransfer;
use crate::{
    maps::{tile_index, TileType, NUM_TILES_X, NUM_TILES_Y},
    random::Rng,
};

pub fn build_farmer_tom_coup(rng: &Rng) -> MapTransfer {
    let mut tiles = vec![TileType::Grass; NUM_TILES_X * NUM_TILES_Y];
    let mut features = vec![TileType::None; NUM_TILES_X * NUM_TILES_Y];
    let player_start = (NUM_TILES_X as i32 / 2, NUM_TILES_Y as i32 / 2);
    let mut exits = Vec::new();
    let mut spawns = Vec::new();

    // Coup
    for x in player_start.0 - 5..player_start.0 + 5 {
        for y in player_start.1 - 3..player_start.1 + 3 {
            tiles[tile_index(x, y)] = TileType::Dirt;
            if y == player_start.1 - 3 || y == player_start.1 + 2 {
                features[tile_index(x, y)] = TileType::FenceHorizontal;
            } else if x == player_start.0 - 5 || x == player_start.0 + 4 {
                features[tile_index(x, y)] = TileType::FenceVertical;
            }
        }
    }

    // Cauldron
    features[tile_index(player_start.0 - 3, player_start.1)] = TileType::Cauldron;

    // Boundaries
    for x in 0..NUM_TILES_X as i32 {
        features[tile_index(x, 0)] = TileType::Bush;
        features[tile_index(x, NUM_TILES_Y as i32 - 1)] = TileType::Bush;
        for y in 0..rng.range(1, 5) {
            features[tile_index(x, NUM_TILES_Y as i32 - 1 - y)] = TileType::Bush;
        }
        for y in 0..rng.range(1, 5) {
            features[tile_index(x, y)] = TileType::Bush;
        }
    }
    for y in 0..NUM_TILES_Y as i32 {
        features[tile_index(0, y)] = TileType::Bush;
        features[tile_index(NUM_TILES_X as i32 - 1, y)] = TileType::Bush;
        for x in 0..rng.range(1, 5) {
            features[tile_index(x, y)] = TileType::Bush;
        }
        for x in 0..rng.range(1, 5) {
            features[tile_index(NUM_TILES_X as i32 - 1 - x, y)] = TileType::Bush;
        }
    }

    // Add some pretty flowers and chickens
    tiles.iter_mut().enumerate().for_each(|(idx, t)| {
        if features[idx] == TileType::None && *t == TileType::Grass {
            if rng.range(1, 10) < 2 {
                features[idx] = TileType::Flower;
            }
            if rng.range(1, 20) < 2 {
                let x = idx % NUM_TILES_X;
                let y = idx / NUM_TILES_X;
                spawns.push(("Chicken".to_string(), x as i32, y as i32));
            }
        }
    });

    // Add a road
    for y in 0..player_start.1 - 3 {
        for x in player_start.0 - 1..player_start.0 + 2 {
            tiles[tile_index(x, y)] = TileType::Road;
            features[tile_index(x, y)] = TileType::None;
            if y == 0 {
                exits.push((tile_index(x, y), 1));
            }
        }
    }

    // Spawn a wolf
    let bottom_left = Point::new(0, NUM_TILES_Y - 1);
    let mut candidates: Vec<(usize, f32)> = tiles
        .iter()
        .enumerate()
        .filter(|(idx, t)| **t == TileType::Grass && features[*idx] == TileType::None)
        .map(|(idx, _)| {
            (
                idx,
                DistanceAlg::Pythagoras.distance2d(
                    Point::new((idx % NUM_TILES_X) as i32, (idx / NUM_TILES_X) as i32),
                    bottom_left,
                ),
            )
        })
        .collect();
    candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    spawns.push((
        "WeakWolf".to_string(),
        (candidates[0].0 % NUM_TILES_X) as i32,
        (candidates[0].0 / NUM_TILES_X) as i32,
    ));

    MapTransfer {
        tiles,
        features,
        name: "Farmer Tom's Coup".to_string(),
        player_start,
        exits,
        spawns,
    }
}
