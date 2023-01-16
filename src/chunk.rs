use std::ops::Add;

use bevy::{prelude::Mesh, render::{render_resource::PrimitiveTopology, mesh::Indices}};
use crate::constants::*;
use noise::{
    core::perlin::{perlin_2d, perlin_3d}, 
    permutationtable::PermutationTable, 
    core::simplex::simplex_3d,
};

pub struct ChunkGenerator {
    position: [f64; 3],
    heightmap: Vec<u64>,
}

pub trait MeshBuilder {

    fn add_face(&mut self, coord: [f32; 3], face_kind: &FaceKind, block_kind: &BlockKind);
    fn build(self) -> Mesh;
}

#[derive(Default)]
pub struct ChunkMeshGenerator {
    pub face_count: u32,
    vertices: Vec<[f32; 3]>,
    indicies: Vec<u32>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
}

impl ChunkMeshGenerator {
    pub fn new() -> Self {
        Self::default()

    }
}

impl MeshBuilder for ChunkMeshGenerator {
    fn add_face(&mut self, coord: [f32; 3], face_kind: &FaceKind, block_kind: &BlockKind) {

        let face: &Face = &FACES[face_kind];
        let tex_coord = block_kind.get_tex_coord(&face.kind);

        for vert in &face.vertices  {
            let x_offset = (CHUNK_WIDTH/ 2) as f32;
            let z_offset = (CHUNK_LENGTH / 2) as f32;
            let x = vert.position[Vector::X] + coord[Vector::X] - x_offset; 
            let y = vert.position[Vector::Y] + coord[Vector::Y];
            let z = vert.position[Vector::Z] + coord[Vector::Z] - z_offset;

            let u = ((vert.uv[0] + tex_coord[0]) * ATLAS_OFFSET / ATLAS_WIDTH) as f32;
            let v = ((vert.uv[1] + tex_coord[1]) * ATLAS_OFFSET / ATLAS_HEIGHT) as f32;

            self.vertices.push([x, y, z]);
            self.normals.push(face.normal);
            self.uvs.push([u, v]);
        }

        let mut arr=TRIANGLES.clone();
        self.indicies.extend_from_slice({
            for i in &mut arr {
                *i += 4 * self.face_count;
            }
            &arr
        });

        self.face_count += 1;
    }

    fn build(self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs);
        mesh.set_indices(Some(Indices::U32(self.indicies)));

        mesh

    }
}

impl ChunkGenerator {
    pub fn new() -> Self {
        ChunkGenerator {
            position: [0., 0., 0.],
            heightmap: vec![]
        }
    }

    pub fn generate(&self, chunk_x: u64, chunk_z: u64) -> BlockArray {
        let mut blocks = Vec::new();
        let hasher = PermutationTable::new(0);


        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_WIDTH {
                for x in 0..CHUNK_LENGTH {
                    let n_x = (x * chunk_x) as f64 / CHUNK_LENGTH as f64;
                    let n_y = y as f64 / CHUNK_HEIGHT as f64;
                    let n_z = (z * chunk_z) as f64  / CHUNK_WIDTH as f64;
                    let mut height = perlin_2d([n_x, n_z], &hasher) * CHUNK_HEIGHT as f64;
                    noise::Perlin
                    let val = perlin_3d([n_x, n_y, n_z], &hasher);

                    if val > 0.1 {
                        blocks.push(Block {
                            kind: BlockKind::GRASS,
                            is_placed: false,
                        });
                    } else {
                        blocks.push(Block {
                            kind: BlockKind::AIR,
                            is_placed: false,
                        });
                    }

                }
            }
        }

        BlockArray {blocks, current: 0}
    }
}

#[derive(Debug)]
pub struct BlockArray {
    pub blocks: Vec<Block>,
    current: usize,
}

impl BlockArray {
    pub fn get_block(&self, x: f64, y: f64, z: f64) -> Option<&Block> {
        if x >= CHUNK_LENGTH as f64 || 
        y >= CHUNK_HEIGHT as f64 || 
        z >= CHUNK_WIDTH as f64 || 
        x < 0. || 
        y < 0. || 
        z < 0. {
            return None
        }

        let index = Self::to_1d(x, y, z);
        Some(&self.blocks[index])
    }

    pub fn set_block(&mut self, x: f64, y: f64, z: f64, block: Block) {
        let index = Self::to_1d(x, y, z);
        self.blocks[index] = block;
    }

    fn to_1d(x: f64, y: f64, z: f64) -> usize {
        let x_max: f64 = CHUNK_LENGTH as f64;
        let z_max: f64 = CHUNK_WIDTH as f64;
        ((y * x_max * z_max) + (z * x_max) + x) as usize
    }
}

#[derive(Debug)]
pub struct Block {
    pub kind: BlockKind,
    pub is_placed: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BlockKind {
    AIR,
    GRASS,
}

impl BlockKind {
    pub fn get_tex_coord(&self, face: &FaceKind ) -> [f32; 2] {
        match self {
            BlockKind::GRASS => {
                match face {
                    FaceKind::Left => [1., 10.],
                    FaceKind::Right => [1., 10.],
                    FaceKind::Front => [1., 10.],
                    FaceKind::Back => [1., 10.],
                    FaceKind::Top => [14., 10.],
                    FaceKind::Bottom => [2., 5.],
                }
            },
            _ => [22., 0.] 
        }
    }
}

pub struct BlockInfo {
    top: [u64; 2],
    sides: [u64; 2],
    bottom: [u64; 2],
}