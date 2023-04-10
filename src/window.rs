//! This example illustrates how to run a winit window in a reactive, low power mode.
//!
//! This is useful for making desktop applications, or any other program that doesn't need to be
//! running the event loop non-stop.

pub use bevy::{
    prelude::*,
    utils::Duration,
    window::{PresentMode, RequestRedraw, WindowPlugin},
    winit::WinitSettings,
};

pub fn calc_windows() {
    App::new()
        // Power-saving reactive rendering for applications.
        .insert_resource(WinitSettings::desktop_app())
        // You can also customize update behavior with the fields of [`WinitConfig`]
        .insert_resource(WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Continuous,
            unfocused_mode: bevy::winit::UpdateMode::ReactiveLowPower {
                max_wait: Duration::from_millis(10),
            },
            ..default()
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // Turn off vsync to maximize CPU/GPU usage
                present_mode: PresentMode::AutoNoVsync,
                title: ("My Calculator").to_string(),
                ..default()
            }),
            ..default()
        }))
        .run();
}

