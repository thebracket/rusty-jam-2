use super::{spawn_big_feature, MapTransfer};
use crate::{
    maps::{tile_index, TileType, NUM_TILES_X, NUM_TILES_Y},
    random::Rng,
};

pub fn build_toms_house(rng: &Rng) -> MapTransfer {
    let mut tiles = vec![TileType::Grass; NUM_TILES_X * NUM_TILES_Y];
    let mut features = vec![TileType::None; NUM_TILES_X * NUM_TILES_Y];
    let player_start = (NUM_TILES_X as i32 / 2, NUM_TILES_Y as i32 / 2);
    let mut exits = Vec::new();
    let mut spawns = Vec::new();

    // Boundaries
    for x in 0..NUM_TILES_X as i32 {
        features[tile_index(x, 0)] = TileType::Bush;
        features[tile_index(x, NUM_TILES_Y as i32 - 1)] = TileType::Bush;
        for y in 0..rng.range(1, 3) {
            features[tile_index(x, NUM_TILES_Y as i32 - 1 - y)] = TileType::Bush;
        }
        for y in 0..rng.range(1, 3) {
            features[tile_index(x, y)] = TileType::Bush;
        }
    }
    for y in 0..NUM_TILES_Y as i32 {
        features[tile_index(0, y)] = TileType::Bush;
        features[tile_index(NUM_TILES_X as i32 - 1, y)] = TileType::Bush;
        for x in 0..rng.range(1, 3) {
            features[tile_index(x, y)] = TileType::Bush;
        }
        for x in 0..rng.range(1, 3) {
            features[tile_index(NUM_TILES_X as i32 - 1 - x, y)] = TileType::Bush;
        }
    }

    // Add a road
    let half_width = NUM_TILES_X as i32 / 2;
    for y in NUM_TILES_Y as i32 - 5..NUM_TILES_Y as i32 {
        for x in half_width - 1..half_width + 2 {
            tiles[tile_index(x, y)] = TileType::Road;
            features[tile_index(x, y)] = TileType::None;
            if y == NUM_TILES_Y as i32 - 1 {
                exits.push((tile_index(x, y), 0));
            }
        }
    }

    // Cobbles
    for x in 13..25 {
        for y in 6..15 {
            if y == 6 {
                tiles[tile_index(x, y)] = TileType::CobbleT;
            } else if y == 14 {
                tiles[tile_index(x, y)] = TileType::CobbleB;
            } else if x == 13 {
                tiles[tile_index(x, y)] = TileType::CobbleL;
            } else if x == 24 {
                tiles[tile_index(x, y)] = TileType::CobbleR;
            } else {
                tiles[tile_index(x, y)] = TileType::Cobble;
            }
        }
    }
    tiles[tile_index(13, 6)] = TileType::CobbleTL;
    tiles[tile_index(24, 6)] = TileType::CobbleTR;
    tiles[tile_index(13, 14)] = TileType::CobbleBL;
    tiles[tile_index(24, 14)] = TileType::CobbleBR;

    // Anvil
    features[tile_index(23, 13)] = TileType::Anvil;

    // Add a haycart
    spawn_big_feature(10, 5, TileType::HayCart, &mut features);

    // Add a barn
    spawn_big_feature(14, 5, TileType::Barn, &mut features);

    // Add a rocky outcropping
    spawn_big_feature(0, 11, TileType::LeftButte, &mut features);

    // Add a cave mouth
    spawn_big_feature(25, 0, TileType::CaveMouth, &mut features);
    //exits.push((tile_index(25, 3), 2));
    exits.push((tile_index(26, 3), 2));
    exits.push((tile_index(27, 3), 2));
    exits.push((tile_index(28, 3), 2));
    exits.push((tile_index(29, 3), 2));
    //exits.push((tile_index(30, 3), 2));

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

    // Add the farmer
    spawns.push(("Farmer".to_string(), 18, 7));

    MapTransfer {
        tiles,
        features,
        name: "Farmer Tom's House".to_string(),
        player_start,
        exits,
        spawns,
    }
}
