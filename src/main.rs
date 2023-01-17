mod constants;
mod config;
mod chunk;

use std::{collections::HashMap};

use bevy::{
    prelude::*,
    pbr::wireframe::{Wireframe, WireframePlugin},
    render::{render_resource::PrimitiveTopology, settings::{WgpuSettings, WgpuFeatures}},
    render::{mesh::Indices}, log::{LogPlugin, Level},
};
use bevy_flycam::{PlayerPlugin, MovementSettings};

use constants::*;
use chunk::*;

#[derive(Resource, Debug)]
struct GameState {
    seed: String,
    chunks: HashMap<[usize; 2], Chunk>
}

fn main() {
    App::new()
        .insert_resource(WgpuSettings {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..default()
        })
        .insert_resource(MovementSettings {
            sensitivity: 0.00012,
            speed: 48.0,
        })
        .add_plugins( DefaultPlugins.set(
            WindowPlugin {
                window: WindowDescriptor { 
                    width: 1280.,
                    height: 1024.,
                    ..default()
                },
                ..default()
            })
            .set(ImagePlugin::default_nearest())
            .set(AssetPlugin {
                watch_for_changes: false,
                ..default()
            })
            .set(LogPlugin {
                filter: "".to_string(),
                level: Level::ERROR,
            })
        )
        .add_plugin(WireframePlugin)
        .add_plugin(PlayerPlugin)
        .insert_resource(GameState {
            seed: "asdf".to_string(),
            chunks: HashMap::new(),

        })
        .add_startup_system(generate_world)
        // .add_startup_system(generate_cube_mesh)
        .add_startup_system(generate_mesh.after(generate_world))
        .add_startup_system(setup)
        .add_system(rotate)
        .run();
}

#[derive(Component)]
struct Rotate;

fn generate_cube_mesh(
    mut commands: Commands, 
    mut meshes: ResMut< Assets<Mesh> >, 
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::LIME_GREEN,
        unlit: true,
        ..default()
    });

    let mut mesh_generator = ChunkMeshGenerator::new();

    mesh_generator.add_face([0., 0., 0.], &FaceKind::Front, &BlockKind::GRASS);
    mesh_generator.add_face([0., 0., 0.], &FaceKind::Back, &BlockKind::GRASS);
    mesh_generator.add_face([0., 0., 0.], &FaceKind::Left, &BlockKind::GRASS);
    mesh_generator.add_face([0., 0., 0.], &FaceKind::Right, &BlockKind::GRASS);
    mesh_generator.add_face([0., 0., 0.], &FaceKind::Top, &BlockKind::GRASS);
    mesh_generator.add_face([0., 0., 0.], &FaceKind::Bottom, &BlockKind::GRASS);

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh_generator.build()),
            material: material.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                rotation: Quat::from_euler(EulerRot::XYZ, 0., 0., 0.),
                scale: Vec3::new(1., 1., 1.)
            },
            ..default()
        },
        Rotate,
    ));
}

fn generate_world(mut game_state: ResMut<GameState>) {
    for x in 0..16 {
        for z in 0..16 {
            let chunk = Chunk::generate(x, z);
            game_state.chunks.insert([x, z], chunk);
        }
    }
}

fn generate_mesh(
    mut commands: Commands,
    game_state: Res<GameState>, 
    asset_server: Res<AssetServer>, 
    mut meshes: ResMut< Assets<Mesh> >,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let atlas = asset_server.load("atlas.png");

    let material = materials.add(StandardMaterial {
        base_color_texture: Some(atlas),
        unlit: true,
        ..default()
    });

    for (position, chunk) in game_state.chunks.iter() {
        let mut mesh_generator = ChunkMeshGenerator::new();
        let chunk_x: f32 = (position[0] as u64 * CHUNK_WIDTH) as f32;
        let chunk_z: f32 = (position[1] as u64 * CHUNK_LENGTH) as f32;

        for (i, voxel) in chunk.voxels.iter().enumerate() {
            let (x, y, z)  = Chunk::get_local_coord(i as u64);

            if voxel.kind == BlockKind::AIR {
                continue;
            }

            for i in 0..FACES.len() {
                let face = &FACES[i];
                let dir = [
                    x + face.normal[Vector::X] as f64,
                    y + face.normal[Vector::Y] as f64,
                    z + face.normal[Vector::Z] as f64,
                ];

                if !OPTIMIZED_MESH {
                    mesh_generator.add_face([x as f32, y as f32, z as f32], &face.kind, &voxel.kind);
                    break;
                }

                let neighbor = chunk.get_voxel((dir[Vector::X], dir[Vector::Y] as f64, dir[Vector::Z]));
                match neighbor {
                    Some(block) => {
                        match block.kind {
                            BlockKind::AIR => {
                                mesh_generator.add_face([x as f32, y as f32, z as f32], &face.kind, &voxel.kind);
                                continue;
                            },
                            _ => {}
                        }
                    },
                    None => {
                        mesh_generator.add_face([x as f32, y as f32, z as f32], &face.kind, &voxel.kind);
                        continue;
                    },
                }

            }
        }

        println!("FACE COUNT: {}", mesh_generator.face_count);
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(mesh_generator.build()),
                material: material.clone(),
                transform: Transform {
                    translation: Vec3::new(chunk_x - CHUNK_LENGTH as f32 / 2., 0., chunk_z - CHUNK_WIDTH as f32),
                    rotation: Quat::from_euler(EulerRot::XYZ, 0., 0., 0.),
                    scale: Vec3::new(1., 1., 1.)
                },
                ..default()
            },
        ));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 26.0, 26.0),
        ..default()
    });
}

fn rotate(mut q: Query<(&Rotate, &mut Transform)>, timer: Res<Time>) {
    for (_, mut transform) in &mut q {
        transform.rotate_y(0.5 * timer.delta_seconds());

    }
}