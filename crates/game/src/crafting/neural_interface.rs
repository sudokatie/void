//! Neural Interface for Titan communication.
//!
//! Allows players to interface with the Titan's nervous system,
//! providing limited guidance and mood influence capabilities.

use glam::IVec3;
use serde::{Deserialize, Serialize};

/// Neural fluid cost for guidance command.
pub const GUIDANCE_COST: u32 = 5;

/// Neural fluid cost for mood influence.
pub const MOOD_INFLUENCE_COST: u32 = 10;

/// Cooldown for guidance commands in seconds.
pub const GUIDANCE_COOLDOWN: f32 = 60.0;

/// Cooldown for mood influence in seconds.
pub const MOOD_COOLDOWN: f32 = 120.0;

/// Result of a neural interface action.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeuralResult {
    /// Action was successful.
    Success,
    /// Not enough neural fluid.
    InsufficientFluid,
    /// Interface is on cooldown.
    OnCooldown,
    /// Interface is not operational.
    NotOperational,
    /// Invalid command.
    InvalidCommand,
}

impl NeuralResult {
    /// Check if the result was successful.
    #[must_use]
    pub fn is_success(&self) -> bool {
        matches!(self, NeuralResult::Success)
    }
}

/// The Neural Interface device.
///
/// Allows limited communication with the Titan's nervous system,
/// enabling guidance suggestions and mood calming. Requires neural
/// fluid harvested from neural nodes.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeuralInterface {
    /// Whether the interface is operational.
    operational: bool,
    /// Remaining cooldown for guidance.
    guidance_cooldown: f32,
    /// Remaining cooldown for mood influence.
    mood_cooldown: f32,
    /// Connection strength (affects success rate).
    connection_strength: f32,
    /// Total successful commands issued.
    commands_issued: u32,
}

impl NeuralInterface {
    /// Create a new Neural Interface.
    #[must_use]
    pub fn new() -> Self {
        Self {
            operational: true,
            guidance_cooldown: 0.0,
            mood_cooldown: 0.0,
            connection_strength: 1.0,
            commands_issued: 0,
        }
    }

    /// Create a Neural Interface with custom connection strength.
    #[must_use]
    pub fn with_connection(strength: f32) -> Self {
        Self {
            operational: true,
            guidance_cooldown: 0.0,
            mood_cooldown: 0.0,
            connection_strength: strength.clamp(0.1, 2.0),
            commands_issued: 0,
        }
    }

    /// Attempt to guide the Titan in a direction.
    ///
    /// This is a suggestion only - the Titan may or may not follow it.
    /// Costs neural fluid and has a cooldown.
    ///
    /// Returns true if the command was accepted (Titan may still ignore).
    pub fn guide_titan(&mut self, direction: IVec3, neural_fluid: &mut u32) -> bool {
        if !self.can_guide() {
            return false;
        }

        if *neural_fluid < GUIDANCE_COST {
            return false;
        }

        // Consume neural fluid
        *neural_fluid -= GUIDANCE_COST;

        // Start cooldown
        self.guidance_cooldown = GUIDANCE_COOLDOWN;

        // Record command
        self.commands_issued += 1;

        // Success based on connection strength and direction magnitude
        let magnitude = (direction.x.abs() + direction.y.abs() + direction.z.abs()) as f32;
        let success_chance = (self.connection_strength / (1.0 + magnitude * 0.1)).min(1.0);

        // Simplified success check (would use RNG in real implementation)
        success_chance >= 0.5
    }

    /// Attempt to influence the Titan's mood (calm it down).
    ///
    /// Costs more neural fluid and has a longer cooldown than guidance.
    pub fn influence_mood(&mut self, neural_fluid: &mut u32) -> bool {
        if !self.can_influence_mood() {
            return false;
        }

        if *neural_fluid < MOOD_INFLUENCE_COST {
            return false;
        }

        // Consume neural fluid
        *neural_fluid -= MOOD_INFLUENCE_COST;

        // Start cooldown
        self.mood_cooldown = MOOD_COOLDOWN;

        // Record command
        self.commands_issued += 1;

        // Success based on connection strength
        self.connection_strength >= 0.8
    }

    /// Check if guidance can be issued.
    #[must_use]
    pub fn can_guide(&self) -> bool {
        self.operational && self.guidance_cooldown <= 0.0
    }

    /// Check if mood influence can be used.
    #[must_use]
    pub fn can_influence_mood(&self) -> bool {
        self.operational && self.mood_cooldown <= 0.0
    }

    /// Update cooldowns.
    pub fn tick(&mut self, dt: f32) {
        self.guidance_cooldown = (self.guidance_cooldown - dt).max(0.0);
        self.mood_cooldown = (self.mood_cooldown - dt).max(0.0);
    }

