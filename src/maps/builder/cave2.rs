use super::{MapToBuild, MapTransfer};
use crate::{
    maps::{tile_index, TileType, NUM_TILES_X, NUM_TILES_Y},
    random::Rng,
};

pub fn build(rng: &Rng, from: Option<MapToBuild>) -> MapTransfer {
    let mut tiles = vec![TileType::Water; NUM_TILES_X * NUM_TILES_Y];
    let mut features = vec![TileType::None; NUM_TILES_X * NUM_TILES_Y];
    let mut exits = Vec::new();
    let mut spawns = Vec::new();
    let player_start = if let Some(from) = from {
        match from {
            _ => (4, NUM_TILES_Y as i32 - 2),
        }
    } else {
        (4, NUM_TILES_Y as i32 - 2)
    };

    for y in NUM_TILES_Y as i32 - 3..NUM_TILES_Y as i32 - 1 {
        for x in 3..=5 {
            let idx = tile_index(x, y);
            tiles[idx] = TileType::CaveFloor;
            if y == NUM_TILES_Y as i32 - 1 {
                exits.push((idx, 2));
            }
        }
    }

    for y in 9..NUM_TILES_Y as i32 - 3 {
        for x in 2..=8 {
            let idx = tile_index(x, y);
            if y % 2 == 0 {
                tiles[idx] = TileType::Fire;
            } else {
                tiles[idx] = TileType::CaveFloor;
                if (x + y) % 2 == 0 {
                    spawns.push(("Spikes1".to_string(), x, y));
                } else {
                    if rng.range(1, 5) > 1 {
                        spawns.push(("Spikes2".to_string(), x, y));
                    }
                }
            }
        }
    }

    for y in 5..9 {
        for x in 3..=5 {
            let idx = tile_index(x, y);
            tiles[idx] = TileType::CaveFloor;
        }
    }
    features[tile_index(3, 5)] = TileType::Grain;

    for y in 6..10 {
        tiles[tile_index(6, y)] = TileType::CaveFloor;
    }
    for y in 7..11 {
        tiles[tile_index(7, y)] = TileType::CaveFloor;
    }
    for x in 4..11 {
        tiles[tile_index(x, 5)] = TileType::CaveFloor;
    }
    for y in 5..12 {
        for x in 10..25 {
            tiles[tile_index(x, y)] = TileType::CaveFloor;
        }
    }
    for x in 10..NUM_TILES_X as i32 - 1 {
        tiles[tile_index(x, 8)] = TileType::CaveFloor;
    }
    features[tile_index(30, 8)] = TileType::GoldEgg;
    spawns.push(("WhiteWolf".to_string(), 20, 8));

    MapTransfer {
        tiles,
        features,
        name: "Farmer Tom's House".to_string(),
        player_start,
        exits,
        spawns,
    }
}
