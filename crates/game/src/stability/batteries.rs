//! Stability batteries for energy storage.
//!
//! Portable energy storage devices of varying capacities.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Battery capacity tiers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StabilityBatteryTier {
    /// Small battery (100 capacity).
    Small,
    /// Medium battery (500 capacity).
    Medium,
    /// Large battery (2000 capacity).
    Large,
}

impl StabilityBatteryTier {
    /// Get the capacity for this tier.
    #[must_use]
    pub fn capacity(&self) -> f32 {
        match self {
            StabilityBatteryTier::Small => 100.0,
            StabilityBatteryTier::Medium => 500.0,
            StabilityBatteryTier::Large => 2000.0,
        }
    }

    /// Get all tier variants.
    #[must_use]
    pub fn all() -> &'static [StabilityBatteryTier] {
        &[
            StabilityBatteryTier::Small,
            StabilityBatteryTier::Medium,
            StabilityBatteryTier::Large,
        ]
    }
}

impl fmt::Display for StabilityBatteryTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StabilityBatteryTier::Small => write!(f, "Small"),
            StabilityBatteryTier::Medium => write!(f, "Medium"),
            StabilityBatteryTier::Large => write!(f, "Large"),
        }
    }
}

/// A stability battery for storing energy.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StabilityBattery {
    /// Battery tier.
    tier: StabilityBatteryTier,
    /// Maximum capacity.
    capacity: f32,
    /// Current charge level.
    charge: f32,
}

impl StabilityBattery {
    /// Create a new fully charged battery of the given tier.
    #[must_use]
    pub fn new(tier: StabilityBatteryTier) -> Self {
        let capacity = tier.capacity();
        Self {
            tier,
            capacity,
            charge: capacity,
        }
    }

    /// Discharge energy from the battery.
    ///
    /// Returns the actual amount discharged (may be less if insufficient).
    pub fn discharge(&mut self, amount: f32) -> f32 {
        if amount <= 0.0 {
            return 0.0;
        }

        let discharged = amount.min(self.charge);
        self.charge -= discharged;
        discharged
    }

    /// Recharge the battery.
    ///
    /// Returns the actual amount recharged (capped at capacity).
    pub fn recharge(&mut self, amount: f32) -> f32 {
        if amount <= 0.0 {
            return 0.0;
        }

        let space = self.capacity - self.charge;
        let recharged = amount.min(space);
        self.charge += recharged;
        recharged
    }

    /// Get remaining charge as a percentage (0.0 to 1.0).
    #[must_use]
    pub fn remaining_percentage(&self) -> f32 {
        if self.capacity <= 0.0 {
            return 0.0;
        }
        (self.charge / self.capacity).clamp(0.0, 1.0)
    }

    /// Check if the battery is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.charge <= 0.0
    }

    /// Check if the battery is fully charged.
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.charge >= self.capacity
    }

    /// Get the battery tier.
    #[must_use]
    pub fn tier(&self) -> StabilityBatteryTier {
        self.tier
    }

    /// Get the battery capacity.
    #[must_use]
    pub fn capacity(&self) -> f32 {
        self.capacity
    }

    /// Get the current charge level.
    #[must_use]
    pub fn charge(&self) -> f32 {
        self.charge
    }
}

/// Anchor fuel cell tiers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnchorFuelCellTier {
    /// Small fuel cell (50 fuel).
    Small,
    /// Medium fuel cell (200 fuel).
    Medium,
    /// Large fuel cell (500 fuel).
    Large,
}

impl AnchorFuelCellTier {
    /// Get the fuel amount for this tier.
    #[must_use]
    pub fn fuel_amount(&self) -> f32 {
        match self {
            AnchorFuelCellTier::Small => 50.0,
            AnchorFuelCellTier::Medium => 200.0,
            AnchorFuelCellTier::Large => 500.0,
        }
    }

