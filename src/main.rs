use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode, WindowResolution, WindowTheme};

mod demo;
mod maze;
mod tools;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "SuperSDG3: From dust to Rust".to_string(),
                        present_mode: PresentMode::AutoVsync,
                        window_theme: Some(WindowTheme::Dark),
                        mode: WindowMode::Windowed,
                        position: WindowPosition::At(IVec2 { x: 0, y: 0 }),
                        resolution: WindowResolution::new(1280., 1460.),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin { ..default() }),
            maze::MazeGamePlugins,
            tools::ToolsPlugins,
            //demo::DemoLightPlugin,
            //LogDiagnosticsPlugin::default(),
            //FrameTimeDiagnosticsPlugin,
        ))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
