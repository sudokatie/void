//! Creature systems for space station survival.
//!
//! Provides hostile and passive creatures that inhabit the void.

mod hostile;
mod passive;

pub use hostile::{AbilityResult, HostileCreature, HostileType, SpecialAbilityInfo};
pub use passive::{PassiveCreature, PassiveType};