    /// Get remaining guidance cooldown.
    #[must_use]
    pub fn guidance_cooldown(&self) -> f32 {
        self.guidance_cooldown
    }

    /// Get remaining mood cooldown.
    #[must_use]
    pub fn mood_cooldown(&self) -> f32 {
        self.mood_cooldown
    }

    /// Check if the interface is operational.
    #[must_use]
    pub fn is_operational(&self) -> bool {
        self.operational
    }

    /// Set the operational state.
    pub fn set_operational(&mut self, operational: bool) {
        self.operational = operational;
    }

    /// Get the connection strength.
    #[must_use]
    pub fn connection_strength(&self) -> f32 {
        self.connection_strength
    }

    /// Set the connection strength.
    pub fn set_connection_strength(&mut self, strength: f32) {
        self.connection_strength = strength.clamp(0.1, 2.0);
    }

    /// Improve connection through calibration or upgrade.
    pub fn improve_connection(&mut self, amount: f32) {
        self.connection_strength = (self.connection_strength + amount).clamp(0.1, 2.0);
    }

    /// Get total commands issued.
    #[must_use]
    pub fn commands_issued(&self) -> u32 {
        self.commands_issued
    }

    /// Reset all cooldowns (for testing or special events).
    pub fn reset_cooldowns(&mut self) {
        self.guidance_cooldown = 0.0;
        self.mood_cooldown = 0.0;
    }

    /// Calculate guidance success rate for a direction.
    #[must_use]
    pub fn guidance_success_rate(&self, direction: IVec3) -> f32 {
        let magnitude = (direction.x.abs() + direction.y.abs() + direction.z.abs()) as f32;
        (self.connection_strength / (1.0 + magnitude * 0.1)).min(1.0)
    }

    /// Calculate mood influence success rate.
    #[must_use]
    pub fn mood_success_rate(&self) -> f32 {
        (self.connection_strength * 0.5).min(1.0)
    }
}

