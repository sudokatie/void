//! Survival mechanics: health, hunger, sickness, and status effects.

mod combat;
mod death;
mod health;
mod hunger;
mod loop_hunger;
mod mining;
mod sickness;
mod survival_manager;
mod thirst;
mod transmutation;

pub use combat::{
    attempt_attack, calculate_knockback, can_attack, AttackCooldown, AttackResult, CombatStats,
};
pub use death::{
    DeathCause, DeathHandler, DeathResult, DroppedItem, ITEM_DESPAWN_TIME, PICKUP_DELAY_SECS,
};
pub use health::{DamageSource, Health};
pub use hunger::Hunger;
pub use loop_hunger::LoopHunger;
pub use mining::{
    calculate_mining_time, BlockPos, MiningProgress, MiningResult, BASE_MINE_TIME_SECS,
    BASE_MINING_SPEED,
};
pub use sickness::{FractureSickness, SicknessLevel, PRIME_RECOVERY_RATE};
pub use survival_manager::SurvivalManager;
pub use thirst::Thirst;
pub use transmutation::{TransmutationRule, TransmutationTable};
