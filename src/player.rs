use crate::{
    assets::GameAssets,
    region_map::RegionMap,
    tilemap::{tile_to_screen, LerpMove, TilePosition, NUM_TILES_X, NUM_TILES_Y}, console::Console,
};
use bevy::prelude::*;

pub enum Facing {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component)]
pub struct Player {
    facing: Facing,
}

pub fn spawn_player(commands: &mut Commands, assets: &GameAssets, start: (i32, i32)) {
    let pos = tile_to_screen(start.0, start.1);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: assets.player_chicken.clone(),
            transform: Transform::from_xyz(pos.0, pos.1, 2.0),
            ..default()
        })
        .insert(Player {
            facing: Facing::Down,
        })
        .insert(TilePosition {
            x: start.0,
            y: start.1,
        });
}

pub fn player_movement(
    mut player: Query<
        (Entity, &mut Player, &TilePosition, &mut TextureAtlasSprite),
        Without<LerpMove>,
    >,
    keyboard: Res<Input<KeyCode>>,
    map: Res<RegionMap>,
    mut commands: Commands,
    console: Res<Console>,
) {
    for (entity, mut player, tile_pos, mut sprite) in player.iter_mut() {
        let mut jumping = false;
        let delta: (i32, i32) = if keyboard.pressed(KeyCode::Left) {
            player.facing = Facing::Left;
            (-1, 0)
        } else if keyboard.pressed(KeyCode::Right) {
            player.facing = Facing::Right;
            (1, 0)
        } else if keyboard.pressed(KeyCode::Up) {
            player.facing = Facing::Up;
            (0, -1)
        } else if keyboard.pressed(KeyCode::Down) {
            player.facing = Facing::Down;
            (0, 1)
        } else if keyboard.just_pressed(KeyCode::J) {
            jumping = true;
            match player.facing {
                Facing::Left => (-2, 0),
                Facing::Right => (2, 0),
                Facing::Up => (0, -2),
                Facing::Down => (0, 2),
            }
        } else {
            (0, 0)
        };

        if keyboard.just_pressed(KeyCode::Space) {
            let mut target = (tile_pos.x, tile_pos.y);
            match player.facing {
                Facing::Left => target.0 -= 1,
                Facing::Right => target.0 -= 1,
                Facing::Up => target.1 -= 1,
                Facing::Down => target.1 += 1,
            }
            map.interact(target.0, target.1, &console);
        }

        if delta != (0, 0) {
            let destination = (
                (tile_pos.x + delta.0).clamp(0, NUM_TILES_X as i32 - 1),
                (tile_pos.y + delta.1).clamp(0, NUM_TILES_Y as i32 - 1),
            );
            if map.can_player_enter(destination.0, destination.1) {
                commands.entity(entity).insert(LerpMove {
                    start: (tile_pos.x, tile_pos.y),
                    end: destination,
                    step: 0,
                    jumping,
                });
            }
        }

        sprite.index = match player.facing {
            Facing::Left => 12,
            Facing::Right => 0,
            Facing::Up => 6,
            Facing::Down => 18,
        };
    }
}
