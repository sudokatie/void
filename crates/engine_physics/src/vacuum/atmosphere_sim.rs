//! Room atmosphere simulation for space station environments.
//!
//! Simulates gas composition, pressure, and temperature in enclosed spaces.

use serde::{Deserialize, Serialize};

/// Normal atmospheric constants.
pub const NORMAL_O2: f32 = 21.0;
pub const NORMAL_N2: f32 = 78.0;
pub const NORMAL_CO2: f32 = 0.04;
pub const NORMAL_PRESSURE: f32 = 101.3;
pub const NORMAL_TEMPERATURE: f32 = 22.0;
pub const DEFAULT_VOLUME: f32 = 100.0;
pub const SPACE_TEMPERATURE: f32 = -270.0;

/// Simulates the atmosphere within a room.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoomAtmosphereSim {
    /// Oxygen percentage (0-100).
    pub o2: f32,
    /// Nitrogen percentage (0-100).
    pub n2: f32,
    /// Carbon dioxide percentage (0-100).
    pub co2: f32,
    /// Pressure in kPa.
    pub pressure: f32,
    /// Temperature in Celsius.
    pub temperature: f32,
    /// Volume in cubic meters.
    pub volume: f32,
}

impl Default for RoomAtmosphereSim {
    fn default() -> Self {
        Self::new()
    }
}

impl RoomAtmosphereSim {
    /// Create a new atmosphere with normal Earth-like conditions.
    #[must_use]
    pub fn new() -> Self {
        Self {
            o2: NORMAL_O2,
            n2: NORMAL_N2,
            co2: NORMAL_CO2,
            pressure: NORMAL_PRESSURE,
            temperature: NORMAL_TEMPERATURE,
            volume: DEFAULT_VOLUME,
        }
    }

    /// Create a new atmosphere with the specified volume.
    #[must_use]
    pub fn with_volume(volume: f32) -> Self {
        Self {
            volume,
            ..Self::new()
        }
    }

    /// Check if the atmosphere is breathable.
    ///
    /// Breathable conditions:
    /// - O2: 16-25%
    /// - Pressure: 80-120 kPa
    /// - Temperature: 10-40°C
    #[must_use]
    pub fn is_breathable(&self) -> bool {
        let o2_ok = (16.0..=25.0).contains(&self.o2);
        let pressure_ok = (80.0..=120.0).contains(&self.pressure);
        let temp_ok = (10.0..=40.0).contains(&self.temperature);
        o2_ok && pressure_ok && temp_ok
    }

    /// Get the oxygen percentage.
    #[must_use]
    pub fn o2_percentage(&self) -> f32 {
        self.o2
    }

    /// Get the carbon dioxide percentage.
    #[must_use]
    pub fn co2_percentage(&self) -> f32 {
        self.co2
    }

    /// Flow gas from another room into this one.
    ///
    /// Equalizes gas composition based on opening size and time delta.
    pub fn flow_from(&mut self, other: &mut RoomAtmosphereSim, opening_size: f32, dt: f32) {
        if opening_size <= 0.0 || dt <= 0.0 {
            return;
        }

        // Flow rate based on opening size and pressure differential
        let pressure_diff = other.pressure - self.pressure;
        let flow_rate = opening_size * dt * 0.1;

        // Calculate volume-weighted mixing
        let total_volume = self.volume + other.volume;
        let self_ratio = self.volume / total_volume;
        let other_ratio = other.volume / total_volume;

        // Mix gases proportionally based on flow
        let mix_factor = (flow_rate * pressure_diff.abs() / 100.0).min(0.5);

        if pressure_diff > 0.0 {
            // Gas flows from other to self
            self.o2 += (other.o2 - self.o2) * mix_factor * other_ratio;
            self.n2 += (other.n2 - self.n2) * mix_factor * other_ratio;
            self.co2 += (other.co2 - self.co2) * mix_factor * other_ratio;
            self.pressure += pressure_diff * mix_factor * other_ratio;

            other.o2 -= (other.o2 - self.o2) * mix_factor * self_ratio;
            other.n2 -= (other.n2 - self.n2) * mix_factor * self_ratio;
            other.co2 -= (other.co2 - self.co2) * mix_factor * self_ratio;
            other.pressure -= pressure_diff * mix_factor * self_ratio;
        } else {
            // Gas flows from self to other
            other.o2 += (self.o2 - other.o2) * mix_factor * self_ratio;
            other.n2 += (self.n2 - other.n2) * mix_factor * self_ratio;
            other.co2 += (self.co2 - other.co2) * mix_factor * self_ratio;
            other.pressure += pressure_diff.abs() * mix_factor * self_ratio;

            self.o2 -= (self.o2 - other.o2) * mix_factor * other_ratio;
            self.n2 -= (self.n2 - other.n2) * mix_factor * other_ratio;
            self.co2 -= (self.co2 - other.co2) * mix_factor * other_ratio;
            self.pressure -= pressure_diff.abs() * mix_factor * other_ratio;
        }

        // Equalize temperature
        let temp_diff = other.temperature - self.temperature;
        self.temperature += temp_diff * mix_factor * 0.5;
        other.temperature -= temp_diff * mix_factor * 0.5;
    }

