use std::f32::consts::PI;

use bevy::{color::palettes::css::GOLD, prelude::*};

use super::{camera::MainCamera, player::PlayerAnimation};

pub struct MazeLightPlugin;

impl Plugin for MazeLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, animate_light_direction);
    }
}

const INNER_ANGLE: f32 = PI / 18.0; // More focused beam (10 degrees)
const OUTER_ANGLE: f32 = PI / 12.0; // 15 degrees
// Adjust light intensity based on distance and height
const BASE_INTENSITY: f32 = 5_000_000.0;
const DISTANCE_RANGE: (f32, f32) = (5.0, 40.0); // min, max
const HEIGHT_RANGE: (f32, f32) = (5.0, 30.0); // min, max

fn setup(mut commands: Commands) {
    // Ambient light (now a component in Bevy 0.18)
    commands.spawn(AmbientLight {
        color: GOLD.into(),
        brightness: 0.10, // Slightly brighter ambient light
        ..Default::default()
    });

    // Only spawn the spotlight (no directional light)
    commands.spawn(SpotLight {
        intensity: 10_000_000.0, // Increased intensity for brighter beam
        range: 300.0,
        color: Color::WHITE,
        shadows_enabled: true,
        inner_angle: INNER_ANGLE,
        outer_angle: OUTER_ANGLE,
        ..default()
    });
}

type LightQueryFilter = (
    With<SpotLight>,
    Without<MainCamera>,
    Without<PlayerAnimation>,
);
type CameraQueryFilter = (
    Changed<Transform>,
    With<MainCamera>,
    Without<PlayerAnimation>,
);

fn animate_light_direction(
    mut light_query: Query<(&mut Transform, &mut SpotLight), LightQueryFilter>,
    camera_query: Query<&Transform, CameraQueryFilter>,
    player_position: Query<&Transform, With<PlayerAnimation>>,
) {
    if let (Ok((mut light_transform, mut spotlight)), Ok(player), Ok(camera)) = (
        light_query.single_mut(),
        player_position.single(),
        camera_query.single(),
    ) {
        // Position the light behind the camera and point it in the same direction
        let light_position = camera.translation - camera.forward() * 0.5;
        *light_transform = Transform::from_translation(light_position)
            .looking_at(camera.translation + camera.forward() * 10.0, Vec3::Y);

        // Calculate normalized factors (0.0 to 1.0)
        let normalize = |value: f32, range: (f32, f32)| {
            ((value - range.0) / (range.1 - range.0)).clamp(0.0, 1.0)
        };

        let distance_factor = 0.5
            + normalize(
                camera.translation.distance(player.translation),
                DISTANCE_RANGE,
            ) * 2.0;
        let height_factor = 1.0 + normalize(camera.translation.y, HEIGHT_RANGE) * 1.5;

        spotlight.intensity = BASE_INTENSITY * distance_factor * height_factor;
    }
}
