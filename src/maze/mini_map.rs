use std::cmp;

use bevy::{
    camera::Viewport,
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

use super::level::{MazeLevel, MazeRegenerated};
use crate::core::AppState;

pub struct MiniMapPlugin;

impl Plugin for MiniMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, set_camera_viewports)
            .add_systems(OnEnter(AppState::InGame), show_mini_map)
            .add_systems(OnExit(AppState::InGame), hide_mini_map)
            .add_observer(update_camera_on_maze_regenerated);
    }
}

#[derive(Component, Default, Clone)]
struct MiniMapCamera;

fn setup(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        #MiniMapCamera
        MiniMapCamera
        Camera3d
        Camera { order: 1, clear_color: ClearColorConfig::None, is_active: false }
    });
}

fn calc_camera_position(level: &MazeLevel) -> (f32, f32, f32) {
    let mid_x = level.width as f32 / 2.0;
    let mid_z = level.height as f32 / 2.0;
    let height = level.width.max(level.height) as f32 * 1.5;
    (mid_x, mid_z, height)
}

fn show_mini_map(
    mut camera: Single<&mut Camera, With<MiniMapCamera>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    camera.is_active = true;
    update_mini_map_viewport(&mut camera, &window);
}

fn update_mini_map_viewport(camera: &mut Camera, window: &Window) {
    let window_size = window.resolution.physical_size();
    let mini_size = cmp::min(window_size.x / 4, window_size.y / 3);
    let margin = 4u32;

    camera.viewport = Some(Viewport {
        physical_position: UVec2::new(window_size.x - mini_size - margin, margin),
        physical_size: UVec2::new(mini_size, mini_size),
        ..default()
    });
}

fn hide_mini_map(mut camera: Single<&mut Camera, With<MiniMapCamera>>) {
    camera.is_active = false;
}

fn set_camera_viewports(
    windows: Query<&Window>,
    mut resize_events: MessageReader<WindowResized>,
    mut camera: Single<&mut Camera, With<MiniMapCamera>>,
) {
    for resize_event in resize_events.read() {
        if let Ok(window) = windows.get(resize_event.window)
            && camera.is_active
        {
            update_mini_map_viewport(&mut camera, window);
        }
    }
}

fn update_camera_on_maze_regenerated(
    _: On<MazeRegenerated>,
    level: Res<MazeLevel>,
    mut transform: Single<&mut Transform, With<MiniMapCamera>>,
) {
    let (mid_x, mid_z, height) = calc_camera_position(&level);
    **transform = Transform::from_xyz(mid_x, height, mid_z)
        .looking_at(Vec3::new(mid_x, 0.0, mid_z), Vec3::NEG_Z);
}
