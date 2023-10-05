use bevy::prelude::*;

use super::{camera::*, level::MazeLevel};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, keyboard_input_system);
    }
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    level: Res<MazeLevel>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add Player
    let start = level.start;
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.5,
                ..default()
            })),
            material: standard_materials.add(StandardMaterial {
                base_color: Color::RED,
                ..default()
            }),
            transform: Transform::from_xyz(start.0 as f32 + 0.5, 0.5, start.1 as f32 + 0.5),
            ..default()
        },
        Player,
    ));

    // Add exit
    let exit = level.exit;
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius: 0.5,
            ..default()
        })),
        material: standard_materials.add(StandardMaterial {
            base_color: Color::LIME_GREEN,
            ..default()
        }),
        transform: Transform::from_xyz(exit.0 as f32 + 0.5, 0.5, exit.1 as f32 + 0.5),
        ..default()
    });
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        if keyboard_input.just_released(KeyCode::Left) {
            player.translation.x -= 1.0;
        }
        if keyboard_input.just_released(KeyCode::Right) {
            player.translation.x += 1.0;
        }
        if keyboard_input.just_released(KeyCode::Down) {
            player.translation.z += 1.0;
        }
        if keyboard_input.just_released(KeyCode::Up) {
            player.translation.z -= 1.0;
        }
    }
}