    /// Get all tier variants.
    #[must_use]
    pub fn all() -> &'static [AnchorFuelCellTier] {
        &[
            AnchorFuelCellTier::Small,
            AnchorFuelCellTier::Medium,
            AnchorFuelCellTier::Large,
        ]
    }
}

impl fmt::Display for AnchorFuelCellTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnchorFuelCellTier::Small => write!(f, "Small"),
            AnchorFuelCellTier::Medium => write!(f, "Medium"),
            AnchorFuelCellTier::Large => write!(f, "Large"),
        }
    }
}

/// A fuel cell for powering dimensional anchors.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnchorFuelCell {
    /// Fuel cell tier.
    tier: AnchorFuelCellTier,
    /// Remaining fuel amount.
    fuel_amount: f32,
}

impl AnchorFuelCell {
    /// Create a new fuel cell of the given tier.
    #[must_use]
    pub fn new(tier: AnchorFuelCellTier) -> Self {
        Self {
            tier,
            fuel_amount: tier.fuel_amount(),
        }
    }

    /// Create a fuel cell from a tier string.
    #[must_use]
    pub fn from_tier_str(tier: &str) -> Option<Self> {
        let cell_tier = match tier.to_lowercase().as_str() {
            "small" => AnchorFuelCellTier::Small,
            "medium" => AnchorFuelCellTier::Medium,
            "large" => AnchorFuelCellTier::Large,
            _ => return None,
        };
        Some(Self::new(cell_tier))
    }

    /// Use fuel from the cell.
    ///
    /// Returns the actual amount of fuel used (may be less if insufficient).
    pub fn use_fuel(&mut self, amount: f32) -> f32 {
        if amount <= 0.0 {
            return 0.0;
        }

        let used = amount.min(self.fuel_amount);
        self.fuel_amount -= used;
        used
    }

    /// Get the remaining fuel amount.
    #[must_use]
    pub fn fuel_remaining(&self) -> f32 {
        self.fuel_amount
    }

    /// Get the fuel cell tier.
    #[must_use]
    pub fn tier(&self) -> AnchorFuelCellTier {
        self.tier
    }

    /// Get remaining fuel as a percentage (0.0 to 1.0).
    #[must_use]
    pub fn remaining_percentage(&self) -> f32 {
        let max = self.tier.fuel_amount();
        if max <= 0.0 {
            return 0.0;
        }
        (self.fuel_amount / max).clamp(0.0, 1.0)
    }

