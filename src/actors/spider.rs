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
pub struct Spider;

pub fn spawn_spider(x: i32, y: i32, assets: &GameAssets, commands: &mut Commands) {
    let pos = tile_to_screen(x, y);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: assets.spiders.clone(),
            transform: Transform::from_xyz(pos.0, pos.1, 2.0),
            sprite: TextureAtlasSprite::new(12),
            ..default()
        })
        .insert(TilePosition { x, y })
        .insert(Spider)
        .insert(FieldOfView::new(8))
        .insert(MapElement)
        .insert(Health {
            current: 3,
            max: 3,
        })
        .insert(Hostile)
        .insert(ScaresChickens)
        .insert(AnimationSet {
            animations: vec![
                // Left
                vec![10, 11, 12, 13, 14],
                // Right
                vec![30, 31, 32, 33, 34],
                // Up
                vec![0, 1, 2, 3, 4],
                // Down
                vec![20, 21, 22, 23],
            ],
        })
        .insert(GameElement);
}
