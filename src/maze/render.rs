use bevy::prelude::*;

use super::level::MazeLevel;

pub struct MazeRenderPlugin;

impl Plugin for MazeRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, render_maze_on_change);
    }
}

/// Marker component for maze wall entities.
#[derive(Component)]
pub struct MazeWall;

/// Marker component for maze floor entities.
#[derive(Component)]
pub struct MazeFloor;

#[derive(Resource)]
struct MazeRenderState {
    rendered_generation: Option<u32>,
    wall_handle: Handle<Image>,
    wall_normal_handle: Handle<Image>,
    floor_handle: Handle<Image>,
    floor_normal_handle: Handle<Image>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(MazeRenderState {
        rendered_generation: None,
        wall_handle: asset_server.load("textures/wall.png"),
        wall_normal_handle: asset_server.load("textures/wall_normal.png"),
        floor_handle: asset_server.load("textures/floor.png"),
        floor_normal_handle: asset_server.load("textures/floor_normal.png"),
    });
}

fn render_maze_on_change(
    mut commands: Commands,
    level: Res<MazeLevel>,
    mut render_state: ResMut<MazeRenderState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    walls_query: Query<Entity, With<MazeWall>>,
    floors_query: Query<Entity, With<MazeFloor>>,
) {
    // Only re-render if generation changed
    if render_state.rendered_generation == Some(level.generation) {
        return;
    }
    render_state.rendered_generation = Some(level.generation);

    // Despawn all existing walls and floors
    for entity in walls_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in floors_query.iter() {
        commands.entity(entity).despawn();
    }

    // Create meshes and materials
    let cube_handle = meshes.add(Cuboid {
        half_size: Vec3::splat(0.5),
    });
    let wall_material = materials.add(StandardMaterial {
        base_color_texture: Some(render_state.wall_handle.clone()),
        normal_map_texture: Some(render_state.wall_normal_handle.clone()),
        ..default()
    });

    let plane_handle = meshes.add(Plane3d::default().mesh().size(1.0, 1.0));
    let floor_material = materials.add(StandardMaterial {
        base_color_texture: Some(render_state.floor_handle.clone()),
        normal_map_texture: Some(render_state.floor_normal_handle.clone()),
        ..default()
    });

    // Spawn walls and floors based on maze layout
    for (z, s) in level.map.iter().enumerate() {
        for (x, &c) in s.iter().enumerate() {
            if c == '#' {
                commands.spawn((
                    Mesh3d(cube_handle.clone()),
                    MeshMaterial3d(wall_material.clone()),
                    Transform::from_xyz(x as f32 + 0.5, 0.5, z as f32 + 0.5),
                    MazeWall,
                ));
            } else {
                commands.spawn((
                    Mesh3d(plane_handle.clone()),
                    MeshMaterial3d(floor_material.clone()),
                    Transform::from_xyz(x as f32 + 0.5, 0.0, z as f32 + 0.5),
                    MazeFloor,
                ));
            }
        }
    }
}
