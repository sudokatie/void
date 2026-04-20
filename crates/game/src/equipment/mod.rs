//! Equipment systems for time-loop survival.
//!
//! Provides temporal gear for interacting with time loop mechanics.

mod balance_gear;
mod phase_suits;
pub mod stability_equip;
pub mod temporal_gear;

pub use balance_gear::{BalanceEquipment, BalanceGear};
pub use phase_suits::{PhaseSuit, PhaseSuitTier, MAX_DURABILITY};
pub use stability_equip::{AnchorBuilder, StabilityDetector, VoidTether};
pub use temporal_gear::{TemporalEquipment, TemporalGear};
