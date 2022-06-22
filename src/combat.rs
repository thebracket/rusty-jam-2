use bevy::prelude::*;
use crate::{assets::GameAssets, actors::{Henry, Player}};

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component)]
pub struct PlayerHealthLabel;

pub fn setup_health_hud(commands: &mut Commands, assets: &GameAssets) {
    commands.spawn_bundle(TextBundle {
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
    }).insert(PlayerHealthLabel);
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
