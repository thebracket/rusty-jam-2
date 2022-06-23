use super::Tasty;
use crate::{
    ai::{Action, ActionRequest, AnimationSet, Facing},
    assets::GameAssets,
    combat::Health,
    maps::RegionMap,
    maps::{
        tile_index, tile_to_screen, LerpMove, TilePosition, TileType, NUM_TILES_X, NUM_TILES_Y,
    },
    GameElement, GameState,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub facing: Facing,
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
            facing: Facing::Left,
        })
        .insert(TilePosition {
            x: start.0,
            y: start.1,
        })
        .insert(Health {
            current: 10,
            max: 10,
        })
        .insert(Tasty)
        .insert(AnimationSet {
            animations: vec![
                // Left
                vec![12, 13, 14, 15, 16, 17],
                // Right
                vec![0, 1, 2, 3, 4, 5],
                // Up
                vec![6, 7, 8, 9, 10, 11],
                // Down
                vec![18, 19, 20, 21, 22, 23],
            ],
        })
        .insert(GameElement);
}

pub fn player_movement(
    mut player: Query<
        (Entity, &mut Player, &TilePosition, &mut TextureAtlasSprite),
        Without<LerpMove>,
    >,
    keyboard: Res<Input<KeyCode>>,
    map: Res<RegionMap>,
    mut actions: EventWriter<ActionRequest>,
    mut state: ResMut<State<GameState>>,
) {
    for (entity, mut player, tile_pos, mut sprite) in player.iter_mut() {
        let mut jumping = false;
        let delta: (i32, i32) = if keyboard.pressed(KeyCode::Left) || keyboard.pressed(KeyCode::A) {
            player.facing = Facing::Left;
            (-1, 0)
        } else if keyboard.pressed(KeyCode::Right) || keyboard.pressed(KeyCode::D) {
            player.facing = Facing::Right;
            (1, 0)
        } else if keyboard.pressed(KeyCode::Up) || keyboard.pressed(KeyCode::W) {
            player.facing = Facing::Up;
            (0, -1)
        } else if keyboard.pressed(KeyCode::Down) || keyboard.pressed(KeyCode::S) {
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

        sprite.index = match player.facing {
            Facing::Left => 12,
            Facing::Right => 0,
            Facing::Up => 6,
            Facing::Down => 18,
        };

        if delta != (0, 0) {
            let destination = (
                (tile_pos.x + delta.0).clamp(0, NUM_TILES_X as i32 - 1),
                (tile_pos.y + delta.1).clamp(0, NUM_TILES_Y as i32 - 1),
            );
            if map.can_player_enter(destination.0, destination.1) {
                if map.features[tile_index(destination.0, destination.1)] == TileType::GoldEgg {
                    let _ = state.set(GameState::Won);
                } else {
                    actions.send(ActionRequest {
                        entity,
                        action: Action::Move {
                            from: (tile_pos.x, tile_pos.y),
                            to: destination,
                            jumping,
                        },
                        priority: 1,
                    });
                }
            }
        }
    }
}
