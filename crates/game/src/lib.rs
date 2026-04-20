//! Titan survival game client.
//!
//! Core game logic including ECS components, systems, and entity management.
//! Survival mechanics on a living colossus with balance, harvesting, and crafting.

pub mod ai;
pub mod audio;
pub mod crafting;
pub mod creatures;
pub mod ecs;
pub mod entities;
pub mod equipment;
pub mod inventory;
pub mod networking;
pub mod survival;
pub mod world;

#[cfg(test)]
mod integration_tests;
