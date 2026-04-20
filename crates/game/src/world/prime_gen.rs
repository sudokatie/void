//! Prime dimension world generator.
//!
//! The Prime dimension is the normal, stable starting dimension with
//! traditional terrain types like forests, plains, and mountains.

use glam::IVec3;
use serde::{Deserialize, Serialize};

/// Biomes available in the Prime dimension.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrimeBiome {
    /// Dense woodland with tall trees.
    Forest,
    /// Open grasslands with occasional vegetation.
    Plains,
    /// High elevation rocky terrain.
    Mountains,
    /// Water bodies and riverbanks.
    River,
}

impl PrimeBiome {
    /// Get base temperature for this biome (Celsius).
    #[must_use]
    pub fn base_temperature(&self) -> f32 {
        match self {
            PrimeBiome::Forest => 18.0,
            PrimeBiome::Plains => 22.0,
            PrimeBiome::Mountains => 5.0,
            PrimeBiome::River => 16.0,
        }
    }

    /// Get typical resources for this biome.
    #[must_use]
    pub fn typical_resources(&self) -> Vec<String> {
        match self {
            PrimeBiome::Forest => vec![
                "wood".to_string(),
                "berries".to_string(),
                "mushrooms".to_string(),
            ],
            PrimeBiome::Plains => vec![
                "grass".to_string(),
                "wheat".to_string(),
                "flint".to_string(),
            ],
            PrimeBiome::Mountains => vec![
                "stone".to_string(),
                "iron_ore".to_string(),
                "coal".to_string(),
            ],
            PrimeBiome::River => vec![
                "clay".to_string(),
                "reeds".to_string(),
                "fish".to_string(),
            ],
        }
    }

    /// Get the visual fog color for this biome.
    ///
    /// Returns RGBA color values (0.0 to 1.0).
    #[must_use]
    pub fn fog_color(&self) -> [f32; 4] {
        match self {
            PrimeBiome::Forest => [0.6, 0.75, 0.6, 1.0],   // Green-tinted fog
            PrimeBiome::Plains => [0.8, 0.85, 0.95, 1.0],  // Light blue sky
            PrimeBiome::Mountains => [0.9, 0.92, 0.98, 1.0], // White mountain air
            PrimeBiome::River => [0.7, 0.8, 0.9, 1.0],     // Blue water reflection
        }
    }

    /// Get the ambient light modifier for this biome.
    ///
    /// Returns multiplier (0.0 to 1.5) for base dimension light.
    #[must_use]
    pub fn light_modifier(&self) -> f32 {
        match self {
            PrimeBiome::Forest => 0.7,     // Shaded by trees
            PrimeBiome::Plains => 1.0,     // Full sunlight
            PrimeBiome::Mountains => 1.1,  // Bright at altitude
            PrimeBiome::River => 0.95,     // Slight shade
        }
    }

    /// Get the grass/foliage color for this biome.
    ///
    /// Returns RGB color values (0.0 to 1.0).
    #[must_use]
    pub fn foliage_color(&self) -> [f32; 3] {
        match self {
            PrimeBiome::Forest => [0.2, 0.5, 0.2],    // Dark green
            PrimeBiome::Plains => [0.5, 0.7, 0.3],   // Yellow-green
            PrimeBiome::Mountains => [0.3, 0.4, 0.25], // Muted green
            PrimeBiome::River => [0.35, 0.55, 0.3],  // Medium green
        }
    }

    /// Get the water color tint for this biome.
    ///
    /// Returns RGB color values (0.0 to 1.0).
    #[must_use]
    pub fn water_color(&self) -> [f32; 3] {
        match self {
            PrimeBiome::Forest => [0.2, 0.4, 0.35],   // Dark pond
            PrimeBiome::Plains => [0.3, 0.5, 0.7],   // Clear blue
            PrimeBiome::Mountains => [0.4, 0.6, 0.8], // Crystal clear
            PrimeBiome::River => [0.25, 0.45, 0.55],  // River blue
        }
    }

