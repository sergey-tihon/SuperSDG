use std::f32::consts::{FRAC_PI_2, PI};

use bevy::{prelude::*, render::camera::Viewport, window::WindowResized};

use super::{player::Player, player::PlayerMovedEvent};

pub struct MazeCameraPlugin;

impl Plugin for MazeCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraSettings {
            height: 15.0,
            radius: 20.0,
            angle: 2.0 * PI,
        })
        .add_event::<CameraChangedEvent>()
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

#[derive(Event)]
pub struct CameraChangedEvent;

// Setup camera objects but without any exact position
// Proper position will be calculated by `CameraChangedEvent` handler
fn setup(mut commands: Commands, mut camera_changed_event_writer: EventWriter<CameraChangedEvent>) {
    // Main camera
    commands.spawn((Camera3dBundle { ..default() }, MainCamera));
    camera_changed_event_writer.send(CameraChangedEvent);
}

fn set_camera_viewports(
    windows: Query<&Window>,
    mut resize_events: EventReader<WindowResized>,
    mut main_camera: Query<&mut Camera, With<MainCamera>>,
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
    }
}

const HEIGHT_MIN: f32 = 3.0;
const HEIGHT_MAX: f32 = 30.0;
const ANGLE_MOVE_SPEED: f32 = 0.5;
const HEIGHT_MOVE_SPEED: f32 = 10.0;

fn keyboard_input_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_settings: ResMut<CameraSettings>,
    mut camera_changed_event_writer: EventWriter<CameraChangedEvent>,
) {
    if keyboard_input.pressed(KeyCode::D) {
        camera_settings.angle -= ANGLE_MOVE_SPEED * time.delta_seconds();
        if camera_settings.angle < 0.0 {
            camera_settings.angle += 2.0 * PI;
        }
        camera_changed_event_writer.send(CameraChangedEvent);
    }
    if keyboard_input.pressed(KeyCode::A) {
        camera_settings.angle += ANGLE_MOVE_SPEED * time.delta_seconds();
        if camera_settings.angle > 2.0 * PI {
            camera_settings.angle -= 2.0 * PI;
        }
        camera_changed_event_writer.send(CameraChangedEvent);
    }
    if keyboard_input.pressed(KeyCode::S) {
        camera_settings.height -= HEIGHT_MOVE_SPEED * time.delta_seconds();
        if camera_settings.height < HEIGHT_MIN {
            camera_settings.height = HEIGHT_MIN;
        }
        camera_changed_event_writer.send(CameraChangedEvent);
    }
    if keyboard_input.pressed(KeyCode::W) {
        camera_settings.height += HEIGHT_MOVE_SPEED * time.delta_seconds();
        if camera_settings.height > HEIGHT_MAX {
            camera_settings.height = HEIGHT_MAX;
        }
        camera_changed_event_writer.send(CameraChangedEvent);
    }
}

fn update_camera_position(
    camera_settings: Res<CameraSettings>,
    player_position: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    camera_changed_events: EventReader<CameraChangedEvent>,
    player_moved_events: EventReader<PlayerMovedEvent>,
    mut main_camera: Query<&mut Transform, With<MainCamera>>,
) {
    if !camera_changed_events.is_empty() || !player_moved_events.is_empty() {
        let player = player_position.single().translation;
        let camera = get_camera_position(player, &camera_settings);

        // Main camera position update
        let mut main_camera = main_camera.single_mut();
        *main_camera =
            Transform::from_xyz(camera.x, camera.y, camera.z).looking_at(player, Vec3::Y);
    }
}

fn get_camera_position(player: Vec3, camera_settings: &CameraSettings) -> Vec3 {
    Vec3 {
        x: player.x + camera_settings.radius * camera_settings.angle.cos(),
        y: camera_settings.height,
        z: player.z + camera_settings.radius * camera_settings.angle.sin(),
    }
}
