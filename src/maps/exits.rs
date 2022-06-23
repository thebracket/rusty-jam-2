use super::{tile_index, LerpMove, MapElement, RegionMap, TilePosition};
use crate::{
    actors::{Henry, Player},
    ai::ActionRequest,
    assets::GameAssets,
    combat::DamageMessage,
    random::Rng,
};
use bevy::{ecs::event::Events, prelude::*};

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
    mut events: ResMut<Events<ActionRequest>>,
    mut damage: ResMut<Events<DamageMessage>>,
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
        // Clear the events queue
        events.clear();
        damage.clear();
        //events.update();
        //damage.update();

        let starting_pos = map.transition_to(
            new_map,
            &mut commands,
            &queries.p1(),
            &assets,
            &mut meshes,
            &rng,
        );

        // Adjust player position
        let mut player_pos = (0, 0);
        for (_player, mut ppos) in queries.p2().iter_mut() {
            player_pos = (starting_pos.x, starting_pos.y);
            ppos.x = starting_pos.x;
            ppos.y = starting_pos.y;
        }
        for (henry, mut henry_pos) in queries.p3().iter_mut() {
            henry_pos.x = player_pos.0 - 1;
            henry_pos.y = player_pos.1;
            commands.entity(henry).remove::<LerpMove>();
        }

        events.clear();
        damage.clear();
        events.update();
        damage.update();
    }
}
