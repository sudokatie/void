//! Rendering system for the Cascade game engine.
//!
//! Provides GPU abstraction, voxel rendering, and visual effects
//! including temporal effects for the time-loop survival game.

pub mod backend;
pub mod camera;
pub mod dimension;
pub mod fog;
pub mod ghost_block;
pub mod lighting;
mod renderer;
pub mod sky;
pub mod temporal;
pub mod titan;
pub mod voxel;

pub use renderer::{TriangleRenderer, Vertex};
