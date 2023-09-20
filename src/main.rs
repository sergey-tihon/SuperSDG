use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode, WindowResolution, WindowTheme};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use demo::DemoLightPlugin;
use maze::MazePlugin;

mod demo;
mod maze;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "SuperSDG3: From dust to Rust".to_string(),
                    present_mode: PresentMode::AutoVsync,
                    window_theme: Some(WindowTheme::Dark),
                    mode: WindowMode::Windowed,
                    position: WindowPosition::At(IVec2 { x: 0, y: 0 }),
                    resolution: WindowResolution::new(1024., 1460.),
                    ..default()
                }),
                ..default()
            }),
            //DemoLightPlugin,
            MazePlugin,
            WorldInspectorPlugin::new(),
            //LogDiagnosticsPlugin::default(),
            //FrameTimeDiagnosticsPlugin,
        ))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
