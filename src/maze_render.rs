use bevy::prelude::*;

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let map = vec![
        "####################",
        "#                  #",
        "#  ##### ########  #",
        "#      #      #    #",
        "#  ########## ###  #",
        "#        ####      #",
        "#  ##### ########  #",
        "#      #      #    #",
        "#  ########## ###  #",
        "#    ###           #",
        "#  ##### ########  #",
        "#      #      #    #",
        "#  ########## ###  #",
        "#     #####        #",
        "#                  #",
        "#  ##### ########  #",
        "#    ###      #    #",
        "#  ########## ###  #",
        "#                  #",
        "####################",
    ];

    for (z, &s) in map.iter().enumerate() {
        for (x, c) in s.chars().enumerate() {
            if c == '#' {
                commands.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box {
                        min_x: x as f32,
                        max_x: x as f32 + 1.0,
                        min_y: 0.0,
                        max_y: 1.0,
                        min_z: z as f32,
                        max_z: z as f32 + 1.0,
                    })),
                    material: materials.add(StandardMaterial {
                        base_color: Color::OLIVE,
                        perceptual_roughness: 1.0,
                        ..default()
                    }),
                    ..default()
                });
            }
        }
    }
    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 20.0,
            subdivisions: 0,
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::DARK_GREEN,
            perceptual_roughness: 1.0,
            ..default()
        }),
        transform: Transform::from_translation(Vec3::new(10.0, 0.0, 10.0)),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(17.0, 25.0, 35.0)
            .looking_at(Vec3::new(10.0, 0.0, 10.0), Vec3::Y),
        ..default()
    });

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.02,
    });

    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });
}
