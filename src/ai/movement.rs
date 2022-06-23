use super::{AnimationSet, Facing};
use crate::{combat::LerpAttack, maps::LerpMove};
use bevy::{prelude::*, utils::HashMap};

#[derive(Clone, Copy)]
pub enum Action {
    Move {
        from: (i32, i32),
        to: (i32, i32),
        jumping: bool,
    },
    WantsToAttack {
        from: (i32, i32),
        to: (i32, i32),
        target: Entity,
    },
}

#[derive(Clone, Copy)]
pub struct ActionRequest {
    pub entity: Entity,
    pub action: Action,
    pub priority: i32,
}

pub fn process_actions(
    mut actions: EventReader<ActionRequest>,
    mut commands: Commands,
    animation_query: Query<(Entity, &AnimationSet)>,
) {
    let mut final_action: HashMap<Entity, ActionRequest> = HashMap::new();
    for action in actions.iter() {
        if let Some(current) = final_action.get_mut(&action.entity) {
            if current.priority < action.priority {
                current.action = action.action;
            }
        } else {
            final_action.insert(action.entity, *action);
        }
    }

    // Process the resultant query list
    for (entity, action) in final_action.iter() {
        match action.action {
            Action::Move { from, to, jumping } => {
                let direction = if from.0 < to.0 {
                    Facing::Right
                } else if from.0 > to.0 {
                    Facing::Left
                } else if from.1 < to.1 {
                    Facing::Down
                } else if from.1 > to.1 {
                    Facing::Up
                } else {
                    Facing::Left
                };

                commands.entity(*entity).insert(LerpMove {
                    start: from,
                    end: to,
                    step: 0,
                    jumping,
                    animate: find_animation(&animation_query, *entity, &direction),
                });
            }
            Action::WantsToAttack { from, to, target } => {
                commands.entity(action.entity).insert(LerpAttack {
                    target,
                    start: from,
                    end: to,
                    step: 0,
                });
            }
        }
    }
}

fn find_animation(
    query: &Query<(Entity, &AnimationSet)>,
    target: Entity,
    direction: &Facing,
) -> Option<Vec<usize>> {
    for (entity, animations) in query.iter() {
        if entity == target {
            return Some(animations.animations[direction.index()].clone());
        }
    }
    None
}
