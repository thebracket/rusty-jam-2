use super::ScaresChickens;
use crate::{
    assets::GameAssets,
    combat::{AttackMessage, Health, Hostile, LerpAttack, Unconscious},
    fov::FieldOfView,
    maps::{
        tile_index, tile_to_screen, LerpMove, MapElement, RegionMap, TilePosition, NUM_TILES_X,
        NUM_TILES_Y,
    },
};
use bevy::prelude::*;
use bracket_pathfinding::prelude::{DijkstraMap, DistanceAlg, Point};

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
        .insert(ScaresChickens);
}

pub fn wolf_ai(
    map: Res<RegionMap>,
    mut ai_query: Query<
        (Entity, &TilePosition, &FieldOfView, &mut TextureAtlasSprite),
        (With<Wolf>, Without<LerpMove>, Without<LerpAttack>),
    >,
    tasty_query: Query<(Entity, &TilePosition), (With<Tasty>, Without<Unconscious>)>,
    mut commands: Commands,
    mut attack: EventWriter<AttackMessage>,
) {
    let mut delta = None;
    for (entity, pos, fov, mut _sprite) in ai_query.iter_mut() {
        // Look for things to eat
        tasty_query.iter().for_each(|(e, tpos)| {
            if DistanceAlg::Pythagoras
                .distance2d(Point::new(pos.x, pos.y), Point::new(tpos.x, tpos.y))
                < 1.2
            {
                attack.send(AttackMessage(entity, e));
                return;
            }
        });

        // Check for things to eat
        if !fov.fov_set.is_empty() {
            let mut starts = Vec::new();
            for (_, epos) in tasty_query.iter() {
                let pt = Point::new(epos.x, epos.y);
                if fov.fov_set.contains(&pt) {
                    starts.push(tile_index(pt.x, pt.y));
                }
                if !starts.is_empty() {
                    let scary_map = DijkstraMap::new(NUM_TILES_X, NUM_TILES_Y, &starts, &*map, 9.0);
                    if let Some(exit) =
                        DijkstraMap::find_lowest_exit(&scary_map, tile_index(pos.x, pos.y), &*map)
                    {
                        let x = (exit % NUM_TILES_X) as i32;
                        let y = (exit / NUM_TILES_X) as i32;
                        delta = Some((x - pos.x, y - pos.y));
                    }
                }
            }
        }

        if let Some(delta) = delta {
            if map.can_player_enter(pos.x + delta.0, pos.y + delta.1) {
                commands.entity(entity).insert(LerpMove {
                    jumping: false,
                    start: (pos.x, pos.y),
                    end: (pos.x + delta.0, pos.y + delta.1),
                    step: 0,
                    animate: None,
                });
            }
        }
    }
}
