use bevy::prelude::*;

use super::{camera::MainCamera, level::MazeLevel};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (keyboard_input_system, animate_player_movement));
    }
}

pub struct AnimationState {
    time: f32,
    direction_2d: (i32, i32),
    direction_3d: Vec3,
}

#[derive(Component)]
pub struct Player {
    animation: Option<AnimationState>,
}

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
        Player { animation: None },
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

const DIRECTIONS_2D: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Player, (With<Player>, Without<MainCamera>)>,
    camera_query: Query<&Transform, With<MainCamera>>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        let index_delta = if keyboard_input.pressed(KeyCode::Up) {
            Some(0)
        } else if keyboard_input.pressed(KeyCode::Right) {
            Some(1)
        } else if keyboard_input.pressed(KeyCode::Down) {
            Some(2)
        } else if keyboard_input.pressed(KeyCode::Left) {
            Some(3)
        } else {
            None
        };

        if let Some(index_delta) = index_delta {
            if player.animation.is_none() {
                let camera = camera_query.get_single().unwrap();
                let camera_forward = (*camera).forward();

                let mut base_index = 0;
                let mut base_cosine = f32::MIN;
                for (index, direction) in DIRECTIONS_2D.iter().enumerate() {
                    let cosine = get_direction_3d(*direction).dot(camera_forward);
                    if cosine > base_cosine {
                        base_cosine = cosine;
                        base_index = index;
                    }
                }

                let index = (base_index + index_delta as usize) % 4;
                let direction_2d = DIRECTIONS_2D[index];
                player.animation = Some(AnimationState {
                    time: 0.0,
                    direction_2d,
                    direction_3d: get_direction_3d(direction_2d),
                });
            }
        }
    }
}

fn get_direction_3d(direction: (i32, i32)) -> Vec3 {
    Vec3::new(direction.0 as f32, 0.0, direction.1 as f32)
}

const MOVEMENT_TIME: f32 = 0.2;

fn animate_player_movement(
    mut level: ResMut<MazeLevel>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &mut Player), Without<MainCamera>>,
) {
    if let Ok((mut player_transform, mut player)) = player_query.get_single_mut() {
        if let Some(animation) = &mut player.animation {
            let delta = time.delta_seconds();
            animation.time += delta;
            if animation.time > MOVEMENT_TIME {
                let level = level.as_mut();
                level.start.0 += animation.direction_2d.0;
                level.start.1 += animation.direction_2d.1;
                player_transform.translation = Vec3 {
                    x: level.start.0 as f32 + 0.5,
                    y: 0.5,
                    z: level.start.1 as f32 + 0.5,
                };

                player.animation = None;
            } else {
                player_transform.translation += animation.direction_3d * delta / MOVEMENT_TIME;
            }
        }
    }
}
