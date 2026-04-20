//! Creature systems for time-loop survival.
//!
//! Provides hostile and passive creatures that exist within the time loop.

mod hostile;
mod passive;

pub use hostile::{
    AbilityResult, HostileCreature, HostileSpawnCondition, HostileType, LoopPhaseSpawn,
    SpecialAbilityInfo,
};
pub use passive::{LoopParity, PassiveCreature, PassiveLoopPhase, PassiveSpawnCondition, PassiveType};

/// Time of day for creature spawning (re-exported for compatibility).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum TimeOfDay {
    /// Daytime.
    Day,
    /// Nighttime.
    Night,
    /// Any time.
    Any,
}

/// Mood requirement placeholder for compatibility.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MoodRequirement;
