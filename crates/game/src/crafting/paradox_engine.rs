//! Paradox Engine crafting station for time-loop survival.
//!
//! Converts paradox energy into usable power.

use serde::{Deserialize, Serialize};

/// Power output from the Paradox Engine.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ParadoxPowerOutput {
    /// Power generated this tick.
    pub power: u32,
    /// Stability loss from generation.
    pub stability_cost: f32,
    /// Whether generation was successful.
    pub success: bool,
}

/// Paradox Engine crafting station.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParadoxEngine {
    /// Current paradox energy stored.
    paradox_energy: f32,
    /// Maximum paradox energy capacity.
    max_energy: f32,
    /// Conversion efficiency (0.0 - 1.0).
    efficiency: f32,
    /// Whether the engine is running.
    running: bool,
    /// Total power generated.
    total_power_generated: u32,
    /// Current stability level.
    stability: f32,
    /// Power output per tick when running.
    power_per_tick: u32,
}

impl ParadoxEngine {
    /// Create a new Paradox Engine.
    #[must_use]
    pub fn new() -> Self {
        Self {
            paradox_energy: 0.0,
            max_energy: 100.0,
            efficiency: 0.5,
            running: false,
            total_power_generated: 0,
            stability: 100.0,
            power_per_tick: 5,
        }
    }

    /// Get current paradox energy.
    #[must_use]
    pub fn paradox_energy(&self) -> f32 {
        self.paradox_energy
    }

    /// Get maximum energy capacity.
    #[must_use]
    pub fn max_energy(&self) -> f32 {
        self.max_energy
    }

    /// Get conversion efficiency.
    #[must_use]
    pub fn efficiency(&self) -> f32 {
        self.efficiency
    }

    /// Check if the engine is running.
    #[must_use]
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Get total power generated.
    #[must_use]
    pub fn total_power_generated(&self) -> u32 {
        self.total_power_generated
    }

    /// Get current stability.
    #[must_use]
    pub fn stability(&self) -> f32 {
        self.stability
    }

    /// Get power output per tick.
    #[must_use]
    pub fn power_per_tick(&self) -> u32 {
        self.power_per_tick
    }

    /// Add paradox energy to the engine.
    pub fn add_paradox_energy(&mut self, amount: f32) {
        self.paradox_energy = (self.paradox_energy + amount).min(self.max_energy);
    }

    /// Set the conversion efficiency.
    pub fn set_efficiency(&mut self, efficiency: f32) {
        self.efficiency = efficiency.clamp(0.0, 1.0);
    }

    /// Upgrade the engine's efficiency.
    pub fn upgrade_efficiency(&mut self, amount: f32) {
        self.efficiency = (self.efficiency + amount).min(1.0);
    }

    /// Upgrade the engine's capacity.
    pub fn upgrade_capacity(&mut self, amount: f32) {
        self.max_energy += amount;
    }

    /// Start the engine.
    pub fn start(&mut self) -> bool {
        if self.paradox_energy >= 10.0 && self.stability > 20.0 {
            self.running = true;
            true
        } else {
            false
        }
    }

    /// Stop the engine.
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Convert paradox energy to power.
    ///
    /// Returns the power output information.
    pub fn convert(&mut self) -> ParadoxPowerOutput {
        if !self.running || self.paradox_energy < 1.0 {
            self.running = false;
            return ParadoxPowerOutput::default();
        }

        // Consume paradox energy
        let consumed = 2.0_f32.min(self.paradox_energy);
        self.paradox_energy -= consumed;

        // Calculate power output based on efficiency
        let power = (consumed * self.efficiency * self.power_per_tick as f32) as u32;
        self.total_power_generated += power;

        // Stability cost inversely related to efficiency
        let stability_cost = consumed * (1.0 - self.efficiency) * 0.5;
        self.stability = (self.stability - stability_cost).max(0.0);

        // Stop if stability too low
        if self.stability < 10.0 {
            self.running = false;
        }

        ParadoxPowerOutput {
            power,
            stability_cost,
            success: true,
        }
    }

    /// Update the engine (call each tick).
    pub fn update(&mut self) -> ParadoxPowerOutput {
        if self.running {
            self.convert()
        } else {
            // Slowly recover stability when not running
            self.stability = (self.stability + 0.1).min(100.0);
            ParadoxPowerOutput::default()
        }
    }

    /// Repair stability.
    pub fn repair_stability(&mut self, amount: f32) {
        self.stability = (self.stability + amount).min(100.0);
    }

    /// Check if the engine needs repairs.
    #[must_use]
    pub fn needs_repair(&self) -> bool {
        self.stability < 50.0
    }

    /// Get the current fill percentage.
    #[must_use]
    pub fn fill_percentage(&self) -> f32 {
        self.paradox_energy / self.max_energy
    }

    /// Reset for a new loop.
    pub fn reset_for_loop(&mut self) {
        self.running = false;
        self.stability = 100.0;
        // Keep paradox_energy and upgrades
    }
}

impl Default for ParadoxEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paradox_engine_new() {
        let engine = ParadoxEngine::new();

