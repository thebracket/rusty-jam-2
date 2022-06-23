use crate::{
    combat::{LerpAttack, Unconscious},
    fov::FieldOfView,
    maps::{tile_index, LerpMove, RegionMap, TilePosition, NUM_TILES_X, NUM_TILES_Y},
    TimeStepResource,
};
use bevy::prelude::*;
use bracket_pathfinding::prelude::{DijkstraMap, Point};

use super::{Action, ActionRequest};

pub fn chase_after<TYPE, TARGET>(
    ai_query: Query<
        (Entity, &TilePosition, &FieldOfView),
        (
            With<TYPE>,
            Without<LerpMove>,
            Without<Unconscious>,
            Without<LerpAttack>,
        ),
    >,
    target_query: Query<&TilePosition, (With<TARGET>, Without<Unconscious>)>,
    map: Res<RegionMap>,
    mut actions: EventWriter<ActionRequest>,
    timer: Res<TimeStepResource>,
) where
    TARGET: Component,
    TYPE: Component,
{
    if !timer.timer.finished() {
        return;
    }
    for (entity, pos, fov) in ai_query.iter() {
        if !fov.fov_set.is_empty() {
            let mut starts = Vec::new();
            for epos in target_query.iter() {
                let pt = Point::new(epos.x, epos.y);
                if fov.fov_set.contains(&pt) {
                    starts.push(tile_index(pt.x, pt.y));
                }
                if !starts.is_empty() {
                    let scary_map = DijkstraMap::new(NUM_TILES_X, NUM_TILES_Y, &starts, &*map, 9.0);
                    if let Some(exit) =
                        DijkstraMap::find_lowest_exit(&scary_map, tile_index(pos.x, pos.y), &*map)
                    {
                        let x = (exit % NUM_TILES_X) as i32;
                        let y = (exit / NUM_TILES_X) as i32;
                        actions.send(ActionRequest {
                            entity,
                            priority: 2,
                            action: Action::Move {
                                to: (x, y),
                                from: (pos.x, pos.y),
                                jumping: false,
                            },
                        });
                    }
                }
            }
        }
    }
}
