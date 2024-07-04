use std::f32::consts::{FRAC_PI_2, PI};

use bevy::{prelude::*, render::camera::Viewport, window::WindowResized};

use super::player::PlayerAnimation;

pub struct MazeCameraPlugin;

impl Plugin for MazeCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraSettings {
            height: 15.0,
            radius: 20.0,
            angle: FRAC_PI_2,
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                set_camera_viewports,
                keyboard_input_system,
                update_camera_position,
            ),
        );
    }
}

#[derive(Resource)]
pub struct CameraSettings {
    pub height: f32,
    pub radius: f32,
    pub angle: f32,
}

#[derive(Component)]
pub struct MainCamera;

// Setup camera objects but without any exact position
// Proper position will be calculated by `CameraChangedEvent` handler
fn setup(mut commands: Commands) {
    // Main camera
    commands.spawn((Camera3dBundle { ..default() }, MainCamera));
}

fn set_camera_viewports(
    windows: Query<&Window>,
    mut resize_events: EventReader<WindowResized>,
    mut main_camera: Query<&mut Camera, With<MainCamera>>,
) {
    // We need to dynamically resize the camera's viewports whenever the window size changes
    // A resize_event is sent when the window is first created, allowing us to reuse this system for initial setup.
    for resize_event in resize_events.read() {
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
    }
}

const HEIGHT_MIN: f32 = 3.0;
const HEIGHT_MAX: f32 = 30.0;
const ANGLE_MOVE_SPEED: f32 = 0.8;
const HEIGHT_MOVE_SPEED: f32 = 15.0;

fn keyboard_input_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_settings: ResMut<CameraSettings>,
) {
    if keyboard_input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            camera_settings.angle -= ANGLE_MOVE_SPEED * time.delta_seconds();
            if camera_settings.angle < 0.0 {
                camera_settings.angle += 2.0 * PI;
            }
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            camera_settings.angle += ANGLE_MOVE_SPEED * time.delta_seconds();
            if camera_settings.angle > 2.0 * PI {
                camera_settings.angle -= 2.0 * PI;
            }
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            camera_settings.height -= HEIGHT_MOVE_SPEED * time.delta_seconds();
            if camera_settings.height < HEIGHT_MIN {
                camera_settings.height = HEIGHT_MIN;
            }
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            camera_settings.height += HEIGHT_MOVE_SPEED * time.delta_seconds();
            if camera_settings.height > HEIGHT_MAX {
                camera_settings.height = HEIGHT_MAX;
            }
        }
    }
}

fn update_camera_position(
    camera_settings: Res<CameraSettings>,
    player_position: Query<Ref<Transform>, (With<PlayerAnimation>, Without<MainCamera>)>,
    mut main_camera: Query<&mut Transform, With<MainCamera>>,
) {
    if let Ok(player) = player_position.get_single() {
        if camera_settings.is_changed() || player.is_changed() {
            let camera = get_camera_position(player.translation, &camera_settings);

            // Main camera position update
            let mut main_camera = main_camera.single_mut();
            *main_camera = Transform::from_xyz(camera.x, camera.y, camera.z)
                .looking_at(player.translation, Vec3::Y);
        }
    }
}

fn get_camera_position(player: Vec3, camera_settings: &CameraSettings) -> Vec3 {
    Vec3 {
        x: player.x + camera_settings.radius * camera_settings.angle.cos(),
        y: camera_settings.height,
        z: player.z + camera_settings.radius * camera_settings.angle.sin(),
    }
}
