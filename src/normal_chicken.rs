use bevy::prelude::*;
use bracket_pathfinding::prelude::{Point, DijkstraMap};
use crate::{assets::GameAssets, maps::{tile_to_screen, TilePosition, RegionMap, LerpMove, NUM_TILES_X, NUM_TILES_Y, tile_index}, fov::FieldOfView, interactions::Interaction, random::Rng};

#[derive(Component)]
pub struct Chicken;

#[derive(Component)]
pub struct ScaresChickens;

pub fn spawn_chicken(
    x: i32,
    y: i32,
    assets: &GameAssets,
    commands: &mut Commands,
) {
    let pos = tile_to_screen(x, y);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: assets.chick.clone(),
            transform: Transform::from_xyz(pos.0, pos.1, 2.0),
            sprite: TextureAtlasSprite::new(2),
            ..default()
        })
        .insert(TilePosition {
            x,
            y,
        })
        .insert(Chicken)
        .insert(Interaction{
            output: vec![
                ("The chicken clucks. It lacks the heart of a mega-chicken.".to_string(), Color::WHITE),
            ]
        })
        .insert(FieldOfView::new(3));
}

pub fn chicken_ai(
    map: Res<RegionMap>,
    mut ai_query: Query<(Entity, &TilePosition, &FieldOfView, &mut TextureAtlasSprite), (With<Chicken>, Without<LerpMove>)>,
    scary_query: Query<&TilePosition, With<ScaresChickens>>,
    mut commands: Commands,
    rng: Res<Rng>,
) {
    let mut delta = None;
    for (entity, pos, fov, mut sprite) in ai_query.iter_mut() {
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
                    if let Some(exit) = DijkstraMap::find_highest_exit(&scary_map, tile_index(pos.x, pos.y), &*map) {
                        let x = (exit % NUM_TILES_X) as i32;
                        let y = (exit / NUM_TILES_X) as i32;
                        delta = Some((
                            x - pos.x,
                            y - pos.y,
                        ));
                    }
                }
            }
        }

        // Do chicken things
        if delta.is_none() {
            match rng.range(0, 100) {
                1 => sprite.index = 0,
                2 => sprite.index = 1,
                3 => sprite.index = 2,
                4 => sprite.index = 26,
                6 => if map.can_player_enter(pos.x-1, pos.y) { delta = Some((-1, 0)) }
                7 => if map.can_player_enter(pos.x+1, pos.y) { delta = Some((1, 0)) }
                8 => if map.can_player_enter(pos.x, pos.y-1) { delta = Some((0, -1)) }
                9 => if map.can_player_enter(pos.x, pos.y+1) { delta = Some((0, 1)) }
                _ => {}
            }
        }

        if let Some(delta) = delta {
            if map.can_player_enter(pos.x + delta.0, pos.y + delta.1) {
                commands.entity(entity).insert(LerpMove{
                    jumping: false,
                    start: (pos.x, pos.y),
                    end: (pos.x + delta.0, pos.y + delta.1),
                    step: 0,
                    animate: match delta {
                        (-1, 0) => Some(vec![1, 2, 3, 4, 5]),
                        (1, 0) => Some(vec![16, 17, 18, 19, 20]),
                        (0, 1) => Some(vec![8, 9, 10, 11, 12]),
                        (0, 0) => Some(vec![24, 25, 26, 27]),
                        _ => None,
                    }
                });
            }
        }
    }
}