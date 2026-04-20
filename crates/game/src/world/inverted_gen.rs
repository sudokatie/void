//! Inverted dimension world generator.
//!
//! The Inverted dimension is a volcanic, high-temperature mirror
//! dimension with altered physics. Base temperature is 60C.

use glam::IVec3;
use serde::{Deserialize, Serialize};

use super::ChunkData;

/// Base temperature for the Inverted dimension in Celsius.
pub const BASE_TEMP: f32 = 60.0;

/// Biomes available in the Inverted dimension.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InvertedBiome {
    /// Lakes of molten lava.
    LavaLakes,
    /// Tall obsidian formations.
    ObsidianSpires,
    /// Underground crystal formations.
    CrystalCaves,
    /// Flat volcanic terrain with ash.
    VolcanicPlains,
}

impl InvertedBiome {
    /// Get temperature modifier for this biome.
    #[must_use]
    pub fn temperature_modifier(&self) -> f32 {
        match self {
            InvertedBiome::LavaLakes => 40.0,
            InvertedBiome::ObsidianSpires => 10.0,
            InvertedBiome::CrystalCaves => -15.0,
            InvertedBiome::VolcanicPlains => 5.0,
        }
    }

    /// Get typical resources for this biome.
    #[must_use]
    pub fn typical_resources(&self) -> Vec<String> {
        match self {
            InvertedBiome::LavaLakes => vec![
                "magma_stone".to_string(),
                "fire_essence".to_string(),
            ],
            InvertedBiome::ObsidianSpires => vec![
                "obsidian".to_string(),
                "volcanic_glass".to_string(),
                "basite".to_string(),
            ],
            InvertedBiome::CrystalCaves => vec![
                "heat_crystal".to_string(),
                "ember_gem".to_string(),
                "thermal_ore".to_string(),
            ],
            InvertedBiome::VolcanicPlains => vec![
                "ash".to_string(),
                "sulfite".to_string(),
                "charite".to_string(),
            ],
        }
    }
}

/// World generator for the Inverted dimension.
#[derive(Clone, Debug)]
pub struct InvertedGenerator {
    /// Seed for deterministic generation.
    seed: u64,
}

impl InvertedGenerator {
    /// Create a new Inverted dimension generator.
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
    fn select_biome(&self, pos: IVec3) -> InvertedBiome {
        let hash = self.hash_position(pos);

        // Underground tends to be caves
        if pos.y < 30 {
            return InvertedBiome::CrystalCaves;
        }

        // Low areas are lava lakes
        if pos.y < 50 && hash % 10 < 4 {
            return InvertedBiome::LavaLakes;
        }

        // High areas are spires
        if pos.y > 80 {
            return InvertedBiome::ObsidianSpires;
        }

        // Distribution based on noise
        match hash % 100 {
            0..=25 => InvertedBiome::LavaLakes,
            26..=45 => InvertedBiome::ObsidianSpires,
            46..=60 => InvertedBiome::CrystalCaves,
            _ => InvertedBiome::VolcanicPlains,
        }
    }

    /// Calculate temperature based on position and biome.
    fn calculate_temperature(&self, pos: IVec3, biome: InvertedBiome) -> f32 {
        let modifier = biome.temperature_modifier();
        // Depth increases temperature
        let depth_modifier = (64 - pos.y).max(0) as f32 * 0.2;
        // Slight variation from hash
        let noise = (self.hash_position(pos) % 100) as f32 * 0.1 - 5.0;

        BASE_TEMP + modifier + depth_modifier + noise
    }

    /// Generate resources for a chunk.
    fn generate_resources(&self, pos: IVec3, biome: InvertedBiome) -> Vec<String> {
        let mut resources = biome.typical_resources();
        let hash = self.hash_position(pos);

        // Rare resources in Inverted
        if hash % 30 == 0 {
            resources.push("hellfire_crystal".to_string());
        }
        if hash % 75 == 0 {
            resources.push("demon_core".to_string());
        }

        resources
    }

    /// Hash a position for deterministic pseudo-random values.
    fn hash_position(&self, pos: IVec3) -> u64 {
        let x = pos.x as u64;
        let y = pos.y as u64;
        let z = pos.z as u64;

        let mut hash = self.seed.wrapping_add(0xdead_beef);
        hash = hash.wrapping_mul(37).wrapping_add(x);
        hash = hash.wrapping_mul(37).wrapping_add(y);
        hash = hash.wrapping_mul(37).wrapping_add(z);
        hash ^= hash >> 17;
        hash = hash.wrapping_mul(0x94d0_49bb);
        hash ^= hash >> 15;
        hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inverted_biome_temperature() {
        assert!(InvertedBiome::LavaLakes.temperature_modifier() > InvertedBiome::CrystalCaves.temperature_modifier());
        assert!(InvertedBiome::ObsidianSpires.temperature_modifier() > 0.0);
    }

    #[test]
    fn test_inverted_biome_resources() {
        let lava_resources = InvertedBiome::LavaLakes.typical_resources();
        assert!(lava_resources.contains(&"fire_essence".to_string()));

        let crystal_resources = InvertedBiome::CrystalCaves.typical_resources();
        assert!(crystal_resources.contains(&"heat_crystal".to_string()));
    }

    #[test]
    fn test_inverted_generator_new() {
        let generator = InvertedGenerator::new(54321);
        assert_eq!(generator.seed(), 54321);
    }

    #[test]
    fn test_inverted_generator_deterministic() {
        let gen1 = InvertedGenerator::new(99);
        let gen2 = InvertedGenerator::new(99);

        let pos = IVec3::new(15, 45, 30);
        let chunk1 = gen1.generate_chunk(pos);
        let chunk2 = gen2.generate_chunk(pos);

        assert_eq!(chunk1.biome, chunk2.biome);
        assert!((chunk1.temperature - chunk2.temperature).abs() < f32::EPSILON);
    }

    #[test]
    fn test_base_temperature_is_hot() {
        let generator = InvertedGenerator::new(1);
        let chunk = generator.generate_chunk(IVec3::new(0, 64, 0));

        // Temperature should be around or above BASE_TEMP
        assert!(chunk.temperature >= BASE_TEMP - 20.0);
    }

    #[test]
    fn test_underground_is_caves() {
        let generator = InvertedGenerator::new(1);
        let chunk = generator.generate_chunk(IVec3::new(0, 20, 0));

        assert_eq!(chunk.biome, "CrystalCaves");
    }

    #[test]
    fn test_high_altitude_spires() {
        let generator = InvertedGenerator::new(1);
        let chunk = generator.generate_chunk(IVec3::new(0, 100, 0));

        assert_eq!(chunk.biome, "ObsidianSpires");
    }
}
