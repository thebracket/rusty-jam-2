use super::{tile_index, LerpMove, MapElement, RegionMap, TilePosition, NUM_TILES_X, NUM_TILES_Y};
use crate::{
    actors::{Henry, Player},
    ai::Facing,
    assets::GameAssets,
    random::Rng,
};
use bevy::prelude::*;

pub fn map_exits(
    mut map: ResMut<RegionMap>,
    mut queries: ParamSet<(
        Query<&TilePosition, (With<Player>, Changed<TilePosition>)>,
        Query<Entity, With<MapElement>>,
        Query<(&Player, &mut TilePosition)>,
        Query<(Entity, &mut TilePosition), With<Henry>>,
    )>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    rng: Res<Rng>,
) {
    let mut transition = None;
    for player_pos in queries.p0().iter() {
        let player_idx = tile_index(player_pos.x, player_pos.y);
        for (exit, new_map) in map.exits.iter() {
            if *exit == player_idx {
                transition = Some(*new_map);
            }
        }
    }

    if let Some(new_map) = transition {
        map.transition_to(
            new_map,
            &mut commands,
            &queries.p1(),
            &assets,
            &mut meshes,
            &rng,
        );

        // Adjust player position
        let mut player_pos = (0, 0);
        for (player, mut ppos) in queries.p2().iter_mut() {
            player_pos = (ppos.x, ppos.y);
            match player.facing {
                Facing::Up => {
                    ppos.y = NUM_TILES_Y as i32 - 1;
                    player_pos.1 = NUM_TILES_Y as i32 - 1;
                }
                Facing::Down => {
                    ppos.y = 0;
                    player_pos.1 = 0;
                }
                Facing::Left => {
                    ppos.x = NUM_TILES_X as i32 - 1;
                    player_pos.0 = NUM_TILES_X as i32 - 1;
                }
                Facing::Right => {
                    ppos.x = 0;
                    player_pos.0 = 0;
                }
            }
        }
        for (henry, mut henry_pos) in queries.p3().iter_mut() {
            henry_pos.x = player_pos.0 - 1;
            henry_pos.y = player_pos.1;
            commands.entity(henry).remove::<LerpMove>();
        }
    }
}