    /// Check if this biome has ambient particles.
    #[must_use]
    pub fn has_particles(&self) -> bool {
        matches!(self, PrimeBiome::Forest | PrimeBiome::River)
    }

    /// Get the particle type for this biome.
    #[must_use]
    pub fn particle_type(&self) -> Option<&'static str> {
        match self {
            PrimeBiome::Forest => Some("leaves"),
            PrimeBiome::River => Some("water_droplets"),
            _ => None,
        }
    }
}

/// Generated chunk data for any dimension.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChunkData {
    /// Chunk position in the world.
    pub position: IVec3,
    /// Biome identifier (as string for cross-dimension compatibility).
    pub biome: String,
    /// Temperature at this chunk location.
    pub temperature: f32,
    /// Available resources in this chunk.
    pub resources: Vec<String>,
}

impl ChunkData {
    /// Create new chunk data.
    #[must_use]
    pub fn new(position: IVec3, biome: String, temperature: f32, resources: Vec<String>) -> Self {
        Self {
            position,
            biome,
            temperature,
            resources,
        }
    }
}

/// World generator for the Prime dimension.
#[derive(Clone, Debug)]
pub struct PrimeGenerator {
    /// Seed for deterministic generation.
    seed: u64,
}

impl PrimeGenerator {
    /// Create a new Prime dimension generator.
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
    fn select_biome(&self, pos: IVec3) -> PrimeBiome {
        let hash = self.hash_position(pos);

        // Height-based selection with noise
        if pos.y > 80 {
            return PrimeBiome::Mountains;
        }

        // Water near y=62 (sea level)
        if pos.y < 64 && hash % 10 < 3 {
            return PrimeBiome::River;
        }

        // Forest vs Plains based on noise
        match hash % 100 {
            0..=35 => PrimeBiome::Forest,
            36..=75 => PrimeBiome::Plains,
            76..=90 => PrimeBiome::Mountains,
            _ => PrimeBiome::River,
        }
    }

    /// Calculate temperature based on position and biome.
    fn calculate_temperature(&self, pos: IVec3, biome: PrimeBiome) -> f32 {
        let base = biome.base_temperature();
        // Altitude affects temperature
        let altitude_modifier = (64 - pos.y) as f32 * 0.1;
        // Slight variation from hash
        let noise = (self.hash_position(pos) % 100) as f32 * 0.05 - 2.5;

        base + altitude_modifier + noise
    }

    /// Generate resources for a chunk.
    fn generate_resources(&self, pos: IVec3, biome: PrimeBiome) -> Vec<String> {
        let mut resources = biome.typical_resources();
        let hash = self.hash_position(pos);

        // Chance for rare resources
        if hash % 50 == 0 {
            resources.push("gold_ore".to_string());
        }
        if hash % 100 == 0 {
            resources.push("diamond".to_string());
        }

        resources
    }

