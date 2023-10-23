use std::f32::consts::PI;

use bevy::prelude::*;

use super::{camera::MainCamera, player::Player};

pub struct MazeLightPlugin;

impl Plugin for MazeLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, animate_light_direction);
    }
}

const OUTER_ANGLE: f32 = PI / 8.0;

fn setup(mut commands: Commands) {
    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.02,
    });

    // directional 'sun' light
    commands.spawn(SpotLightBundle {
        spot_light: SpotLight {
            intensity: 25000.0, // lumens
            range: 200.0,
            color: Color::WHITE,
            shadows_enabled: true,
            inner_angle: OUTER_ANGLE * 0.1,
            outer_angle: OUTER_ANGLE,
            ..default()
        },
        ..default()
    });
}

fn animate_light_direction(
    mut light_query: Query<&mut Transform, (With<SpotLight>, Without<MainCamera>, Without<Player>)>,
    camera_query: Query<&Transform, (Changed<Transform>, With<MainCamera>, Without<Player>)>,
    player_position: Query<&Transform, With<Player>>,
) {
    if let (Ok(mut light), Ok(player), Ok(camera)) = (
        light_query.get_single_mut(),
        player_position.get_single(),
        camera_query.get_single(),
    ) {
        (*light) =
            Transform::from_translation(camera.translation).looking_at(player.translation, Vec3::Y);
    }
}
