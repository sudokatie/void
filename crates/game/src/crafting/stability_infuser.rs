//! Stability Infuser crafting station.
//!
//! Infuses items with stability energy to add dimensional resistance
//! properties to equipment.

use serde::{Deserialize, Serialize};

/// Minimum energy required for infusion.
pub const MIN_INFUSION_ENERGY: f32 = 10.0;

/// Energy cost per infusion level.
pub const ENERGY_PER_LEVEL: f32 = 25.0;

/// A stability infuser crafting station.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StabilityInfuser {
    /// Whether the infuser is operational.
    operational: bool,
    /// Accumulated energy for next infusion.
    stored_energy: f32,
    /// Maximum energy storage.
    max_energy: f32,
}

impl StabilityInfuser {
    /// Create a new stability infuser.
    #[must_use]
    pub fn new() -> Self {
        Self {
            operational: true,
            stored_energy: 0.0,
            max_energy: 100.0,
        }
    }

    /// Create a stability infuser with custom max energy.
    #[must_use]
    pub fn with_max_energy(max_energy: f32) -> Self {
        Self {
            operational: true,
            stored_energy: 0.0,
            max_energy: max_energy.max(MIN_INFUSION_ENERGY),
        }
    }

    /// Attempt to infuse an item with stability energy.
    ///
    /// Returns the name of the infused item if successful.
    /// Consumes energy from the infuser.
    #[must_use]
    pub fn infuse(&mut self, item: &str, energy: f32) -> Option<String> {
        if !self.operational {
            return None;
        }

        if energy < MIN_INFUSION_ENERGY {
            return None;
        }

        let energy_to_use = energy.min(self.stored_energy);
        if energy_to_use < MIN_INFUSION_ENERGY {
            return None;
        }

        self.stored_energy -= energy_to_use;

        // Calculate infusion level based on energy used
        let infusion_level = (energy_to_use / ENERGY_PER_LEVEL).floor() as u32;
        let infusion_level = infusion_level.max(1).min(3);

        // Generate infused item name
        let suffix = match infusion_level {
            1 => "stability_infused",
            2 => "stability_enhanced",
            _ => "stability_perfected",
        };

        Some(format!("{}_{}", item, suffix))
    }

    /// Add energy to the infuser.
    ///
    /// Returns the actual amount stored.
    pub fn add_energy(&mut self, amount: f32) -> f32 {
        if amount <= 0.0 {
            return 0.0;
        }

        let space = self.max_energy - self.stored_energy;
        let stored = amount.min(space);
        self.stored_energy += stored;
        stored
    }

    /// Get the stored energy.
    #[must_use]
    pub fn stored_energy(&self) -> f32 {
        self.stored_energy
    }

    /// Get the maximum energy capacity.
    #[must_use]
    pub fn max_energy(&self) -> f32 {
        self.max_energy
    }

    /// Get stored energy as a percentage (0.0 to 1.0).
    #[must_use]
    pub fn energy_percentage(&self) -> f32 {
        if self.max_energy <= 0.0 {
            return 0.0;
        }
        (self.stored_energy / self.max_energy).clamp(0.0, 1.0)
    }

    /// Check if the infuser has enough energy for a basic infusion.
    #[must_use]
    pub fn can_infuse(&self) -> bool {
        self.operational && self.stored_energy >= MIN_INFUSION_ENERGY
    }

    /// Check if the infuser is operational.
    #[must_use]
    pub fn is_operational(&self) -> bool {
        self.operational
    }

    /// Set the operational state.
    pub fn set_operational(&mut self, operational: bool) {
        self.operational = operational;
    }
}

impl Default for StabilityInfuser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stability_infuser_new() {
        let infuser = StabilityInfuser::new();
        assert!(infuser.is_operational());
        assert!((infuser.stored_energy() - 0.0).abs() < f32::EPSILON);
        assert!((infuser.max_energy() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_stability_infuser_with_max_energy() {
        let infuser = StabilityInfuser::with_max_energy(200.0);
        assert!((infuser.max_energy() - 200.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_stability_infuser_add_energy() {
        let mut infuser = StabilityInfuser::new();

        let stored = infuser.add_energy(50.0);
        assert!((stored - 50.0).abs() < f32::EPSILON);
        assert!((infuser.stored_energy() - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_stability_infuser_add_energy_capped() {
        let mut infuser = StabilityInfuser::new();
        infuser.add_energy(80.0);

        let stored = infuser.add_energy(50.0);
        assert!((stored - 20.0).abs() < f32::EPSILON);
        assert!((infuser.stored_energy() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_stability_infuser_add_energy_negative() {
        let mut infuser = StabilityInfuser::new();

        let stored = infuser.add_energy(-10.0);
        assert!((stored - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_stability_infuser_infuse_basic() {
        let mut infuser = StabilityInfuser::new();
        infuser.add_energy(50.0);

        let result = infuser.infuse("sword", 20.0);
        assert!(result.is_some());
        assert!(result.unwrap().starts_with("sword_stability"));
    }

    #[test]
    fn test_stability_infuser_infuse_levels() {
        let mut infuser = StabilityInfuser::with_max_energy(200.0);
        infuser.add_energy(200.0);

        // Level 1: 10-24 energy
        let result = infuser.infuse("armor", 15.0);
        assert_eq!(result, Some("armor_stability_infused".to_string()));

        // Level 2: 25-49 energy -> needs >=50 for floor to give 2
        let result = infuser.infuse("armor", 50.0);
        assert_eq!(result, Some("armor_stability_enhanced".to_string()));

        // Level 3: 50+ energy
        let result = infuser.infuse("armor", 75.0);
        assert_eq!(result, Some("armor_stability_perfected".to_string()));
    }

    #[test]
    fn test_stability_infuser_infuse_consumes_energy() {
        let mut infuser = StabilityInfuser::new();
        infuser.add_energy(50.0);

        infuser.infuse("item", 30.0);
        assert!((infuser.stored_energy() - 20.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_stability_infuser_infuse_insufficient_energy() {
        let mut infuser = StabilityInfuser::new();
        infuser.add_energy(5.0);

        let result = infuser.infuse("item", 20.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_stability_infuser_infuse_below_minimum() {
        let mut infuser = StabilityInfuser::new();
        infuser.add_energy(50.0);

        let result = infuser.infuse("item", 5.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_stability_infuser_infuse_not_operational() {
        let mut infuser = StabilityInfuser::new();
        infuser.add_energy(50.0);
        infuser.set_operational(false);

        let result = infuser.infuse("item", 20.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_stability_infuser_can_infuse() {
        let mut infuser = StabilityInfuser::new();
        assert!(!infuser.can_infuse());

        infuser.add_energy(10.0);
        assert!(infuser.can_infuse());

        infuser.set_operational(false);
        assert!(!infuser.can_infuse());
    }

    #[test]
    fn test_stability_infuser_energy_percentage() {
        let mut infuser = StabilityInfuser::new();
        assert!((infuser.energy_percentage() - 0.0).abs() < f32::EPSILON);

        infuser.add_energy(50.0);
        assert!((infuser.energy_percentage() - 0.5).abs() < f32::EPSILON);

        infuser.add_energy(50.0);
        assert!((infuser.energy_percentage() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_stability_infuser_set_operational() {
        let mut infuser = StabilityInfuser::new();
        assert!(infuser.is_operational());

        infuser.set_operational(false);
        assert!(!infuser.is_operational());
    }
}
