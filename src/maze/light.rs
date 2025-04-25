use std::f32::consts::PI;

use bevy::{color::palettes::css::ORANGE_RED, prelude::*};

use super::{camera::MainCamera, player::PlayerAnimation};

pub struct MazeLightPlugin;

impl Plugin for MazeLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, animate_light_direction);
    }
}

const INNER_ANGLE: f32 = PI / 12.0;

fn setup(mut commands: Commands) {
    // ambient light
    commands.insert_resource(AmbientLight {
        color: ORANGE_RED.into(),
        brightness: 0.02,
        ..Default::default()
    });

    // directional 'sun' light
    commands.spawn(SpotLight {
        intensity: 7_000_000.0, // lumens
        range: 300.0,
        color: Color::WHITE,
        shadows_enabled: true,
        inner_angle: INNER_ANGLE,
        outer_angle: INNER_ANGLE * 1.5,
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
    mut light_query: Query<&mut Transform, LightQueryFilter>,
    camera_query: Query<&Transform, CameraQueryFilter>,
    player_position: Query<&Transform, With<PlayerAnimation>>,
) {
    if let (Ok(mut light), Ok(player), Ok(camera)) = (
        light_query.single_mut(),
        player_position.single(),
        camera_query.single(),
    ) {
        (*light) =
            Transform::from_translation(camera.translation).looking_at(player.translation, Vec3::Y);
    }
}
