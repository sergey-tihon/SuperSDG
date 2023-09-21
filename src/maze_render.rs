use bevy::{
    asset::LoadState,
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
};

pub struct MazePlugin;

impl Plugin for MazePlugin {
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

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(17.0, 15.0, 30.0)
            .looking_at(Vec3::new(10.0, 0.0, 10.0), Vec3::Y),
        ..default()
    });

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.01,
    });

    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 50_000.,
            color: Color::Rgba {
                red: 248. / 255.,
                green: 244. / 255.,
                blue: 234. / 255.,
                alpha: 1.0,
            },
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_8),
            ..default()
        },
        ..default()
    });
}

fn create_array_texture(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_texture: ResMut<LoadingTexture>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut texture_materials: ResMut<Assets<TextureMaterial>>,
) {
    if loading_texture.is_loaded
        || asset_server.get_load_state(loading_texture.wall_handle.clone()) != LoadState::Loaded
        || asset_server.get_load_state(loading_texture.floor_handle.clone()) != LoadState::Loaded
    {
        return;
    }
    loading_texture.is_loaded = true;

    let map = vec![
        "####################",
        "#                  #",
        "#  ##### ######### #",
        "#      #      #    #",
        "#  ########## ###  #",
        "#        ####      #",
        "#  ##### ######### #",
        "#      #      #    #",
        "#  ########## ###  #",
        "#    ###           #",
        "#  ##### ######### #",
        "#      #      #  # #",
        "#  ########## #### #",
        "#     #####        #",
        "#                # #",
        "#  ##### ######### #",
        "#    ###      #    #",
        "#  ########## ###  #",
        "#                  #",
        "####################",
    ];

    let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let wall_material_handle = texture_materials.add(TextureMaterial {
        texture: loading_texture.wall_handle.clone(),
    });

    let plane_handle = meshes.add(Mesh::from(shape::Plane {
        size: 1.0,
        subdivisions: 0,
    }));
    let floor_material_handle = texture_materials.add(TextureMaterial {
        texture: loading_texture.floor_handle.clone(),
    });

    for (z, &s) in map.iter().enumerate() {
        for (x, c) in s.chars().enumerate() {
            if c == '#' {
                commands.spawn(MaterialMeshBundle {
                    mesh: cube_handle.clone(),
                    material: wall_material_handle.clone(),
                    transform: Transform::from_xyz(x as f32 + 0.5, 0.5, z as f32 + 0.5),
                    ..default()
                });
            } else {
                commands.spawn(MaterialMeshBundle {
                    mesh: plane_handle.clone(),
                    material: floor_material_handle.clone(),
                    transform: Transform::from_xyz(x as f32 + 0.5, 0.0, z as f32 + 0.5),
                    ..default()
                });
            }
        }
    }
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid, TypePath)]
#[uuid = "9c5a0ddf-1eaf-41b4-9832-ed736fd26af3"]
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
