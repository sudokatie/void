//! Room systems and their states.
//!
//! Tracks individual systems within rooms.

use serde::{Deserialize, Serialize};
use std::fmt;

/// State of a room system.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SystemState {
    /// System is fully operational.
    Online,
    /// System is running at reduced capacity.
    Degraded,
    /// System is not operational.
    Offline,
}

impl SystemState {
    /// Get the efficiency multiplier for this state.
    #[must_use]
    pub fn efficiency(&self) -> f32 {
        match self {
            SystemState::Online => 1.0,
            SystemState::Degraded => 0.5,
            SystemState::Offline => 0.0,
        }
    }
}

impl fmt::Display for SystemState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SystemState::Online => write!(f, "Online"),
            SystemState::Degraded => write!(f, "Degraded"),
            SystemState::Offline => write!(f, "Offline"),
        }
    }
}

/// A system within a room.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoomSystem {
    /// Name of the system.
    name: String,
    /// Power draw when active.
    power_draw: f32,
    /// Current state.
    state: SystemState,
}

impl RoomSystem {
    /// Create a new room system.
    #[must_use]
    pub fn new(name: String, power_draw: f32) -> Self {
        Self {
            name,
            power_draw,
            state: SystemState::Online,
        }
    }

    /// Get the system name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the power draw.
    #[must_use]
    pub fn power_draw(&self) -> f32 {
        self.power_draw
    }

    /// Get the current state.
    #[must_use]
    pub fn state(&self) -> SystemState {
        self.state
    }

    /// Get the effective power draw (based on state).
    #[must_use]
    pub fn effective_power_draw(&self) -> f32 {
        self.power_draw * self.state.efficiency()
    }

    /// Power on the system.
    pub fn power_on(&mut self) {
        if self.state == SystemState::Offline {
            self.state = SystemState::Online;
        }
    }

    /// Power off the system.
    pub fn power_off(&mut self) {
        self.state = SystemState::Offline;
    }

    /// Set the system to degraded state.
    pub fn degrade(&mut self) {
        if self.state == SystemState::Online {
            self.state = SystemState::Degraded;
        }
    }

    /// Repair a degraded system.
    pub fn repair(&mut self) {
        if self.state == SystemState::Degraded {
            self.state = SystemState::Online;
        }
    }

    /// Check if the system is online.
    #[must_use]
    pub fn is_online(&self) -> bool {
        self.state == SystemState::Online
    }

    /// Check if the system is operational (online or degraded).
    #[must_use]
    pub fn is_operational(&self) -> bool {
        self.state != SystemState::Offline
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_state_efficiency() {
        assert!((SystemState::Online.efficiency() - 1.0).abs() < f32::EPSILON);
        assert!((SystemState::Degraded.efficiency() - 0.5).abs() < f32::EPSILON);
        assert!((SystemState::Offline.efficiency() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_system_state_display() {
        assert_eq!(format!("{}", SystemState::Online), "Online");
        assert_eq!(format!("{}", SystemState::Degraded), "Degraded");
        assert_eq!(format!("{}", SystemState::Offline), "Offline");
    }

    #[test]
    fn test_room_system_new() {
        let system = RoomSystem::new("Life Support".to_string(), 25.0);
        assert_eq!(system.name(), "Life Support");
        assert!((system.power_draw() - 25.0).abs() < f32::EPSILON);
        assert_eq!(system.state(), SystemState::Online);
    }

    #[test]
    fn test_room_system_power_off() {
        let mut system = RoomSystem::new("Lights".to_string(), 5.0);
        system.power_off();
        assert_eq!(system.state(), SystemState::Offline);
        assert!(!system.is_online());
        assert!(!system.is_operational());
    }

    #[test]
    fn test_room_system_power_on() {
        let mut system = RoomSystem::new("Sensors".to_string(), 10.0);
        system.power_off();
        system.power_on();
        assert_eq!(system.state(), SystemState::Online);
        assert!(system.is_online());
    }

    #[test]
    fn test_room_system_degrade() {
        let mut system = RoomSystem::new("Comms".to_string(), 8.0);
        system.degrade();
        assert_eq!(system.state(), SystemState::Degraded);
        assert!(!system.is_online());
        assert!(system.is_operational());
    }

    #[test]
    fn test_room_system_repair() {
        let mut system = RoomSystem::new("Navigation".to_string(), 12.0);
        system.degrade();
        system.repair();
        assert_eq!(system.state(), SystemState::Online);
    }

    #[test]
    fn test_room_system_effective_power_draw() {
        let mut system = RoomSystem::new("Shields".to_string(), 20.0);
        assert!((system.effective_power_draw() - 20.0).abs() < f32::EPSILON);

        system.degrade();
        assert!((system.effective_power_draw() - 10.0).abs() < f32::EPSILON);

        system.power_off();
        assert!((system.effective_power_draw() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_room_system_degrade_from_offline() {
        let mut system = RoomSystem::new("Test".to_string(), 5.0);
        system.power_off();
        system.degrade();
        // Should stay offline, degrade only works from online
        assert_eq!(system.state(), SystemState::Offline);
    }

    #[test]
    fn test_room_system_power_on_from_degraded() {
        let mut system = RoomSystem::new("Test".to_string(), 5.0);
        system.degrade();
        system.power_on();
        // power_on only works from offline
        assert_eq!(system.state(), SystemState::Degraded);
    }
}
