use core::panic;

use bevy::{prelude::*, render::render_resource::PrimitiveTopology, render::mesh::Indices};
use crate::{constants::*, chunk::{BlockArray, BlockKind, ChunkMeshGenerator, MeshBuilder}};