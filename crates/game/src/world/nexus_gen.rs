//! Nexus dimension world generator.
//!
//! The Nexus dimension connects all other dimensions with chaotic
//! terrain that bleeds between realities. Base temperature is 30C.

use glam::IVec3;
use serde::{Deserialize, Serialize};

use super::ChunkData;

/// Base temperature for the Nexus dimension in Celsius.
pub const BASE_TEMP: f32 = 30.0;

/// Biomes available in the Nexus dimension.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NexusBiome {
    /// Areas where other dimensions bleed through.
    DimensionBleed,
    /// Crystalline structures that anchor reality.
    CrystalNexus,
    /// Highly unstable regions of shifting terrain.
    ChaosZone,
    /// Rare stable pockets safe for habitation.
    StabilityOasis,
}

impl NexusBiome {
    /// Get temperature modifier for this biome.
    #[must_use]
    pub fn temperature_modifier(&self) -> f32 {
        match self {
            NexusBiome::DimensionBleed => 15.0, // Varies wildly
            NexusBiome::CrystalNexus => 0.0,
            NexusBiome::ChaosZone => 20.0,
            NexusBiome::StabilityOasis => -5.0,
        }
    }

    /// Get typical resources for this biome.
    #[must_use]
    pub fn typical_resources(&self) -> Vec<String> {
        match self {
            NexusBiome::DimensionBleed => vec![
                "bleed_essence".to_string(),
                "dimensional_shard".to_string(),
            ],
            NexusBiome::CrystalNexus => vec![
                "nexus_crystal".to_string(),
                "stabilizer_core".to_string(),
                "reality_thread".to_string(),
            ],
            NexusBiome::ChaosZone => vec![
                "chaos_fragment".to_string(),
                "entropy_dust".to_string(),
            ],
            NexusBiome::StabilityOasis => vec![
                "oasis_water".to_string(),
                "calm_stone".to_string(),
                "anchor_root".to_string(),
            ],
        }
    }

    /// Get the instability level of this biome (0.0 - 1.0).
    #[must_use]
    pub fn instability(&self) -> f32 {
        match self {
            NexusBiome::DimensionBleed => 0.7,
            NexusBiome::CrystalNexus => 0.3,
            NexusBiome::ChaosZone => 0.95,
            NexusBiome::StabilityOasis => 0.05,
        }
    }

    /// Get the visual fog color for this biome.
    ///
    /// Returns RGBA color values (0.0 to 1.0).
    #[must_use]
    pub fn fog_color(&self) -> [f32; 4] {
        match self {
            NexusBiome::DimensionBleed => [0.6, 0.3, 0.7, 0.8],   // Purple bleed
            NexusBiome::CrystalNexus => [0.4, 0.5, 0.8, 0.6],    // Crystal blue
            NexusBiome::ChaosZone => [0.8, 0.2, 0.4, 0.9],       // Chaotic red-purple
            NexusBiome::StabilityOasis => [0.5, 0.6, 0.7, 0.4],  // Calm gray-blue
        }
    }

    /// Get the ambient light modifier for this biome.
    ///
    /// Returns multiplier (0.0 to 1.5) for base dimension light.
    #[must_use]
    pub fn light_modifier(&self) -> f32 {
        match self {
            NexusBiome::DimensionBleed => 0.8,
            NexusBiome::CrystalNexus => 1.2,      // Crystal glow
            NexusBiome::ChaosZone => 0.5,         // Dim chaos
            NexusBiome::StabilityOasis => 1.0,
        }
    }

    /// Get the glow color for this biome.
    ///
    /// Returns RGB color values (0.0 to 1.0).
    #[must_use]
    pub fn glow_color(&self) -> [f32; 3] {
        match self {
            NexusBiome::DimensionBleed => [0.8, 0.4, 0.9],   // Purple glow
            NexusBiome::CrystalNexus => [0.3, 0.8, 1.0],    // Cyan crystal glow
            NexusBiome::ChaosZone => [1.0, 0.3, 0.5],       // Red chaos glow
            NexusBiome::StabilityOasis => [0.5, 0.7, 0.5],  // Calm green glow
        }
    }

