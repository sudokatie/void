//! World generation systems for Fracture dimensions.
//!
//! Each dimension has its own terrain generator with unique biomes,
//! temperatures, and resources.

mod inverted_gen;
mod nexus_gen;
mod prime_gen;
mod void_gen;
mod weak_point_gen;

pub use inverted_gen::{InvertedBiome, InvertedGenerator};
pub use nexus_gen::{NexusBiome, NexusGenerator};
pub use prime_gen::{ChunkData, PrimeBiome, PrimeGenerator};
pub use void_gen::{VoidBiome, VoidGenerator};
pub use weak_point_gen::{WeakPointGenerator, WeakPointPlacement};