    /// Check if the fuel cell is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.fuel_amount <= 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battery_tier_capacity() {
        assert!((StabilityBatteryTier::Small.capacity() - 100.0).abs() < f32::EPSILON);
        assert!((StabilityBatteryTier::Medium.capacity() - 500.0).abs() < f32::EPSILON);
        assert!((StabilityBatteryTier::Large.capacity() - 2000.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_battery_tier_display() {
        assert_eq!(format!("{}", StabilityBatteryTier::Small), "Small");
        assert_eq!(format!("{}", StabilityBatteryTier::Medium), "Medium");
        assert_eq!(format!("{}", StabilityBatteryTier::Large), "Large");
    }

    #[test]
    fn test_battery_new() {
        let battery = StabilityBattery::new(StabilityBatteryTier::Medium);
        assert_eq!(battery.tier(), StabilityBatteryTier::Medium);
        assert!((battery.capacity() - 500.0).abs() < f32::EPSILON);
        assert!((battery.charge() - 500.0).abs() < f32::EPSILON);
        assert!(battery.is_full());
    }

    #[test]
    fn test_battery_discharge() {
        let mut battery = StabilityBattery::new(StabilityBatteryTier::Small);

        let discharged = battery.discharge(30.0);
        assert!((discharged - 30.0).abs() < f32::EPSILON);
        assert!((battery.charge() - 70.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_battery_discharge_insufficient() {
        let mut battery = StabilityBattery::new(StabilityBatteryTier::Small);
        battery.discharge(80.0);

        let discharged = battery.discharge(50.0);
        assert!((discharged - 20.0).abs() < f32::EPSILON);
        assert!(battery.is_empty());
    }

    #[test]
    fn test_battery_recharge() {
        let mut battery = StabilityBattery::new(StabilityBatteryTier::Small);
        battery.discharge(50.0);

        let recharged = battery.recharge(30.0);
        assert!((recharged - 30.0).abs() < f32::EPSILON);
        assert!((battery.charge() - 80.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_battery_recharge_capped() {
        let mut battery = StabilityBattery::new(StabilityBatteryTier::Small);
        battery.discharge(20.0);

        let recharged = battery.recharge(50.0);
        assert!((recharged - 20.0).abs() < f32::EPSILON);
        assert!(battery.is_full());
    }

    #[test]
    fn test_battery_remaining_percentage() {
        let mut battery = StabilityBattery::new(StabilityBatteryTier::Small);
        assert!((battery.remaining_percentage() - 1.0).abs() < f32::EPSILON);

        battery.discharge(50.0);
        assert!((battery.remaining_percentage() - 0.5).abs() < f32::EPSILON);
    }

    // AnchorFuelCell tests
    #[test]
    fn test_fuel_cell_tier_fuel_amount() {
        assert!((AnchorFuelCellTier::Small.fuel_amount() - 50.0).abs() < f32::EPSILON);
        assert!((AnchorFuelCellTier::Medium.fuel_amount() - 200.0).abs() < f32::EPSILON);
        assert!((AnchorFuelCellTier::Large.fuel_amount() - 500.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_fuel_cell_tier_display() {
        assert_eq!(format!("{}", AnchorFuelCellTier::Small), "Small");
        assert_eq!(format!("{}", AnchorFuelCellTier::Medium), "Medium");
        assert_eq!(format!("{}", AnchorFuelCellTier::Large), "Large");
    }

    #[test]
    fn test_fuel_cell_new() {
        let cell = AnchorFuelCell::new(AnchorFuelCellTier::Medium);
        assert_eq!(cell.tier(), AnchorFuelCellTier::Medium);
        assert!((cell.fuel_remaining() - 200.0).abs() < f32::EPSILON);
        assert!(!cell.is_empty());
    }

    #[test]
    fn test_fuel_cell_from_tier_str() {
        let cell = AnchorFuelCell::from_tier_str("small");
        assert!(cell.is_some());
        assert_eq!(cell.unwrap().tier(), AnchorFuelCellTier::Small);

        let cell = AnchorFuelCell::from_tier_str("MEDIUM");
        assert!(cell.is_some());
        assert_eq!(cell.unwrap().tier(), AnchorFuelCellTier::Medium);

        let cell = AnchorFuelCell::from_tier_str("invalid");
        assert!(cell.is_none());
    }

    #[test]
    fn test_fuel_cell_use_fuel() {
        let mut cell = AnchorFuelCell::new(AnchorFuelCellTier::Small);

        let used = cell.use_fuel(20.0);
        assert!((used - 20.0).abs() < f32::EPSILON);
        assert!((cell.fuel_remaining() - 30.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_fuel_cell_use_fuel_insufficient() {
        let mut cell = AnchorFuelCell::new(AnchorFuelCellTier::Small);

        let used = cell.use_fuel(100.0);
        assert!((used - 50.0).abs() < f32::EPSILON);
        assert!(cell.is_empty());
    }

    #[test]
    fn test_fuel_cell_use_fuel_negative() {
        let mut cell = AnchorFuelCell::new(AnchorFuelCellTier::Small);

        let used = cell.use_fuel(-10.0);
        assert!((used - 0.0).abs() < f32::EPSILON);
        assert!((cell.fuel_remaining() - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_fuel_cell_remaining_percentage() {
        let mut cell = AnchorFuelCell::new(AnchorFuelCellTier::Small);
        assert!((cell.remaining_percentage() - 1.0).abs() < f32::EPSILON);

        cell.use_fuel(25.0);
        assert!((cell.remaining_percentage() - 0.5).abs() < f32::EPSILON);
    }
}