    /// Get the distortion strength for this biome.
    ///
    /// Returns 0.0 (none) to 1.0 (maximum distortion).
    #[must_use]
    pub fn distortion_strength(&self) -> f32 {
        match self {
            NexusBiome::DimensionBleed => 0.6,
            NexusBiome::CrystalNexus => 0.2,
            NexusBiome::ChaosZone => 0.9,
            NexusBiome::StabilityOasis => 0.0,
        }
    }

    /// Check if this biome has ambient particles.
    #[must_use]
    pub fn has_particles(&self) -> bool {
        true // All Nexus biomes have particles
    }

    /// Get the particle type for this biome.
    #[must_use]
    pub fn particle_type(&self) -> &'static str {
        match self {
            NexusBiome::DimensionBleed => "dimensional_sparks",
            NexusBiome::CrystalNexus => "crystal_shards",
            NexusBiome::ChaosZone => "chaos_embers",
            NexusBiome::StabilityOasis => "calm_motes",
        }
    }

    /// Get the color shift speed for this biome.
    ///
    /// Returns cycles per second for color animation.
    #[must_use]
    pub fn color_shift_speed(&self) -> f32 {
        match self {
            NexusBiome::DimensionBleed => 0.5,
            NexusBiome::CrystalNexus => 0.2,
            NexusBiome::ChaosZone => 2.0,
            NexusBiome::StabilityOasis => 0.0,
        }
    }

    /// Get the terrain shimmer intensity for this biome.
    #[must_use]
    pub fn shimmer_intensity(&self) -> f32 {
        match self {
            NexusBiome::DimensionBleed => 0.4,
            NexusBiome::CrystalNexus => 0.8,
            NexusBiome::ChaosZone => 0.6,
            NexusBiome::StabilityOasis => 0.1,
        }
    }
}

/// World generator for the Nexus dimension.
#[derive(Clone, Debug)]
pub struct NexusGenerator {
    /// Seed for deterministic generation.
    seed: u64,
}

impl NexusGenerator {
    /// Create a new Nexus dimension generator.
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
    fn select_biome(&self, pos: IVec3) -> NexusBiome {
        let hash = self.hash_position(pos);

        // Rare stability oases
        if hash % 200 < 5 {
            return NexusBiome::StabilityOasis;
        }

        // Crystal nexus nodes at specific intervals
        let is_node = (pos.x % 64).abs() < 8 && (pos.z % 64).abs() < 8;
        if is_node && hash % 3 == 0 {
            return NexusBiome::CrystalNexus;
        }

        // Distribution based on chaos
        match hash % 100 {
            0..=35 => NexusBiome::DimensionBleed,
            36..=55 => NexusBiome::CrystalNexus,
            56..=85 => NexusBiome::ChaosZone,
            _ => NexusBiome::StabilityOasis,
        }
    }

    /// Calculate temperature based on position and biome.
    fn calculate_temperature(&self, pos: IVec3, biome: NexusBiome) -> f32 {
        let modifier = biome.temperature_modifier();
        // Chaotic temperature swings based on hash
        let chaos = ((self.hash_position(pos) % 200) as f32 - 100.0) * 0.15;

        BASE_TEMP + modifier + chaos
    }

    /// Generate resources for a chunk.
    fn generate_resources(&self, pos: IVec3, biome: NexusBiome) -> Vec<String> {
        let mut resources = biome.typical_resources();
        let hash = self.hash_position(pos);

        // Rare resources in Nexus
        if hash % 25 == 0 {
            resources.push("nexus_key".to_string());
        }
        if hash % 80 == 0 {
            resources.push("dimensional_anchor".to_string());
        }
        if hash % 150 == 0 {
            resources.push("reality_core".to_string());
        }

        resources
    }

