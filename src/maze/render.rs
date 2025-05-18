use bevy::prelude::*;

use super::level::MazeLevel;

pub struct MazeRenderPlugin;

impl Plugin for MazeRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, create_array_texture);
    }
}

#[derive(Resource)]
struct LoadingTexture {
    is_loaded: bool,
    wall_handle: Handle<Image>,
    wall_normal_handle: Handle<Image>,
    floor_handle: Handle<Image>,
    floor_normal_handle: Handle<Image>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Start loading the textures and normal maps.
    commands.insert_resource(LoadingTexture {
        is_loaded: false,
        wall_handle: asset_server.load("textures/wall.png"),
        wall_normal_handle: asset_server.load("textures/wall_normal.png"),
        floor_handle: asset_server.load("textures/floor.png"),
        floor_normal_handle: asset_server.load("textures/floor_normal.png"),
    });
}

fn create_array_texture(
    mut commands: Commands,
    level: Res<MazeLevel>,
    mut loading_texture: ResMut<LoadingTexture>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if loading_texture.is_loaded {
        return;
    }
    loading_texture.is_loaded = true;

    let cube_handle = meshes.add(Cuboid {
        half_size: Vec3::splat(0.5),
    });
    let wall_material = materials.add(StandardMaterial {
        base_color_texture: Some(loading_texture.wall_handle.clone()),
        normal_map_texture: Some(loading_texture.wall_normal_handle.clone()),
        ..default()
    });

    let plane_handle = meshes.add(Plane3d::default().mesh().size(1.0, 1.0));
    let floor_material = materials.add(StandardMaterial {
        base_color_texture: Some(loading_texture.floor_handle.clone()),
        normal_map_texture: Some(loading_texture.floor_normal_handle.clone()),
        ..default()
    });

    for (z, s) in level.map.iter().enumerate() {
        for (x, &c) in s.iter().enumerate() {
            if c == '#' {
                commands.spawn((
                    Mesh3d(cube_handle.clone()),
                    MeshMaterial3d(wall_material.clone()),
                    Transform::from_xyz(x as f32 + 0.5, 0.5, z as f32 + 0.5),
                ));
            } else {
                commands.spawn((
                    Mesh3d(plane_handle.clone()),
                    MeshMaterial3d(floor_material.clone()),
                    Transform::from_xyz(x as f32 + 0.5, 0.0, z as f32 + 0.5),
                ));
            }
        }
    }
}
