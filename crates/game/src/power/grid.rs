//! Power grid management.
//!
//! Tracks supply, demand, and battery storage.

use serde::{Deserialize, Serialize};

/// Default battery capacity.
const DEFAULT_BATTERY_CAPACITY: f32 = 50.0;

/// Power grid for the station.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PowerGrid {
    /// Current power supply.
    supply: f32,
    /// Current power demand.
    demand: f32,
    /// Battery capacity.
    battery: f32,
    /// Current battery charge.
    battery_charge: f32,
}

impl Default for PowerGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl PowerGrid {
    /// Create a new power grid.
    #[must_use]
    pub fn new() -> Self {
        Self {
            supply: 0.0,
            demand: 0.0,
            battery: DEFAULT_BATTERY_CAPACITY,
            battery_charge: DEFAULT_BATTERY_CAPACITY,
        }
    }

    /// Get the current supply.
    #[must_use]
    pub fn supply(&self) -> f32 {
        self.supply
    }

    /// Get the current demand.
    #[must_use]
    pub fn demand(&self) -> f32 {
        self.demand
    }

    /// Get the battery capacity.
    #[must_use]
    pub fn battery_capacity(&self) -> f32 {
        self.battery
    }

    /// Get the current battery charge.
    #[must_use]
    pub fn battery_charge(&self) -> f32 {
        self.battery_charge
    }

    /// Get battery charge as a percentage.
    #[must_use]
    pub fn battery_percentage(&self) -> f32 {
        if self.battery <= 0.0 {
            return 0.0;
        }
        (self.battery_charge / self.battery) * 100.0
    }

    /// Add to the power supply.
    pub fn add_supply(&mut self, amount: f32) {
        self.supply += amount.max(0.0);
    }

    /// Set the power supply.
    pub fn set_supply(&mut self, amount: f32) {
        self.supply = amount.max(0.0);
    }

    /// Add to the power demand.
    pub fn add_demand(&mut self, amount: f32) {
        self.demand += amount.max(0.0);
    }

    /// Set the power demand.
    pub fn set_demand(&mut self, amount: f32) {
        self.demand = amount.max(0.0);
    }

    /// Reset supply and demand for recalculation.
    pub fn reset(&mut self) {
        self.supply = 0.0;
        self.demand = 0.0;
    }

    /// Check if supply meets or exceeds demand.
    #[must_use]
    pub fn is_balanced(&self) -> bool {
        self.supply >= self.demand
    }

    /// Get the power deficit (demand - supply).
    ///
    /// Returns 0 if supply exceeds demand.
    #[must_use]
    pub fn deficit(&self) -> f32 {
        (self.demand - self.supply).max(0.0)
    }

    /// Get the power surplus (supply - demand).
    ///
    /// Returns 0 if demand exceeds supply.
    #[must_use]
    pub fn surplus(&self) -> f32 {
        (self.supply - self.demand).max(0.0)
    }

    /// Charge the battery from surplus power.
    pub fn charge_battery(&mut self, dt: f32) {
        let surplus = self.surplus();
        let charge_amount = (surplus * dt).min(self.battery - self.battery_charge);
        self.battery_charge += charge_amount;
    }

    /// Drain the battery to cover deficit.
    ///
    /// Returns the amount of power provided.
    pub fn drain_battery(&mut self, dt: f32) -> f32 {
        let deficit = self.deficit();
        let drain_amount = (deficit * dt).min(self.battery_charge);
        self.battery_charge -= drain_amount;
        drain_amount
    }

    /// Check if the battery is empty.
    #[must_use]
    pub fn battery_empty(&self) -> bool {
        self.battery_charge <= 0.0
    }

    /// Check if the battery is full.
    #[must_use]
    pub fn battery_full(&self) -> bool {
        self.battery_charge >= self.battery
    }