    /// Hash a position for deterministic pseudo-random values.
    fn hash_position(&self, pos: IVec3) -> u64 {
        let x = pos.x as u64;
        let y = pos.y as u64;
        let z = pos.z as u64;

        let mut hash = self.seed.wrapping_add(0xfeed_face);
        hash = hash.wrapping_mul(43).wrapping_add(x);
        hash = hash.wrapping_mul(43).wrapping_add(y);
        hash = hash.wrapping_mul(43).wrapping_add(z);
        hash ^= hash >> 21;
        hash = hash.wrapping_mul(0xc4ce_b9fe);
        hash ^= hash >> 19;
        hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nexus_biome_temperature() {
        assert!(NexusBiome::ChaosZone.temperature_modifier() > NexusBiome::StabilityOasis.temperature_modifier());
    }

    #[test]
    fn test_nexus_biome_instability() {
        assert!(NexusBiome::ChaosZone.instability() > NexusBiome::CrystalNexus.instability());
        assert!(NexusBiome::StabilityOasis.instability() < 0.1);
    }

    #[test]
    fn test_nexus_biome_resources() {
        let nexus_resources = NexusBiome::CrystalNexus.typical_resources();
        assert!(nexus_resources.contains(&"nexus_crystal".to_string()));

        let oasis_resources = NexusBiome::StabilityOasis.typical_resources();
        assert!(oasis_resources.contains(&"oasis_water".to_string()));
    }

    #[test]
    fn test_nexus_generator_new() {
        let generator = NexusGenerator::new(99999);
        assert_eq!(generator.seed(), 99999);
    }

    #[test]
    fn test_nexus_generator_deterministic() {
        let gen1 = NexusGenerator::new(333);
        let gen2 = NexusGenerator::new(333);

        let pos = IVec3::new(25, 64, 75);
        let chunk1 = gen1.generate_chunk(pos);
        let chunk2 = gen2.generate_chunk(pos);

        assert_eq!(chunk1.biome, chunk2.biome);
        assert!((chunk1.temperature - chunk2.temperature).abs() < f32::EPSILON);
    }

    #[test]
    fn test_base_temperature_moderate() {
        let generator = NexusGenerator::new(1);
        let chunk = generator.generate_chunk(IVec3::new(0, 64, 0));

        // Temperature should be in a reasonable range around BASE_TEMP
        assert!(chunk.temperature > BASE_TEMP - 30.0);
        assert!(chunk.temperature < BASE_TEMP + 50.0);
    }

    #[test]
    fn test_resources_not_empty() {
        let generator = NexusGenerator::new(1);
        let chunk = generator.generate_chunk(IVec3::new(0, 64, 0));

        assert!(!chunk.resources.is_empty());
    }

    #[test]
    fn test_biome_fog_color() {
        let bleed_fog = NexusBiome::DimensionBleed.fog_color();
        assert_eq!(bleed_fog.len(), 4);

        let chaos_fog = NexusBiome::ChaosZone.fog_color();
        assert!(chaos_fog[3] > bleed_fog[3]); // Chaos has denser fog
    }

    #[test]
    fn test_biome_light_modifier() {
        assert!(NexusBiome::CrystalNexus.light_modifier() > NexusBiome::ChaosZone.light_modifier());
    }

    #[test]
    fn test_biome_glow_color() {
        let crystal_glow = NexusBiome::CrystalNexus.glow_color();
        assert_eq!(crystal_glow.len(), 3);
        assert!(crystal_glow[2] > crystal_glow[0]); // Blue-cyan dominant
    }

    #[test]
    fn test_biome_distortion() {
        assert!(NexusBiome::ChaosZone.distortion_strength() > NexusBiome::CrystalNexus.distortion_strength());
        assert!((NexusBiome::StabilityOasis.distortion_strength() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_biome_particles() {
        // All Nexus biomes have particles
        assert!(NexusBiome::DimensionBleed.has_particles());
        assert!(NexusBiome::CrystalNexus.has_particles());
        assert!(NexusBiome::ChaosZone.has_particles());
        assert!(NexusBiome::StabilityOasis.has_particles());
    }

    #[test]
    fn test_biome_particle_types() {
        assert_eq!(NexusBiome::CrystalNexus.particle_type(), "crystal_shards");
        assert_eq!(NexusBiome::ChaosZone.particle_type(), "chaos_embers");
    }

    #[test]
    fn test_biome_color_shift_speed() {
        assert!(NexusBiome::ChaosZone.color_shift_speed() > NexusBiome::CrystalNexus.color_shift_speed());
        assert!((NexusBiome::StabilityOasis.color_shift_speed() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_biome_shimmer_intensity() {
        assert!(NexusBiome::CrystalNexus.shimmer_intensity() > NexusBiome::StabilityOasis.shimmer_intensity());
    }
}
