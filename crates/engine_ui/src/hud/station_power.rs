//! Station power HUD display.
//!
//! Shows power grid status, consumption, and battery levels.

use serde::{Deserialize, Serialize};

/// Power status level.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PowerStatus {
    /// Normal operation.
    Normal,
    /// Power shortage.
    Shortage,
    /// Critical power.
    Critical,
    /// No power.
    Offline,
}

impl PowerStatus {
    /// Get display name.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            PowerStatus::Normal => "Normal",
            PowerStatus::Shortage => "Shortage",
            PowerStatus::Critical => "Critical",
            PowerStatus::Offline => "Offline",
        }
    }

    /// Get color (RGBA).
    #[must_use]
    pub fn color(&self) -> [f32; 4] {
        match self {
            PowerStatus::Normal => [0.0, 1.0, 0.0, 1.0],
            PowerStatus::Shortage => [1.0, 1.0, 0.0, 1.0],
            PowerStatus::Critical => [1.0, 0.0, 0.0, 1.0],
            PowerStatus::Offline => [0.5, 0.5, 0.5, 1.0],
        }
    }
}

/// Station power display state.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StationPowerDisplay {
    /// Reactor output.
    reactor_output: f32,
    /// Total consumption.
    consumption: f32,
    /// Battery charge (0-100).
    battery_percent: f32,
    /// Power status.
    status: PowerStatus,
    /// Whether display is visible.
    visible: bool,
}

impl Default for StationPowerDisplay {
    fn default() -> Self {
        Self::new()
    }
}

impl StationPowerDisplay {
    /// Create a new power display.
    #[must_use]
    pub fn new() -> Self {
        Self {
            reactor_output: 100.0,
            consumption: 0.0,
            battery_percent: 100.0,
            status: PowerStatus::Normal,
            visible: true,
        }
    }

    /// Update display with new values.
    pub fn update(&mut self, output: f32, consumption: f32, battery: f32) {
        self.reactor_output = output;
        self.consumption = consumption;
        self.battery_percent = battery.clamp(0.0, 100.0);
        self.update_status();
    }

    /// Update status based on current values.
    fn update_status(&mut self) {
        if self.reactor_output <= 0.0 && self.battery_percent <= 0.0 {
            self.status = PowerStatus::Offline;
        } else if self.consumption > self.reactor_output && self.battery_percent < 10.0 {
            self.status = PowerStatus::Critical;
        } else if self.consumption > self.reactor_output {
            self.status = PowerStatus::Shortage;
        } else {
            self.status = PowerStatus::Normal;
        }
    }

    /// Get reactor output.
    #[must_use]
    pub fn reactor_output(&self) -> f32 {
        self.reactor_output
    }

    /// Get consumption.
    #[must_use]
    pub fn consumption(&self) -> f32 {
        self.consumption
    }

    /// Get battery percentage.
    #[must_use]
    pub fn battery_percent(&self) -> f32 {
        self.battery_percent
    }

    /// Get power status.
    #[must_use]
    pub fn status(&self) -> PowerStatus {
        self.status
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

    /// Get power surplus/deficit.
    #[must_use]
    pub fn power_balance(&self) -> f32 {
        self.reactor_output - self.consumption
    }

    /// Check if power is sufficient.
    #[must_use]
    pub fn is_sufficient(&self) -> bool {
        self.power_balance() >= 0.0 || self.battery_percent > 0.0
    }

    /// Get formatted output string.
    #[must_use]
    pub fn output_string(&self) -> String {
        format!("{:.0} kW", self.reactor_output)
    }

    /// Get formatted consumption string.
    #[must_use]
    pub fn consumption_string(&self) -> String {
        format!("{:.0} kW", self.consumption)
    }

    /// Get formatted battery string.
    #[must_use]
    pub fn battery_string(&self) -> String {
        format!("{:.0}%", self.battery_percent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_status_display_names() {
        assert_eq!(PowerStatus::Normal.display_name(), "Normal");
        assert_eq!(PowerStatus::Shortage.display_name(), "Shortage");
        assert_eq!(PowerStatus::Critical.display_name(), "Critical");
        assert_eq!(PowerStatus::Offline.display_name(), "Offline");
    }

    #[test]
    fn test_power_status_colors() {
        let normal = PowerStatus::Normal.color();
        assert!((normal[1] - 1.0).abs() < f32::EPSILON); // Green
    }

    #[test]
    fn test_station_power_display_new() {
        let display = StationPowerDisplay::new();
        assert!((display.reactor_output() - 100.0).abs() < f32::EPSILON);
        assert_eq!(display.status(), PowerStatus::Normal);
    }

    #[test]
    fn test_station_power_display_update() {
        let mut display = StationPowerDisplay::new();
        display.update(80.0, 60.0, 75.0);

        assert!((display.reactor_output() - 80.0).abs() < f32::EPSILON);
        assert!((display.consumption() - 60.0).abs() < f32::EPSILON);
        assert!((display.battery_percent() - 75.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_station_power_display_status_normal() {
        let mut display = StationPowerDisplay::new();
        display.update(100.0, 80.0, 100.0);
        assert_eq!(display.status(), PowerStatus::Normal);
    }

    #[test]
    fn test_station_power_display_status_shortage() {
        let mut display = StationPowerDisplay::new();
        display.update(50.0, 80.0, 50.0);
        assert_eq!(display.status(), PowerStatus::Shortage);
    }

    #[test]
    fn test_station_power_display_status_critical() {
        let mut display = StationPowerDisplay::new();
        display.update(50.0, 80.0, 5.0);
        assert_eq!(display.status(), PowerStatus::Critical);
    }

    #[test]
    fn test_station_power_display_status_offline() {
        let mut display = StationPowerDisplay::new();
        display.update(0.0, 0.0, 0.0);
        assert_eq!(display.status(), PowerStatus::Offline);
    }

    #[test]
    fn test_station_power_display_power_balance() {
        let mut display = StationPowerDisplay::new();
        display.update(100.0, 80.0, 100.0);
        assert!((display.power_balance() - 20.0).abs() < f32::EPSILON);

        display.update(50.0, 80.0, 100.0);
        assert!((display.power_balance() - -30.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_station_power_display_sufficient() {
        let mut display = StationPowerDisplay::new();
        display.update(100.0, 80.0, 100.0);
        assert!(display.is_sufficient());

        display.update(50.0, 80.0, 0.0);
        assert!(!display.is_sufficient());
    }

    #[test]
    fn test_station_power_display_formatted_strings() {
        let mut display = StationPowerDisplay::new();
        display.update(85.0, 60.0, 75.0);

        assert_eq!(display.output_string(), "85 kW");
        assert_eq!(display.consumption_string(), "60 kW");
        assert_eq!(display.battery_string(), "75%");
    }

    #[test]
    fn test_station_power_display_default() {
        let display = StationPowerDisplay::default();
        assert_eq!(display.status(), PowerStatus::Normal);
    }
}
