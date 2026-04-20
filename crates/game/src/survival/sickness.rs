//! Fracture sickness system.
//!
//! Tracks the accumulated sickness from dimension exposure,
//! with effects ranging from mild disorientation to critical hallucinations.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Recovery rate per tick when in Prime dimension.
pub const PRIME_RECOVERY_RATE: f32 = 2.0;

/// Sickness severity levels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum SicknessLevel {
    /// No sickness (0).
    #[default]
    None,
    /// Mild sickness (1-25).
    Mild,
    /// Moderate sickness (26-50).
    Moderate,
    /// Severe sickness (51-75).
    Severe,
    /// Critical sickness (76-100).
    Critical,
}

impl SicknessLevel {
    /// Get the level from a numeric sickness value.
    #[must_use]
    pub fn from_value(value: f32) -> Self {
        match value {
            v if v <= 0.0 => SicknessLevel::None,
            v if v <= 25.0 => SicknessLevel::Mild,
            v if v <= 50.0 => SicknessLevel::Moderate,
            v if v <= 75.0 => SicknessLevel::Severe,
            _ => SicknessLevel::Critical,
        }
    }
}

impl fmt::Display for SicknessLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SicknessLevel::None => write!(f, "None"),
            SicknessLevel::Mild => write!(f, "Mild"),
            SicknessLevel::Moderate => write!(f, "Moderate"),
            SicknessLevel::Severe => write!(f, "Severe"),
            SicknessLevel::Critical => write!(f, "Critical"),
        }
    }
}

/// Fracture sickness tracker for a player.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FractureSickness {
    /// Current sickness level (0-100).
    level: f32,
    /// Cached enum level.
    current_level_enum: SicknessLevel,
    /// Whether the player is currently in Prime dimension.
    in_prime: bool,
}

impl FractureSickness {
    /// Create a new sickness tracker at zero sickness.
    #[must_use]
    pub fn new() -> Self {
        Self {
            level: 0.0,
            current_level_enum: SicknessLevel::None,
            in_prime: true,
        }
    }

    /// Add sickness (capped at 100).
    pub fn add_sickness(&mut self, amount: f32) {
        if amount <= 0.0 {
            return;
        }

        self.level = (self.level + amount).min(100.0);
        self.current_level_enum = SicknessLevel::from_value(self.level);
    }

    /// Tick the sickness system.
    ///
    /// Recovers sickness if in Prime dimension.
    /// Returns the current sickness level.
    pub fn tick(&mut self, _dt: f32) -> SicknessLevel {
        if self.in_prime && self.level > 0.0 {
            self.level = (self.level - PRIME_RECOVERY_RATE).max(0.0);
            self.current_level_enum = SicknessLevel::from_value(self.level);
        }

        self.current_level_enum
    }

    /// Set whether the player is in Prime dimension.
    pub fn set_in_prime(&mut self, in_prime: bool) {
        self.in_prime = in_prime;
    }

    /// Get the raw sickness level (0-100).
    #[must_use]
    pub fn sickness_level(&self) -> f32 {
        self.level
    }

    /// Get the sickness as an enum level.
    #[must_use]
    pub fn sickness_enum(&self) -> SicknessLevel {
        self.current_level_enum
    }

    /// Check if sickness is at critical level.
    #[must_use]
    pub fn is_critical(&self) -> bool {
        self.current_level_enum == SicknessLevel::Critical
    }

    /// Get movement speed penalty (0.0 to 0.8).
    ///
    /// Returns:
    /// - None/Mild: 0.0
    /// - Moderate: 0.3
    /// - Severe: 0.5
    /// - Critical: 0.8
    #[must_use]
    pub fn movement_penalty(&self) -> f32 {
        match self.current_level_enum {
            SicknessLevel::None | SicknessLevel::Mild => 0.0,
            SicknessLevel::Moderate => 0.3,
            SicknessLevel::Severe => 0.5,
            SicknessLevel::Critical => 0.8,
        }
    }

    /// Check if hallucinations should occur (Severe and above).
    #[must_use]
    pub fn has_hallucinations(&self) -> bool {
        matches!(
            self.current_level_enum,
            SicknessLevel::Severe | SicknessLevel::Critical
        )
    }

    /// Check if random teleportation can occur (Critical only).
    #[must_use]
    pub fn can_teleport_randomly(&self) -> bool {
        self.current_level_enum == SicknessLevel::Critical
    }

    /// Reset sickness to zero.
    pub fn reset(&mut self) {
        self.level = 0.0;
        self.current_level_enum = SicknessLevel::None;
    }

    // HUD Display helpers

    /// Get the display color for the current sickness level.
    ///
    /// Returns RGB color values (0.0 to 1.0).
    #[must_use]
    pub fn display_color(&self) -> [f32; 3] {
        match self.current_level_enum {
            SicknessLevel::None => [0.3, 0.9, 0.3],     // Green - healthy
            SicknessLevel::Mild => [0.8, 0.9, 0.3],    // Yellow-green
            SicknessLevel::Moderate => [1.0, 0.8, 0.2], // Yellow-orange
            SicknessLevel::Severe => [1.0, 0.5, 0.2],  // Orange
            SicknessLevel::Critical => [1.0, 0.2, 0.2], // Red
        }
    }

