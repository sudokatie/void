//! Titan survival game client.
//!
//! Core game logic including ECS components, systems, and entity management.
//! Survival mechanics on a living colossus with balance, harvesting, and crafting.

pub mod ai;
pub mod audio;
pub mod balance;
pub mod building;
pub mod crafting;
pub mod creatures;
pub mod dimension;
pub mod ecs;
pub mod entities;
pub mod equipment;
pub mod harvesting;
pub mod inventory;
pub mod knowledge;
pub mod messages;
pub mod movement;
pub mod networking;
pub mod parasite;
pub mod stability;
pub mod survival;
pub mod temporal;
pub mod temporal_chest;
pub mod titan;
pub mod world;

#[cfg(test)]
mod integration_tests;
