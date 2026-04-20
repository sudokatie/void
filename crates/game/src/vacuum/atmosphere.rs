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

/// Temperature threshold for cold damage (Celsius).
const COLD_DAMAGE_THRESHOLD: f32 = 0.0;
/// Temperature threshold for heat damage (Celsius).
const HEAT_DAMAGE_THRESHOLD: f32 = 60.0;
/// Optimal O2 concentration for fire risk calculation.
const FIRE_RISK_O2_THRESHOLD: f32 = 23.0;
/// Temperature at which fire risk becomes significant (Celsius).
const FIRE_RISK_TEMP_THRESHOLD: f32 = 40.0;

/// Result of temperature effects on player health.
#[derive(Clone, Debug, PartialEq)]
pub struct TemperatureEffects {
    /// Damage per second from temperature (0 if safe).
    pub damage_per_second: f32,
    /// Type of damage (cold, heat, or none).
    pub damage_type: TemperatureDamageType,
    /// Current temperature.
    pub temperature: f32,
}

/// Type of temperature damage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TemperatureDamageType {
    /// No damage - safe temperature.
    None,
    /// Cold damage - below freezing.
    Cold,
    /// Heat damage - above safe threshold.
    Heat,
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

    /// Gradually vent atmosphere from a room (smooth decompression).
    ///
    /// Returns the amount of pressure lost this tick.
    pub fn smooth_decompression(&mut self, room_id: usize, rate: f32, dt: f32) -> f32 {
        if let Some(room) = self.gas_model.get_room_mut(room_id) {
            let pressure_loss = (rate * dt).min(room.pressure);
            room.pressure = (room.pressure - pressure_loss).max(0.0);
            // Also reduce O2 proportionally
            let o2_loss = (rate * dt * 0.21).min(room.o2); // 21% O2 in air
            room.o2 = (room.o2 - o2_loss).max(0.0);
            pressure_loss
        } else {
            0.0
        }
    }

    /// Calculate temperature effects on player health.
    ///
    /// Returns damage info based on room temperature.
    #[must_use]
    pub fn temperature_effects(&self, room_id: usize) -> TemperatureEffects {
        if let Some(room) = self.get_room(room_id) {
            let temp = room.temperature;

            if temp < COLD_DAMAGE_THRESHOLD {
                // Cold damage: 1 DPS per 10 degrees below 0
                let damage = (COLD_DAMAGE_THRESHOLD - temp).abs() / 10.0;
                TemperatureEffects {
                    damage_per_second: damage,
                    damage_type: TemperatureDamageType::Cold,
                    temperature: temp,
                }
            } else if temp > HEAT_DAMAGE_THRESHOLD {
                // Heat damage: 2 DPS per 10 degrees above 60
                let damage = (temp - HEAT_DAMAGE_THRESHOLD) / 10.0 * 2.0;
                TemperatureEffects {
                    damage_per_second: damage,
                    damage_type: TemperatureDamageType::Heat,
                    temperature: temp,
                }
            } else {
                TemperatureEffects {
                    damage_per_second: 0.0,
                    damage_type: TemperatureDamageType::None,
                    temperature: temp,
                }
            }
        } else {
            TemperatureEffects {
                damage_per_second: 0.0,
                damage_type: TemperatureDamageType::None,
                temperature: 20.0,
            }
        }
    }

    /// Calculate fire risk based on O2 concentration and temperature.
    ///
    /// Returns a value from 0.0 (no risk) to 1.0 (extreme risk).
    #[must_use]
    pub fn fire_risk(&self, room_id: usize) -> f32 {
        if let Some(room) = self.get_room(room_id) {
            // Fire needs both high O2 and high temperature
            let o2_factor = if room.o2 > FIRE_RISK_O2_THRESHOLD {
                ((room.o2 - FIRE_RISK_O2_THRESHOLD) / 10.0).min(1.0)
            } else {
                0.0
            };

            let temp_factor = if room.temperature > FIRE_RISK_TEMP_THRESHOLD {
                ((room.temperature - FIRE_RISK_TEMP_THRESHOLD) / 60.0).min(1.0)
            } else {
                0.0
            };

            // Both factors must be present for significant risk
            (o2_factor * temp_factor).min(1.0)
        } else {
            0.0
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

    // Task 18: Smooth decompression tests
    #[test]
    fn test_smooth_decompression_reduces_pressure() {
        let mut manager = AtmosphereManager::new();
        manager.add_room(100.0, true);

        // Set initial pressure
        if let Some(room) = manager.get_room_mut(0) {
            room.pressure = 100.0;
            room.o2 = 21.0;
        }

        let lost = manager.smooth_decompression(0, 10.0, 1.0);
        assert!(lost > 0.0);
        assert!(manager.get_room(0).unwrap().pressure < 100.0);
    }

    #[test]
    fn test_smooth_decompression_reduces_o2() {
        let mut manager = AtmosphereManager::new();
        manager.add_room(100.0, true);

        if let Some(room) = manager.get_room_mut(0) {
            room.pressure = 100.0;
            room.o2 = 21.0;
        }

        manager.smooth_decompression(0, 10.0, 1.0);
        assert!(manager.get_room(0).unwrap().o2 < 21.0);
    }

    #[test]
    fn test_smooth_decompression_invalid_room() {
        let mut manager = AtmosphereManager::new();
        let lost = manager.smooth_decompression(999, 10.0, 1.0);
        assert!((lost - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_smooth_decompression_zero_pressure() {
        let mut manager = AtmosphereManager::new();
        manager.add_room(100.0, true);

        if let Some(room) = manager.get_room_mut(0) {
            room.pressure = 0.0;
        }

        let lost = manager.smooth_decompression(0, 10.0, 1.0);
        assert!((lost - 0.0).abs() < f32::EPSILON);
    }

    // Task 18: Temperature effects tests
    #[test]
    fn test_temperature_effects_cold_damage() {
        let mut manager = AtmosphereManager::new();
        manager.add_room(100.0, true);

        if let Some(room) = manager.get_room_mut(0) {
            room.temperature = -20.0;
        }

        let effects = manager.temperature_effects(0);
        assert_eq!(effects.damage_type, TemperatureDamageType::Cold);
        assert!(effects.damage_per_second > 0.0);
    }

    #[test]
    fn test_temperature_effects_heat_damage() {
        let mut manager = AtmosphereManager::new();
        manager.add_room(100.0, true);

        if let Some(room) = manager.get_room_mut(0) {
            room.temperature = 80.0;
        }

        let effects = manager.temperature_effects(0);
        assert_eq!(effects.damage_type, TemperatureDamageType::Heat);
        assert!(effects.damage_per_second > 0.0);
    }

    #[test]
    fn test_temperature_effects_safe_temperature() {
        let mut manager = AtmosphereManager::new();
        manager.add_room(100.0, true);

        if let Some(room) = manager.get_room_mut(0) {
            room.temperature = 22.0;
        }

        let effects = manager.temperature_effects(0);
        assert_eq!(effects.damage_type, TemperatureDamageType::None);
        assert!((effects.damage_per_second - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_temperature_effects_edge_cold() {
        let mut manager = AtmosphereManager::new();
        manager.add_room(100.0, true);

        if let Some(room) = manager.get_room_mut(0) {
            room.temperature = 0.0;
        }

        let effects = manager.temperature_effects(0);
        assert_eq!(effects.damage_type, TemperatureDamageType::None);
    }

    #[test]
    fn test_temperature_effects_edge_heat() {
        let mut manager = AtmosphereManager::new();
        manager.add_room(100.0, true);

        if let Some(room) = manager.get_room_mut(0) {
            room.temperature = 60.0;
        }

        let effects = manager.temperature_effects(0);
        assert_eq!(effects.damage_type, TemperatureDamageType::None);
    }

    #[test]
    fn test_temperature_effects_invalid_room() {
        let manager = AtmosphereManager::new();
        let effects = manager.temperature_effects(999);
        assert_eq!(effects.damage_type, TemperatureDamageType::None);
    }

    // Task 18: Fire risk tests
    #[test]
    fn test_fire_risk_high_o2_high_temp() {
        let mut manager = AtmosphereManager::new();
        manager.add_room(100.0, true);

        if let Some(room) = manager.get_room_mut(0) {
            room.o2 = 30.0;
            room.temperature = 80.0;
        }

        let risk = manager.fire_risk(0);
        assert!(risk > 0.0);
    }

    #[test]
    fn test_fire_risk_normal_conditions() {
        let mut manager = AtmosphereManager::new();
        manager.add_room(100.0, true);

        if let Some(room) = manager.get_room_mut(0) {
            room.o2 = 21.0;
            room.temperature = 22.0;
        }

        let risk = manager.fire_risk(0);
        assert!((risk - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_fire_risk_high_o2_low_temp() {
        let mut manager = AtmosphereManager::new();
        manager.add_room(100.0, true);

        if let Some(room) = manager.get_room_mut(0) {
            room.o2 = 35.0;
            room.temperature = 20.0;
        }

        let risk = manager.fire_risk(0);
        assert!((risk - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_fire_risk_invalid_room() {
        let manager = AtmosphereManager::new();
        let risk = manager.fire_risk(999);
        assert!((risk - 0.0).abs() < f32::EPSILON);
    }
}
