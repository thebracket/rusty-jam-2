use std::collections::HashSet;

use crate::{
    assets::GameAssets,
    combat::{AttackMessage, Health, Hostile, Unconscious},
    fov::FieldOfView,
    interactions::Interaction,
    maps::{
        tile_index, tile_to_screen, LerpMove, RegionMap, TilePosition, NUM_TILES_X, NUM_TILES_Y,
    },
};
use bevy::prelude::*;
use bracket_pathfinding::prelude::{DijkstraMap, DistanceAlg, Point};

use super::{Facing, Player, ScaresChickens};

#[derive(Component)]
pub struct Henry {
    facing: Facing,
}

pub fn spawn_henry(commands: &mut Commands, assets: &GameAssets, start: (i32, i32)) {
    let pos = tile_to_screen(start.0, start.1);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: assets.doggies.clone(),
            transform: Transform::from_xyz(pos.0, pos.1, 2.0),
            sprite: TextureAtlasSprite::new(8),
            ..default()
        })
        .insert(TilePosition {
            x: start.0,
            y: start.1,
        })
        .insert(Henry {
            facing: Facing::Right,
        })
        .insert(Interaction {
            output: vec![
                ("Henry wags his tail".to_string(), Color::YELLOW),
                ("Henry slurps your face".to_string(), Color::YELLOW),
                (
                    "Henry encourages you to find the golden egg and win the game".to_string(),
                    Color::YELLOW,
                ),
            ],
        })
        .insert(FieldOfView::new(8))
        .insert(ScaresChickens)
        .insert(Health {
            current: 10,
            max: 10,
        });
    //.insert(Tasty);
}

pub fn distance(pos1: &TilePosition, pos2: &TilePosition) -> f32 {
    let dx = f32::abs(pos1.x as f32 - pos2.x as f32);
    let dy = f32::abs(pos1.y as f32 - pos2.y as f32);
    f32::sqrt((dx * dx) + (dy * dy))
}

pub fn henry_ai(
    mut queries: ParamSet<(
        Query<&TilePosition, With<Player>>,
        Query<
            (Entity, &mut Henry, &TilePosition, &FieldOfView),
            (Without<LerpMove>, Without<Unconscious>),
        >,
        Query<(Entity, &TilePosition), (With<Hostile>, Without<Unconscious>)>,
    )>,
    map: Res<RegionMap>,
    mut commands: Commands,
    mut attack: EventWriter<AttackMessage>,
) {
    let player_pos = queries.p0().single().clone();
    {
        // This is a borrow-checker fighting mess
        let (hpos, henry_entity, fov_set) = if let Ok(hq) = queries.p1().get_single() {
            let hpos = (hq.2.x, hq.2.y);
            let henry_entity = hq.0.clone();
            (hpos, Some(henry_entity), hq.3.fov_set.clone())
        } else {
            ((0, 0), None, HashSet::new())
        };

        // Combat Henry
        queries.p2().iter().for_each(|(e, tpos)| {
            if DistanceAlg::Pythagoras
                .distance2d(Point::new(hpos.0, hpos.1), Point::new(tpos.x, tpos.y))
                < 1.2
            {
                attack.send(AttackMessage(henry_entity.unwrap(), e));
                return;
            }
        });

        // Check for things to chase
        let mut delta = None;
        if !fov_set.is_empty() {
            let mut starts = Vec::new();
            for (_, epos) in queries.p2().iter() {
                let pt = Point::new(epos.x, epos.y);
                if fov_set.contains(&pt) {
                    starts.push(tile_index(pt.x, pt.y));
                }
                if !starts.is_empty() {
                    let scary_map = DijkstraMap::new(NUM_TILES_X, NUM_TILES_Y, &starts, &*map, 9.0);
                    if let Some(exit) =
                        DijkstraMap::find_lowest_exit(&scary_map, tile_index(hpos.0, hpos.1), &*map)
                    {
                        let x = (exit % NUM_TILES_X) as i32;
                        let y = (exit / NUM_TILES_X) as i32;
                        delta = Some((x - hpos.0, y - hpos.1));
                    }
                }
            }
        }

        if let Some(delta) = delta {
            let destination = (
                (hpos.0 + delta.0).clamp(0, NUM_TILES_X as i32 - 1),
                (hpos.1 + delta.1).clamp(0, NUM_TILES_Y as i32 - 1),
            );
            if map.can_player_enter(destination.0, destination.1) {
                commands.entity(henry_entity.unwrap()).insert(LerpMove {
                    start: (hpos.0, hpos.1),
                    end: destination,
                    step: 0,
                    jumping: false,
                    animate: None,
                });
            }
        }
    }

    // Normal Henry
    for (entity, mut henry, henry_pos, _) in queries.p1().iter_mut() {
        let distance = distance(&henry_pos, &player_pos);
        if distance > 1.6 {
            let x = henry_pos.x;
            let y = henry_pos.y;
            let mut jumping = false;

            let delta = if x < player_pos.x && map.can_player_enter(x + 1, y) {
                henry.facing = Facing::Right;
                (1, 0)
            } else if x > player_pos.x && map.can_player_enter(x - 1, y) {
                henry.facing = Facing::Left;
                (-1, 0)
            } else if y < player_pos.y && map.can_player_enter(x, y + 1) {
                henry.facing = Facing::Down;
                (0, 1)
            } else if y > player_pos.y && map.can_player_enter(x, y - 1) {
                henry.facing = Facing::Up;
                (0, -1)
            } else if x < player_pos.x && map.can_player_enter(x + 2, y) {
                henry.facing = Facing::Right;
                jumping = true;
                (2, 0)
            } else if x > player_pos.x && map.can_player_enter(x - 2, y) {
                henry.facing = Facing::Left;
                jumping = true;
                (-2, 0)
            } else if y < player_pos.y && map.can_player_enter(x, y + 2) {
                henry.facing = Facing::Down;
                jumping = true;
                (0, 2)
            } else if y > player_pos.y && map.can_player_enter(x, y - 2) {
                henry.facing = Facing::Up;
                jumping = true;
                (0, -2)
            // LEAPING
            } else {
                (0, 0)
            };

            if delta != (0, 0) {
                let destination = (
                    (x + delta.0).clamp(0, NUM_TILES_X as i32 - 1),
                    (y + delta.1).clamp(0, NUM_TILES_Y as i32 - 1),
                );
                if map.can_player_enter(destination.0, destination.1) {
                    commands.entity(entity).insert(LerpMove {
                        start: (x, y),
                        end: destination,
                        step: 0,
                        jumping,
                        animate: match henry.facing {
                            Facing::Left => Some(vec![56, 57, 58]),
                            Facing::Right => Some(vec![8, 9, 10]),
                            Facing::Up => Some(vec![24, 25, 26]),
                            Facing::Down => Some(vec![72, 73, 74]),
                        },
                    });
                }
            }
        }
    }
}
