//! Phase suits for protection against fracture sickness.
//!
//! Phase suits provide varying levels of sickness reduction based on tier,
//! with durability that degrades over time.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Phase suit tiers with increasing protection levels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PhaseSuitTier {
    /// Basic phase suit (25% sickness reduction).
    Basic,
    /// Standard phase suit (50% sickness reduction).
    Standard,
    /// Military-grade phase suit (75% sickness reduction).
    Military,
    /// Nexus-infused phase suit (90% sickness reduction).
    Nexus,
}

impl fmt::Display for PhaseSuitTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PhaseSuitTier::Basic => write!(f, "Basic"),
            PhaseSuitTier::Standard => write!(f, "Standard"),
            PhaseSuitTier::Military => write!(f, "Military"),
            PhaseSuitTier::Nexus => write!(f, "Nexus"),
        }
    }
}

impl PhaseSuitTier {
    /// Get the sickness reduction percentage for this tier.
    #[must_use]
    pub fn sickness_reduction(&self) -> f32 {
        match self {
            PhaseSuitTier::Basic => 0.25,
            PhaseSuitTier::Standard => 0.50,
            PhaseSuitTier::Military => 0.75,
            PhaseSuitTier::Nexus => 0.90,
        }
    }

    /// Get all tier variants.
    #[must_use]
    pub fn all() -> &'static [PhaseSuitTier] {
        &[
            PhaseSuitTier::Basic,
            PhaseSuitTier::Standard,
            PhaseSuitTier::Military,
            PhaseSuitTier::Nexus,
        ]
    }
}

/// Maximum durability for all phase suits.
pub const MAX_DURABILITY: f32 = 100.0;

/// A phase suit that protects against fracture sickness.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PhaseSuit {
    /// Tier of the phase suit.
    tier: PhaseSuitTier,
    /// Sickness reduction multiplier (0.0 to 1.0).
    sickness_reduction: f32,
    /// Current durability (0 to 100).
    durability: f32,
    /// Whether the suit is operational.
    operational: bool,
}

impl PhaseSuit {
    /// Create a new phase suit of the given tier.
    #[must_use]
    pub fn new(tier: PhaseSuitTier) -> Self {
        Self {
            tier,
            sickness_reduction: tier.sickness_reduction(),
            durability: MAX_DURABILITY,
            operational: true,
        }
    }

    /// Process incoming sickness through the suit.
    ///
    /// Returns the reduced sickness amount after suit protection.
    /// Also degrades suit durability slightly.
    pub fn tick(&mut self, sickness_amount: f32) -> f32 {
        if !self.operational || sickness_amount <= 0.0 {
            return sickness_amount;
        }

        // Degrade durability based on sickness blocked
        let blocked = sickness_amount * self.sickness_reduction;
        let durability_loss = blocked * 0.01; // 1% of blocked sickness damages suit
        self.durability = (self.durability - durability_loss).max(0.0);

        // Check if suit is still operational
        if self.durability <= 0.0 {
            self.operational = false;
            return sickness_amount; // No protection when broken
        }

        // Return reduced sickness
        sickness_amount * (1.0 - self.sickness_reduction)
    }

    /// Apply damage to the suit.
    pub fn take_damage(&mut self, amount: f32) {
        if amount <= 0.0 {
            return;
        }

        self.durability = (self.durability - amount).max(0.0);
        if self.durability <= 0.0 {
            self.operational = false;
        }
    }

    /// Repair the suit.
    ///
    /// Returns the actual amount repaired.
    pub fn repair(&mut self, amount: f32) -> f32 {
        if amount <= 0.0 {
            return 0.0;
        }

        let space = MAX_DURABILITY - self.durability;
        let repaired = amount.min(space);
        self.durability += repaired;

        // Reactivate if repaired from zero
        if self.durability > 0.0 {
            self.operational = true;
        }

        repaired
    }

    /// Check if the suit is operational.
    #[must_use]
    pub fn is_operational(&self) -> bool {
        self.operational
    }

    /// Get the suit tier.
    #[must_use]
    pub fn tier(&self) -> PhaseSuitTier {
        self.tier
    }

    /// Get the sickness reduction multiplier.
    #[must_use]
    pub fn sickness_reduction(&self) -> f32 {
        self.sickness_reduction
    }

    /// Get the current durability.
    #[must_use]
    pub fn durability(&self) -> f32 {
        self.durability
    }

    /// Get durability as a percentage (0.0 to 1.0).
    #[must_use]
    pub fn durability_percentage(&self) -> f32 {
        self.durability / MAX_DURABILITY
    }

