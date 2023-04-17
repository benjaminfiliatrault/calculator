mod setup;
mod button_system;
mod config;
mod token_manager;

use bevy::{
    prelude::*,
    utils::Duration,
    window::{PresentMode, WindowPlugin, WindowResolution},
    winit::WinitSettings
};

use config::{WINDOW_WIDTH, WINDOW_HEIGHT};
use setup::setup;
use button_system::button_system;
use token_manager::*;

fn main() {
    App::new()
    // Power-saving reactive rendering for applications.
    .insert_resource(WinitSettings::desktop_app())
    // You can also customize update behavior with the fields of [`WinitConfig`]
    .insert_resource(WinitSettings {
        focused_mode: bevy::winit::UpdateMode::Continuous,
        unfocused_mode: bevy::winit::UpdateMode::ReactiveLowPower {
            max_wait: Duration::from_millis(100),
        },
        ..default()
    })
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            // Turn off vsync to maximize CPU/GPU usage
            present_mode: PresentMode::AutoNoVsync,
            title: ("Calculator").to_string(),
            resizable: false,
            transparent: true,
            resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            ..default()
        }),
        ..default()
    }))
    .init_resource::<TokenManager>()
    .add_startup_system(setup)
    .add_system(button_system)
    .add_system(token_system)
    .run();
}