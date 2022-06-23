use crate::{
    ai::{Action, ActionRequest},
    assets::GameAssets,
    combat::Health,
    fov::FieldOfView,
    interactions::Interaction,
    maps::{tile_to_screen, LerpMove, MapElement, RegionMap, TilePosition},
    random::Rng,
};
use bevy::prelude::*;

use super::Tasty;

#[derive(Component)]
pub struct Chicken;

#[derive(Component)]
pub struct ScaresChickens;

pub fn spawn_chicken(x: i32, y: i32, assets: &GameAssets, commands: &mut Commands) {
    let pos = tile_to_screen(x, y);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: assets.chick.clone(),
            transform: Transform::from_xyz(pos.0, pos.1, 2.0),
            sprite: TextureAtlasSprite::new(2),
            ..default()
        })
        .insert(TilePosition { x, y })
        .insert(Chicken)
        .insert(Interaction {
            output: vec![(
                "The chicken clucks. It lacks the heart of a mega-chicken.".to_string(),
                Color::WHITE,
            )],
        })
        .insert(FieldOfView::new(3))
        .insert(MapElement)
        .insert(Health { current: 1, max: 1 })
        .insert(Tasty); // Don't persist chickens between levels
}

pub fn chicken_ai(
    map: Res<RegionMap>,
    mut ai_query: Query<
        (Entity, &TilePosition, &mut TextureAtlasSprite),
        (With<Chicken>, Without<LerpMove>),
    >,
    rng: Res<Rng>,
    mut actions: EventWriter<ActionRequest>,
) {
    let mut delta = None;
    for (entity, pos, mut sprite) in ai_query.iter_mut() {
        // Do chicken things
        if delta.is_none() {
            match rng.range(0, 100) {
                1 => sprite.index = 0,
                2 => sprite.index = 1,
                3 => sprite.index = 2,
                4 => sprite.index = 26,
                6 => {
                    if map.can_player_enter(pos.x - 1, pos.y) {
                        delta = Some((-1, 0))
                    }
                }
                7 => {
                    if map.can_player_enter(pos.x + 1, pos.y) {
                        delta = Some((1, 0))
                    }
                }
                8 => {
                    if map.can_player_enter(pos.x, pos.y - 1) {
                        delta = Some((0, -1))
                    }
                }
                9 => {
                    if map.can_player_enter(pos.x, pos.y + 1) {
                        delta = Some((0, 1))
                    }
                }
                _ => {}
            }
        }

        if let Some(delta) = delta {
            if map.can_player_enter(pos.x + delta.0, pos.y + delta.1) {
                actions.send(ActionRequest {
                    entity,
                    action: Action::Move {
                        from: (pos.x, pos.y),
                        to: (pos.x + delta.0, pos.y + delta.1),
                        jumping: false,
                    },
                    priority: 1,
                });
            }
        }
    }
}
