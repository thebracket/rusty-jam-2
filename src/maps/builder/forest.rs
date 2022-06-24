use super::{
    MapToBuild, MapTransfer, unreachable::unreachable,
};
use crate::{
    maps::{tile_index, TileType, NUM_TILES_X, NUM_TILES_Y},
    random::Rng,
};

pub fn build(rng: &Rng, from: Option<MapToBuild>) -> MapTransfer {
    let mut tiles = vec![TileType::Dirt; NUM_TILES_X * NUM_TILES_Y];
    let mut features = vec![TileType::None; NUM_TILES_X * NUM_TILES_Y];
    let mut exits = Vec::new();
    let mut spawns = Vec::new();
    let player_start = if let Some(from) = from {
        match from {
            MapToBuild::Cave1 => (27, 0),
            _ => (27, NUM_TILES_Y as i32 - 1),
        }
    } else {
        (27, NUM_TILES_Y as i32 - 1)
    };

    // Randomize (false is passable)
    let mut map = Vec::with_capacity(NUM_TILES_X * NUM_TILES_Y);
    for _ in 0..NUM_TILES_X * NUM_TILES_Y {
        let roll = rng.range(1, 101);
        if roll > 55 { map.push(false); } else { map.push(true); }
    }

    for _ in 0..10 {
        let mut new = map.clone();
        for y in 1..NUM_TILES_Y-2 {
            for x in 1..NUM_TILES_X-2 {
                let idx = tile_index(x as i32, y as i32);
                let mut neighbors = 0;
                if map[idx-1] { neighbors += 1 }
                if map[idx+1] { neighbors += 1 }
                if map[idx-NUM_TILES_X] { neighbors += 1 }
                if map[idx+NUM_TILES_X] { neighbors += 1 }
                if map[idx-(NUM_TILES_X-1)] { neighbors += 1 }
                if map[idx-(NUM_TILES_X+1)] { neighbors += 1 }
                if map[idx+(NUM_TILES_X-1)] { neighbors += 1 }
                if map[idx+(NUM_TILES_X+1)] { neighbors += 1 }
                if neighbors > 4 || neighbors == 0 {
                    new[idx] = true;
                } else {
                    new[idx] = false;
                }
            }
        }
        map = new;
    }
    map.iter().enumerate().for_each(|(idx, t)| {
        if *t {
            tiles[idx] = TileType::Grass;
            features[idx] = TileType::Bush;
        } else {
            tiles[idx] = TileType::Grass;
            features[idx] = TileType::None;
        }
    });

    // Cover the exit
    for x in 0..NUM_TILES_X as i32 {
        features[tile_index(x, 0)] = TileType::Bush;
        features[tile_index(x, NUM_TILES_Y as i32 -1)] = TileType::Bush;
    }
    for y in 0..NUM_TILES_Y as i32 -1 {
        features[tile_index(0, y)] = TileType::Bush;
        features[tile_index(NUM_TILES_X as i32 -1, y)] = TileType::Bush;
    }

    // Ensure entrance to exit
    for y in 5..NUM_TILES_Y as i32 {
        for x in 26..=29 {
            features[tile_index(x, y)] = TileType::None;
        }
    }
    for x in 1..NUM_TILES_X as i32 - 1 {
        features[tile_index(x, 5)] = TileType::None;
    }

    // Exit
    for y in 0..5 {
        features[tile_index(27, y)] = TileType::None;
    }
    exits.push((tile_index(27, 0), 2));
    tiles[tile_index(27, 0)] = TileType::CaveFloor;

    for idx in unreachable(&tiles, &features, &vec![tile_index(27, NUM_TILES_Y as i32 - 1)]) {
        features[idx] = TileType::Bush;
    }

    // Spawn stuff
    let mut open_spots : Vec<usize> = features.iter().enumerate().filter(|(_,f)| **f == TileType::None).map(|(idx,_)| idx).collect();
    for i in 0..20 {
        let spot_index = rng.range(0, open_spots.len() as i32) as usize;
        let spot = open_spots[spot_index];
        match i {
            0 => features[spot] = TileType::Grain,
            1..=10 => features[spot] = TileType::Web,
            _ => spawns.push(("Spider".to_string(), (spot % NUM_TILES_X) as i32, (spot / NUM_TILES_X) as i32))
        }
        open_spots.remove(spot_index);
    }

    MapTransfer {
        tiles,
        features,
        name: "Into the Woods".to_string(),
        player_start,
        exits,
        spawns,
    }
}
