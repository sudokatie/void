//! Room atmosphere HUD display.
//!
//! Shows current room atmosphere status including O2, pressure, and temperature.

use serde::{Deserialize, Serialize};

/// Atmosphere warning level.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AtmosphereWarning {
    /// Normal atmosphere.
    Normal,
    /// Caution - atmosphere degrading.
    Caution,
    /// Warning - atmosphere critical.
    Warning,
    /// Danger - atmosphere hazardous.
    Danger,
}

impl AtmosphereWarning {
    /// Get display name.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            AtmosphereWarning::Normal => "Normal",
            AtmosphereWarning::Caution => "Caution",
            AtmosphereWarning::Warning => "Warning",
            AtmosphereWarning::Danger => "Danger",
        }
    }

    /// Get color (RGBA).
    #[must_use]
    pub fn color(&self) -> [f32; 4] {
        match self {
            AtmosphereWarning::Normal => [0.0, 1.0, 0.0, 1.0],
            AtmosphereWarning::Caution => [1.0, 1.0, 0.0, 1.0],
            AtmosphereWarning::Warning => [1.0, 0.5, 0.0, 1.0],
            AtmosphereWarning::Danger => [1.0, 0.0, 0.0, 1.0],
        }
    }
}

/// Room atmosphere display state.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoomAtmosphereDisplay {
    /// Current O2 percentage.
    o2_percent: f32,
    /// Current pressure (kPa).
    pressure: f32,
    /// Current temperature (Celsius).
    temperature: f32,
    /// Warning level.
    warning: AtmosphereWarning,
    /// Room name.
    room_name: String,
    /// Whether display is visible.
    visible: bool,
}

impl Default for RoomAtmosphereDisplay {
    fn default() -> Self {
        Self::new()
    }
}

impl RoomAtmosphereDisplay {
    /// Create a new atmosphere display.
    #[must_use]
    pub fn new() -> Self {
        Self {
            o2_percent: 21.0,
            pressure: 101.3,
            temperature: 20.0,
            warning: AtmosphereWarning::Normal,
            room_name: String::new(),
            visible: true,
        }
    }

    /// Update display with new values.
    pub fn update(&mut self, o2: f32, pressure: f32, temperature: f32) {
        self.o2_percent = o2;
        self.pressure = pressure;
        self.temperature = temperature;
        self.update_warning();
    }

    /// Update warning level based on current values.
    fn update_warning(&mut self) {
        if self.o2_percent < 10.0 || self.pressure < 30.0 {
            self.warning = AtmosphereWarning::Danger;
        } else if self.o2_percent < 16.0 || self.pressure < 50.0 {
            self.warning = AtmosphereWarning::Warning;
        } else if self.o2_percent < 19.0 || self.pressure < 80.0 {
            self.warning = AtmosphereWarning::Caution;
        } else {
            self.warning = AtmosphereWarning::Normal;
        }
    }

    /// Get O2 percentage.
    #[must_use]
    pub fn o2_percent(&self) -> f32 {
        self.o2_percent
    }

    /// Get pressure.
    #[must_use]
    pub fn pressure(&self) -> f32 {
        self.pressure
    }

    /// Get temperature.
    #[must_use]
    pub fn temperature(&self) -> f32 {
        self.temperature
    }

    /// Get warning level.
    #[must_use]
    pub fn warning(&self) -> AtmosphereWarning {
        self.warning
    }

    /// Get room name.
    #[must_use]
    pub fn room_name(&self) -> &str {
        &self.room_name
    }

    /// Set room name.
    pub fn set_room_name(&mut self, name: String) {
        self.room_name = name;
    }

    /// Check if visible.
    #[must_use]
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Set visibility.
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Check if atmosphere is breathable.
    #[must_use]
    pub fn is_breathable(&self) -> bool {
        self.o2_percent >= 16.0 && self.pressure >= 50.0
    }

    /// Get formatted O2 string.
    #[must_use]
    pub fn o2_string(&self) -> String {
        format!("{:.1}%", self.o2_percent)
    }

    /// Get formatted pressure string.
    #[must_use]
    pub fn pressure_string(&self) -> String {
        format!("{:.1} kPa", self.pressure)
    }

    /// Get formatted temperature string.
    #[must_use]
    pub fn temperature_string(&self) -> String {
        format!("{:.1}°C", self.temperature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atmosphere_warning_display_names() {
        assert_eq!(AtmosphereWarning::Normal.display_name(), "Normal");
        assert_eq!(AtmosphereWarning::Caution.display_name(), "Caution");
        assert_eq!(AtmosphereWarning::Warning.display_name(), "Warning");
        assert_eq!(AtmosphereWarning::Danger.display_name(), "Danger");
    }

    #[test]
    fn test_atmosphere_warning_colors() {
        let normal = AtmosphereWarning::Normal.color();
        assert!((normal[1] - 1.0).abs() < f32::EPSILON); // Green

        let danger = AtmosphereWarning::Danger.color();
        assert!((danger[0] - 1.0).abs() < f32::EPSILON); // Red
    }

    #[test]
    fn test_room_atmosphere_display_new() {
        let display = RoomAtmosphereDisplay::new();
        assert!((display.o2_percent() - 21.0).abs() < f32::EPSILON);
        assert!((display.pressure() - 101.3).abs() < f32::EPSILON);
        assert!(display.is_visible());
    }

    #[test]
    fn test_room_atmosphere_display_update() {
        let mut display = RoomAtmosphereDisplay::new();
        display.update(18.0, 90.0, 22.0);

        assert!((display.o2_percent() - 18.0).abs() < f32::EPSILON);
        assert!((display.pressure() - 90.0).abs() < f32::EPSILON);
        assert!((display.temperature() - 22.0).abs() < f32::EPSILON);
        assert_eq!(display.warning(), AtmosphereWarning::Caution);
    }

    #[test]
    fn test_room_atmosphere_display_warning_levels() {
        let mut display = RoomAtmosphereDisplay::new();

        display.update(21.0, 101.3, 20.0);
        assert_eq!(display.warning(), AtmosphereWarning::Normal);

        display.update(18.0, 90.0, 20.0);
        assert_eq!(display.warning(), AtmosphereWarning::Caution);

        display.update(14.0, 70.0, 20.0);
        assert_eq!(display.warning(), AtmosphereWarning::Warning);

        display.update(8.0, 20.0, 20.0);
        assert_eq!(display.warning(), AtmosphereWarning::Danger);
    }

    #[test]
    fn test_room_atmosphere_display_breathable() {
        let mut display = RoomAtmosphereDisplay::new();
        assert!(display.is_breathable());

        display.update(14.0, 40.0, 20.0);
        assert!(!display.is_breathable());
    }

    #[test]
    fn test_room_atmosphere_display_room_name() {
        let mut display = RoomAtmosphereDisplay::new();
        display.set_room_name("Bridge".to_string());
        assert_eq!(display.room_name(), "Bridge");
    }

    #[test]
    fn test_room_atmosphere_display_visibility() {
        let mut display = RoomAtmosphereDisplay::new();
        display.set_visible(false);
        assert!(!display.is_visible());
    }

    #[test]
    fn test_room_atmosphere_display_formatted_strings() {
        let mut display = RoomAtmosphereDisplay::new();
        display.update(20.5, 99.8, 21.3);

        assert_eq!(display.o2_string(), "20.5%");
        assert_eq!(display.pressure_string(), "99.8 kPa");
        assert_eq!(display.temperature_string(), "21.3°C");
    }

    #[test]
    fn test_room_atmosphere_display_default() {
        let display = RoomAtmosphereDisplay::default();
        assert!(display.is_breathable());
    }
}
