use crate::{
    actors::Player,
    ai::{ActionRequest, Facing},
    combat::Hostile,
    console::Console,
    maps::{LerpMove, RegionMap, TilePosition},
    random::Rng,
};
use bevy::prelude::*;
use bracket_pathfinding::prelude::{DistanceAlg, Point};

#[derive(Component)]
pub struct Interaction {
    pub output: Vec<(String, Color)>,
}

pub fn player_interaction(
    player: Query<(Entity, &Player, &TilePosition), Without<LerpMove>>,
    interactions: Query<(&Interaction, &TilePosition), Without<LerpMove>>,
    hostiles: Query<(Entity, &TilePosition), With<Hostile>>,
    keyboard: Res<Input<KeyCode>>,
    map: Res<RegionMap>,
    console: Res<Console>,
    rng: Res<Rng>,
    mut actions: EventWriter<ActionRequest>,
) {
    for (entity, player, tile_pos) in player.iter() {
        if keyboard.just_pressed(KeyCode::Space) {
            let my_pt = Point::new(tile_pos.x, tile_pos.y);
            for (hostile, hpos) in hostiles.iter() {
                let their_pt = Point::new(hpos.x, hpos.y);
                let distance = DistanceAlg::Pythagoras.distance2d(my_pt, their_pt);
                if distance < 1.4 {
                    actions.send(ActionRequest {
                        entity,
                        action: crate::ai::Action::WantsToAttack {
                            from: (my_pt.x, my_pt.y),
                            to: (their_pt.x, their_pt.y),
                            target: hostile,
                        },
                        priority: 5,
                    });
                    return;
                }
            }

            let mut target = (tile_pos.x, tile_pos.y);
            match player.facing {
                Facing::Left => target.0 -= 1,
                Facing::Right => target.0 += 1,
                Facing::Up => target.1 -= 1,
                Facing::Down => target.1 += 1,
            }
            map.interact(target.0, target.1, &console);

            for (interact, ipos) in interactions.iter() {
                if ipos.x == target.0 && ipos.y == target.1 {
                    if let Some(i) = rng.random_slice_entry(&interact.output) {
                        console.write(&i.0, i.1);
                    }
                }
            }
        }
    }
}
