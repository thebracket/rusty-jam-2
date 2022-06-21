use crate::assets::GameAssets;
use bevy::prelude::*;
use std::sync::Mutex;

pub struct Console {
    text: Mutex<Vec<(String, Color)>>,
    dirty: Mutex<bool>,
}

const NUM_LINES: usize = 6;

impl Console {
    pub fn new() -> Self {
        let mut text = vec![(String::new(), Color::WHITE); NUM_LINES];
        text[0] = ("Welcome to Happy Chicken".to_string(), Color::YELLOW);
        text[1] = (
            "Use cursor keys to move, J to jump, SPACE to interact with the object you are facing."
                .to_string(),
            Color::CYAN,
        );
        Self {
            text: Mutex::new(text),
            dirty: Mutex::new(true),
        }
    }

    pub fn write<S: ToString>(&self, text: S, color: Color) {
        let mut text_lock = self.text.lock().unwrap();
        for i in (1..NUM_LINES).rev() {
            text_lock[i] = text_lock[i - 1].clone();
        }
        text_lock[0] = (text.to_string(), color);
        let mut dirty_lock = self.dirty.lock().unwrap();
        *dirty_lock = true;
    }

    fn is_dirty(&self) -> bool {
        let dirty_lock = self.dirty.lock().unwrap();
        *dirty_lock
    }

    fn clean(&self) {
        let mut dirty_lock = self.dirty.lock().unwrap();
        *dirty_lock = false;
    }
}

#[derive(Component)]
pub struct ConsoleLine(usize);

pub fn console_setup(assets: &GameAssets, commands: &mut Commands, console: &Console) {
    const FONT_SIZE: f32 = 18.0;
    let text_lock = console.text.lock().unwrap();
    for (i, (line, color)) in text_lock.iter().enumerate() {
        commands
            .spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Px(4.0),
                        bottom: Val::Px(110.0 - (i as f32 * (FONT_SIZE + 1.0))),
                        ..default()
                    },
                    ..default()
                },
                text: Text::with_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    line.clone(),
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: FONT_SIZE,
                        color: *color,
                    },
                    // Note: You can use `Default::default()` in place of the `TextAlignment`
                    TextAlignment {
                        horizontal: HorizontalAlign::Left,
                        ..default()
                    },
                ),
                ..default()
            })
            .insert(ConsoleLine(i));
    }
}

pub fn update_consoles(mut query: Query<(&ConsoleLine, &mut Text)>, console: Res<Console>) {
    if console.is_dirty() {
        let line_lock = console.text.lock().unwrap();
        for (line, mut text) in query.iter_mut() {
            text.sections[0].value = line_lock[line.0].0.clone();
            text.sections[0].style.color = line_lock[line.0].1;
        }
        console.clean();
    }
}
