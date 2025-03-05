use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy::winit::WinitSettings;
use simulated_life::GamePlugin; 

#[bevy_main]
fn main() {
    App::new()
        .insert_resource(WinitSettings::mobile())
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                    ..default()
                }),
                ..default()
            }),
            GamePlugin,
        ))
        .run();
}
