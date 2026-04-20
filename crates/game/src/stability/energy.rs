//! Stability energy management.
//!
//! Core energy resource used to power dimensional stabilization systems.

use serde::{Deserialize, Serialize};

/// Stability energy pool for a player or structure.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StabilityEnergy {
    /// Current energy level.
    current: f32,
    /// Maximum energy capacity.
    max: f32,
}

impl StabilityEnergy {
    /// Create a new energy pool with specified maximum.
    #[must_use]
    pub fn new(max: f32) -> Self {
        Self {
            current: max,
            max: max.max(0.0),
        }
    }

    /// Consume energy from the pool.
    ///
    /// Returns the actual amount consumed (may be less if insufficient).
    pub fn consume(&mut self, amount: f32) -> f32 {
        if amount <= 0.0 {
            return 0.0;
        }

        let consumed = amount.min(self.current);
        self.current -= consumed;
        consumed
    }

    /// Harvest (add) energy to the pool.
    ///
    /// Returns the actual amount harvested (capped at max).
    pub fn harvest(&mut self, amount: f32) -> f32 {
        if amount <= 0.0 {
            return 0.0;
        }

        let space = self.max - self.current;
        let harvested = amount.min(space);
        self.current += harvested;
        harvested
    }

    /// Get current energy level.
    #[must_use]
    pub fn current_energy(&self) -> f32 {
        self.current
    }

    /// Get remaining energy as a percentage (0.0 to 1.0).
    #[must_use]
    pub fn remaining_percentage(&self) -> f32 {
        if self.max <= 0.0 {
            return 0.0;
        }
        (self.current / self.max).clamp(0.0, 1.0)
    }

    /// Check if energy is completely depleted.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.current <= 0.0
    }

    /// Check if energy is at maximum capacity.
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.current >= self.max
    }

    /// Get maximum energy capacity.
    #[must_use]
    pub fn max_energy(&self) -> f32 {
        self.max
    }
}

impl Default for StabilityEnergy {
    fn default() -> Self {
        Self::new(100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let energy = StabilityEnergy::new(100.0);
        assert!((energy.current_energy() - 100.0).abs() < f32::EPSILON);
        assert!((energy.max_energy() - 100.0).abs() < f32::EPSILON);
        assert!(energy.is_full());
        assert!(!energy.is_empty());
    }

    #[test]
    fn test_consume() {
        let mut energy = StabilityEnergy::new(100.0);

        let consumed = energy.consume(30.0);
        assert!((consumed - 30.0).abs() < f32::EPSILON);
        assert!((energy.current_energy() - 70.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_consume_insufficient() {
        let mut energy = StabilityEnergy::new(50.0);

        let consumed = energy.consume(80.0);
        assert!((consumed - 50.0).abs() < f32::EPSILON);
        assert!(energy.is_empty());
    }

    #[test]
    fn test_harvest() {
        let mut energy = StabilityEnergy::new(100.0);
        energy.consume(50.0);

        let harvested = energy.harvest(30.0);
        assert!((harvested - 30.0).abs() < f32::EPSILON);
        assert!((energy.current_energy() - 80.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_harvest_capped() {
        let mut energy = StabilityEnergy::new(100.0);
        energy.consume(20.0);

        let harvested = energy.harvest(50.0);
        assert!((harvested - 20.0).abs() < f32::EPSILON);
        assert!(energy.is_full());
    }

    #[test]
    fn test_remaining_percentage() {
        let mut energy = StabilityEnergy::new(100.0);
        assert!((energy.remaining_percentage() - 1.0).abs() < f32::EPSILON);

        energy.consume(50.0);
        assert!((energy.remaining_percentage() - 0.5).abs() < f32::EPSILON);

        energy.consume(50.0);
        assert!((energy.remaining_percentage() - 0.0).abs() < f32::EPSILON);
    }
}
