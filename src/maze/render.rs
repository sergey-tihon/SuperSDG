use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
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

/// Check if a cell is a wall. Out-of-bounds returns false to ensure boundary faces are rendered.
fn is_wall_at(level: &MazeLevel, x: i32, z: i32) -> bool {
    if x < 0 || z < 0 {
        return false; // Out of bounds = not a wall, so boundary faces render
    }
    let (xu, zu) = (x as usize, z as usize);
    if zu >= level.map.len() || xu >= level.map[zu].len() {
        return false; // Out of bounds = not a wall
    }
    level.map[zu][xu] == '#'
}

/// Wall face definition matching Bevy's Cuboid exactly.
/// Each face has 4 vertices with (position, normal, uv).
/// Indices pattern: [0, 1, 2, 2, 3, 0]
struct WallFace {
    /// Direction to check for adjacent wall (if wall exists, skip this face)
    check_dir: (i32, i32), // (dx, dz)
    /// 4 vertices: (position offset, normal, uv) - exactly matching Bevy's Cuboid
    vertices: [([f32; 3], [f32; 3], [f32; 2]); 4],
}

// Vertex data copied directly from Bevy's Cuboid mesh (min=0, max=1)
// See: https://docs.rs/bevy_mesh/0.17.3/src/bevy_mesh/primitives/dim3/cuboid.rs.html
const WALL_FACES: [WallFace; 5] = [
    // Front (+Z)
    WallFace {
        check_dir: (0, 1),
        vertices: [
            ([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0]),
            ([1.0, 0.0, 1.0], [0.0, 0.0, 1.0], [1.0, 0.0]),
            ([1.0, 1.0, 1.0], [0.0, 0.0, 1.0], [1.0, 1.0]),
            ([0.0, 1.0, 1.0], [0.0, 0.0, 1.0], [0.0, 1.0]),
        ],
    },
    // Back (-Z)
    WallFace {
        check_dir: (0, -1),
        vertices: [
            ([0.0, 1.0, 0.0], [0.0, 0.0, -1.0], [1.0, 0.0]),
            ([1.0, 1.0, 0.0], [0.0, 0.0, -1.0], [0.0, 0.0]),
            ([1.0, 0.0, 0.0], [0.0, 0.0, -1.0], [0.0, 1.0]),
            ([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], [1.0, 1.0]),
        ],
    },
    // Right (+X)
    WallFace {
        check_dir: (1, 0),
        vertices: [
            ([1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0]),
            ([1.0, 1.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0]),
            ([1.0, 1.0, 1.0], [1.0, 0.0, 0.0], [1.0, 1.0]),
            ([1.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0]),
        ],
    },
    // Left (-X)
    WallFace {
        check_dir: (-1, 0),
        vertices: [
            ([0.0, 0.0, 1.0], [-1.0, 0.0, 0.0], [1.0, 0.0]),
            ([0.0, 1.0, 1.0], [-1.0, 0.0, 0.0], [0.0, 0.0]),
            ([0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 1.0]),
            ([0.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [1.0, 1.0]),
        ],
    },
    // Top (+Y) - always rendered
    WallFace {
        check_dir: (0, 0), // special: always render
        vertices: [
            ([1.0, 1.0, 0.0], [0.0, 1.0, 0.0], [1.0, 0.0]),
            ([0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0]),
            ([0.0, 1.0, 1.0], [0.0, 1.0, 0.0], [0.0, 1.0]),
            ([1.0, 1.0, 1.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ],
    },
    // Bottom (-Y) - skipped entirely (floors cover it)
];

/// Build a single merged mesh for all wall geometry with hidden face culling.
fn build_wall_mesh(level: &MazeLevel) -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for (z, row) in level.map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell != '#' {
                continue;
            }

            let xi = x as i32;
            let zi = z as i32;
            let base_x = x as f32;
            let base_z = z as f32;

            for face in &WALL_FACES {
                // Check if this face should be rendered
                let should_render = if face.check_dir == (0, 0) {
                    // Top face: always render
                    true
                } else {
                    // Only render if adjacent cell is NOT a wall
                    !is_wall_at(level, xi + face.check_dir.0, zi + face.check_dir.1)
                };

                if !should_render {
                    continue;
                }

                let base_index = positions.len() as u32;

                // Add 4 vertices for this quad
                for (pos, normal, uv) in &face.vertices {
                    positions.push([base_x + pos[0], pos[1], base_z + pos[2]]);
                    normals.push(*normal);
                    uvs.push(*uv);
                }

                // Add 6 indices (2 triangles) matching Bevy's Cuboid pattern
                indices.extend_from_slice(&[
                    base_index,
                    base_index + 1,
                    base_index + 2,
                    base_index + 2,
                    base_index + 3,
                    base_index,
                ]);
            }
        }
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}

/// Build a single merged mesh for all floor geometry.
fn build_floor_mesh(level: &MazeLevel) -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    // Floor vertices: matching Bevy's Top face but at y=0
    // (pos, normal, uv) - exactly like Bevy's Cuboid top face
    let floor_vertices: [([f32; 3], [f32; 3], [f32; 2]); 4] = [
        ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 0.0]),
        ([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0]),
        ([0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [0.0, 1.0]),
        ([1.0, 0.0, 1.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
    ];

    for (z, row) in level.map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == '#' {
                continue;
            }

            let base_index = positions.len() as u32;
            let base_x = x as f32;
            let base_z = z as f32;

            for (pos, normal, uv) in &floor_vertices {
                positions.push([base_x + pos[0], pos[1], base_z + pos[2]]);
                normals.push(*normal);
                uvs.push(*uv);
            }

            // 2 triangles matching Bevy's pattern
            indices.extend_from_slice(&[
                base_index,
                base_index + 1,
                base_index + 2,
                base_index + 2,
                base_index + 3,
                base_index,
            ]);
        }
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
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

    // Despawn existing wall and floor entities (now only 2 max)
    for entity in walls_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in floors_query.iter() {
        commands.entity(entity).despawn();
    }

    // Build merged meshes
    let wall_mesh = build_wall_mesh(&level);
    let floor_mesh = build_floor_mesh(&level);

    // Create materials
    let wall_material = materials.add(StandardMaterial {
        base_color_texture: Some(render_state.wall_handle.clone()),
        normal_map_texture: Some(render_state.wall_normal_handle.clone()),
        ..default()
    });
    let floor_material = materials.add(StandardMaterial {
        base_color_texture: Some(render_state.floor_handle.clone()),
        normal_map_texture: Some(render_state.floor_normal_handle.clone()),
        ..default()
    });

    // Spawn only 2 entities (one wall mesh, one floor mesh)
    commands.spawn((
        Mesh3d(meshes.add(wall_mesh)),
        MeshMaterial3d(wall_material),
        Transform::default(),
        MazeWall,
    ));
    commands.spawn((
        Mesh3d(meshes.add(floor_mesh)),
        MeshMaterial3d(floor_material),
        Transform::default(),
        MazeFloor,
    ));
}
