use bevy::{
    asset::LoadState,
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};

use super::level::MazeLevel;

pub struct MazeRenderPlugin;

impl Plugin for MazeRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<TextureMaterial>::default())
            .add_systems(Startup, setup)
            .add_systems(Update, create_array_texture);
    }
}

#[derive(Resource)]
struct LoadingTexture {
    is_loaded: bool,
    wall_handle: Handle<Image>,
    floor_handle: Handle<Image>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Start loading the texture.
    commands.insert_resource(LoadingTexture {
        is_loaded: false,
        wall_handle: asset_server.load("textures/wall.png"),
        floor_handle: asset_server.load("textures/floor.png"),
    });
}

fn create_array_texture(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level: Res<MazeLevel>,
    mut loading_texture: ResMut<LoadingTexture>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut texture_materials: ResMut<Assets<TextureMaterial>>,
) {
    if loading_texture.is_loaded
        || asset_server.load_state(loading_texture.wall_handle.clone()) != LoadState::Loaded
        || asset_server.load_state(loading_texture.floor_handle.clone()) != LoadState::Loaded
    {
        return;
    }
    loading_texture.is_loaded = true;

    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let wall_material = texture_materials.add(TextureMaterial {
        texture: loading_texture.wall_handle.clone(),
    });

    let plane_handle = meshes.add(Mesh::from(shape::Plane {
        size: 1.0,
        subdivisions: 0,
    }));
    let floor_material = texture_materials.add(TextureMaterial {
        texture: loading_texture.floor_handle.clone(),
    });

    for (z, s) in level.map.iter().enumerate() {
        for (x, &c) in s.iter().enumerate() {
            if c == '#' {
                commands.spawn(MaterialMeshBundle {
                    mesh: cube_handle.clone(),
                    material: wall_material.clone(),
                    transform: Transform::from_xyz(x as f32 + 0.5, 0.5, z as f32 + 0.5),
                    ..default()
                });
            } else {
                commands.spawn(MaterialMeshBundle {
                    mesh: plane_handle.clone(),
                    material: floor_material.clone(),
                    transform: Transform::from_xyz(x as f32 + 0.5, 0.0, z as f32 + 0.5),
                    ..default()
                });
            }
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct TextureMaterial {
    #[texture(0, dimension = "2d")]
    #[sampler(1)]
    texture: Handle<Image>,
}

impl Material for TextureMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/texture.wgsl".into()
    }
}
