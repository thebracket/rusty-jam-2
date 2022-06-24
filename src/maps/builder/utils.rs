use crate::{
    maps::{tile_index, TileType, NUM_TILES_X, NUM_TILES_Y},
    random::Rng,
};

pub fn decorate_water(tiles: &mut [TileType], rng: &Rng) {
    tiles
        .iter_mut()
        .filter(|t| **t == TileType::Water)
        .for_each(|t| {
            let variant = rng.range(0, 20);
            match variant {
                0 => *t = TileType::WaterV2,
                1 => *t = TileType::WaterV3,
                2 => *t = TileType::WaterV4,
                3 => *t = TileType::WaterV5,
                4 => *t = TileType::WaterV6,
                _ => {}
            }
        });
}

pub fn decorate_beach(tiles: &mut [TileType]) {
    let tweaks: Vec<(usize, TileType)> = tiles
        .iter()
        .enumerate()
        .filter(|(_idx, t)| **t == TileType::CaveFloor)
        .map(|(idx, _)| {
            let (x, y) = ((idx % NUM_TILES_X) as i32, (idx / NUM_TILES_X) as i32);
            let mut bits = 0u8;
            if check(&tiles, x - 1, y) {
                bits += 1
            }
            if check(&tiles, x + 1, y) {
                bits += 2
            }
            if check(&tiles, x, y - 1) {
                bits += 4
            }
            if check(&tiles, x, y + 1) {
                bits += 8
            }

            (idx, bits)
        })
        .map(|(idx, bits)| {
            let output = match bits {
                1 => TileType::WaterL,
                2 => TileType::WaterR,
                3 => TileType::WaterTB, // East-west
                4 => TileType::WaterT,
                5 => TileType::WaterTL,
                6 => TileType::WaterTR,
                8 => TileType::WaterB,
                9 => TileType::WaterBL,
                10 => TileType::WaterBR,
                _ => TileType::CaveFloor,
            };
            (idx, output)
        })
        .collect();
    for (idx, t) in tweaks.iter() {
        tiles[*idx] = *t;
    }
}

// Taken from the roguelike tutorial
fn check(tiles: &[TileType], x: i32, y: i32) -> bool {
    x >= 0
        && x < NUM_TILES_X as i32 - 1
        && y >= 0
        && y < NUM_TILES_Y as i32 - 1
        && tiles[tile_index(x, y)] != TileType::CaveFloor
}
