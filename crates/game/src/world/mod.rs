//! World generation systems.

mod void_gen;

pub use void_gen::{VoidBiome, VoidGenerator};

/// Data for a generated chunk.
#[derive(Clone, Debug)]
pub struct ChunkData {
    pub pos: glam::IVec3,
    pub biome_name: String,
    pub temperature: f32,
    pub resources: Vec<String>,
}

impl ChunkData {
    pub fn new(pos: glam::IVec3, biome_name: String, temperature: f32, resources: Vec<String>) -> Self {
        Self { pos, biome_name, temperature, resources }
    }
}
