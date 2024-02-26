use bevy::prelude::*;

mod fps_counter;
mod fps_counter_config;

use crate::fps_counter::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FpsCounterPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

