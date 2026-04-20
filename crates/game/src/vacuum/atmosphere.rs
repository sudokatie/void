//! Atmosphere management for space station.
//!
//! High-level atmosphere management including life support systems.

use engine_physics::vacuum::RoomAtmosphereSim;
use serde::{Deserialize, Serialize};

use super::{DecompressionEvent, DecompressionType, GasModel};

/// Manages atmosphere and life support for the entire station.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AtmosphereManager {
    /// Gas model for atmosphere simulation.
    gas_model: GasModel,
    /// Life support active status per room.
    life_support_active: Vec<bool>,
    /// Sealed breach status per room.
    sealed: Vec<bool>,
}

impl AtmosphereManager {
    /// Create a new atmosphere manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            gas_model: GasModel::new(),
            life_support_active: Vec::new(),
            sealed: Vec::new(),
        }
    }

    /// Add a room with the specified volume and life support status.
    pub fn add_room(&mut self, volume: f32, has_life_support: bool) -> usize {
        let id = self.gas_model.add_room(RoomAtmosphereSim::with_volume(volume));
        self.life_support_active.push(has_life_support);
        self.sealed.push(true);
        id
    }

    /// Trigger a breach in a room.
    pub fn breach(&mut self, room_id: usize, breach_type: DecompressionType) -> DecompressionEvent {
        if room_id < self.sealed.len() {
            self.sealed[room_id] = false;
        }
        self.gas_model.trigger_breach(room_id, breach_type)
    }

    /// Seal a breach in a room.
    pub fn seal_breach(&mut self, room_id: usize) -> bool {
        if room_id < self.sealed.len() {
            self.sealed[room_id] = true;
        }
        self.gas_model.seal_breach(room_id)
    }

    /// Check if a room is sealed.
    #[must_use]
    pub fn is_sealed(&self, room_id: usize) -> bool {
        self.sealed.get(room_id).copied().unwrap_or(false)
    }

    /// Enable or disable life support in a room.
    pub fn set_life_support(&mut self, room_id: usize, active: bool) {
        if room_id < self.life_support_active.len() {
            self.life_support_active[room_id] = active;
        }
    }

    /// Check if life support is active in a room.
    #[must_use]
    pub fn has_life_support(&self, room_id: usize) -> bool {
        self.life_support_active.get(room_id).copied().unwrap_or(false)
    }

    /// Get a reference to a room's atmosphere.
    #[must_use]
    pub fn get_room(&self, room_id: usize) -> Option<&RoomAtmosphereSim> {
        self.gas_model.get_room(room_id)
    }

    /// Get a mutable reference to a room's atmosphere.
    pub fn get_room_mut(&mut self, room_id: usize) -> Option<&mut RoomAtmosphereSim> {
        self.gas_model.get_room_mut(room_id)
    }

    /// Connect two rooms.
    pub fn connect_rooms(&mut self, a: usize, b: usize, opening: f32) {
        self.gas_model.connect_rooms(a, b, opening);
    }

    /// Get the number of rooms.
    #[must_use]
    pub fn room_count(&self) -> usize {
        self.gas_model.room_count()
    }

    /// Tick the atmosphere manager.
    ///
    /// Processes gas flow, breaches, and life support.
    pub fn tick(&mut self, dt: f32) -> Vec<DecompressionEvent> {
        // Run life support for active rooms
        for (i, &active) in self.life_support_active.iter().enumerate() {
            if active {
                if let Some(room) = self.gas_model.get_room_mut(i) {
                    room.life_support_tick(dt);
                }
            }
        }

        // Natural tick for all rooms (temperature drift)
        for i in 0..self.gas_model.room_count() {
            if let Some(room) = self.gas_model.get_room_mut(i) {
                room.tick(dt);
            }
        }

        // Process gas model
        self.gas_model.tick(dt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atmosphere_manager_new() {
        let manager = AtmosphereManager::new();
        assert_eq!(manager.room_count(), 0);
    }

    #[test]
    fn test_atmosphere_manager_add_room() {
        let mut manager = AtmosphereManager::new();
        let id = manager.add_room(100.0, true);
        assert_eq!(id, 0);
        assert_eq!(manager.room_count(), 1);
    }

    #[test]
    fn test_atmosphere_manager_add_room_with_life_support() {
        let mut manager = AtmosphereManager::new();
        manager.add_room(100.0, true);
        assert!(manager.has_life_support(0));
    }

    #[test]
    fn test_atmosphere_manager_add_room_without_life_support() {
        let mut manager = AtmosphereManager::new();
        manager.add_room(100.0, false);
        assert!(!manager.has_life_support(0));
    }

    #[test]
    fn test_atmosphere_manager_breach() {
        let mut manager = AtmosphereManager::new();
        manager.add_room(100.0, true);
        assert!(manager.is_sealed(0));

        let event = manager.breach(0, DecompressionType::Rapid);
        assert_eq!(event.room_id, 0);
        assert!(!manager.is_sealed(0));
    }

    #[test]
    fn test_atmosphere_manager_seal_breach() {
        let mut manager = AtmosphereManager::new();
        manager.add_room(100.0, true);
        manager.breach(0, DecompressionType::Slow);
        assert!(!manager.is_sealed(0));

        assert!(manager.seal_breach(0));
        assert!(manager.is_sealed(0));
    }

    #[test]
    fn test_atmosphere_manager_set_life_support() {
        let mut manager = AtmosphereManager::new();
        manager.add_room(100.0, false);
        assert!(!manager.has_life_support(0));

        manager.set_life_support(0, true);
        assert!(manager.has_life_support(0));

        manager.set_life_support(0, false);
        assert!(!manager.has_life_support(0));
    }

    #[test]
    fn test_atmosphere_manager_get_room() {
        let mut manager = AtmosphereManager::new();
        manager.add_room(150.0, true);

        let room = manager.get_room(0);
        assert!(room.is_some());
        assert!((room.unwrap().volume - 150.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_atmosphere_manager_get_room_mut() {
        let mut manager = AtmosphereManager::new();
        manager.add_room(100.0, true);

        if let Some(room) = manager.get_room_mut(0) {
            room.pressure = 50.0;
        }

        assert!((manager.get_room(0).unwrap().pressure - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_atmosphere_manager_connect_rooms() {
        let mut manager = AtmosphereManager::new();
        manager.add_room(100.0, true);
        manager.add_room(100.0, true);
        manager.connect_rooms(0, 1, 1.0);
        // Connection is stored in gas_model, verify by checking tick doesn't panic
        manager.tick(1.0);
    }

    #[test]
    fn test_atmosphere_manager_tick_life_support() {
        let mut manager = AtmosphereManager::new();
        manager.add_room(100.0, true);

        // Lower O2 to test life support
        if let Some(room) = manager.get_room_mut(0) {
            room.o2 = 18.0;
        }

        manager.tick(1.0);

        // O2 should have increased
        assert!(manager.get_room(0).unwrap().o2 > 18.0);
    }

    #[test]
    fn test_atmosphere_manager_tick_no_life_support() {
        let mut manager = AtmosphereManager::new();
        manager.add_room(100.0, false);

        // Lower O2 to test without life support
        if let Some(room) = manager.get_room_mut(0) {
            room.o2 = 18.0;
        }

        manager.tick(1.0);

        // O2 should not have increased (might still be 18.0 or slightly less due to tick)
        assert!(manager.get_room(0).unwrap().o2 <= 18.0);
    }

    #[test]
    fn test_atmosphere_manager_default() {
        let manager = AtmosphereManager::default();
        assert_eq!(manager.room_count(), 0);
    }

    #[test]
    fn test_atmosphere_manager_has_life_support_invalid_room() {
        let manager = AtmosphereManager::new();
        assert!(!manager.has_life_support(999));
    }

    #[test]
    fn test_atmosphere_manager_is_sealed_invalid_room() {
        let manager = AtmosphereManager::new();
        assert!(!manager.is_sealed(999));
    }
}
