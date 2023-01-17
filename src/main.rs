mod constants;
mod config;
mod utils;
mod chunk;

use std::{collections::HashMap, f32::consts::PI, ops::Add};

use bevy::{
    prelude::*,
    window::WindowResized, 
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    render::{render_resource::PrimitiveTopology, settings::{WgpuSettings, WgpuFeatures}},
    render::{*, mesh::Indices}, log::{LogPlugin, Level},
};
use bevy_flycam::{PlayerPlugin, FlyCam, MovementSettings};

use constants::*;
use utils::*;
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
        .add_system(update_up_down)
        .add_system(rotate)
        .run();
}

#[derive(Component)]
struct Rotate;

// fn load_resources(commands: Commands, asset_server: Res<AssetServer>) {
//     let atlas = asset_server.load("atlas.png");
//     commands.insert_resource(atlas);
// }

fn build_cube_mesh() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let mut triangles: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = vec![];
    let mut indicies: Vec<u32> = Vec::new();

    for (i, face) in FACES.iter().enumerate() {
        let u = 0.;
        let v = 0.;

        for vert in &face.vertices{
            let v_x = vert.position[Vector::X] as f32;
            let v_y = vert.position[Vector::Y] as f32;
            let v_z = vert.position[Vector::Z] as f32;

            triangles.push([v_x, v_y, v_z]);
            normals.push(face.normal);
            uvs.push([
                (vert.uv[0] + u) * ATLAS_OFFSET / ATLAS_WIDTH,
                (vert.uv[1] + v) * ATLAS_OFFSET / ATLAS_HEIGHT
            ])
        }
    }


    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, triangles);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indicies)));

    mesh
}

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
        Wireframe
    ));



}


fn generate_world(mut game_state: ResMut<GameState>) {
    for x in 0..4 {
        for z in 0..4 {
            let mut chunk = Chunk::new();
            let voxels = chunk.generate(x, z);
            chunk.voxels = voxels;
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
            Wireframe,
        ));
    }

}




#[derive(Component)]
struct UpDown;


fn setup(mut commands: Commands) {
    let generator = ChunkMeshGenerator::new();

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

fn update_up_down(mut q: Query<&mut Transform, With<UpDown>>, time: Res<Time>) {
    let y = time.elapsed_seconds().sin() * 20.;
    for mut t in &mut q {
        t.look_at(Vec3::new(0., 0., 0.), Vec3::new(0., 1., 0.));
        t.translation = Vec3::new(-30., y, 50.)
    }
 }

fn rotate(mut q: Query<(&Rotate, &mut Transform)>, timer: Res<Time>) {
    for (_, mut transform) in &mut q {
        transform.rotate_y(0.5 * timer.delta_seconds());

    }
}