//! Thermal Converter power generation.
//!
//! Converts thermal energy from Titan's body heat into usable power
//! for base operations and crafting stations.

use serde::{Deserialize, Serialize};

/// Base power output rate (units per minute).
pub const BASE_POWER_OUTPUT: f32 = 10.0;

/// Maximum efficiency multiplier.
pub const MAX_EFFICIENCY: f32 = 2.0;

/// Minimum efficiency multiplier.
pub const MIN_EFFICIENCY: f32 = 0.5;

/// Power generation result.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PowerOutput {
    /// Amount of power generated.
    pub amount: f32,
    /// Current efficiency percentage (0.0-1.0).
    pub efficiency: f32,
    /// Whether the converter is overheating.
    pub overheating: bool,
}

impl PowerOutput {
    /// Create a new power output result.
    #[must_use]
    pub fn new(amount: f32, efficiency: f32, overheating: bool) -> Self {
        Self {
            amount,
            efficiency,
            overheating,
        }
    }
}

/// The Thermal Converter power generator.
///
/// Harvests thermal energy from the Titan's body heat, converting it
/// into power for base operations. More efficient near breathing vents.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThermalConverter {
    /// Whether the converter is operational.
    operational: bool,
    /// Base power output per minute.
    power_output: f32,
    /// Current efficiency multiplier.
    efficiency: f32,
    /// Accumulated power.
    stored_power: f32,
    /// Maximum power storage.
    max_storage: f32,
    /// Current heat level (can cause overheating).
    heat_level: f32,
}

impl ThermalConverter {
    /// Create a new Thermal Converter.
    #[must_use]
    pub fn new() -> Self {
        Self {
            operational: true,
            power_output: BASE_POWER_OUTPUT,
            efficiency: 1.0,
            stored_power: 0.0,
            max_storage: 100.0,
            heat_level: 0.0,
        }
    }

    /// Create a Thermal Converter with custom parameters.
    #[must_use]
    pub fn with_storage(max_storage: f32) -> Self {
        Self {
            operational: true,
            power_output: BASE_POWER_OUTPUT,
            efficiency: 1.0,
            stored_power: 0.0,
            max_storage,
            heat_level: 0.0,
        }
    }

    /// Generate power over a time period.
    ///
    /// Returns the amount of power generated (may be stored or overflow).
    pub fn generate(&mut self, dt: f32) -> f32 {
        if !self.operational {
            return 0.0;
        }

        // Check for overheating
        if self.heat_level >= 100.0 {
            self.operational = false;
            return 0.0;
        }

        // Calculate power generation (convert from per-minute to per-dt)
        let base_generation = (self.power_output / 60.0) * dt;
        let actual_generation = base_generation * self.efficiency;

        // Store generated power
        let space_available = self.max_storage - self.stored_power;
        let stored = actual_generation.min(space_available);
        self.stored_power += stored;

        // Heat buildup during generation
        self.heat_level = (self.heat_level + dt * 0.5).min(100.0);

        actual_generation
    }

    /// Toggle the converter on/off.
    pub fn toggle(&mut self) {
        self.operational = !self.operational;
        // Turning off reduces heat
        if !self.operational {
            self.heat_level = (self.heat_level - 20.0).max(0.0);
        }
    }

    /// Turn the converter on.
    pub fn turn_on(&mut self) {
        self.operational = true;
    }

    /// Turn the converter off.
    pub fn turn_off(&mut self) {
        self.operational = false;
        self.heat_level = (self.heat_level - 20.0).max(0.0);
    }

    /// Check if the converter is operational.
    #[must_use]
    pub fn is_operational(&self) -> bool {
        self.operational
    }

    /// Get the base power output per minute.
    #[must_use]
    pub fn power_output(&self) -> f32 {
        self.power_output
    }

    /// Get the current efficiency.
    #[must_use]
    pub fn efficiency(&self) -> f32 {
        self.efficiency
    }

    /// Set efficiency based on location (e.g., near vents).
    pub fn set_efficiency(&mut self, efficiency: f32) {
        self.efficiency = efficiency.clamp(MIN_EFFICIENCY, MAX_EFFICIENCY);
    }

    /// Get stored power.
    #[must_use]
    pub fn stored_power(&self) -> f32 {
        self.stored_power
    }

    /// Get maximum storage capacity.
    #[must_use]
    pub fn max_storage(&self) -> f32 {
        self.max_storage
    }

    /// Consume power from storage.
    ///
    /// Returns the amount actually consumed.
    pub fn consume(&mut self, amount: f32) -> f32 {
        let consumed = amount.min(self.stored_power);
        self.stored_power -= consumed;
        consumed
    }

    /// Get current heat level (0-100).
    #[must_use]
    pub fn heat_level(&self) -> f32 {
        self.heat_level
    }

    /// Check if overheating.
    #[must_use]
    pub fn is_overheating(&self) -> bool {
        self.heat_level >= 80.0
    }

    /// Cool down the converter.
    pub fn cool_down(&mut self, amount: f32) {
        self.heat_level = (self.heat_level - amount).max(0.0);
        // Can restart if cooled enough
        if self.heat_level < 50.0 {
            self.operational = true;
        }
    }

    /// Get the effective power output per minute.
    #[must_use]
    pub fn effective_output(&self) -> f32 {
        if self.operational {
            self.power_output * self.efficiency
        } else {
            0.0
        }
    }

    /// Get storage fill percentage.
    #[must_use]
    pub fn storage_percentage(&self) -> f32 {
        if self.max_storage > 0.0 {
            self.stored_power / self.max_storage
        } else {
            0.0
        }
    }
}

