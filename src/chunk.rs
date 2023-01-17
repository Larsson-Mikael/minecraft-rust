use bevy::{prelude::Mesh, render::{render_resource::PrimitiveTopology, mesh::Indices}};
use crate::constants::*;
use noise::{ Perlin, NoiseFn };

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
        let x_offset = (CHUNK_LENGTH / 2) as f32;
        let z_offset = (CHUNK_WIDTH / 2) as f32;

        for vert in &face.vertices  {
            let x = vert.position[Vector::X] + coord[Vector::X] - x_offset; 
            let y = vert.position[Vector::Y] + coord[Vector::Y];
            let z = vert.position[Vector::Z] + coord[Vector::Z] - z_offset;

            let u = ((vert.uv[0] + tex_coord[0]) * ATLAS_OFFSET / ATLAS_WIDTH) as f32;
            let v = ((vert.uv[1] + tex_coord[1]) * ATLAS_OFFSET / ATLAS_HEIGHT) as f32;

            self.vertices.push([x, y, z]);
            self.normals.push(face.normal);
            self.uvs.push([u, v]);
        }

        let offset = 4 * self.face_count;
        self.indicies.extend(TRIANGLES.iter().map(|x| x + offset));
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




#[derive(Debug)]
pub struct Chunk {
    pub voxels: Vec<Block>,
}

impl Chunk {
    pub fn new() -> Self {

        let size = CHUNK_WIDTH * CHUNK_LENGTH * CHUNK_HEIGHT;
        let voxels = (0..size).map(|_| {
            Block {
                kind: BlockKind::GRASS,
                is_placed: false,
            }
        })
        .collect();
        Chunk {
            voxels
        }
    }

    pub fn generate(&mut self, chunk_x: usize, chunk_z: usize) -> Vec<Block> {
        let mut voxels = Vec::new();
        let perlin = Perlin::new(123);

        for (i, _) in self.voxels.iter().enumerate() {
            let (x, y, z) = Self::get_local_coord(i as u64);

            let scale: f64 = 0.009;
            let n_x = (x * chunk_x as f64) * scale;
            let n_z = (z *  chunk_z as f64) * scale;
            let height = perlin.get([n_x, n_z]) * CHUNK_HEIGHT as f64;

            if y < height * 0.5 {
                voxels.push(Block {
                    kind: BlockKind::GRASS,
                    is_placed: false,
                });
            } else {
                voxels.push(Block {
                    kind: BlockKind::AIR,
                    is_placed: false,
                });
            }
        }

        voxels
    }

    pub fn get_index(coord: (f64, f64, f64)) -> usize {
        let x_max: f64 = CHUNK_WIDTH as f64;
        let z_max: f64 = CHUNK_LENGTH  as f64;
        let (x, y, z) = coord;

        ((y * x_max * z_max) + (z * x_max) + x) as usize
    }

    pub fn get_local_coord(index: u64) -> (f64, f64, f64) {
        let y = index as u64 / (CHUNK_WIDTH * CHUNK_LENGTH);
        let i = index - y  * (CHUNK_WIDTH * CHUNK_LENGTH);
        let x = i as u64 % CHUNK_WIDTH;
        let z = i as u64 / CHUNK_LENGTH;

        (x as f64, y as f64, z as f64)
    }

    pub fn get_voxel(&self, coord: (f64, f64, f64)) -> Option<&Block> {
        let (x, y, z) = coord;

        if x >= CHUNK_WIDTH as f64 || 
        y >= CHUNK_HEIGHT as f64 || 
        z >= CHUNK_LENGTH as f64 || 
        x < 0. || 
        y < 0. || 
        z < 0. {
            return None
        }

        let index = Self::get_index(coord);
        Some(&self.voxels[index])
    }
}

#[derive(Debug)]
pub struct Block {
    pub kind: BlockKind,
    pub is_placed: bool,
}

impl Block {
    fn new(kind: BlockKind, is_placed: bool) -> Self {
        Self {kind, is_placed}
    }
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