        assert!((engine.paradox_energy() - 0.0).abs() < f32::EPSILON);
        assert!((engine.max_energy() - 100.0).abs() < f32::EPSILON);
        assert!((engine.efficiency() - 0.5).abs() < f32::EPSILON);
        assert!(!engine.is_running());
        assert_eq!(engine.total_power_generated(), 0);
        assert!((engine.stability() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_paradox_engine_add_energy() {
        let mut engine = ParadoxEngine::new();

        engine.add_paradox_energy(50.0);
        assert!((engine.paradox_energy() - 50.0).abs() < f32::EPSILON);

        engine.add_paradox_energy(100.0);
        assert!((engine.paradox_energy() - 100.0).abs() < f32::EPSILON); // Capped
    }

    #[test]
    fn test_paradox_engine_set_efficiency() {
        let mut engine = ParadoxEngine::new();

        engine.set_efficiency(0.8);
        assert!((engine.efficiency() - 0.8).abs() < f32::EPSILON);

        engine.set_efficiency(1.5);
        assert!((engine.efficiency() - 1.0).abs() < f32::EPSILON); // Clamped

        engine.set_efficiency(-0.5);
        assert!((engine.efficiency() - 0.0).abs() < f32::EPSILON); // Clamped
    }

    #[test]
    fn test_paradox_engine_upgrade_efficiency() {
        let mut engine = ParadoxEngine::new();

        engine.upgrade_efficiency(0.3);
        assert!((engine.efficiency() - 0.8).abs() < f32::EPSILON);

        engine.upgrade_efficiency(0.5);
        assert!((engine.efficiency() - 1.0).abs() < f32::EPSILON); // Capped at 1.0
    }

    #[test]
    fn test_paradox_engine_upgrade_capacity() {
        let mut engine = ParadoxEngine::new();

        engine.upgrade_capacity(50.0);
        assert!((engine.max_energy() - 150.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_paradox_engine_start_success() {
        let mut engine = ParadoxEngine::new();
        engine.add_paradox_energy(50.0);

        assert!(engine.start());
        assert!(engine.is_running());
    }

    #[test]
    fn test_paradox_engine_start_insufficient_energy() {
        let mut engine = ParadoxEngine::new();
        engine.add_paradox_energy(5.0);

        assert!(!engine.start());
        assert!(!engine.is_running());
    }

    #[test]
    fn test_paradox_engine_start_low_stability() {
        let mut engine = ParadoxEngine::new();
        engine.add_paradox_energy(50.0);
        engine.stability = 10.0;

        assert!(!engine.start());
    }

    #[test]
    fn test_paradox_engine_stop() {
        let mut engine = ParadoxEngine::new();
        engine.add_paradox_energy(50.0);
        engine.start();
        assert!(engine.is_running());

        engine.stop();
        assert!(!engine.is_running());
    }

    #[test]
    fn test_paradox_engine_convert() {
        let mut engine = ParadoxEngine::new();
        engine.add_paradox_energy(50.0);
        engine.start();

        let output = engine.convert();
        assert!(output.success);
        assert!(output.power > 0);
        assert!(engine.paradox_energy() < 50.0);
        assert!(engine.total_power_generated() > 0);
    }

    #[test]
    fn test_paradox_engine_convert_not_running() {
        let mut engine = ParadoxEngine::new();
        engine.add_paradox_energy(50.0);

        let output = engine.convert();
        assert!(!output.success);
        assert_eq!(output.power, 0);
    }

    #[test]
    fn test_paradox_engine_convert_depletes_energy() {
        let mut engine = ParadoxEngine::new();
        engine.add_paradox_energy(3.0);
        engine.start();

        engine.convert();
        engine.convert();

        assert!(!engine.is_running()); // Should stop when energy too low
    }

    #[test]
    fn test_paradox_engine_update_running() {
        let mut engine = ParadoxEngine::new();
        engine.add_paradox_energy(50.0);
        engine.start();

        let output = engine.update();
        assert!(output.success);
    }

    #[test]
    fn test_paradox_engine_update_not_running_recovers_stability() {
        let mut engine = ParadoxEngine::new();
        engine.stability = 50.0;

        engine.update();
        assert!(engine.stability() > 50.0);
    }

    #[test]
    fn test_paradox_engine_repair_stability() {
        let mut engine = ParadoxEngine::new();
        engine.stability = 20.0;

        engine.repair_stability(50.0);
        assert!((engine.stability() - 70.0).abs() < f32::EPSILON);

        engine.repair_stability(100.0);
        assert!((engine.stability() - 100.0).abs() < f32::EPSILON); // Capped
    }

    #[test]
    fn test_paradox_engine_needs_repair() {
        let mut engine = ParadoxEngine::new();
        assert!(!engine.needs_repair());

        engine.stability = 40.0;
        assert!(engine.needs_repair());
    }

    #[test]
    fn test_paradox_engine_fill_percentage() {
        let mut engine = ParadoxEngine::new();
        assert!((engine.fill_percentage() - 0.0).abs() < f32::EPSILON);

        engine.add_paradox_energy(50.0);
        assert!((engine.fill_percentage() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_paradox_engine_reset_for_loop() {
        let mut engine = ParadoxEngine::new();
        engine.add_paradox_energy(50.0);
        engine.start();
        engine.stability = 30.0;
        engine.upgrade_efficiency(0.3);

        engine.reset_for_loop();

        assert!(!engine.is_running());
        assert!((engine.stability() - 100.0).abs() < f32::EPSILON);
        assert!((engine.paradox_energy() - 50.0).abs() < f32::EPSILON); // Kept
        assert!((engine.efficiency() - 0.8).abs() < f32::EPSILON); // Kept
    }

    #[test]
    fn test_paradox_power_output_default() {
        let output = ParadoxPowerOutput::default();
        assert_eq!(output.power, 0);
        assert!(!output.success);
    }
}