    /// Set battery charge directly (for testing).
    pub fn set_battery_charge(&mut self, charge: f32) {
        self.battery_charge = charge.clamp(0.0, self.battery);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_grid_new() {
        let grid = PowerGrid::new();
        assert!((grid.supply() - 0.0).abs() < f32::EPSILON);
        assert!((grid.demand() - 0.0).abs() < f32::EPSILON);
        assert!((grid.battery_capacity() - 50.0).abs() < f32::EPSILON);
        assert!((grid.battery_charge() - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_power_grid_add_supply() {
        let mut grid = PowerGrid::new();
        grid.add_supply(100.0);
        assert!((grid.supply() - 100.0).abs() < f32::EPSILON);

        grid.add_supply(50.0);
        assert!((grid.supply() - 150.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_power_grid_add_demand() {
        let mut grid = PowerGrid::new();
        grid.add_demand(30.0);
        assert!((grid.demand() - 30.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_power_grid_set_supply() {
        let mut grid = PowerGrid::new();
        grid.add_supply(100.0);
        grid.set_supply(50.0);
        assert!((grid.supply() - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_power_grid_set_demand() {
        let mut grid = PowerGrid::new();
        grid.set_demand(75.0);
        assert!((grid.demand() - 75.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_power_grid_is_balanced() {
        let mut grid = PowerGrid::new();
        grid.add_supply(100.0);
        grid.add_demand(50.0);
        assert!(grid.is_balanced());

        grid.add_demand(60.0);
        assert!(!grid.is_balanced());
    }

    #[test]
    fn test_power_grid_deficit() {
        let mut grid = PowerGrid::new();
        grid.add_supply(50.0);
        grid.add_demand(80.0);
        assert!((grid.deficit() - 30.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_power_grid_deficit_zero() {
        let mut grid = PowerGrid::new();
        grid.add_supply(100.0);
        grid.add_demand(50.0);
        assert!((grid.deficit() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_power_grid_surplus() {
        let mut grid = PowerGrid::new();
        grid.add_supply(100.0);
        grid.add_demand(60.0);
        assert!((grid.surplus() - 40.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_power_grid_surplus_zero() {
        let mut grid = PowerGrid::new();
        grid.add_supply(50.0);
        grid.add_demand(80.0);
        assert!((grid.surplus() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_power_grid_charge_battery() {
        let mut grid = PowerGrid::new();
        grid.battery_charge = 30.0;
        grid.add_supply(100.0);
        grid.add_demand(80.0);

        grid.charge_battery(1.0);
        assert!(grid.battery_charge() > 30.0);
    }

    #[test]
    fn test_power_grid_charge_battery_max() {
        let mut grid = PowerGrid::new();
        grid.add_supply(100.0);
        grid.add_demand(50.0);

        grid.charge_battery(1.0);
        assert!((grid.battery_charge() - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_power_grid_drain_battery() {
        let mut grid = PowerGrid::new();
        grid.add_supply(50.0);
        grid.add_demand(80.0);

        let drained = grid.drain_battery(1.0);
        assert!(drained > 0.0);
        assert!(grid.battery_charge() < 50.0);
    }

    #[test]
    fn test_power_grid_drain_battery_empty() {
        let mut grid = PowerGrid::new();
        grid.battery_charge = 10.0;
        grid.add_supply(50.0);
        grid.add_demand(100.0);

        let drained = grid.drain_battery(1.0);
        assert!((drained - 10.0).abs() < f32::EPSILON);
        assert!(grid.battery_empty());
    }

    #[test]
    fn test_power_grid_battery_percentage() {
        let mut grid = PowerGrid::new();
        assert!((grid.battery_percentage() - 100.0).abs() < f32::EPSILON);

        grid.battery_charge = 25.0;
        assert!((grid.battery_percentage() - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_power_grid_battery_empty() {
        let mut grid = PowerGrid::new();
        assert!(!grid.battery_empty());

        grid.battery_charge = 0.0;
        assert!(grid.battery_empty());
    }

    #[test]
    fn test_power_grid_battery_full() {
        let grid = PowerGrid::new();
        assert!(grid.battery_full());
    }

    #[test]
    fn test_power_grid_reset() {
        let mut grid = PowerGrid::new();
        grid.add_supply(100.0);
        grid.add_demand(50.0);
        grid.reset();
        assert!((grid.supply() - 0.0).abs() < f32::EPSILON);
        assert!((grid.demand() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_power_grid_default() {
        let grid = PowerGrid::default();
        assert!((grid.battery_capacity() - 50.0).abs() < f32::EPSILON);
    }
}
