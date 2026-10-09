use std::f32::consts::{FRAC_PI_2, TAU};

use bevy::prelude::*;

use super::light::flashlight;
use super::{
    level::{MazeLevel, MazeRegenerated},
    player::Player,
};

pub struct MazeCameraPlugin;

const DEFAULT_HEIGHT: f32 = 15.0;
const DEFAULT_RADIUS: f32 = 20.0;
const DEFAULT_ANGLE: f32 = FRAC_PI_2;

impl Plugin for MazeCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraSettings>()
            .add_observer(reset_camera_on_maze_regenerated)
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                keyboard_input_system
                    .run_if(in_state(crate::core::AppState::InGame))
                    .in_set(super::MazeSystems::Input),
            )
            .add_systems(
                Update,
                update_camera_position.in_set(super::MazeSystems::Camera),
            );
    }
}

#[derive(Resource)]
pub struct CameraSettings {
    pub height: f32,
    pub radius: f32,
    pub angle: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            height: DEFAULT_HEIGHT,
            radius: DEFAULT_RADIUS,
            angle: DEFAULT_ANGLE,
        }
    }
}
#[derive(Component, Default, Clone)]
#[require(Camera3d)]
pub struct MainCamera;

fn setup(mut commands: Commands, level: Res<MazeLevel>, settings: Res<CameraSettings>) {
    let player_start: Vec3 = level.start.into();
    let position = get_camera_position(player_start, &settings);
    let transform =
        Transform::from_xyz(position.x, position.y, position.z).looking_at(player_start, Vec3::Y);
    commands.spawn_scene(bsn! {
        #MainCamera
        MainCamera
        transform
        Children [ @flashlight() ]
    });
}

fn reset_camera_on_maze_regenerated(
    _: On<MazeRegenerated>,
    mut camera_settings: ResMut<CameraSettings>,
) {
    *camera_settings = CameraSettings::default();
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
    let delta = time.delta_secs();
    if keyboard_input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            camera_settings.angle =
                (camera_settings.angle - ANGLE_MOVE_SPEED * delta).rem_euclid(TAU);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            camera_settings.angle =
                (camera_settings.angle + ANGLE_MOVE_SPEED * delta).rem_euclid(TAU);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            camera_settings.height =
                (camera_settings.height - HEIGHT_MOVE_SPEED * delta).clamp(HEIGHT_MIN, HEIGHT_MAX);
            adjust_radius_based_on_height(&mut camera_settings);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            camera_settings.height =
                (camera_settings.height + HEIGHT_MOVE_SPEED * delta).clamp(HEIGHT_MIN, HEIGHT_MAX);
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
    player: Single<Ref<Transform>, (With<Player>, Without<MainCamera>)>,
    mut camera: Single<&mut Transform, With<MainCamera>>,
) {
    if !(camera_settings.is_changed() || player.is_changed()) {
        return;
    }

    let position = get_camera_position(player.translation, &camera_settings);
    **camera = Transform::from_xyz(position.x, position.y, position.z)
        .looking_at(player.translation, Vec3::Y);
}

fn get_camera_position(player: Vec3, camera_settings: &CameraSettings) -> Vec3 {
    Vec3 {
        x: player.x + camera_settings.radius * camera_settings.angle.cos(),
        y: camera_settings.height,
        z: player.z + camera_settings.radius * camera_settings.angle.sin(),
    }
}
