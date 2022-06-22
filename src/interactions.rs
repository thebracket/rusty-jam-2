use bevy::prelude::*;
use crate::{player::{Player, Facing}, maps::{TilePosition, LerpMove, RegionMap}, console::Console, random::Rng};

#[derive(Component)]
pub struct Interaction {
    pub output: Vec<(String, Color)>,
}

pub fn player_interaction(
    player: Query<
        (&Player, &TilePosition),
        Without<LerpMove>,
    >,
    interactions: Query<(&Interaction, &TilePosition), Without<LerpMove>>,
    keyboard: Res<Input<KeyCode>>,
    map: Res<RegionMap>,
    console: Res<Console>,
    rng: Res<Rng>,
) {
    for ( player, tile_pos) in player.iter() {
        if keyboard.just_pressed(KeyCode::Space) {
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