use bevy::{
    color::palettes::css::{LIMEGREEN, RED},
    prelude::*,
};

use super::{
    camera::MainCamera,
    level::{Directions, MazeLevel, MazeRegenerated, Vec2i},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(sync_positions_on_maze_regenerated)
            .add_systems(
                OnTransition {
                    exited: crate::core::AppState::Menu,
                    entered: crate::core::AppState::InGame,
                },
                spawn_player_and_exit.after(super::level::apply_selected_complexity),
            )
            .add_systems(
                Update,
                keyboard_input_system
                    .run_if(in_state(crate::core::AppState::InGame))
                    .in_set(super::MazeSystems::Input),
            )
            .add_systems(
                Update,
                animate_player_movement
                    .run_if(in_state(crate::core::AppState::InGame))
                    .in_set(super::MazeSystems::Movement),
            );
    }
}

#[derive(Component, Default, Clone)]
#[require(GridPosition, PlayerAnimation, PressedDirectionIndex)]
pub struct Player;

#[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
pub struct GridPosition(pub Vec2i);

#[derive(Clone)]
pub struct AnimationState {
    time: f32,
    direction_index: usize,
}

#[derive(Component, Default, Clone)]
struct PlayerAnimation(Option<AnimationState>);

#[derive(Component, Default, Clone)]
struct PressedDirectionIndex(Option<usize>);

#[derive(Component, Default, Clone)]
pub struct ExitPoint;

fn spawn_player_and_exit(mut commands: Commands, level: Res<MazeLevel>) {
    let grid = GridPosition(level.start);
    let player_pos: Vec3 = level.start.into();
    let exit_pos: Vec3 = level.exit.into();
    let player_color: Color = RED.into();
    let exit_color: Color = LIMEGREEN.into();

    commands.spawn_scene(bsn! {
        #Player
        Player
        grid
        Mesh3d(asset_value(Sphere::new(0.5)))
        MeshMaterial3d::<StandardMaterial>(asset_value(player_color))
        Transform::from_translation(player_pos)
    });
    commands.spawn_scene(bsn! {
        #Exit
        ExitPoint
        Mesh3d(asset_value(Sphere::new(0.5)))
        MeshMaterial3d::<StandardMaterial>(asset_value(exit_color))
        Transform::from_translation(exit_pos)
    });
}

fn sync_positions_on_maze_regenerated(
    _: On<MazeRegenerated>,
    level: Res<MazeLevel>,
    player: Single<(&mut Transform, &mut GridPosition, &mut PlayerAnimation), With<Player>>,
    mut exit: Single<&mut Transform, (With<ExitPoint>, Without<Player>)>,
) {
    let (mut transform, mut grid, mut animation) = player.into_inner();
    grid.0 = level.start;
    transform.translation = level.start.into();
    animation.0 = None;
    exit.translation = level.exit.into();
}

fn keyboard_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    level: Res<MazeLevel>,
    player: Single<
        (
            &GridPosition,
            &mut PlayerAnimation,
            &mut PressedDirectionIndex,
        ),
        With<Player>,
    >,
    camera: Single<&Transform, With<MainCamera>>,
) {
    let (grid, mut animation, mut direction_index) = player.into_inner();
    if let Some(index_delta) = get_pressed_index_delta(&keyboard_input) {
        let camera_forward = camera.forward();
        let up_direction_index = Directions::get_closest(camera_forward.into());
        let index = (up_direction_index + index_delta as usize) % 4;
        direction_index.0 = Some(index);

        let next_position = grid.0.get_next(index);
        if animation.0.is_none() && level.is_cell_empty(next_position) {
            animation.0 = Some(AnimationState {
                time: 0.0,
                direction_index: index,
            });
        }
    } else {
        direction_index.0 = None;
    }
}

fn get_pressed_index_delta(keyboard_input: &ButtonInput<KeyCode>) -> Option<i32> {
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
    level: Res<MazeLevel>,
    time: Res<Time>,
    player: Single<
        (
            &mut Transform,
            &mut GridPosition,
            &mut PlayerAnimation,
            &PressedDirectionIndex,
        ),
        With<Player>,
    >,
) {
    let (mut transform, mut grid, mut animation_state, direction_index) = player.into_inner();
    let Some(animation) = &mut animation_state.0 else {
        return;
    };

    let delta = time.delta_secs();
    animation.time += delta;
    let direction_3d = Directions::get_3d(animation.direction_index);
    if animation.time < MOVEMENT_TIME {
        transform.translation += direction_3d * delta / MOVEMENT_TIME;
    } else {
        grid.0 = grid.0.get_next(animation.direction_index);
        let next_next_position = grid.0.get_next(animation.direction_index);

        if Some(animation.direction_index) == direction_index.0
            && level.is_cell_empty(next_next_position)
        {
            transform.translation += direction_3d * delta / MOVEMENT_TIME;
            animation.time -= MOVEMENT_TIME;
        } else {
            transform.translation = grid.0.into();
            animation_state.0 = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maze_regeneration_syncs_player_and_exit_positions() {
        let mut app = App::new();
        app.insert_resource(MazeLevel::new(5, 5))
            .add_observer(sync_positions_on_maze_regenerated);

        let player = app
            .world_mut()
            .spawn((
                Player,
                Transform::default(),
                PlayerAnimation(Some(AnimationState {
                    time: 0.1,
                    direction_index: 0,
                })),
            ))
            .id();
        let exit = app
            .world_mut()
            .spawn((Transform::default(), ExitPoint))
            .id();

        app.world_mut()
            .resource_mut::<MazeLevel>()
            .regenerate_with_size(7, 7);
        let expected_grid = GridPosition(app.world().resource::<MazeLevel>().start);
        let expected_player: Vec3 = app.world().resource::<MazeLevel>().start.into();
        let expected_exit: Vec3 = app.world().resource::<MazeLevel>().exit.into();
        app.world_mut().trigger(MazeRegenerated);

        assert_eq!(
            app.world().get::<Transform>(player).unwrap().translation,
            expected_player
        );
        assert_eq!(
            app.world().get::<GridPosition>(player),
            Some(&expected_grid)
        );
        assert!(
            app.world()
                .get::<PlayerAnimation>(player)
                .unwrap()
                .0
                .is_none()
        );
        assert_eq!(
            app.world().get::<Transform>(exit).unwrap().translation,
            expected_exit
        );
    }
}
