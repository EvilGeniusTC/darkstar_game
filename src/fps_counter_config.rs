use bevy::prelude::*;
use std::time::Duration;

pub const FONT_SIZE: f32 = 32.;
pub const FONT_COLOR: Color = Color::WHITE;
pub const UPDATE_INTERVAL: Duration = Duration::from_secs(1);

pub const STRING_FORMAT: &str = "FPS: ";
pub const STRING_INITIAL: &str = "FPS: ...";
pub const STRING_MISSING: &str = "FPS: ???";