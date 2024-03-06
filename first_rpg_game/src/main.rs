use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode, WindowPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "First RPG".into(),
                present_mode: PresentMode::AutoNoVsync,
                mode: WindowMode::Windowed,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .run()
}
