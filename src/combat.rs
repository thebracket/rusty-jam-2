use crate::{
    actors::{Henry, Player},
    assets::GameAssets,
    maps::{tile_to_screen, TilePosition},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component)]
pub struct PlayerHealthLabel;

#[derive(Component)]
pub struct Hostile;

pub struct AttackMessage(pub Entity, pub Entity);

pub struct DamageMessage(pub Entity);

pub fn setup_health_hud(commands: &mut Commands, assets: &GameAssets) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(10.0),
                    right: Val::Px(0.0),
                    ..default()
                },
                ..default()
            },
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "Health",
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Right,
                    ..default()
                },
            ),
            ..default()
        })
        .insert(PlayerHealthLabel);
}

pub fn update_health_hud(
    mut health_hud: Query<&mut Text, With<PlayerHealthLabel>>,
    player_health: Query<&Health, With<Player>>,
    //henry_health: Query<&Health, With<Henry>>,
) {
    //let henry = henry_health.get_single();
    let player = player_health.get_single();

    for mut txt in health_hud.iter_mut() {
        let mut new_text = "HEALTH:\n".to_string();
        if let Ok(player) = player {
            new_text += &format!("You: {}/{}", player.current, player.max);
        }
        txt.sections[0].value = new_text;
    }
}

pub fn combat_system(
    mut events: EventReader<AttackMessage>,
    mut commands: Commands,
    pos_query: Query<(Entity, &TilePosition)>,
) {
    for attack in events.iter() {
        // Remove any Lerping for consistency
        /*commands.entity(attack.0).remove::<LerpMove>();
        commands.entity(attack.0).remove::<LerpAttack>();
        commands.entity(attack.1).remove::<LerpMove>();*/

        // Find positions
        let mut apos = (0, 0);
        let mut tpos = (0, 0);
        for (te, pos) in pos_query.iter() {
            if te == attack.0 {
                apos = (pos.x, pos.y);
            } else if te == attack.1 {
                tpos = (pos.x, pos.y);
            }
        }

        // Insert the attack lerp
        commands.entity(attack.0).insert(LerpAttack {
            target: attack.1,
            start: (apos.0, apos.1),
            end: (tpos.0, tpos.1),
            step: 0,
        });
    }
}

#[derive(Component)]
pub struct LerpAttack {
    target: Entity,
    start: (i32, i32),
    end: (i32, i32),
    step: u32,
}

pub fn combat_lerp(
    mut query: Query<(Entity, &TilePosition, &mut LerpAttack, &mut Transform)>,
    mut commands: Commands,
    mut damage: EventWriter<DamageMessage>,
) {
    for (entity, pos, mut lerp, mut trans) in query.iter_mut() {
        lerp.step += 1;
        let start = tile_to_screen(lerp.start.0, lerp.start.1);
        let end = tile_to_screen(lerp.end.0, lerp.end.1);
        let step = ((end.0 - start.0) / 8.0, (end.1 - start.1) / 8.0);

        trans.translation.x = start.0 + (step.0 * lerp.step as f32);
        trans.translation.y = start.1 + (step.1 * lerp.step as f32);

        if lerp.step > 3 {
            damage.send(DamageMessage(lerp.target));
            let tts = tile_to_screen(pos.x, pos.y);
            trans.translation = Vec3::new(tts.0, tts.1, trans.translation.z);
            commands.entity(entity).remove::<LerpAttack>();
        }
    }
}

#[derive(Component)]
pub struct Unconscious;

pub fn damage_system(
    mut events: EventReader<DamageMessage>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Health, Option<&Henry>, Option<&Player>)>,
) {
    for damage in events.iter() {
        for (e, mut health, henry, player) in query.iter_mut() {
            if e == damage.0 {
                health.current -= 1;
                if health.current < 1 {
                    if henry.is_some() {
                        // Knock poor Henry out
                        //commands.entity(e).insert(Unconscious);
                        health.current = health.max;
                    } else if player.is_some() {
                        // End the game
                    } else {
                        commands.entity(e).despawn();
                    }
                }
            }
        }
    }
}