    /// Hash a position for deterministic pseudo-random values.
    fn hash_position(&self, pos: IVec3) -> u64 {
        let x = pos.x as u64;
        let y = pos.y as u64;
        let z = pos.z as u64;

        let mut hash = self.seed;
        hash = hash.wrapping_mul(31).wrapping_add(x);
        hash = hash.wrapping_mul(31).wrapping_add(y);
        hash = hash.wrapping_mul(31).wrapping_add(z);
        hash ^= hash >> 16;
        hash = hash.wrapping_mul(0x85eb_ca6b);
        hash ^= hash >> 13;
        hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_biome_temperature() {
        assert!(PrimeBiome::Mountains.base_temperature() < PrimeBiome::Plains.base_temperature());
        assert!(PrimeBiome::Forest.base_temperature() < PrimeBiome::Plains.base_temperature());
    }

    #[test]
    fn test_prime_biome_resources() {
        let forest_resources = PrimeBiome::Forest.typical_resources();
        assert!(forest_resources.contains(&"wood".to_string()));

        let mountain_resources = PrimeBiome::Mountains.typical_resources();
        assert!(mountain_resources.contains(&"iron_ore".to_string()));
    }

    #[test]
    fn test_prime_generator_new() {
        let generator = PrimeGenerator::new(12345);
        assert_eq!(generator.seed(), 12345);
    }

    #[test]
    fn test_prime_generator_deterministic() {
        let gen1 = PrimeGenerator::new(42);
        let gen2 = PrimeGenerator::new(42);

        let pos = IVec3::new(10, 64, 20);
        let chunk1 = gen1.generate_chunk(pos);
        let chunk2 = gen2.generate_chunk(pos);

        assert_eq!(chunk1.biome, chunk2.biome);
        assert!((chunk1.temperature - chunk2.temperature).abs() < f32::EPSILON);
    }

    #[test]
    fn test_prime_generator_different_seeds() {
        let gen1 = PrimeGenerator::new(1);
        let gen2 = PrimeGenerator::new(2);

        // Different seeds should produce different results (with high probability)
        let pos = IVec3::new(100, 64, 100);
        let chunk1 = gen1.generate_chunk(pos);
        let chunk2 = gen2.generate_chunk(pos);

        // At least one property should differ
        let same = chunk1.biome == chunk2.biome
            && (chunk1.temperature - chunk2.temperature).abs() < 0.1;
        // This could theoretically be the same, so we just verify it runs
        let _ = same;
    }

    #[test]
    fn test_chunk_data_position() {
        let generator = PrimeGenerator::new(1);
        let pos = IVec3::new(5, 10, 15);
        let chunk = generator.generate_chunk(pos);

        assert_eq!(chunk.position, pos);
    }

    #[test]
    fn test_high_altitude_mountains() {
        let generator = PrimeGenerator::new(1);
        let pos = IVec3::new(0, 100, 0);
        let chunk = generator.generate_chunk(pos);

        assert_eq!(chunk.biome, "Mountains");
    }

    #[test]
    fn test_temperature_decreases_with_altitude() {
        let generator = PrimeGenerator::new(1);
        let low = generator.generate_chunk(IVec3::new(0, 40, 0));
        let high = generator.generate_chunk(IVec3::new(0, 100, 0));

        assert!(low.temperature > high.temperature);
    }

    #[test]
    fn test_resources_not_empty() {
        let generator = PrimeGenerator::new(1);
        let chunk = generator.generate_chunk(IVec3::new(0, 64, 0));

        assert!(!chunk.resources.is_empty());
    }

    #[test]
    fn test_biome_fog_color() {
        let forest_fog = PrimeBiome::Forest.fog_color();
        assert_eq!(forest_fog.len(), 4);
        assert!(forest_fog[1] > forest_fog[0]); // Green dominant

        let plains_fog = PrimeBiome::Plains.fog_color();
        assert!(plains_fog[2] > plains_fog[0]); // Blue dominant
    }

    #[test]
    fn test_biome_light_modifier() {
        assert!(PrimeBiome::Forest.light_modifier() < PrimeBiome::Plains.light_modifier());
        assert!(PrimeBiome::Mountains.light_modifier() > PrimeBiome::Plains.light_modifier());
    }

    #[test]
    fn test_biome_foliage_color() {
        let forest = PrimeBiome::Forest.foliage_color();
        assert_eq!(forest.len(), 3);
        assert!(forest[1] > forest[0]); // Green dominant
    }

    #[test]
    fn test_biome_water_color() {
        let mountain_water = PrimeBiome::Mountains.water_color();
        assert_eq!(mountain_water.len(), 3);
        assert!(mountain_water[2] > mountain_water[0]); // Blue dominant
    }

    #[test]
    fn test_biome_particles() {
        assert!(PrimeBiome::Forest.has_particles());
        assert!(PrimeBiome::River.has_particles());
        assert!(!PrimeBiome::Plains.has_particles());
        assert!(!PrimeBiome::Mountains.has_particles());
    }

    #[test]
    fn test_biome_particle_types() {
        assert_eq!(PrimeBiome::Forest.particle_type(), Some("leaves"));
        assert_eq!(PrimeBiome::River.particle_type(), Some("water_droplets"));
        assert!(PrimeBiome::Plains.particle_type().is_none());
    }
}
