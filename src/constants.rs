use std::ops::{Index, self};

impl Index<&FaceKind> for [Face; 6] {
    type Output = Face;

    fn index(&self, index: &FaceKind) -> &Self::Output {
        let i = index.clone() as usize;
        &self[i]
    }
}

pub enum VertexIndex {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}

impl Index<VertexIndex> for [Vertex; 4] {
    type Output = Vertex;

    fn index(&self, index: VertexIndex) -> &Self::Output {
        &self[index as usize]
    }
}


pub enum Vector {
    X,
    Y,
    Z
}

impl Index<Vector> for [f32; 3] {
    type Output = f32;

    fn index(&self, index: Vector) -> &Self::Output {
        &self[index as usize]
    }
}

impl Index<Vector> for [f64; 3] {
    type Output = f64;

    fn index(&self, index: Vector) -> &Self::Output {
        &self[index as usize]
    }
}

impl Index<Vector> for [u8; 3] {
    type Output = u8;

    fn index(&self, index: Vector) -> &Self::Output {
        &self[index as usize]
    }
}

impl Index<Vector> for [u32; 3] {
    type Output = u32;

    fn index(&self, index: Vector) -> &Self::Output {
        &self[index as usize]
    }
}

impl Index<Vector> for [u64; 3] {
    type Output = u64;

    fn index(&self, index: Vector) -> &Self::Output {
        &self[index as usize]
    }
}

pub const TRIANGLES: [u32; 6] = [0, 1, 2, 2, 1, 3];

const V_MIN: f32 = -0.5;
const V_MAX: f32 = 0.5;

pub const FACES: [Face; 6] = [
    Face {
        kind: FaceKind::Left,
        normal: [-1., 0., 0.],
        vertices: [
            Vertex {position: [-0.5, -0.5, -0.5], uv: [0., 1.]}, // bl
            Vertex {position: [-0.5, -0.5, 0.5], uv: [1., 1.]}, // br
            Vertex {position: [-0.5, 0.5, -0.5], uv: [0., 0.]}, // tl
            Vertex {position: [-0.5, 0.5, 0.5], uv: [1., 0.]}, // tr
        ]
    },
    Face {
        kind: FaceKind::Right,
        normal: [1., 0., 0.],
        vertices: [
            Vertex {position: [0.5, -0.5, 0.5], uv: [0., 1.]}, // bl
            Vertex {position: [0.5, -0.5, -0.5], uv: [1., 1.]}, // br
            Vertex {position: [0.5, 0.5, 0.5], uv: [0., 0.]}, // tl
            Vertex {position: [0.5, 0.5, -0.5], uv: [1., 0.]}, // tr
        ]
    },
    Face {
        kind: FaceKind::Front,
        normal: [0., 0., 1.],
        vertices: [
            Vertex {position: [-0.5, -0.5, 0.5], uv: [0., 1.]}, // bl
            Vertex {position: [0.5, -0.5, 0.5], uv: [1., 1.]}, // br
            Vertex {position: [-0.5, 0.5, 0.5], uv: [0., 0.]}, // tl
            Vertex {position: [0.5, 0.5, 0.5], uv: [1., 0.]}, // tr
        ]
    },
    Face {
        kind: FaceKind::Back,
        normal: [0., 0., -1.],
        vertices: [
            Vertex {position: [0.5, -0.5, -0.5], uv: [0., 1.]}, // bl
            Vertex {position: [-0.5, -0.5, -0.5], uv: [1., 1.]}, // br
            Vertex {position: [0.5, 0.5, -0.5], uv: [0., 0.]}, // tl
            Vertex {position: [-0.5, 0.5, -0.5], uv: [1., 0.]}, // tr
        ]
    },
    Face {
        kind: FaceKind::Top,
        normal: [0., 1., 0.],
        vertices: [
            Vertex {position: [-0.5, 0.5, 0.5], uv: [0., 1.]}, // bl
            Vertex {position: [0.5, 0.5, 0.5], uv: [1., 1.]}, // br
            Vertex {position: [-0.5, 0.5, -0.5], uv: [0., 0.]}, // tl
            Vertex {position: [0.5, 0.5, -0.5], uv: [1., 0.]}, // tr
        ]
    },
    Face {
        kind: FaceKind::Bottom,
        normal: [0., -1., 0.],
        vertices: [
            Vertex {position: [-0.5, -0.5, -0.5], uv: [0., 1.]}, // bl
            Vertex {position: [0.5, -0.5, -0.5], uv: [1., 1.]}, // br
            Vertex {position: [-0.5, -0.5, 0.5], uv: [0., 0.]}, // tl
            Vertex {position: [0.5, -0.5, 0.5], uv: [1., 0.]}, // tr
        ]
    },
];

pub const ATLAS_WIDTH: f32 = 1024.;
pub const ATLAS_HEIGHT: f32 = 1024.;
pub const ATLAS_OFFSET: f32 = 32.;

pub const CHUNK_LENGTH: u64 = 64;
pub const CHUNK_WIDTH: u64 = 64;
pub const CHUNK_HEIGHT: u64 = 256;
pub const CHUNK_MAX_BLOCK: u64 = CHUNK_WIDTH * CHUNK_HEIGHT * CHUNK_LENGTH;


pub const OPTIMIZED_MESH: bool = true;


pub struct Face {
    pub kind: FaceKind,
    pub normal: [f32; 3],
    pub vertices: [Vertex; 4]
}

pub struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],

}

#[derive(Clone)]
pub enum FaceKind {
    Left,
    Right,
    Front,
    Back,
    Top,
    Bottom
}

impl FaceKind {
    pub fn value(&self) -> usize {
        match self {
            Self::Left => 0,
            Self::Right => 1,
            Self::Front=> 2,
            Self::Back => 3,
            Self::Top => 4,
            Self::Bottom => 5,
        }
    }

    pub fn get_face(index: usize) -> FaceKind {
        match index {
            0 => Self::Left,
            1 => Self::Right,
            2 => Self::Front,
            3 => Self::Back,
            4 => Self::Top,
            5 => Self::Bottom,
            _ => Self::Bottom
        }
    }
}