use crate::TimeStepResource;

use super::{tile_to_screen, TilePosition};
use bevy::prelude::*;

#[derive(Component)]
pub struct LerpMove {
    pub start: (i32, i32),
    pub end: (i32, i32),
    pub step: u32,
    pub jumping: bool,
    pub animate: Option<Vec<usize>>,
}

pub fn tile_location_added(
    mut query: Query<
        (&TilePosition, &mut Transform),
        Or<(Added<TilePosition>, Changed<TilePosition>)>,
    >,
) {
    query.for_each_mut(|(tile_pos, mut trans)| {
        let tts = tile_to_screen(tile_pos.x, tile_pos.y);
        trans.translation = Vec3::new(tts.0, tts.1, trans.translation.z);
    });
}

pub fn tile_lerp(
    mut query: Query<(
        Entity,
        &mut LerpMove,
        &mut TilePosition,
        &mut Transform,
        Option<&mut TextureAtlasSprite>,
    )>,
    mut commands: Commands,
    timer: Res<TimeStepResource>,
) {
    if !timer.timer.finished() {
        return;
    }
    for (entity, mut lerp, mut pos, mut trans, mut sprite) in query.iter_mut() {
        lerp.step += 1;

        let start = tile_to_screen(lerp.start.0, lerp.start.1);
        let end = tile_to_screen(lerp.end.0, lerp.end.1);
        let step = ((end.0 - start.0) / 8.0, (end.1 - start.1) / 8.0);

        trans.translation.x = start.0 + (step.0 * lerp.step as f32);
        trans.translation.y = start.1 + (step.1 * lerp.step as f32);

        if let Some(animate) = &lerp.animate {
            let frame = lerp.step % animate.len() as u32;
            if let Some(sprite) = &mut sprite {
                sprite.index = animate[frame as usize];
            }
        }

        if lerp.jumping {
            match lerp.step {
                1 => trans.translation.y += 8.0,
                2 => trans.translation.y += 16.0,
                3 => trans.translation.y += 24.0,
                4 => trans.translation.y += 32.0,
                5 => trans.translation.y += 24.0,
                6 => trans.translation.y += 16.0,
                7 => trans.translation.y += 8.0,
                _ => {}
            }
        }

        // Finish the move
        if lerp.step > 8 {
            if let Some(animate) = &lerp.animate {
                if let Some(sprite) = &mut sprite {
                    sprite.index = animate[0];
                }
            }
            pos.x = lerp.end.0;
            pos.y = lerp.end.1;
            let tts = tile_to_screen(pos.x, pos.y);
            trans.translation = Vec3::new(tts.0, tts.1, trans.translation.z);
            commands.entity(entity).remove::<LerpMove>();
        }
    }
}
