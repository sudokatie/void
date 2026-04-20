//! EVA suit status HUD display.
//!
//! Shows suit O2, thruster fuel, and integrity.

use serde::{Deserialize, Serialize};

/// Suit warning level.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuitWarning {
    /// All systems nominal.
    Nominal,
    /// Low resources.
    Low,
    /// Critical resources.
    Critical,
    /// Emergency.
    Emergency,
}

impl SuitWarning {
    /// Get display name.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            SuitWarning::Nominal => "Nominal",
            SuitWarning::Low => "Low",
            SuitWarning::Critical => "Critical",
            SuitWarning::Emergency => "Emergency",
        }
    }

    /// Get color (RGBA).
    #[must_use]
    pub fn color(&self) -> [f32; 4] {
        match self {
            SuitWarning::Nominal => [0.0, 1.0, 0.0, 1.0],
            SuitWarning::Low => [1.0, 1.0, 0.0, 1.0],
            SuitWarning::Critical => [1.0, 0.5, 0.0, 1.0],
            SuitWarning::Emergency => [1.0, 0.0, 0.0, 1.0],
        }
    }
}

/// Suit status display state.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuitStatusDisplay {
    /// O2 remaining (0-100).
    o2_percent: f32,
    /// Thruster fuel (0-100).
    fuel_percent: f32,
    /// Suit integrity (0-100).
    integrity_percent: f32,
    /// Overall warning level.
    warning: SuitWarning,
    /// Whether in EVA.
    in_eva: bool,
    /// Whether display is visible.
    visible: bool,
}

impl Default for SuitStatusDisplay {
    fn default() -> Self {
        Self::new()
    }
}

impl SuitStatusDisplay {
    /// Create a new suit status display.
    #[must_use]
    pub fn new() -> Self {
        Self {
            o2_percent: 100.0,
            fuel_percent: 100.0,
            integrity_percent: 100.0,
            warning: SuitWarning::Nominal,
            in_eva: false,
            visible: true,
        }
    }

    /// Update display with new values.
    pub fn update(&mut self, o2: f32, fuel: f32, integrity: f32) {
        self.o2_percent = o2.clamp(0.0, 100.0);
        self.fuel_percent = fuel.clamp(0.0, 100.0);
        self.integrity_percent = integrity.clamp(0.0, 100.0);
        self.update_warning();
    }

    /// Update warning level.
    fn update_warning(&mut self) {
        let min_value = self.o2_percent.min(self.fuel_percent).min(self.integrity_percent);

        if min_value < 10.0 {
            self.warning = SuitWarning::Emergency;
        } else if min_value < 25.0 {
            self.warning = SuitWarning::Critical;
        } else if min_value < 50.0 {
            self.warning = SuitWarning::Low;
        } else {
            self.warning = SuitWarning::Nominal;
        }
    }

    /// Get O2 percentage.
    #[must_use]
    pub fn o2_percent(&self) -> f32 {
        self.o2_percent
    }

    /// Get fuel percentage.
    #[must_use]
    pub fn fuel_percent(&self) -> f32 {
        self.fuel_percent
    }

    /// Get integrity percentage.
    #[must_use]
    pub fn integrity_percent(&self) -> f32 {
        self.integrity_percent
    }

    /// Get warning level.
    #[must_use]
    pub fn warning(&self) -> SuitWarning {
        self.warning
    }

    /// Check if in EVA.
    #[must_use]
    pub fn in_eva(&self) -> bool {
        self.in_eva
    }

    /// Set EVA state.
    pub fn set_in_eva(&mut self, in_eva: bool) {
        self.in_eva = in_eva;
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

    /// Get formatted O2 string.
    #[must_use]
    pub fn o2_string(&self) -> String {
        format!("{:.0}%", self.o2_percent)
    }

    /// Get formatted fuel string.
    #[must_use]
    pub fn fuel_string(&self) -> String {
        format!("{:.0}%", self.fuel_percent)
    }

    /// Get formatted integrity string.
    #[must_use]
    pub fn integrity_string(&self) -> String {
        format!("{:.0}%", self.integrity_percent)
    }

    /// Check if suit is operational.
    #[must_use]
    pub fn is_operational(&self) -> bool {
        self.integrity_percent > 0.0 && self.o2_percent > 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suit_warning_display_names() {
        assert_eq!(SuitWarning::Nominal.display_name(), "Nominal");
        assert_eq!(SuitWarning::Low.display_name(), "Low");
        assert_eq!(SuitWarning::Critical.display_name(), "Critical");
        assert_eq!(SuitWarning::Emergency.display_name(), "Emergency");
    }

    #[test]
    fn test_suit_status_display_new() {
        let display = SuitStatusDisplay::new();
        assert!((display.o2_percent() - 100.0).abs() < f32::EPSILON);
        assert!(!display.in_eva());
        assert!(display.is_visible());
    }

    #[test]
    fn test_suit_status_display_update() {
        let mut display = SuitStatusDisplay::new();
        display.update(80.0, 60.0, 90.0);

        assert!((display.o2_percent() - 80.0).abs() < f32::EPSILON);
        assert!((display.fuel_percent() - 60.0).abs() < f32::EPSILON);
        assert!((display.integrity_percent() - 90.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_suit_status_display_warning_nominal() {
        let mut display = SuitStatusDisplay::new();
        display.update(80.0, 70.0, 90.0);
        assert_eq!(display.warning(), SuitWarning::Nominal);
    }

    #[test]
    fn test_suit_status_display_warning_low() {
        let mut display = SuitStatusDisplay::new();
        display.update(40.0, 70.0, 90.0);
        assert_eq!(display.warning(), SuitWarning::Low);
    }

    #[test]
    fn test_suit_status_display_warning_critical() {
        let mut display = SuitStatusDisplay::new();
        display.update(20.0, 70.0, 90.0);
        assert_eq!(display.warning(), SuitWarning::Critical);
    }

    #[test]
    fn test_suit_status_display_warning_emergency() {
        let mut display = SuitStatusDisplay::new();
        display.update(5.0, 70.0, 90.0);
        assert_eq!(display.warning(), SuitWarning::Emergency);
    }

    #[test]
    fn test_suit_status_display_eva() {
        let mut display = SuitStatusDisplay::new();
        display.set_in_eva(true);
        assert!(display.in_eva());
    }

    #[test]
    fn test_suit_status_display_operational() {
        let mut display = SuitStatusDisplay::new();
        assert!(display.is_operational());

        display.update(0.0, 50.0, 50.0);
        assert!(!display.is_operational());
    }

    #[test]
    fn test_suit_status_display_formatted_strings() {
        let mut display = SuitStatusDisplay::new();
        display.update(85.0, 60.0, 95.0);

        assert_eq!(display.o2_string(), "85%");
        assert_eq!(display.fuel_string(), "60%");
        assert_eq!(display.integrity_string(), "95%");
    }

    #[test]
    fn test_suit_status_display_default() {
        let display = SuitStatusDisplay::default();
        assert_eq!(display.warning(), SuitWarning::Nominal);
    }
}
