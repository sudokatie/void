//! Void dimension world generator.
//!
//! The Void dimension is a cold, dark, low-gravity dimension with
//! floating platforms and empty expanses. Base temperature is -40C.

use glam::IVec3;
use serde::{Deserialize, Serialize};

use super::ChunkData;

/// Base temperature for the Void dimension in Celsius.
pub const BASE_TEMP: f32 = -40.0;

/// Biomes available in the Void dimension.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VoidBiome {
    /// Stable bedrock foundation platforms.
    BedrockPlatform,
    /// Floating masses of rock and resources.
    FloatingIsland,
    /// Dangerous edges where reality frays.
    EdgeZone,
    /// Empty void space with no terrain.
    VoidExpanse,
}

impl VoidBiome {
    /// Get temperature modifier for this biome.
    #[must_use]
    pub fn temperature_modifier(&self) -> f32 {
        match self {
            VoidBiome::BedrockPlatform => 10.0,
            VoidBiome::FloatingIsland => 5.0,
            VoidBiome::EdgeZone => -10.0,
            VoidBiome::VoidExpanse => -20.0,
        }
    }

    /// Get typical resources for this biome.
    #[must_use]
    pub fn typical_resources(&self) -> Vec<String> {
        match self {
            VoidBiome::BedrockPlatform => vec![
                "void_stone".to_string(),
                "dark_iron".to_string(),
                "shadow_moss".to_string(),
            ],
            VoidBiome::FloatingIsland => vec![
                "float_crystal".to_string(),
                "void_ore".to_string(),
                "ether_bloom".to_string(),
            ],
            VoidBiome::EdgeZone => vec![
                "edge_shard".to_string(),
                "reality_fragment".to_string(),
            ],
            VoidBiome::VoidExpanse => vec![],
        }
    }

    /// Check if this biome is traversable (has solid ground).
    #[must_use]
    pub fn is_traversable(&self) -> bool {
        !matches!(self, VoidBiome::VoidExpanse)
    }
}

/// World generator for the Void dimension.
#[derive(Clone, Debug)]
pub struct VoidGenerator {
    /// Seed for deterministic generation.
    seed: u64,
}

impl VoidGenerator {
    /// Create a new Void dimension generator.
    #[must_use]
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    /// Get the generator seed.
    #[must_use]
    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// Generate chunk data for a given position.
    #[must_use]
    pub fn generate_chunk(&self, pos: IVec3) -> ChunkData {
        let biome = self.select_biome(pos);
        let temperature = self.calculate_temperature(pos, biome);
        let resources = self.generate_resources(pos, biome);

        ChunkData::new(pos, format!("{:?}", biome), temperature, resources)
    }

    /// Select biome based on position and seed.
    fn select_biome(&self, pos: IVec3) -> VoidBiome {
        let hash = self.hash_position(pos);

        // Bottom of the void is bedrock
        if pos.y < 10 {
            return VoidBiome::BedrockPlatform;
        }

        // Very high or low positions are void expanse
        if pos.y > 200 || (pos.y > 100 && hash % 3 == 0) {
            return VoidBiome::VoidExpanse;
        }

        // Calculate distance from origin for edge detection
        let dist_sq = (pos.x * pos.x + pos.z * pos.z) as u64;
        if dist_sq > 10000 && hash % 5 < 2 {
            return VoidBiome::EdgeZone;
        }

        // Distribution based on noise
        match hash % 100 {
            0..=20 => VoidBiome::BedrockPlatform,
            21..=55 => VoidBiome::FloatingIsland,
            56..=70 => VoidBiome::EdgeZone,
            _ => VoidBiome::VoidExpanse,
        }
    }

    /// Calculate temperature based on position and biome.
    fn calculate_temperature(&self, pos: IVec3, biome: VoidBiome) -> f32 {
        let modifier = biome.temperature_modifier();
        // Altitude slightly affects temperature (colder higher up)
        let altitude_modifier = -((pos.y - 64).max(0) as f32 * 0.05);
        // Slight variation from hash
        let noise = (self.hash_position(pos) % 100) as f32 * 0.08 - 4.0;

        BASE_TEMP + modifier + altitude_modifier + noise
    }

    /// Generate resources for a chunk.
    fn generate_resources(&self, pos: IVec3, biome: VoidBiome) -> Vec<String> {
        let mut resources = biome.typical_resources();
        let hash = self.hash_position(pos);

        // Rare resources in Void
        if hash % 40 == 0 {
            resources.push("void_heart".to_string());
        }
        if hash % 100 == 0 {
            resources.push("reality_anchor".to_string());
        }

        resources
    }

    /// Hash a position for deterministic pseudo-random values.
    fn hash_position(&self, pos: IVec3) -> u64 {
        let x = pos.x as u64;
        let y = pos.y as u64;
        let z = pos.z as u64;

        let mut hash = self.seed.wrapping_add(0xcafe_babe);
        hash = hash.wrapping_mul(41).wrapping_add(x);
        hash = hash.wrapping_mul(41).wrapping_add(y);
        hash = hash.wrapping_mul(41).wrapping_add(z);
        hash ^= hash >> 19;
        hash = hash.wrapping_mul(0xa136_aaad);
        hash ^= hash >> 17;
        hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_void_biome_temperature() {
        assert!(VoidBiome::BedrockPlatform.temperature_modifier() > VoidBiome::VoidExpanse.temperature_modifier());
        assert!(VoidBiome::EdgeZone.temperature_modifier() < 0.0);
    }

    #[test]
    fn test_void_biome_resources() {
        let island_resources = VoidBiome::FloatingIsland.typical_resources();
        assert!(island_resources.contains(&"float_crystal".to_string()));

        let expanse_resources = VoidBiome::VoidExpanse.typical_resources();
        assert!(expanse_resources.is_empty());
    }

    #[test]
    fn test_void_biome_traversable() {
        assert!(VoidBiome::BedrockPlatform.is_traversable());
        assert!(VoidBiome::FloatingIsland.is_traversable());
        assert!(!VoidBiome::VoidExpanse.is_traversable());
    }

    #[test]
    fn test_void_generator_new() {
        let generator = VoidGenerator::new(11111);
        assert_eq!(generator.seed(), 11111);
    }

    #[test]
    fn test_void_generator_deterministic() {
        let gen1 = VoidGenerator::new(777);
        let gen2 = VoidGenerator::new(777);

        let pos = IVec3::new(50, 64, 50);
        let chunk1 = gen1.generate_chunk(pos);
        let chunk2 = gen2.generate_chunk(pos);

        assert_eq!(chunk1.biome, chunk2.biome);
        assert!((chunk1.temperature - chunk2.temperature).abs() < f32::EPSILON);
    }

    #[test]
    fn test_base_temperature_is_cold() {
        let generator = VoidGenerator::new(1);
        let chunk = generator.generate_chunk(IVec3::new(0, 64, 0));

        // Temperature should be around or below BASE_TEMP
        assert!(chunk.temperature <= BASE_TEMP + 20.0);
    }

    #[test]
    fn test_bottom_is_bedrock() {
        let generator = VoidGenerator::new(1);
        let chunk = generator.generate_chunk(IVec3::new(0, 5, 0));

        assert_eq!(chunk.biome, "BedrockPlatform");
    }
}
