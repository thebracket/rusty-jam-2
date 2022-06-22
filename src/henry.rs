use crate::{
    assets::GameAssets,
    player::{Facing, Player},
    maps::{tile_to_screen, LerpMove, TilePosition, NUM_TILES_X, NUM_TILES_Y, RegionMap}, interactions::Interaction,
};
use bevy::prelude::*;

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
        .insert(Interaction{
            output: vec![
                ("Henry wags his tail".to_string(), Color::YELLOW),
                ("Henry slurps your face".to_string(), Color::YELLOW),
                ("Henry encourages you to find the golden egg and win the game".to_string(), Color::YELLOW),
            ]
        });
}

pub fn distance(pos1: &TilePosition, pos2: &TilePosition) -> f32 {
    let dx = f32::abs(pos1.x as f32 - pos2.x as f32);
    let dy = f32::abs(pos1.y as f32 - pos2.y as f32);
    f32::sqrt((dx * dx) + (dy * dy))
}

pub fn henry_ai(
    mut queries: ParamSet<(
        Query<&TilePosition, With<Player>>,
        Query<(Entity, &mut Henry, &TilePosition), Without<LerpMove>>,
    )>,
    map: Res<RegionMap>,
    mut commands: Commands,
) {
    let player_pos = queries.p0().single().clone();
    for (entity, mut henry, henry_pos) in queries.p1().iter_mut() {
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
