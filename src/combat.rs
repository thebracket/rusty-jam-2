use crate::{
    actors::{Henry, Player},
    ai::ActionRequest,
    assets::GameAssets,
    maps::{tile_to_screen, TilePosition},
    GameElement, GameState, TimeStepResource,
};
use bevy::{ecs::event::Events, prelude::*};

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component)]
pub struct PlayerHealthLabel;

#[derive(Component)]
pub struct Hostile;

pub struct DamageMessage {
    pub from: Option<Entity>,
    pub to: Entity,
}

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
        .insert(PlayerHealthLabel)
        .insert(GameElement);
}

pub fn update_health_hud(
    mut health_hud: Query<&mut Text, With<PlayerHealthLabel>>,
    player_health: Query<&Health, With<Player>>,
    henry_health: Query<&Health, With<Henry>>,
) {
    let henry = henry_health.get_single();
    let player = player_health.get_single();

    for mut txt in health_hud.iter_mut() {
        let mut new_text = "HEALTH:\n".to_string();
        if let Ok(player) = player {
            new_text += &format!("You: {}/{}", player.current, player.max);
        }
        if let Ok(henry) = henry {
            new_text += &format!("\nHenry: {}/{}", henry.current, henry.max);
        }
        txt.sections[0].value = new_text;
    }
}

#[derive(Component)]
pub struct LerpAttack {
    pub target: Entity,
    pub start: (i32, i32),
    pub end: (i32, i32),
    pub step: u32,
}

pub fn combat_lerp(
    mut query: Query<(Entity, &TilePosition, &mut LerpAttack, &mut Transform)>,
    mut commands: Commands,
    mut damage: EventWriter<DamageMessage>,
    timer: Res<TimeStepResource>,
) {
    if !timer.timer.finished() {
        return;
    }
    for (entity, pos, mut lerp, mut trans) in query.iter_mut() {
        lerp.step += 1;
        let start = tile_to_screen(lerp.start.0, lerp.start.1);
        let end = tile_to_screen(lerp.end.0, lerp.end.1);
        let step = ((end.0 - start.0) / 8.0, (end.1 - start.1) / 8.0);

        trans.translation.x = start.0 + (step.0 * lerp.step as f32);
        trans.translation.y = start.1 + (step.1 * lerp.step as f32);

        if lerp.step > 3 {
            damage.send(DamageMessage {
                from: Some(entity),
                to: lerp.target,
            });
            let tts = tile_to_screen(pos.x, pos.y);
            trans.translation = Vec3::new(tts.0, tts.1, trans.translation.z);
            commands.entity(entity).remove::<LerpAttack>();
        }
    }
}

#[derive(Component)]
pub struct Unconscious(pub u32);

pub fn damage_system(
    mut events: EventReader<DamageMessage>,
    mut commands: Commands,
    mut queries: ParamSet<(
        Query<(Entity, &mut Health, Option<&Henry>, Option<&Player>)>,
        Query<(Entity, &mut Transform, &mut Health)>,
    )>,
    mut state: ResMut<State<GameState>>,
    mut action_queue: ResMut<Events<ActionRequest>>,
) {
    let mut killers = Vec::new();
    for damage in events.iter() {
        for (e, mut health, henry, player) in queries.p0().iter_mut() {
            if e == damage.to {
                health.current -= 1;
                if health.current < 1 {
                    if let Some(from) = damage.from {
                        killers.push(from);
                    }
                    if henry.is_some() {
                        // Knock poor Henry out
                        commands.entity(e).insert(Unconscious(30));
                        health.current = health.max;
                    } else if player.is_some() {
                        // End the game
                        let _ = state.set(GameState::Dead);
                    } else {
                        action_queue.update();
                        commands.entity(e).despawn();
                    }
                }
            }
        }
    }

    if !killers.is_empty() {
        for (entity, mut trans, mut health) in queries.p1().iter_mut() {
            if killers.contains(&entity) {
                trans.scale += 0.05;
                health.max += 1;
            }
        }
    }
}
