//! Void survival game client.
//!
//! Core game logic including ECS components, systems, and entity management.
//! Space station decompression survival with atmosphere, power, and EVA mechanics.

pub mod ai;
pub mod audio;
pub mod crafting;
pub mod creatures;
pub mod ecs;
pub mod entities;
pub mod equipment;
pub mod inventory;
pub mod networking;
pub mod power;
pub mod station;
pub mod survival;
pub mod vacuum;
pub mod world;
pub mod zerog;

#[cfg(test)]
mod integration_tests;