    /// Vent atmosphere to vacuum through a breach.
    ///
    /// Returns the pressure lost this tick.
    pub fn vent_to_vacuum(&mut self, breach_size: f32, dt: f32) -> f32 {
        if breach_size <= 0.0 || dt <= 0.0 || self.pressure <= 0.0 {
            return 0.0;
        }

        // Pressure loss rate based on breach size
        let loss_rate = breach_size * dt * 5.0;
        let pressure_lost = (self.pressure * loss_rate / 100.0).min(self.pressure);

        // Reduce all gases proportionally
        let loss_ratio = pressure_lost / self.pressure;
        self.pressure -= pressure_lost;
        self.o2 *= 1.0 - loss_ratio * 0.1;
        self.n2 *= 1.0 - loss_ratio * 0.1;
        self.co2 *= 1.0 - loss_ratio * 0.1;

        // Temperature drops rapidly when venting
        self.temperature -= breach_size * dt * 2.0;
        self.temperature = self.temperature.max(SPACE_TEMPERATURE);

        pressure_lost
    }

    /// Life support tick - adds O2, removes CO2, regulates temperature.
    pub fn life_support_tick(&mut self, dt: f32) {
        if dt <= 0.0 {
            return;
        }

        // Add O2 (0.5% per tick)
        self.o2 = (self.o2 + 0.5 * dt).min(100.0);

        // Remove CO2 (0.3% per tick)
        self.co2 = (self.co2 - 0.3 * dt).max(0.0);

        // Regulate temperature toward 22°C
        let temp_diff = NORMAL_TEMPERATURE - self.temperature;
        self.temperature += temp_diff * 0.1 * dt;

        // Maintain pressure if it's low
        if self.pressure < NORMAL_PRESSURE {
            let pressure_add = (NORMAL_PRESSURE - self.pressure) * 0.1 * dt;
            self.pressure += pressure_add;
        }
    }

