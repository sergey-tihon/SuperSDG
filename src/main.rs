use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode, WindowResolution, WindowTheme};

mod maze;
mod tools;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "SuperSDG 3".to_string(),
                        present_mode: PresentMode::AutoVsync,
                        window_theme: Some(WindowTheme::Dark),
                        mode: WindowMode::Windowed,
                        position: WindowPosition::At(IVec2 { x: 0, y: 0 }),
                        resolution: WindowResolution::new(1280, 1460),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin { ..default() }),
            maze::MazeGamePlugins,
            tools::ToolsPlugins,
        ))
        .add_systems(Update, close_on_esc)
        .run();
}

// TODO: Remove after menu integration
pub fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}
