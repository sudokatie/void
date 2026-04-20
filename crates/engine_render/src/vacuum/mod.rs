//! Vacuum and decompression rendering effects.
//!
//! Provides visual effects for vacuum, decompression, and breach events.

mod decompression_fx;
mod vacuum_rendering;

pub use decompression_fx::{DecompressionFX, FrostEffect, VentingParticle};
pub use vacuum_rendering::{BreachGlow, VacuumRendering, VacuumRenderState};