    /// Natural tick - temperature drifts toward space temperature if unheated.
    pub fn tick(&mut self, dt: f32) {
        if dt <= 0.0 {
            return;
        }

        // Temperature drifts toward space temperature (0.1°C per tick)
        let drift = 0.1 * dt;
        if self.temperature > SPACE_TEMPERATURE {
            self.temperature -= drift;
            self.temperature = self.temperature.max(SPACE_TEMPERATURE);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_atmosphere() {
        let atmo = RoomAtmosphereSim::new();
        assert!((atmo.o2 - NORMAL_O2).abs() < f32::EPSILON);
        assert!((atmo.n2 - NORMAL_N2).abs() < f32::EPSILON);
        assert!((atmo.co2 - NORMAL_CO2).abs() < f32::EPSILON);
        assert!((atmo.pressure - NORMAL_PRESSURE).abs() < f32::EPSILON);
        assert!((atmo.temperature - NORMAL_TEMPERATURE).abs() < f32::EPSILON);
        assert!((atmo.volume - DEFAULT_VOLUME).abs() < f32::EPSILON);
    }

    #[test]
    fn test_with_volume() {
        let atmo = RoomAtmosphereSim::with_volume(200.0);
        assert!((atmo.volume - 200.0).abs() < f32::EPSILON);
        assert!((atmo.o2 - NORMAL_O2).abs() < f32::EPSILON);
    }

    #[test]
    fn test_is_breathable_normal() {
        let atmo = RoomAtmosphereSim::new();
        assert!(atmo.is_breathable());
    }

    #[test]
    fn test_is_breathable_low_o2() {
        let mut atmo = RoomAtmosphereSim::new();
        atmo.o2 = 15.0;
        assert!(!atmo.is_breathable());
    }

    #[test]
    fn test_is_breathable_high_o2() {
        let mut atmo = RoomAtmosphereSim::new();
        atmo.o2 = 26.0;
        assert!(!atmo.is_breathable());
    }

    #[test]
    fn test_is_breathable_low_pressure() {
        let mut atmo = RoomAtmosphereSim::new();
        atmo.pressure = 79.0;
        assert!(!atmo.is_breathable());
    }

    #[test]
    fn test_is_breathable_high_pressure() {
        let mut atmo = RoomAtmosphereSim::new();
        atmo.pressure = 121.0;
        assert!(!atmo.is_breathable());
    }

    #[test]
    fn test_is_breathable_low_temp() {
        let mut atmo = RoomAtmosphereSim::new();
        atmo.temperature = 9.0;
        assert!(!atmo.is_breathable());
    }

    #[test]
    fn test_is_breathable_high_temp() {
        let mut atmo = RoomAtmosphereSim::new();
        atmo.temperature = 41.0;
        assert!(!atmo.is_breathable());
    }

    #[test]
    fn test_o2_percentage() {
        let atmo = RoomAtmosphereSim::new();
        assert!((atmo.o2_percentage() - NORMAL_O2).abs() < f32::EPSILON);
    }

    #[test]
    fn test_co2_percentage() {
        let atmo = RoomAtmosphereSim::new();
        assert!((atmo.co2_percentage() - NORMAL_CO2).abs() < f32::EPSILON);
    }

    #[test]
    fn test_vent_to_vacuum() {
        let mut atmo = RoomAtmosphereSim::new();
        let initial_pressure = atmo.pressure;
        let lost = atmo.vent_to_vacuum(1.0, 1.0);
        assert!(lost > 0.0);
        assert!(atmo.pressure < initial_pressure);
    }

    #[test]
    fn test_vent_to_vacuum_no_pressure() {
        let mut atmo = RoomAtmosphereSim::new();
        atmo.pressure = 0.0;
        let lost = atmo.vent_to_vacuum(1.0, 1.0);
        assert!((lost - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_vent_to_vacuum_zero_breach() {
        let mut atmo = RoomAtmosphereSim::new();
        let initial_pressure = atmo.pressure;
        let lost = atmo.vent_to_vacuum(0.0, 1.0);
        assert!((lost - 0.0).abs() < f32::EPSILON);
        assert!((atmo.pressure - initial_pressure).abs() < f32::EPSILON);
    }

    #[test]
    fn test_vent_temperature_drops() {
        let mut atmo = RoomAtmosphereSim::new();
        let initial_temp = atmo.temperature;
        atmo.vent_to_vacuum(1.0, 1.0);
        assert!(atmo.temperature < initial_temp);
    }

    #[test]
    fn test_life_support_adds_o2() {
        let mut atmo = RoomAtmosphereSim::new();
        atmo.o2 = 18.0;
        atmo.life_support_tick(1.0);
        assert!(atmo.o2 > 18.0);
    }

    #[test]
    fn test_life_support_removes_co2() {
        let mut atmo = RoomAtmosphereSim::new();
        atmo.co2 = 2.0;
        atmo.life_support_tick(1.0);
        assert!(atmo.co2 < 2.0);
    }

    #[test]
    fn test_life_support_regulates_temp() {
        let mut atmo = RoomAtmosphereSim::new();
        atmo.temperature = 15.0;
        atmo.life_support_tick(1.0);
        assert!(atmo.temperature > 15.0);
    }

    #[test]
    fn test_tick_temperature_drift() {
        let mut atmo = RoomAtmosphereSim::new();
        let initial_temp = atmo.temperature;
        atmo.tick(1.0);
        assert!(atmo.temperature < initial_temp);
    }

    #[test]
    fn test_tick_temperature_minimum() {
        let mut atmo = RoomAtmosphereSim::new();
        atmo.temperature = SPACE_TEMPERATURE;
        atmo.tick(1.0);
        assert!((atmo.temperature - SPACE_TEMPERATURE).abs() < f32::EPSILON);
    }

    #[test]
    fn test_flow_from_higher_pressure() {
        let mut low = RoomAtmosphereSim::with_volume(100.0);
        let mut high = RoomAtmosphereSim::with_volume(100.0);
        low.pressure = 80.0;
        high.pressure = 120.0;

        low.flow_from(&mut high, 1.0, 1.0);
        assert!(low.pressure > 80.0);
        assert!(high.pressure < 120.0);
    }

    #[test]
    fn test_flow_from_zero_opening() {
        let mut a = RoomAtmosphereSim::new();
        let mut b = RoomAtmosphereSim::new();
        b.pressure = 150.0;
        let initial_a = a.pressure;

        a.flow_from(&mut b, 0.0, 1.0);
        assert!((a.pressure - initial_a).abs() < f32::EPSILON);
    }

    #[test]
    fn test_default_trait() {
        let atmo = RoomAtmosphereSim::default();
        assert!((atmo.o2 - NORMAL_O2).abs() < f32::EPSILON);
    }
}
