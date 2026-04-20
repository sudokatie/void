//! Thirst system for time-loop survival.
//!
//! Tracks player hydration with a faster drain rate than hunger.

use serde::{Deserialize, Serialize};

/// Default maximum thirst value.
pub const DEFAULT_MAX_THIRST: f32 = 100.0;

/// Base thirst drain rate per tick.
pub const BASE_THIRST_DRAIN: f32 = 1.5;

/// Thirst component for players.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Thirst {
    /// Current thirst level (0-100).
    value: f32,
    /// Drain rate per tick.
    drain_rate: f32,
}

impl Thirst {
    /// Create a new thirst at full hydration.
    #[must_use]
    pub fn new() -> Self {
        Self {
            value: DEFAULT_MAX_THIRST,
            drain_rate: BASE_THIRST_DRAIN,
        }
    }

    /// Update thirst over time.
    ///
    /// Returns the current thirst value after draining.
    pub fn tick(&mut self, dt: f32) -> f32 {
        self.value = (self.value - self.drain_rate * dt).max(0.0);
        self.value
    }

    /// Restore thirst by drinking.
    pub fn drink(&mut self, amount: f32) {
        self.value = (self.value + amount).min(DEFAULT_MAX_THIRST);
    }

    /// Check if player is dehydrated.
    #[must_use]
    pub fn is_dehydrated(&self) -> bool {
        self.value <= 0.0
    }

    /// Get current thirst value.
    #[must_use]
    pub fn value(&self) -> f32 {
        self.value
    }

    /// Get thirst as a fraction (0.0 to 1.0).
    #[must_use]
    pub fn fraction(&self) -> f32 {
        (self.value / DEFAULT_MAX_THIRST).clamp(0.0, 1.0)
    }

    /// Check if thirst is low (warning threshold).
    #[must_use]
    pub fn is_low(&self) -> bool {
        self.value <= 25.0
    }

    /// Fully restore thirst.
    pub fn restore(&mut self) {
        self.value = DEFAULT_MAX_THIRST;
    }

    /// Get the drain rate.
    #[must_use]
    pub fn drain_rate(&self) -> f32 {
        self.drain_rate
    }
}

impl Default for Thirst {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thirst_new() {
        let thirst = Thirst::new();
        assert!((thirst.value() - 100.0).abs() < f32::EPSILON);
        assert!(!thirst.is_dehydrated());
        assert!(!thirst.is_low());
    }

    #[test]
    fn test_thirst_tick() {
        let mut thirst = Thirst::new();
        let value = thirst.tick(1.0);
        assert!((value - 98.5).abs() < f32::EPSILON); // 100 - 1.5*1
    }

    #[test]
    fn test_thirst_tick_multiple() {
        let mut thirst = Thirst::new();
        thirst.tick(10.0);
        assert!((thirst.value() - 85.0).abs() < f32::EPSILON); // 100 - 1.5*10
    }

    #[test]
    fn test_thirst_drink() {
        let mut thirst = Thirst::new();
        thirst.tick(20.0); // Drain to 70
        thirst.drink(15.0);
        assert!((thirst.value() - 85.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_thirst_drink_caps_at_max() {
        let mut thirst = Thirst::new();
        thirst.tick(10.0);
        thirst.drink(100.0);
        assert!((thirst.value() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_thirst_is_dehydrated() {
        let mut thirst = Thirst::new();
        assert!(!thirst.is_dehydrated());

        // Drain completely
        thirst.tick(100.0);
        assert!(thirst.is_dehydrated());
    }

    #[test]
    fn test_thirst_fraction() {
        let mut thirst = Thirst::new();
        assert!((thirst.fraction() - 1.0).abs() < f32::EPSILON);

        thirst.tick(50.0 / 1.5); // Drain to 50
        assert!((thirst.fraction() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_thirst_is_low() {
        let mut thirst = Thirst::new();
        assert!(!thirst.is_low());

        thirst.value = 25.0;
        assert!(thirst.is_low());

        thirst.value = 20.0;
        assert!(thirst.is_low());
    }

    #[test]
    fn test_thirst_restore() {
        let mut thirst = Thirst::new();
        thirst.tick(50.0);
        thirst.restore();
        assert!((thirst.value() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_thirst_drain_rate() {
        let thirst = Thirst::new();
        assert!((thirst.drain_rate() - BASE_THIRST_DRAIN).abs() < f32::EPSILON);
    }

    #[test]
    fn test_thirst_cannot_go_negative() {
        let mut thirst = Thirst::new();
        thirst.tick(1000.0);
        assert!((thirst.value() - 0.0).abs() < f32::EPSILON);
    }
}