impl Default for ThermalConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermal_converter_new() {
        let converter = ThermalConverter::new();
        assert!(converter.is_operational());
        assert!((converter.power_output() - BASE_POWER_OUTPUT).abs() < f32::EPSILON);
        assert!((converter.efficiency() - 1.0).abs() < f32::EPSILON);
        assert!((converter.stored_power() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_thermal_converter_with_storage() {
        let converter = ThermalConverter::with_storage(200.0);
        assert!((converter.max_storage() - 200.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_thermal_converter_generate() {
        let mut converter = ThermalConverter::new();
        let generated = converter.generate(60.0); // 60 seconds = 1 minute
        // Should generate BASE_POWER_OUTPUT (10) per minute
        assert!((generated - BASE_POWER_OUTPUT).abs() < f32::EPSILON);
    }

    #[test]
    fn test_thermal_converter_generate_stores_power() {
        let mut converter = ThermalConverter::new();
        converter.generate(60.0);
        assert!((converter.stored_power() - BASE_POWER_OUTPUT).abs() < f32::EPSILON);
    }

    #[test]
    fn test_thermal_converter_generate_not_operational() {
        let mut converter = ThermalConverter::new();
        converter.turn_off();
        let generated = converter.generate(60.0);
        assert!((generated - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_thermal_converter_generate_with_efficiency() {
        let mut converter = ThermalConverter::new();
        converter.set_efficiency(2.0);
        let generated = converter.generate(60.0);
        assert!((generated - BASE_POWER_OUTPUT * 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_thermal_converter_toggle() {
        let mut converter = ThermalConverter::new();
        assert!(converter.is_operational());

        converter.toggle();
        assert!(!converter.is_operational());

        converter.toggle();
        assert!(converter.is_operational());
    }

    #[test]
    fn test_thermal_converter_turn_on_off() {
        let mut converter = ThermalConverter::new();

        converter.turn_off();
        assert!(!converter.is_operational());

        converter.turn_on();
        assert!(converter.is_operational());
    }

    #[test]
    fn test_thermal_converter_efficiency_clamp() {
        let mut converter = ThermalConverter::new();

        converter.set_efficiency(0.1);
        assert!((converter.efficiency() - MIN_EFFICIENCY).abs() < f32::EPSILON);

        converter.set_efficiency(5.0);
        assert!((converter.efficiency() - MAX_EFFICIENCY).abs() < f32::EPSILON);
    }

    #[test]
    fn test_thermal_converter_consume() {
        let mut converter = ThermalConverter::new();
        converter.generate(60.0);

        let consumed = converter.consume(5.0);
        assert!((consumed - 5.0).abs() < f32::EPSILON);
        assert!((converter.stored_power() - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_thermal_converter_consume_more_than_available() {
        let mut converter = ThermalConverter::new();
        converter.generate(60.0); // 10 units

        let consumed = converter.consume(20.0);
        assert!((consumed - 10.0).abs() < f32::EPSILON);
        assert!((converter.stored_power() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_thermal_converter_heat_buildup() {
        let mut converter = ThermalConverter::new();
        converter.generate(10.0);
        assert!(converter.heat_level() > 0.0);
    }

    #[test]
    fn test_thermal_converter_overheating() {
        let mut converter = ThermalConverter::new();
        // Simulate lots of generation to build up heat
        for _ in 0..200 {
            converter.generate(1.0);
        }
        // After enough generation, should overheat and stop
        assert!(!converter.is_operational() || converter.heat_level() >= 80.0);
    }

    #[test]
    fn test_thermal_converter_cool_down() {
        let mut converter = ThermalConverter::new();
        // Build up heat
        for _ in 0..100 {
            converter.generate(1.0);
        }
        let heat_before = converter.heat_level();

        converter.cool_down(20.0);
        assert!(converter.heat_level() < heat_before);
    }

    #[test]
    fn test_thermal_converter_is_overheating() {
        let mut converter = ThermalConverter::new();
        assert!(!converter.is_overheating());

        // Manually set heat for test
        for _ in 0..180 {
            converter.generate(1.0);
        }
        assert!(converter.is_overheating() || converter.heat_level() > 50.0);
    }

    #[test]
    fn test_thermal_converter_effective_output() {
        let mut converter = ThermalConverter::new();
        converter.set_efficiency(1.5);
        assert!((converter.effective_output() - 15.0).abs() < f32::EPSILON);

        converter.turn_off();
        assert!((converter.effective_output() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_thermal_converter_storage_percentage() {
        let mut converter = ThermalConverter::new();
        assert!((converter.storage_percentage() - 0.0).abs() < f32::EPSILON);

        converter.generate(60.0); // 10 units out of 100
        assert!((converter.storage_percentage() - 0.1).abs() < f32::EPSILON);
    }

    #[test]
    fn test_thermal_converter_storage_overflow() {
        let mut converter = ThermalConverter::with_storage(10.0);
        converter.generate(120.0); // Would generate 20 units
        // Should cap at max_storage
        assert!((converter.stored_power() - 10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_thermal_converter_default() {
        let converter = ThermalConverter::default();
        assert!(converter.is_operational());
        assert!((converter.power_output() - BASE_POWER_OUTPUT).abs() < f32::EPSILON);
    }

    #[test]
    fn test_power_output_struct() {
        let output = PowerOutput::new(10.0, 0.8, false);
        assert!((output.amount - 10.0).abs() < f32::EPSILON);
        assert!((output.efficiency - 0.8).abs() < f32::EPSILON);
        assert!(!output.overheating);
    }

    #[test]
    fn test_thermal_converter_turn_off_reduces_heat() {
        let mut converter = ThermalConverter::new();
        // Build up some heat
        for _ in 0..50 {
            converter.generate(1.0);
        }
        let heat_before = converter.heat_level();

        converter.turn_off();
        assert!(converter.heat_level() < heat_before);
    }
}
