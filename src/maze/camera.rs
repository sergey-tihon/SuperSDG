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
        .add_systems(Startup, setup.in_set(super::CameraSwawned))
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
#[require(Camera3d)]
pub struct MainCamera;

// Setup camera objects but without any exact position
// Proper position will be calculated by `CameraChangedEvent` handler
fn setup(mut commands: Commands) {
    // Main camera
    commands.spawn(MainCamera);
}

fn set_camera_viewports(
    windows: Query<&Window>,
    mut resize_events: EventReader<WindowResized>,
    mut main_camera: Query<&mut Camera, With<MainCamera>>,
) {
    // We need to dynamically resize the camera's viewports whenever the window size changes
    // A resize_event is sent when the window is first created, allowing us to reuse this system for initial setup.
    for resize_event in resize_events.read() {
        if let Ok(window) = windows.get(resize_event.window) {
            if let Ok(mut main_camera) = main_camera.single_mut() {
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
    }
}

const HEIGHT_MIN: f32 = 3.0;
const HEIGHT_MAX: f32 = 30.0;
const ANGLE_MOVE_SPEED: f32 = 0.8;
const HEIGHT_MOVE_SPEED: f32 = 15.0;
const RADIUS_MAX: f32 = 20.0;
const RADIUS_MIN: f32 = 0.5; // Minimum radius to prevent glitches at extreme heights

fn keyboard_input_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_settings: ResMut<CameraSettings>,
) {
    if keyboard_input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            camera_settings.angle -= ANGLE_MOVE_SPEED * time.delta_secs();
            if camera_settings.angle < 0.0 {
                camera_settings.angle += 2.0 * PI;
            }
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            camera_settings.angle += ANGLE_MOVE_SPEED * time.delta_secs();
            if camera_settings.angle > 2.0 * PI {
                camera_settings.angle -= 2.0 * PI;
            }
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            camera_settings.height -= HEIGHT_MOVE_SPEED * time.delta_secs();
            if camera_settings.height < HEIGHT_MIN {
                camera_settings.height = HEIGHT_MIN;
            }
            adjust_radius_based_on_height(&mut camera_settings);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            camera_settings.height += HEIGHT_MOVE_SPEED * time.delta_secs();
            if camera_settings.height > HEIGHT_MAX {
                camera_settings.height = HEIGHT_MAX;
            }
            adjust_radius_based_on_height(&mut camera_settings);
        }
    }
}

// Adjust radius based on camera height
// When camera is very high or very low, radius should be close to minimum
// When camera is at medium height, radius should be at maximum
fn adjust_radius_based_on_height(camera_settings: &mut CameraSettings) {
    // Calculate the normalized height (0.0 to 1.0)
    let height_range = HEIGHT_MAX - HEIGHT_MIN;
    let normalized_height = (camera_settings.height - HEIGHT_MIN) / height_range;

    // Calculate a factor that peaks at 0.5 (middle height) and approaches 0 at extremes
    // Using a parabolic function: 4 * x * (1 - x) which peaks at x = 0.5
    let height_factor = 4.0 * normalized_height * (1.0 - normalized_height);

    // Apply the factor to the radius, ensuring it never goes below RADIUS_MIN
    let radius_range = RADIUS_MAX - RADIUS_MIN;
    camera_settings.radius = RADIUS_MIN + radius_range * height_factor;
}

fn update_camera_position(
    camera_settings: Res<CameraSettings>,
    player_position: Query<Ref<Transform>, (With<PlayerAnimation>, Without<MainCamera>)>,
    mut main_camera: Query<&mut Transform, With<MainCamera>>,
) {
    if let Ok(player) = player_position.single() {
        if camera_settings.is_changed() || player.is_changed() {
            let camera = get_camera_position(player.translation, &camera_settings);

            // Main camera position update
            if let Ok(mut main_camera) = main_camera.single_mut() {
                *main_camera = Transform::from_xyz(camera.x, camera.y, camera.z)
                    .looking_at(player.translation, Vec3::Y);
            }
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
