use crate::{
    assets::GameAssets,
    combat::DamageMessage,
    maps::{tile_to_screen, MapElement, TilePosition},
    GameElement,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Spike(Timer);

pub fn spawn_spikes(x: i32, y: i32, extended: bool, assets: &GameAssets, commands: &mut Commands) {
    let pos = tile_to_screen(x, y);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: assets.spikes.clone(),
            transform: Transform::from_xyz(pos.0, pos.1, 2.0),
            sprite: TextureAtlasSprite::new(if extended { 0 } else { 1 }),
            ..default()
        })
        .insert(TilePosition { x, y })
        .insert(Spike(Timer::from_seconds(1.7, true)))
        .insert(MapElement)
        .insert(GameElement);
}

pub fn spike_system(
    mut query: Query<(&mut Spike, &mut TextureAtlasSprite, &TilePosition)>,
    ouch: Query<(Entity, &TilePosition)>,
    time: Res<Time>,
    mut damage: EventWriter<DamageMessage>,
) {
    for (mut spike, mut sprite, spike_pos) in query.iter_mut() {
        spike.0.tick(time.delta());
        if spike.0.finished() {
            if sprite.index == 0 {
                sprite.index = 1;
            } else {
                sprite.index = 0;
            }
        }

        if sprite.index == 1 {
            for (victim, pos) in ouch.iter() {
                if pos.x == spike_pos.x && pos.y == spike_pos.y {
                    damage.send(DamageMessage {
                        from: None,
                        to: victim,
                    })
                }
            }
        }
    }
}
