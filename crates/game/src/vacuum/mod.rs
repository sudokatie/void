//! Vacuum and atmosphere systems for space station survival.
//!
//! Provides gas modeling, decompression events, and atmosphere management.

mod atmosphere;
mod decompression;
mod gas_model;

pub use atmosphere::AtmosphereManager;
pub use decompression::{DecompressionEvent, DecompressionType};
pub use gas_model::GasModel;
