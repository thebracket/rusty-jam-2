use super::ScaresChickens;
use crate::{
    ai::AnimationSet,
    assets::GameAssets,
    combat::{Health, Hostile},
    fov::FieldOfView,
    maps::{tile_to_screen, MapElement, TilePosition},
    GameElement,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Wolf;

#[derive(Component)]
pub struct Tasty;

pub fn spawn_wolf(x: i32, y: i32, health: i32, assets: &GameAssets, commands: &mut Commands) {
    let pos = tile_to_screen(x, y);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: assets.doggies.clone(),
            transform: Transform::from_xyz(pos.0, pos.1, 2.0),
            sprite: TextureAtlasSprite::new(12),
            ..default()
        })
        .insert(TilePosition { x, y })
        .insert(Wolf)
        .insert(FieldOfView::new(8))
        .insert(MapElement)
        .insert(Health {
            current: health,
            max: health,
        })
        .insert(Hostile)
        .insert(ScaresChickens)
        .insert(AnimationSet {
            animations: vec![
                // Left
                vec![60, 61, 62],
                // Right
                vec![12, 13, 14],
                // Up
                vec![28, 29, 30],
                // Down
                vec![76, 77, 78],
            ],
        })
        .insert(GameElement);
}

pub fn spawn_mommy_wolf(x: i32, y: i32, health: i32, assets: &GameAssets, commands: &mut Commands) {
    let pos = tile_to_screen(x, y);

    let mut trans = Transform::from_xyz(pos.0, pos.1, 2.0);
    trans.scale += 0.8;

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: assets.doggies.clone(),
            transform: trans,
            sprite: TextureAtlasSprite::new(0),
            ..default()
        })
        .insert(TilePosition { x, y })
        .insert(Wolf)
        .insert(FieldOfView::new(8))
        .insert(MapElement)
        .insert(Health {
            current: health,
            max: health,
        })
        .insert(Hostile)
        .insert(ScaresChickens)
        .insert(AnimationSet {
            animations: vec![
                // Left
                vec![48, 49, 50],
                // Right
                vec![0, 1, 2],
                // Up
                vec![16, 17, 18],
                // Down
                vec![64, 65, 66],
            ],
        })
        .insert(GameElement);
}
