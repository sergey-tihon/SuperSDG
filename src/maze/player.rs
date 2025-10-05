use bevy::{
    color::palettes::css::{LIMEGREEN, RED},
    prelude::*,
};

use super::{
    camera::MainCamera,
    level::{Directions, MazeLevel},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(crate::AppState::InGame), setup)
            .add_systems(
                Update,
                (keyboard_input_system, animate_player_movement)
                    .run_if(in_state(crate::AppState::InGame)),
            );
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

#[derive(Component)]
pub struct ExitPoint;

fn setup(
    mut commands: Commands,
    level: Res<MazeLevel>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add Player
    let start = level.player_position;
    commands.spawn((
        Mesh3d(meshes.add(Sphere { radius: 0.5 })),
        MeshMaterial3d(standard_materials.add(StandardMaterial {
            base_color: RED.into(),
            ..default()
        })),
        Transform::from_translation(start.into()),
        PlayerAnimation(None),
        PressedDirectionIndex(None),
    ));

    // Add exit
    let exit = level.exit_position;
    commands.spawn((
        Mesh3d(meshes.add(Sphere { radius: 0.5 })),
        MeshMaterial3d(standard_materials.add(StandardMaterial {
            base_color: LIMEGREEN.into(),
            ..default()
        })),
        Transform::from_translation(exit.into()),
        ExitPoint,
    ));
}

fn keyboard_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    level: Res<MazeLevel>,
    mut player_query: Query<
        (&mut PlayerAnimation, &mut PressedDirectionIndex),
        Without<MainCamera>,
    >,
    camera_query: Query<&Transform, With<MainCamera>>,
) {
    if let Ok((mut animation, mut direction_index)) = player_query.single_mut() {
        if let Some(index_delta) = get_pressed_index_delta(keyboard_input) {
            if let Ok(camera) = camera_query.single() {
                let camera_forward = (*camera).forward();

                let up_direction_index = Directions::get_closest(camera_forward.into());
                let index = (up_direction_index + index_delta as usize) % 4;
                direction_index.0 = Some(index);

                let next_position = level.player_position.get_next(index);
                if animation.0.is_none() && level.is_cell_empty(next_position) {
                    animation.0 = Some(AnimationState {
                        time: 0.0,
                        direction_index: index,
                    });
                }
            }
        } else {
            direction_index.0 = None;
        }
    }
}

fn get_pressed_index_delta(keyboard_input: Res<'_, ButtonInput<KeyCode>>) -> Option<i32> {
    if !keyboard_input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            Some(0)
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            Some(1)
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            Some(2)
        } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
            Some(3)
        } else {
            None
        }
    } else {
        None
    }
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
        player_query.single_mut()
        && let Some(animation) = &mut player_animation.0
    {
        let delta = time.delta_secs();
        animation.time += delta;

        let direction_3d = Directions::get_3d(animation.direction_index);
        if animation.time < MOVEMENT_TIME {
            // We are still in the middle of the movement animation
            player_transform.translation += direction_3d * delta / MOVEMENT_TIME;
        } else {
            let level = level.as_mut();
            level.player_position = level.player_position.get_next(animation.direction_index);
            let next_next_position = level.player_position.get_next(animation.direction_index);

            if Some(animation.direction_index) == direction_index.0
                && level.is_cell_empty(next_next_position)
            {
                // We finished the movement animation, but the player is still pressing the same direction button
                // so we continue the animation int the same direction for a smooth camera experience
                player_transform.translation += direction_3d * delta / MOVEMENT_TIME;
                animation.time -= MOVEMENT_TIME;
            } else {
                // We finished the movement animation and the player should stay in the middle of target cell
                player_transform.translation = level.player_position.into();
                player_animation.0 = None;
            }
        }
    }
}
