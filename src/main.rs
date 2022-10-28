use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

mod demo;
mod maze;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "SuperSDG3: From dust to Rust".to_string(),
            //mode: bevy::window::WindowMode::Fullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        //.add_plugin(demo::Demo3DPlugin)
        .add_plugin(maze::MazePlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}
