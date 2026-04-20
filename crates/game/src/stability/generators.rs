//! Stability generators for energy production.
//!
//! Structures that convert resources into stability energy.

use serde::{Deserialize, Serialize};

/// Default conversion rate in units per minute.
pub const DEFAULT_CONVERSION_RATE: f32 = 10.0;

/// A stability generator that produces energy over time.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StabilityGenerator {
    /// Whether the generator is operational.
    operational: bool,
    /// Conversion rate in units per minute.
    conversion_rate: f32,
}

impl StabilityGenerator {
    /// Create a new operational generator with default conversion rate.
    #[must_use]
    pub fn new() -> Self {
        Self {
            operational: true,
            conversion_rate: DEFAULT_CONVERSION_RATE,
        }
    }

    /// Create a generator with a custom conversion rate.
    #[must_use]
    pub fn with_rate(conversion_rate: f32) -> Self {
        Self {
            operational: true,
            conversion_rate: conversion_rate.max(0.0),
        }
    }

    /// Generate energy for this tick.
    ///
    /// Returns the amount of energy produced based on delta time.
    pub fn generate(&self, dt: f32) -> f32 {
        if !self.operational || dt <= 0.0 {
            return 0.0;
        }

        // Convert from units/minute to units/second
        let rate_per_second = self.conversion_rate / 60.0;
        rate_per_second * dt
    }

    /// Toggle the operational state.
    pub fn toggle(&mut self) {
        self.operational = !self.operational;
    }

    /// Check if the generator is operational.
    #[must_use]
    pub fn is_operational(&self) -> bool {
        self.operational
    }

    /// Set the operational state.
    pub fn set_operational(&mut self, operational: bool) {
        self.operational = operational;
    }

    /// Get the conversion rate in units per minute.
    #[must_use]
    pub fn conversion_rate(&self) -> f32 {
        self.conversion_rate
    }
}

impl Default for StabilityGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_new() {
        let generator = StabilityGenerator::new();
        assert!(generator.is_operational());
        assert!((generator.conversion_rate() - 10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_generator_with_rate() {
        let generator = StabilityGenerator::with_rate(30.0);
        assert!((generator.conversion_rate() - 30.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_generator_generate() {
        let generator = StabilityGenerator::new(); // 10 units/min

        // 60 seconds should produce 10 units
        let produced = generator.generate(60.0);
        assert!((produced - 10.0).abs() < f32::EPSILON);

        // 1 second should produce 10/60 units
        let produced = generator.generate(1.0);
        assert!((produced - (10.0 / 60.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn test_generator_generate_when_off() {
        let mut generator = StabilityGenerator::new();
        generator.toggle();

        let produced = generator.generate(60.0);
        assert!((produced - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_generator_toggle() {
        let mut generator = StabilityGenerator::new();
        assert!(generator.is_operational());

        generator.toggle();
        assert!(!generator.is_operational());

        generator.toggle();
        assert!(generator.is_operational());
    }
}
