use std::cmp;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::Viewport,
    window::WindowResized,
};

use super::level::MazeLevel;

pub struct MazeCameraPlugin;

impl Plugin for MazeCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, set_camera_viewports);
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct MiniMapCamera;

fn setup(level: Res<MazeLevel>, mut commands: Commands) {
    let player_x = level.start.0 as f32 + 0.5;
    let player_z = level.start.1 as f32 + 0.5;

    // Main camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(player_x, 15.0, player_z + 20.0)
                .looking_at(Vec3::new(player_x, 0.5, player_z), Vec3::Y),
            ..default()
        },
        MainCamera,
    ));

    let mid_x = level.width as f32 / 2.0;
    let mid_z = level.height as f32 / 2.0;

    // MiniMap camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(mid_x, 2.5 * mid_x, mid_z)
                .looking_at(Vec3::new(mid_x, 0.0, mid_z), -Vec3::Z),
            camera: Camera {
                // Renders the right camera after the left camera, which has a default priority of 0
                order: 1,
                ..default()
            },
            camera_3d: Camera3d {
                // don't clear on the second camera because the first camera already cleared the window
                clear_color: ClearColorConfig::None,
                ..default()
            },
            ..default()
        },
        MiniMapCamera,
    ));
}

fn set_camera_viewports(
    windows: Query<&Window>,
    mut resize_events: EventReader<WindowResized>,
    mut main_camera: Query<&mut Camera, (With<MainCamera>, Without<MiniMapCamera>)>,
    mut mini_camera: Query<&mut Camera, With<MiniMapCamera>>,
) {
    // We need to dynamically resize the camera's viewports whenever the window size changes
    // A resize_event is sent when the window is first created, allowing us to reuse this system for initial setup.
    for resize_event in resize_events.iter() {
        let window = windows.get(resize_event.window).unwrap();
        let mut main_camera = main_camera.single_mut();
        main_camera.viewport = Some(Viewport {
            physical_position: UVec2::new(0, 0),
            physical_size: UVec2::new(
                window.resolution.physical_width(),
                window.resolution.physical_height(),
            ),
            ..default()
        });

        let mut mini_camera = mini_camera.single_mut();
        let mini_camera_size = cmp::min(
            window.resolution.physical_width() / 4,
            window.resolution.physical_height() / 3,
        );

        mini_camera.viewport = Some(Viewport {
            physical_position: UVec2::new(window.resolution.physical_width() - mini_camera_size, 0),
            physical_size: UVec2::new(mini_camera_size, mini_camera_size),

            ..default()
        });
    }
}
