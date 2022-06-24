use super::{tile_index, TileType};
use crate::random::Rng;
mod unreachable;
mod cave1;
mod cave2;
mod coup;
mod tom_house;

#[derive(Clone, Copy)]
pub enum MapToBuild {
    FarmerTomCoup,
    FarmHouse,
    Cave1,
    Cave2,
}

pub struct MapTransfer {
    pub tiles: Vec<TileType>,
    pub features: Vec<TileType>,
    pub name: String,
    pub player_start: (i32, i32),
    pub exits: Vec<(usize, usize)>,
    pub spawns: Vec<(String, i32, i32)>,
}

pub fn builder(map: MapToBuild, rng: &Rng, from: Option<MapToBuild>) -> MapTransfer {
    match map {
        MapToBuild::FarmerTomCoup => coup::build_farmer_tom_coup(rng, from),
        MapToBuild::FarmHouse => tom_house::build_toms_house(rng, from),
        MapToBuild::Cave1 => cave1::build(rng, from),
        MapToBuild::Cave2 => cave2::build(rng, from),
    }
}

fn spawn_big_feature(x: i32, y: i32, feature: TileType, features: &mut [TileType]) {
    let (width, height) = match feature {
        TileType::HayCart => (3, 2),
        TileType::Barn => (2, 3),
        TileType::LeftButte => (2, 7),
        TileType::CaveMouth => (6, 1),
        _ => (0, 0),
    };

    if width == 0 || height == 0 {
        return;
    }

    let base_idx = tile_index(x, y);
    for tx in 0..width {
        for ty in 0..height {
            let idx = tile_index(x + tx, y + ty);
            features[idx] = TileType::ReferTo(base_idx);
        }
    }
    features[base_idx] = feature;
}
