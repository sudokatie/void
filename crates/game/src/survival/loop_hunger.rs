//! Simplified hunger system for time-loop survival.
//!
//! A straightforward hunger drain with simple restoration mechanics.

use serde::{Deserialize, Serialize};

/// Default maximum hunger value.
pub const DEFAULT_MAX_HUNGER: f32 = 100.0;

/// Base hunger drain rate per tick.
pub const BASE_HUNGER_DRAIN: f32 = 1.0;

/// Hunger component for time-loop survival.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoopHunger {
    /// Current hunger level (0-100).
    value: f32,
    /// Drain rate per tick.
    drain_rate: f32,
}

impl LoopHunger {
    /// Create a new hunger at full satiation.
    #[must_use]
    pub fn new() -> Self {
        Self {
            value: DEFAULT_MAX_HUNGER,
            drain_rate: BASE_HUNGER_DRAIN,
        }
    }

    /// Update hunger over time.
    ///
    /// Returns the current hunger value after draining.
    pub fn tick(&mut self, dt: f32) -> f32 {
        self.value = (self.value - self.drain_rate * dt).max(0.0);
        self.value
    }

    /// Restore hunger by eating.
    pub fn eat(&mut self, amount: f32) {
        self.value = (self.value + amount).min(DEFAULT_MAX_HUNGER);
    }

    /// Check if player is starving.
    #[must_use]
    pub fn is_starving(&self) -> bool {
        self.value <= 0.0
    }

    /// Get current hunger value.
    #[must_use]
    pub fn value(&self) -> f32 {
        self.value
    }

    /// Get hunger as a fraction (0.0 to 1.0).
    #[must_use]
    pub fn fraction(&self) -> f32 {
        (self.value / DEFAULT_MAX_HUNGER).clamp(0.0, 1.0)
    }

    /// Check if hunger is low (warning threshold).
    #[must_use]
    pub fn is_low(&self) -> bool {
        self.value <= 25.0
    }

    /// Fully restore hunger.
    pub fn restore(&mut self) {
        self.value = DEFAULT_MAX_HUNGER;
    }

    /// Get the drain rate.
    #[must_use]
    pub fn drain_rate(&self) -> f32 {
        self.drain_rate
    }
}

impl Default for LoopHunger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hunger_new() {
        let hunger = LoopHunger::new();
        assert!((hunger.value() - 100.0).abs() < f32::EPSILON);
        assert!(!hunger.is_starving());
        assert!(!hunger.is_low());
    }

    #[test]
    fn test_hunger_tick() {
        let mut hunger = LoopHunger::new();
        let value = hunger.tick(1.0);
        assert!((value - 99.0).abs() < f32::EPSILON); // 100 - 1.0*1
    }

    #[test]
    fn test_hunger_tick_multiple() {
        let mut hunger = LoopHunger::new();
        hunger.tick(10.0);
        assert!((hunger.value() - 90.0).abs() < f32::EPSILON); // 100 - 1.0*10
    }

    #[test]
    fn test_hunger_eat() {
        let mut hunger = LoopHunger::new();
        hunger.tick(30.0); // Drain to 70
        hunger.eat(15.0);
        assert!((hunger.value() - 85.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hunger_eat_caps_at_max() {
        let mut hunger = LoopHunger::new();
        hunger.tick(10.0);
        hunger.eat(100.0);
        assert!((hunger.value() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hunger_is_starving() {
        let mut hunger = LoopHunger::new();
        assert!(!hunger.is_starving());

        // Drain completely
        hunger.tick(100.0);
        assert!(hunger.is_starving());
    }

    #[test]
    fn test_hunger_fraction() {
        let mut hunger = LoopHunger::new();
        assert!((hunger.fraction() - 1.0).abs() < f32::EPSILON);

        hunger.tick(50.0); // Drain to 50
        assert!((hunger.fraction() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hunger_is_low() {
        let mut hunger = LoopHunger::new();
        assert!(!hunger.is_low());

        hunger.value = 25.0;
        assert!(hunger.is_low());

        hunger.value = 20.0;
        assert!(hunger.is_low());
    }

    #[test]
    fn test_hunger_restore() {
        let mut hunger = LoopHunger::new();
        hunger.tick(50.0);
        hunger.restore();
        assert!((hunger.value() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hunger_drain_rate() {
        let hunger = LoopHunger::new();
        assert!((hunger.drain_rate() - BASE_HUNGER_DRAIN).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hunger_cannot_go_negative() {
        let mut hunger = LoopHunger::new();
        hunger.tick(1000.0);
        assert!((hunger.value() - 0.0).abs() < f32::EPSILON);
    }
}
