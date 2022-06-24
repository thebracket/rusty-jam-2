use super::{
    utils::{decorate_beach, decorate_water},
    MapToBuild, MapTransfer,
};
use crate::{
    maps::{tile_index, TileType, NUM_TILES_X, NUM_TILES_Y},
    random::Rng,
};
use bracket_pathfinding::prelude::Rect;

pub fn build(rng: &Rng, from: Option<MapToBuild>) -> MapTransfer {
    let mut tiles = vec![TileType::Water; NUM_TILES_X * NUM_TILES_Y];
    let features = vec![TileType::None; NUM_TILES_X * NUM_TILES_Y];
    let mut exits = Vec::new();
    let mut spawns = Vec::new();
    let player_start = if let Some(from) = from {
        match from {
            MapToBuild::Cave2 => (3, 0),
            _ => (27, NUM_TILES_Y as i32 - 1),
        }
    } else {
        (NUM_TILES_X as i32 / 2, NUM_TILES_Y as i32 / 2)
    };

    // Add some exits
    for x in 26..=29 {
        let idx = tile_index(x, NUM_TILES_Y as i32 - 1);
        tiles[idx] = TileType::CaveFloor;
        exits.push((idx, 1));
    }

    let mut rooms = Vec::new();
    rooms.push(Rect::with_exact(
        26,
        NUM_TILES_Y as i32 - 2,
        29,
        NUM_TILES_Y as i32 - 1,
    ));
    while rooms.len() < 10 {
        let try_room = Rect::with_size(
            rng.range(0, NUM_TILES_X as i32),
            rng.range(1, NUM_TILES_Y as i32),
            rng.range(2, 5),
            rng.range(2, 5),
        );
        if try_room.x1 > 0
            && try_room.x1 < NUM_TILES_X as i32
            && try_room.x2 > 0
            && try_room.x2 < NUM_TILES_X as i32
            && try_room.y1 > 1
            && try_room.y1 < NUM_TILES_Y as i32
            && try_room.y2 > 0
            && try_room.y2 < NUM_TILES_Y as i32
            && !rooms.iter().any(|r| r.intersect(&try_room))
        {
            rooms.push(try_room);
        }
    }
    rooms.push(Rect::with_exact(3, 0, 5, 0));
    for x in 3..=5 {
        let idx = tile_index(x, 0);
        tiles[idx] = TileType::CaveFloor;
        exits.push((idx, 3));
    }

    for room in rooms.iter() {
        room.for_each(|pt| {
            let idx = tile_index(pt.x, pt.y);
            tiles[idx] = TileType::CaveFloor;
        });
    }

    rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));
    build_corridors(&rng, &rooms, &mut tiles);

    decorate_beach(&mut tiles);
    decorate_water(&mut tiles, &rng);

    rooms.iter().skip(1).for_each(|r| {
        spawns.push(("WeakWolf".to_string(), r.center().x, r.center().y));
    });

    MapTransfer {
        tiles,
        features,
        name: "Sunken Cavern".to_string(),
        player_start,
        exits,
        spawns,
    }
}

// Taken from my book, Hands-on Rust

fn apply_horizontal_tunnel(x1: i32, x2: i32, y: i32, tiles: &mut [TileType]) {
    use std::cmp::{max, min};
    for x in min(x1, x2)..=max(x1, x2) {
        tiles[tile_index(x, y)] = TileType::CaveFloor;
        tiles[tile_index(x, y + 1)] = TileType::CaveFloor;
    }
}

fn apply_vertical_tunnel(y1: i32, y2: i32, x: i32, tiles: &mut [TileType]) {
    use std::cmp::{max, min};
    for y in min(y1, y2)..=max(y1, y2) {
        tiles[tile_index(x, y)] = TileType::CaveFloor;
        tiles[tile_index(x + 1, y)] = TileType::CaveFloor;
    }
}

fn build_corridors(rng: &Rng, rooms: &Vec<Rect>, tiles: &mut [TileType]) {
    for (i, room) in rooms.iter().enumerate().skip(1) {
        // (9)
        let prev = rooms[i - 1].center(); // (10)
        let new = room.center();

        if rng.range(0, 2) == 1 {
            // (11)
            apply_horizontal_tunnel(prev.x, new.x, prev.y, tiles);
            apply_vertical_tunnel(prev.y, new.y, new.x, tiles);
        } else {
            apply_vertical_tunnel(prev.y, new.y, prev.x, tiles);
            apply_horizontal_tunnel(prev.x, new.x, new.y, tiles);
        }
    }
}
