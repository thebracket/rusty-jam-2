use super::{MapToBuild, MapTransfer};
use crate::{
    maps::{tile_index, TileType, NUM_TILES_X, NUM_TILES_Y},
    random::Rng,
};
use bracket_pathfinding::prelude::{DistanceAlg, Point};

pub fn build_farmer_tom_coup(rng: &Rng, from: Option<MapToBuild>) -> MapTransfer {
    let mut tiles = vec![TileType::Grass; NUM_TILES_X * NUM_TILES_Y];
    let mut features = vec![TileType::None; NUM_TILES_X * NUM_TILES_Y];
    let mut exits = Vec::new();
    let mut spawns = Vec::new();

    let player_start = if let Some(_from) = from {
        (17i32, 0i32)
    } else {
        (NUM_TILES_X as i32 / 2, NUM_TILES_Y as i32 / 2)
    };

    // Coup
    for x in 11..21 {
        for y in 7..13 {
            tiles[tile_index(x, y)] = TileType::Dirt;
            if y == 7 || y == 12 {
                features[tile_index(x, y)] = TileType::FenceHorizontal;
            } else if x == 11 || x == 20 {
                features[tile_index(x, y)] = TileType::FenceVertical;
            }
        }
    }

    // Cauldron
    features[tile_index(13, 10)] = TileType::Cauldron;

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
    for y in 0..7 {
        for x in 15..18 {
            tiles[tile_index(x, y)] = TileType::Road;
            features[tile_index(x, y)] = TileType::None;
            if y == 0 {
                exits.push((tile_index(x, y), 1));
            }
        }
    }

    // Spawn some wolves
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
    for i in 0..usize::min(candidates.len() - 1, 5) {
        spawns.push((
            "WeakWolf".to_string(),
            (candidates[i].0 % NUM_TILES_X) as i32,
            (candidates[i].0 / NUM_TILES_X) as i32,
        ));
    }

    MapTransfer {
        tiles,
        features,
        name: "Farmer Tom's Coup".to_string(),
        player_start,
        exits,
        spawns,
    }
}
