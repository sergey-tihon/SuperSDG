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
    direction_index: usize,
}

#[derive(Component)]
pub struct PlayerAnimation(Option<AnimationState>);

#[derive(Component)]
pub struct PressedDirectionIndex(Option<usize>);

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
        PlayerAnimation(None),
        PressedDirectionIndex(None),
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
    mut player_query: Query<
        (&mut PlayerAnimation, &mut PressedDirectionIndex),
        Without<MainCamera>,
    >,
    camera_query: Query<&Transform, With<MainCamera>>,
) {
    if let Ok((mut animation, mut direction_index)) = player_query.get_single_mut() {
        if let Some(index_delta) = get_pressed_index_delta(keyboard_input) {
            let camera = camera_query.get_single().unwrap();
            let camera_forward = (*camera).forward();

            let base_index = get_direction_index(camera_forward);
            let index = (base_index + index_delta as usize) % 4;
            direction_index.0 = Some(index);

            if animation.0.is_none() {
                animation.0 = Some(AnimationState {
                    time: 0.0,
                    direction_index: index,
                });
            }
        } else {
            direction_index.0 = None;
        }
    }
}

fn get_direction_index(camera_forward: Vec3) -> usize {
    let mut base_index = 0;
    let mut base_cosine = f32::MIN;
    for (index, direction) in DIRECTIONS_2D.iter().enumerate() {
        let cosine = get_direction_3d(*direction).dot(camera_forward);
        if cosine > base_cosine {
            base_cosine = cosine;
            base_index = index;
        }
    }
    base_index
}

fn get_pressed_index_delta(keyboard_input: Res<'_, Input<KeyCode>>) -> Option<i32> {
    if keyboard_input.pressed(KeyCode::Up) {
        Some(0)
    } else if keyboard_input.pressed(KeyCode::Right) {
        Some(1)
    } else if keyboard_input.pressed(KeyCode::Down) {
        Some(2)
    } else if keyboard_input.pressed(KeyCode::Left) {
        Some(3)
    } else {
        None
    }
}

fn get_direction_3d(direction: (i32, i32)) -> Vec3 {
    Vec3::new(direction.0 as f32, 0.0, direction.1 as f32)
}

const MOVEMENT_TIME: f32 = 0.2;

fn animate_player_movement(
    mut level: ResMut<MazeLevel>,
    time: Res<Time>,
    mut player_query: Query<
        (
            &mut Transform,
            &mut PlayerAnimation,
            &mut PressedDirectionIndex,
        ),
        Without<MainCamera>,
    >,
) {
    if let Ok((mut player_transform, mut player_animation, direction_index)) =
        player_query.get_single_mut()
    {
        if let Some(animation) = &mut player_animation.0 {
            let delta = time.delta_seconds();
            animation.time += delta;

            let direction_3d = get_direction_3d(DIRECTIONS_2D[animation.direction_index]);
            if animation.time < MOVEMENT_TIME {
                // We are still in the middle of the movement animation
                player_transform.translation += direction_3d * delta / MOVEMENT_TIME;
            } else {
                let level = level.as_mut();

                let direction_2d = DIRECTIONS_2D[animation.direction_index];
                level.start.0 += direction_2d.0;
                level.start.1 += direction_2d.1;

                if Some(animation.direction_index) == direction_index.0 {
                    // We finished the movement animation, but the player is still pressing the same direction button
                    // so we continue the animation int the same direction for a smooth camera experience
                    player_transform.translation += direction_3d * delta / MOVEMENT_TIME;

                    animation.time -= MOVEMENT_TIME;
                } else {
                    // We finished the movement animation and the player should stay in the middle of target cell
                    player_transform.translation = Vec3 {
                        x: level.start.0 as f32 + 0.5,
                        y: 0.5,
                        z: level.start.1 as f32 + 0.5,
                    };

                    player_animation.0 = None;
                }
            }
        }
    }
}