    /// Check if the suit needs repair (below 25% durability).
    #[must_use]
    pub fn needs_repair(&self) -> bool {
        self.durability < MAX_DURABILITY * 0.25
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_suit_tier_display() {
        assert_eq!(format!("{}", PhaseSuitTier::Basic), "Basic");
        assert_eq!(format!("{}", PhaseSuitTier::Standard), "Standard");
        assert_eq!(format!("{}", PhaseSuitTier::Military), "Military");
        assert_eq!(format!("{}", PhaseSuitTier::Nexus), "Nexus");
    }

    #[test]
    fn test_phase_suit_tier_sickness_reduction() {
        assert!((PhaseSuitTier::Basic.sickness_reduction() - 0.25).abs() < f32::EPSILON);
        assert!((PhaseSuitTier::Standard.sickness_reduction() - 0.50).abs() < f32::EPSILON);
        assert!((PhaseSuitTier::Military.sickness_reduction() - 0.75).abs() < f32::EPSILON);
        assert!((PhaseSuitTier::Nexus.sickness_reduction() - 0.90).abs() < f32::EPSILON);
    }

    #[test]
    fn test_phase_suit_tier_all() {
        let all = PhaseSuitTier::all();
        assert_eq!(all.len(), 4);
        assert!(all.contains(&PhaseSuitTier::Basic));
        assert!(all.contains(&PhaseSuitTier::Nexus));
    }

    #[test]
    fn test_phase_suit_new() {
        let suit = PhaseSuit::new(PhaseSuitTier::Standard);
        assert_eq!(suit.tier(), PhaseSuitTier::Standard);
        assert!((suit.sickness_reduction() - 0.50).abs() < f32::EPSILON);
        assert!((suit.durability() - 100.0).abs() < f32::EPSILON);
        assert!(suit.is_operational());
    }

    #[test]
    fn test_phase_suit_tick_reduces_sickness() {
        let mut suit = PhaseSuit::new(PhaseSuitTier::Standard);

        let reduced = suit.tick(10.0);
        assert!((reduced - 5.0).abs() < f32::EPSILON); // 50% reduction
    }

    #[test]
    fn test_phase_suit_tick_degrades_durability() {
        let mut suit = PhaseSuit::new(PhaseSuitTier::Standard);
        let initial_durability = suit.durability();

        suit.tick(100.0);
        assert!(suit.durability() < initial_durability);
    }

    #[test]
    fn test_phase_suit_tick_zero_sickness() {
        let mut suit = PhaseSuit::new(PhaseSuitTier::Basic);
        let initial_durability = suit.durability();

        let reduced = suit.tick(0.0);
        assert!((reduced - 0.0).abs() < f32::EPSILON);
        assert!((suit.durability() - initial_durability).abs() < f32::EPSILON);
    }

    #[test]
    fn test_phase_suit_tick_negative_sickness() {
        let mut suit = PhaseSuit::new(PhaseSuitTier::Basic);

        let reduced = suit.tick(-10.0);
        assert!((reduced - -10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_phase_suit_tick_not_operational() {
        let mut suit = PhaseSuit::new(PhaseSuitTier::Standard);
        suit.take_damage(100.0); // Break the suit

        let reduced = suit.tick(10.0);
        assert!((reduced - 10.0).abs() < f32::EPSILON); // No reduction
    }

    #[test]
    fn test_phase_suit_take_damage() {
        let mut suit = PhaseSuit::new(PhaseSuitTier::Basic);

        suit.take_damage(30.0);
        assert!((suit.durability() - 70.0).abs() < f32::EPSILON);
        assert!(suit.is_operational());
    }

    #[test]
    fn test_phase_suit_take_damage_breaks() {
        let mut suit = PhaseSuit::new(PhaseSuitTier::Basic);

        suit.take_damage(150.0);
        assert!((suit.durability() - 0.0).abs() < f32::EPSILON);
        assert!(!suit.is_operational());
    }

    #[test]
    fn test_phase_suit_take_damage_negative() {
        let mut suit = PhaseSuit::new(PhaseSuitTier::Basic);
        let initial = suit.durability();

        suit.take_damage(-10.0);
        assert!((suit.durability() - initial).abs() < f32::EPSILON);
    }

    #[test]
    fn test_phase_suit_repair() {
        let mut suit = PhaseSuit::new(PhaseSuitTier::Basic);
        suit.take_damage(50.0);

        let repaired = suit.repair(30.0);
        assert!((repaired - 30.0).abs() < f32::EPSILON);
        assert!((suit.durability() - 80.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_phase_suit_repair_capped() {
        let mut suit = PhaseSuit::new(PhaseSuitTier::Basic);
        suit.take_damage(20.0);

        let repaired = suit.repair(50.0);
        assert!((repaired - 20.0).abs() < f32::EPSILON);
        assert!((suit.durability() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_phase_suit_repair_reactivates() {
        let mut suit = PhaseSuit::new(PhaseSuitTier::Basic);
        suit.take_damage(100.0);
        assert!(!suit.is_operational());

        suit.repair(10.0);
        assert!(suit.is_operational());
    }

    #[test]
    fn test_phase_suit_repair_negative() {
        let mut suit = PhaseSuit::new(PhaseSuitTier::Basic);
        suit.take_damage(50.0);

        let repaired = suit.repair(-10.0);
        assert!((repaired - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_phase_suit_durability_percentage() {
        let mut suit = PhaseSuit::new(PhaseSuitTier::Basic);
        assert!((suit.durability_percentage() - 1.0).abs() < f32::EPSILON);

        suit.take_damage(50.0);
        assert!((suit.durability_percentage() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_phase_suit_needs_repair() {
        let mut suit = PhaseSuit::new(PhaseSuitTier::Basic);
        assert!(!suit.needs_repair());

        suit.take_damage(80.0); // 20% remaining
        assert!(suit.needs_repair());
    }

    #[test]
    fn test_phase_suit_nexus_tier_high_reduction() {
        let mut suit = PhaseSuit::new(PhaseSuitTier::Nexus);

        let reduced = suit.tick(100.0);
        assert!((reduced - 10.0).abs() < 0.01); // 90% reduction
    }

    #[test]
    fn test_phase_suit_military_tier() {
        let mut suit = PhaseSuit::new(PhaseSuitTier::Military);

        let reduced = suit.tick(100.0);
        assert!((reduced - 25.0).abs() < f32::EPSILON); // 75% reduction
    }
}
