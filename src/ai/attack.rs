use super::ActionRequest;
use crate::{
    combat::{Dead, LerpAttack, Unconscious},
    maps::{LerpMove, TilePosition},
    TimeStepResource,
};
use bevy::prelude::*;
use bracket_pathfinding::prelude::{DistanceAlg, Point};

pub fn attacks<TYPE, TARGET>(
    me: Query<
        (Entity, &TilePosition),
        (
            With<TYPE>,
            (
                Without<Unconscious>,
                Without<LerpMove>,
                Without<LerpAttack>,
                Without<Dead>,
            ),
        ),
    >,
    them: Query<(Entity, &TilePosition), (With<TARGET>, Without<Unconscious>, Without<Dead>)>,
    mut actions: EventWriter<ActionRequest>,
    timer: Res<TimeStepResource>,
) where
    TYPE: Component,
    TARGET: Component,
{
    if !timer.timer.finished() {
        return;
    }
    for (entity, my_pos) in me.iter() {
        let my_point = Point::new(my_pos.x, my_pos.y);
        for (target, their_pos) in them.iter() {
            let their_point = Point::new(their_pos.x, their_pos.y);
            let distance = DistanceAlg::Pythagoras.distance2d(my_point, their_point);
            if distance < 1.5 {
                actions.send(ActionRequest {
                    entity,
                    action: super::Action::WantsToAttack {
                        from: (my_pos.x, my_pos.y),
                        to: (their_pos.x, their_pos.y),
                        target,
                    },
                    priority: 5,
                });
            }
        }
    }
}
