use bevy::prelude::*;

use super::Tasty;
use crate::{
    assets::GameAssets,
    combat::Health,
    fov::FieldOfView,
    interactions::Interaction,
    maps::{tile_to_screen, MapElement, TilePosition},
    GameElement,
};

#[derive(Component)]
pub struct Farmer(bool);

pub fn spawn_farmer(x: i32, y: i32, assets: &GameAssets, commands: &mut Commands) {
    let pos = tile_to_screen(x, y);

    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.tom.clone(),
            transform: Transform::from_xyz(pos.0, pos.1, 2.0),
            ..default()
        })
        .insert(TilePosition { x, y })
        .insert(Farmer(false))
        .insert(Interaction {
            output: vec![
                (
                    "The farmer yells 'Get away from me!'".to_string(),
                    Color::WHITE,
                ),
                (
                    "The farmer sobs 'I've unleashed a monster!'".to_string(),
                    Color::WHITE,
                ),
                (
                    "The farmer sighs 'I never should have bought Magic Miracle Grow!'".to_string(),
                    Color::WHITE,
                ),
            ],
        })
        .insert(FieldOfView::new(8))
        .insert(MapElement)
        .insert(Health { current: 3, max: 3 })
        .insert(Tasty)
        .insert(GameElement); // Don't persist chickens between levels
}