    /// Get the warning text for the current sickness level.
    #[must_use]
    pub fn warning_text(&self) -> Option<&'static str> {
        match self.current_level_enum {
            SicknessLevel::None => None,
            SicknessLevel::Mild => Some("Mild disorientation"),
            SicknessLevel::Moderate => Some("Vision impaired"),
            SicknessLevel::Severe => Some("Hallucinations occurring"),
            SicknessLevel::Critical => Some("CRITICAL - Return to Prime!"),
        }
    }

    /// Get the effect intensity for visual/audio effects (0.0 to 1.0).
    #[must_use]
    pub fn effect_intensity(&self) -> f32 {
        match self.current_level_enum {
            SicknessLevel::None => 0.0,
            SicknessLevel::Mild => 0.1,
            SicknessLevel::Moderate => 0.3,
            SicknessLevel::Severe => 0.6,
            SicknessLevel::Critical => 1.0,
        }
    }

    /// Get the pulse rate for HUD animations (Hz).
    #[must_use]
    pub fn pulse_rate(&self) -> f32 {
        match self.current_level_enum {
            SicknessLevel::None => 0.0,
            SicknessLevel::Mild => 0.3,
            SicknessLevel::Moderate => 0.5,
            SicknessLevel::Severe => 1.0,
            SicknessLevel::Critical => 2.0,
        }
    }
}

impl Default for FractureSickness {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sickness_level_from_value() {
        assert_eq!(SicknessLevel::from_value(0.0), SicknessLevel::None);
        assert_eq!(SicknessLevel::from_value(15.0), SicknessLevel::Mild);
        assert_eq!(SicknessLevel::from_value(25.0), SicknessLevel::Mild);
        assert_eq!(SicknessLevel::from_value(40.0), SicknessLevel::Moderate);
        assert_eq!(SicknessLevel::from_value(60.0), SicknessLevel::Severe);
        assert_eq!(SicknessLevel::from_value(90.0), SicknessLevel::Critical);
    }

    #[test]
    fn test_sickness_level_display() {
        assert_eq!(format!("{}", SicknessLevel::None), "None");
        assert_eq!(format!("{}", SicknessLevel::Mild), "Mild");
        assert_eq!(format!("{}", SicknessLevel::Moderate), "Moderate");
        assert_eq!(format!("{}", SicknessLevel::Severe), "Severe");
        assert_eq!(format!("{}", SicknessLevel::Critical), "Critical");
    }

    #[test]
    fn test_new() {
        let sickness = FractureSickness::new();
        assert!((sickness.sickness_level() - 0.0).abs() < f32::EPSILON);
        assert_eq!(sickness.sickness_enum(), SicknessLevel::None);
        assert!(!sickness.is_critical());
    }

    #[test]
    fn test_add_sickness() {
        let mut sickness = FractureSickness::new();

        sickness.add_sickness(30.0);
        assert!((sickness.sickness_level() - 30.0).abs() < f32::EPSILON);
        assert_eq!(sickness.sickness_enum(), SicknessLevel::Moderate);
    }

    #[test]
    fn test_add_sickness_capped() {
        let mut sickness = FractureSickness::new();

        sickness.add_sickness(150.0);
        assert!((sickness.sickness_level() - 100.0).abs() < f32::EPSILON);
        assert!(sickness.is_critical());
    }

    #[test]
    fn test_tick_recovery_in_prime() {
        let mut sickness = FractureSickness::new();
        sickness.add_sickness(10.0);

        sickness.tick(1.0);
        assert!((sickness.sickness_level() - 8.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_tick_no_recovery_outside_prime() {
        let mut sickness = FractureSickness::new();
        sickness.add_sickness(10.0);
        sickness.set_in_prime(false);

        sickness.tick(1.0);
        assert!((sickness.sickness_level() - 10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_movement_penalty() {
        let mut sickness = FractureSickness::new();
        assert!((sickness.movement_penalty() - 0.0).abs() < f32::EPSILON);

        sickness.add_sickness(15.0); // Mild
        assert!((sickness.movement_penalty() - 0.0).abs() < f32::EPSILON);

        sickness.add_sickness(20.0); // Moderate (35)
        assert!((sickness.movement_penalty() - 0.3).abs() < f32::EPSILON);

        sickness.add_sickness(25.0); // Severe (60)
        assert!((sickness.movement_penalty() - 0.5).abs() < f32::EPSILON);

        sickness.add_sickness(30.0); // Critical (90)
        assert!((sickness.movement_penalty() - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hallucinations() {
        let mut sickness = FractureSickness::new();
        assert!(!sickness.has_hallucinations());

        sickness.add_sickness(60.0); // Severe
        assert!(sickness.has_hallucinations());

        sickness.add_sickness(30.0); // Critical
        assert!(sickness.has_hallucinations());
    }

    #[test]
    fn test_random_teleport() {
        let mut sickness = FractureSickness::new();
        assert!(!sickness.can_teleport_randomly());

        sickness.add_sickness(60.0); // Severe
        assert!(!sickness.can_teleport_randomly());

        sickness.add_sickness(30.0); // Critical
        assert!(sickness.can_teleport_randomly());
    }

    #[test]
    fn test_reset() {
        let mut sickness = FractureSickness::new();
        sickness.add_sickness(80.0);

        sickness.reset();
        assert!((sickness.sickness_level() - 0.0).abs() < f32::EPSILON);
        assert_eq!(sickness.sickness_enum(), SicknessLevel::None);
    }

    #[test]
    fn test_is_critical() {
        let mut sickness = FractureSickness::new();
        assert!(!sickness.is_critical());

        sickness.add_sickness(50.0); // Moderate
        assert!(!sickness.is_critical());

        sickness.add_sickness(30.0); // Critical (80)
        assert!(sickness.is_critical());
    }
}
