use bevy::prelude::*;
use bracket_pathfinding::prelude::{DijkstraMap, Point};

use crate::{
    assets::GameAssets,
    combat::{Health, LerpAttack},
    console::Console,
    fov::FieldOfView,
    interactions::Interaction,
    maps::{
        tile_index, tile_to_screen, LerpMove, MapElement, RegionMap, TilePosition, NUM_TILES_X,
        NUM_TILES_Y,
    },
};

use super::{Player, Tasty};

#[derive(Component)]
pub struct Farmer(bool);

pub fn spawn_farmer(x: i32, y: i32, assets: &GameAssets, commands: &mut Commands) {
    let pos = tile_to_screen(x, y);

    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.tom.clone(),
            transform: Transform::from_xyz(pos.0, pos.1, 2.0),
            ..default()
        })
        .insert(TilePosition { x, y })
        .insert(Farmer(false))
        .insert(Interaction {
            output: vec![
                (
                    "The farmer yells 'Get away from me!'".to_string(),
                    Color::WHITE,
                ),
                (
                    "The farmer sobs 'I've unleashed a monster!'".to_string(),
                    Color::WHITE,
                ),
                (
                    "The farmer sighs 'I never should have bought Magic Miracle Grow!'".to_string(),
                    Color::WHITE,
                ),
            ],
        })
        .insert(FieldOfView::new(8))
        .insert(MapElement)
        .insert(Health { current: 3, max: 3 })
        .insert(Tasty); // Don't persist chickens between levels
}

pub fn farmer_ai(
    map: Res<RegionMap>,
    mut ai_query: Query<(Entity, &mut Farmer, &TilePosition, &FieldOfView), (Without<LerpMove>, Without<LerpAttack>)>,
    scary_query: Query<&TilePosition, With<Player>>,
    mut commands: Commands,
    console: Res<Console>,
) {
    let mut delta = None;
    for (entity, mut farmer, pos, fov) in ai_query.iter_mut() {
        // Check for things to run away from
        if !fov.fov_set.is_empty() {
            let mut starts = Vec::new();
            for epos in scary_query.iter() {
                let pt = Point::new(epos.x, epos.y);
                if fov.fov_set.contains(&pt) {
                    starts.push(tile_index(pt.x, pt.y));
                }
                if !starts.is_empty() {
                    let scary_map = DijkstraMap::new(NUM_TILES_X, NUM_TILES_Y, &starts, &*map, 9.0);
                    if let Some(exit) =
                        DijkstraMap::find_highest_exit(&scary_map, tile_index(pos.x, pos.y), &*map)
                    {
                        let x = (exit % NUM_TILES_X) as i32;
                        let y = (exit / NUM_TILES_X) as i32;
                        delta = Some((x - pos.x, y - pos.y));
                    }
                }
            }
        }

        if let Some(delta) = delta {
            if map.can_player_enter(pos.x + delta.0, pos.y + delta.1) {
                if !farmer.0 {
                    console.write("The farmer screams about scary giant chickens", Color::PINK);
                    farmer.0 = true;
                }
                commands.entity(entity).insert(LerpMove {
                    jumping: false,
                    start: (pos.x, pos.y),
                    end: (pos.x + delta.0, pos.y + delta.1),
                    step: 0,
                    animate: None,
                });
            }
        }
    }
}
