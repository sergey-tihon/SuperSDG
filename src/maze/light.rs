use std::f32::consts::PI;

use bevy::{color::palettes::css::GOLD, prelude::*};

use super::{MazeSystems, camera::MainCamera, player::Player};

pub struct MazeLightPlugin;

impl Plugin for MazeLightPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalAmbientLight {
            color: GOLD.into(),
            brightness: 0.10,
            ..default()
        })
        .add_systems(
            Update,
            update_flashlight_intensity.in_set(MazeSystems::Lighting),
        );
    }
}

const INNER_ANGLE: f32 = PI / 18.0; // More focused beam (10 degrees)
const OUTER_ANGLE: f32 = PI / 12.0; // 15 degrees
const BASE_INTENSITY: f32 = 5_000_000.0;
const DISTANCE_RANGE: (f32, f32) = (5.0, 40.0);
const HEIGHT_RANGE: (f32, f32) = (5.0, 30.0);

pub(super) fn flashlight() -> impl Scene {
    bsn! {
        #Flashlight
        SpotLight {
            intensity: 10_000_000.0,
            range: 300.0,
            color: Color::WHITE,
            shadow_maps_enabled: true,
            inner_angle: INNER_ANGLE,
            outer_angle: OUTER_ANGLE,
        }
        Transform::from_xyz(0.0, 0.0, 0.5)
    }
}

fn update_flashlight_intensity(
    camera: Single<&Transform, (With<MainCamera>, Changed<Transform>)>,
    player: Single<&Transform, With<Player>>,
    mut flashlight: Single<&mut SpotLight>,
) {
    let normalize =
        |value: f32, range: (f32, f32)| ((value - range.0) / (range.1 - range.0)).clamp(0.0, 1.0);
    let distance_factor = 0.5
        + normalize(
            camera.translation.distance(player.translation),
            DISTANCE_RANGE,
        ) * 2.0;
    let height_factor = 1.0 + normalize(camera.translation.y, HEIGHT_RANGE) * 1.5;
    flashlight.intensity = BASE_INTENSITY * distance_factor * height_factor;
}
