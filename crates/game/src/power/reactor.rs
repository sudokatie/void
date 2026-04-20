//! Reactor power generation.
//!
//! Main power source for the space station.

use serde::{Deserialize, Serialize};

/// Base reactor output.
const BASE_OUTPUT: f32 = 100.0;

/// Damage threshold for automatic shutdown.
const CRITICAL_DAMAGE: f32 = 80.0;

/// A nuclear reactor power source.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Reactor {
    /// Base output capacity.
    output: f32,
    /// Whether the reactor is active.
    active: bool,
    /// Damage level (0-100).
    damage: f32,
}

impl Default for Reactor {
    fn default() -> Self {
        Self::new()
    }
}

impl Reactor {
    /// Create a new reactor.
    #[must_use]
    pub fn new() -> Self {
        Self {
            output: BASE_OUTPUT,
            active: true,
            damage: 0.0,
        }
    }

    /// Get the current power output.
    ///
    /// Output is reduced by damage and zero if inactive.
    #[must_use]
    pub fn output(&self) -> f32 {
        if !self.active {
            return 0.0;
        }
        let damage_factor = 1.0 - (self.damage / 100.0);
        self.output * damage_factor
    }

    /// Get the base output capacity.
    #[must_use]
    pub fn base_output(&self) -> f32 {
        self.output
    }

    /// Check if the reactor is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Get the damage level.
    #[must_use]
    pub fn damage_level(&self) -> f32 {
        self.damage
    }

    /// Restart the reactor.
    ///
    /// Returns true if successful, false if too damaged.
    pub fn restart(&mut self) -> bool {
        if self.damage >= CRITICAL_DAMAGE {
            return false;
        }
        self.active = true;
        true
    }

    /// Shutdown the reactor.
    pub fn shutdown(&mut self) {
        self.active = false;
    }

    /// Apply damage to the reactor.
    ///
    /// Automatically shuts down if critically damaged.
    pub fn damage(&mut self, amount: f32) {
        self.damage = (self.damage + amount).min(100.0);
        if self.damage >= CRITICAL_DAMAGE {
            self.active = false;
        }
    }

    /// Repair the reactor.
    pub fn repair(&mut self, amount: f32) {
        self.damage = (self.damage - amount).max(0.0);
    }

    /// Check if the reactor is critically damaged.
    #[must_use]
    pub fn is_critical(&self) -> bool {
        self.damage >= CRITICAL_DAMAGE
    }

    /// Get the health percentage.
    #[must_use]
    pub fn health(&self) -> f32 {
        100.0 - self.damage
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reactor_new() {
        let reactor = Reactor::new();
        assert!((reactor.base_output() - 100.0).abs() < f32::EPSILON);
        assert!(reactor.is_active());
        assert!((reactor.damage_level() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_reactor_output() {
        let reactor = Reactor::new();
        assert!((reactor.output() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_reactor_output_with_damage() {
        let mut reactor = Reactor::new();
        reactor.damage(50.0);
        assert!((reactor.output() - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_reactor_output_inactive() {
        let mut reactor = Reactor::new();
        reactor.shutdown();
        assert!((reactor.output() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_reactor_shutdown() {
        let mut reactor = Reactor::new();
        reactor.shutdown();
        assert!(!reactor.is_active());
    }

    #[test]
    fn test_reactor_restart() {
        let mut reactor = Reactor::new();
        reactor.shutdown();
        assert!(reactor.restart());
        assert!(reactor.is_active());
    }

    #[test]
    fn test_reactor_restart_critical() {
        let mut reactor = Reactor::new();
        reactor.damage(85.0);
        assert!(!reactor.restart());
        assert!(!reactor.is_active());
    }

    #[test]
    fn test_reactor_damage() {
        let mut reactor = Reactor::new();
        reactor.damage(30.0);
        assert!((reactor.damage_level() - 30.0).abs() < f32::EPSILON);
        assert!(reactor.is_active());
    }

    #[test]
    fn test_reactor_critical_damage_shutdown() {
        let mut reactor = Reactor::new();
        reactor.damage(85.0);
        assert!(!reactor.is_active());
        assert!(reactor.is_critical());
    }

    #[test]
    fn test_reactor_damage_max() {
        let mut reactor = Reactor::new();
        reactor.damage(150.0);
        assert!((reactor.damage_level() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_reactor_repair() {
        let mut reactor = Reactor::new();
        reactor.damage(50.0);
        reactor.repair(30.0);
        assert!((reactor.damage_level() - 20.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_reactor_repair_min() {
        let mut reactor = Reactor::new();
        reactor.damage(20.0);
        reactor.repair(50.0);
        assert!((reactor.damage_level() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_reactor_health() {
        let mut reactor = Reactor::new();
        assert!((reactor.health() - 100.0).abs() < f32::EPSILON);

        reactor.damage(30.0);
        assert!((reactor.health() - 70.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_reactor_default() {
        let reactor = Reactor::default();
        assert!(reactor.is_active());
    }
}