impl Default for NeuralInterface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neural_interface_new() {
        let interface = NeuralInterface::new();
        assert!(interface.is_operational());
        assert!((interface.connection_strength() - 1.0).abs() < f32::EPSILON);
        assert!(interface.can_guide());
        assert!(interface.can_influence_mood());
    }

    #[test]
    fn test_neural_interface_with_connection() {
        let interface = NeuralInterface::with_connection(1.5);
        assert!((interface.connection_strength() - 1.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_neural_interface_with_connection_clamp() {
        let low = NeuralInterface::with_connection(0.0);
        assert!((low.connection_strength() - 0.1).abs() < f32::EPSILON);

        let high = NeuralInterface::with_connection(5.0);
        assert!((high.connection_strength() - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_neural_interface_guide_titan() {
        let mut interface = NeuralInterface::new();
        let mut fluid = 20;

        let result = interface.guide_titan(IVec3::new(1, 0, 0), &mut fluid);
        // With strength 1.0 and small direction, should succeed
        assert!(result);
        assert_eq!(fluid, 15); // Cost 5
    }

    #[test]
    fn test_neural_interface_guide_titan_insufficient_fluid() {
        let mut interface = NeuralInterface::new();
        let mut fluid = 2;

        let result = interface.guide_titan(IVec3::new(1, 0, 0), &mut fluid);
        assert!(!result);
        assert_eq!(fluid, 2); // Unchanged
    }

    #[test]
    fn test_neural_interface_guide_titan_cooldown() {
        let mut interface = NeuralInterface::new();
        let mut fluid = 20;

        interface.guide_titan(IVec3::new(1, 0, 0), &mut fluid);
        assert!(!interface.can_guide());

        let result = interface.guide_titan(IVec3::new(1, 0, 0), &mut fluid);
        assert!(!result);
    }

    #[test]
    fn test_neural_interface_influence_mood() {
        let mut interface = NeuralInterface::new();
        let mut fluid = 20;

        let result = interface.influence_mood(&mut fluid);
        // With strength 1.0, should succeed (>= 0.8)
        assert!(result);
        assert_eq!(fluid, 10); // Cost 10
    }

    #[test]
    fn test_neural_interface_influence_mood_insufficient_fluid() {
        let mut interface = NeuralInterface::new();
        let mut fluid = 5;

        let result = interface.influence_mood(&mut fluid);
        assert!(!result);
        assert_eq!(fluid, 5); // Unchanged
    }

    #[test]
    fn test_neural_interface_influence_mood_cooldown() {
        let mut interface = NeuralInterface::new();
        let mut fluid = 30;

        interface.influence_mood(&mut fluid);
        assert!(!interface.can_influence_mood());

        let result = interface.influence_mood(&mut fluid);
        assert!(!result);
    }

    #[test]
    fn test_neural_interface_tick_cooldowns() {
        let mut interface = NeuralInterface::new();
        let mut fluid = 30;

        interface.guide_titan(IVec3::ZERO, &mut fluid);
        interface.influence_mood(&mut fluid);

        assert!(!interface.can_guide());
        assert!(!interface.can_influence_mood());

        // Tick past guidance cooldown (60s)
        interface.tick(61.0);
        assert!(interface.can_guide());
        assert!(!interface.can_influence_mood());

        // Tick past mood cooldown (120s total)
        interface.tick(60.0);
        assert!(interface.can_influence_mood());
    }

    #[test]
    fn test_neural_interface_not_operational() {
        let mut interface = NeuralInterface::new();
        interface.set_operational(false);

        assert!(!interface.can_guide());
        assert!(!interface.can_influence_mood());
    }

    #[test]
    fn test_neural_interface_improve_connection() {
        let mut interface = NeuralInterface::new();
        interface.improve_connection(0.3);
        assert!((interface.connection_strength() - 1.3).abs() < f32::EPSILON);
    }

    #[test]
    fn test_neural_interface_improve_connection_clamp() {
        let mut interface = NeuralInterface::new();
        interface.improve_connection(5.0);
        assert!((interface.connection_strength() - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_neural_interface_commands_issued() {
        let mut interface = NeuralInterface::new();
        let mut fluid = 50;

        assert_eq!(interface.commands_issued(), 0);

        interface.guide_titan(IVec3::ZERO, &mut fluid);
        assert_eq!(interface.commands_issued(), 1);

        interface.tick(121.0); // Reset cooldowns
        interface.influence_mood(&mut fluid);
        assert_eq!(interface.commands_issued(), 2);
    }

    #[test]
    fn test_neural_interface_reset_cooldowns() {
        let mut interface = NeuralInterface::new();
        let mut fluid = 50;

        interface.guide_titan(IVec3::ZERO, &mut fluid);
        interface.influence_mood(&mut fluid);

        assert!(!interface.can_guide());
        assert!(!interface.can_influence_mood());

        interface.reset_cooldowns();

        assert!(interface.can_guide());
        assert!(interface.can_influence_mood());
    }

    #[test]
    fn test_neural_interface_guidance_success_rate() {
        let interface = NeuralInterface::new();

        // Small direction, high success
        let rate = interface.guidance_success_rate(IVec3::new(1, 0, 0));
        assert!(rate > 0.5);

        // Large direction, lower success
        let rate_large = interface.guidance_success_rate(IVec3::new(10, 10, 10));
        assert!(rate_large < rate);
    }

    #[test]
    fn test_neural_interface_mood_success_rate() {
        let mut interface = NeuralInterface::new();
        assert!((interface.mood_success_rate() - 0.5).abs() < f32::EPSILON);

        interface.set_connection_strength(2.0);
        assert!((interface.mood_success_rate() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_neural_interface_set_connection_strength() {
        let mut interface = NeuralInterface::new();

        interface.set_connection_strength(0.5);
        assert!((interface.connection_strength() - 0.5).abs() < f32::EPSILON);

        interface.set_connection_strength(0.0);
        assert!((interface.connection_strength() - 0.1).abs() < f32::EPSILON);
    }

    #[test]
    fn test_neural_interface_default() {
        let interface = NeuralInterface::default();
        assert!(interface.is_operational());
        assert!((interface.connection_strength() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_neural_result_is_success() {
        assert!(NeuralResult::Success.is_success());
        assert!(!NeuralResult::InsufficientFluid.is_success());
        assert!(!NeuralResult::OnCooldown.is_success());
        assert!(!NeuralResult::NotOperational.is_success());
        assert!(!NeuralResult::InvalidCommand.is_success());
    }

    #[test]
    fn test_neural_interface_guide_with_weak_connection() {
        let mut interface = NeuralInterface::with_connection(0.3);
        let mut fluid = 20;

        // Weak connection with large direction should fail
        let result = interface.guide_titan(IVec3::new(10, 10, 10), &mut fluid);
        // Fluid still consumed
        assert_eq!(fluid, 15);
        // But likely failed
        assert!(!result || interface.guidance_success_rate(IVec3::new(10, 10, 10)) >= 0.5);
    }

    #[test]
    fn test_neural_interface_influence_with_weak_connection() {
        let mut interface = NeuralInterface::with_connection(0.5);
        let mut fluid = 20;

        let result = interface.influence_mood(&mut fluid);
        // Should fail with connection < 0.8
        assert!(!result);
        // Fluid still consumed
        assert_eq!(fluid, 10);
    }
}
