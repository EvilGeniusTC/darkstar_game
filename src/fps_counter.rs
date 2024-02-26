use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},prelude::*,};
use std::fmt::Write;
use crate::fps_counter_config;


/// FPS counter plugin
pub struct FpsCounterPlugin;

impl Plugin for FpsCounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Startup, spawn_text)
            .add_systems(Update, update)
            .init_resource::<FpsCounter>();
    }
}

#[derive(Resource)]
pub struct FpsCounter {
    pub timer: Timer,
    pub update_now: bool,
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self {
            timer: Timer::new(fps_counter_config::UPDATE_INTERVAL, TimerMode::Repeating),
            update_now: true,
        }
    }
}

impl FpsCounter {
    /// Enable FPS counter
    pub fn enable(&mut self) {
        self.timer.unpause();
        self.update_now = true;
    }

    /// Disable FPS counter
    pub fn disable(&mut self) {
        self.timer.pause();
        self.update_now = true;
    }

    /// Check if FPS counter is enabled
    pub fn is_enabled(&self) -> bool {
        !self.timer.paused()
    }
}

/// The marker on the text to be updated
#[derive(Component)]
pub struct FpsCounterText;

fn update(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    state_resources: Option<ResMut<FpsCounter>>,
    mut text_query: Query<&mut Text, With<FpsCounterText>>,
) {
    let Some(mut state) = state_resources else {
        return;
    };
    if !(state.update_now || state.timer.tick(time.delta()).just_finished()) {
        return;
    }
    if state.timer.paused() {
        for mut text in text_query.iter_mut() {
            let value: &mut String = &mut text.sections[0].value;
            value.clear();
        }
    } else {
        let fps_dialog: Option<f64> = extract_fps(&diagnostics);

        for mut text in text_query.iter_mut() {
            let value: &mut String = &mut text.sections[0].value;
            value.clear();

            if let Some(fps) = fps_dialog {
                write!(value, "{}{:.0}", fps_counter_config::STRING_FORMAT, fps).expect("Failed to write");
            } else {
                value.clear();
                write!(value, "{}", fps_counter_config::STRING_MISSING).expect("Failed to write");
            }
        }
    }
}

fn extract_fps(diagnostics: &Res<DiagnosticsStore>) -> Option<f64> {
    diagnostics
        .get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.average())
}

fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("FiraSansBold.ttf");

    commands
        .spawn(TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: fps_counter_config::STRING_INITIAL.to_string(),
                    style: TextStyle {
                        font,
                        font_size: fps_counter_config::FONT_SIZE,
                        color: fps_counter_config::FONT_COLOR,
                    },
                }],
                ..default()
            },
            ..default()
        })
        .insert(FpsCounterText);
}