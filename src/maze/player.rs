use bevy::prelude::*;

use super::level::MazeLevel;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerMovedEvent>()
            .add_systems(Startup, setup)
            .add_systems(Update, keyboard_input_system);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Event)]
pub struct PlayerMovedEvent;

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
    mut level: ResMut<MazeLevel>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut player_moved_event_writer: EventWriter<PlayerMovedEvent>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        let level = level.as_mut();

        if keyboard_input.just_released(KeyCode::Left) {
            player.translation.x -= 1.0;
            level.start.0 -= 1;
            player_moved_event_writer.send(PlayerMovedEvent);
        }
        if keyboard_input.just_released(KeyCode::Right) {
            player.translation.x += 1.0;
            level.start.0 += 1;
            player_moved_event_writer.send(PlayerMovedEvent);
        }
        if keyboard_input.just_released(KeyCode::Down) {
            player.translation.z += 1.0;
            level.start.1 += 1;
            player_moved_event_writer.send(PlayerMovedEvent);
        }
        if keyboard_input.just_released(KeyCode::Up) {
            player.translation.z -= 1.0;
            level.start.1 -= 1;
            player_moved_event_writer.send(PlayerMovedEvent);
        }
    }
}
