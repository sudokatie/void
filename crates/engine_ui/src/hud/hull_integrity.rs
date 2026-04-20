//! Hull integrity HUD display.
//!
//! Shows hull damage status and breach locations.

use serde::{Deserialize, Serialize};

/// Hull damage level.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HullDamageLevel {
    /// No damage.
    Intact,
    /// Minor damage.
    Minor,
    /// Moderate damage.
    Moderate,
    /// Severe damage.
    Severe,
    /// Breached.
    Breached,
}

impl HullDamageLevel {
    /// Get display name.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            HullDamageLevel::Intact => "Intact",
            HullDamageLevel::Minor => "Minor Damage",
            HullDamageLevel::Moderate => "Moderate Damage",
            HullDamageLevel::Severe => "Severe Damage",
            HullDamageLevel::Breached => "Breached",
        }
    }

    /// Get color (RGBA).
    #[must_use]
    pub fn color(&self) -> [f32; 4] {
        match self {
            HullDamageLevel::Intact => [0.0, 1.0, 0.0, 1.0],
            HullDamageLevel::Minor => [0.5, 1.0, 0.0, 1.0],
            HullDamageLevel::Moderate => [1.0, 1.0, 0.0, 1.0],
            HullDamageLevel::Severe => [1.0, 0.5, 0.0, 1.0],
            HullDamageLevel::Breached => [1.0, 0.0, 0.0, 1.0],
        }
    }

    /// Get damage level from integrity percentage.
    #[must_use]
    pub fn from_integrity(integrity: f32) -> Self {
        if integrity >= 100.0 {
            HullDamageLevel::Intact
        } else if integrity >= 75.0 {
            HullDamageLevel::Minor
        } else if integrity >= 50.0 {
            HullDamageLevel::Moderate
        } else if integrity >= 25.0 {
            HullDamageLevel::Severe
        } else {
            HullDamageLevel::Breached
        }
    }
}

/// A breach indicator for the HUD.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BreachIndicator {
    /// Room ID.
    room_id: usize,
    /// Room name.
    room_name: String,
    /// Severity (0-1).
    severity: f32,
}

impl BreachIndicator {
    /// Create a new breach indicator.
    #[must_use]
    pub fn new(room_id: usize, room_name: String, severity: f32) -> Self {
        Self {
            room_id,
            room_name,
            severity: severity.clamp(0.0, 1.0),
        }
    }

    /// Get room ID.
    #[must_use]
    pub fn room_id(&self) -> usize {
        self.room_id
    }

    /// Get room name.
    #[must_use]
    pub fn room_name(&self) -> &str {
        &self.room_name
    }

    /// Get severity.
    #[must_use]
    pub fn severity(&self) -> f32 {
        self.severity
    }
}

/// Hull integrity display state.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HullIntegrityDisplay {
    /// Overall hull integrity (0-100).
    integrity_percent: f32,
    /// Damage level.
    damage_level: HullDamageLevel,
    /// Active breaches.
    breaches: Vec<BreachIndicator>,
    /// Whether display is visible.
    visible: bool,
}

impl Default for HullIntegrityDisplay {
    fn default() -> Self {
        Self::new()
    }
}

impl HullIntegrityDisplay {
    /// Create a new hull integrity display.
    #[must_use]
    pub fn new() -> Self {
        Self {
            integrity_percent: 100.0,
            damage_level: HullDamageLevel::Intact,
            breaches: Vec::new(),
            visible: true,
        }
    }

    /// Update integrity.
    pub fn update(&mut self, integrity: f32) {
        self.integrity_percent = integrity.clamp(0.0, 100.0);
        self.damage_level = HullDamageLevel::from_integrity(self.integrity_percent);
    }

    /// Add a breach indicator.
    pub fn add_breach(&mut self, indicator: BreachIndicator) {
        // Don't duplicate
        if !self.breaches.iter().any(|b| b.room_id() == indicator.room_id()) {
            self.breaches.push(indicator);
        }
    }

    /// Remove a breach indicator.
    pub fn remove_breach(&mut self, room_id: usize) {
        self.breaches.retain(|b| b.room_id() != room_id);
    }

    /// Clear all breaches.
    pub fn clear_breaches(&mut self) {
        self.breaches.clear();
    }

    /// Get integrity percentage.
    #[must_use]
    pub fn integrity_percent(&self) -> f32 {
        self.integrity_percent
    }

    /// Get damage level.
    #[must_use]
    pub fn damage_level(&self) -> HullDamageLevel {
        self.damage_level
    }

    /// Get breach count.
    #[must_use]
    pub fn breach_count(&self) -> usize {
        self.breaches.len()
    }

    /// Get breaches.
    #[must_use]
    pub fn breaches(&self) -> &[BreachIndicator] {
        &self.breaches
    }

    /// Check if there are active breaches.
    #[must_use]
    pub fn has_breaches(&self) -> bool {
        !self.breaches.is_empty()
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

    /// Get formatted integrity string.
    #[must_use]
    pub fn integrity_string(&self) -> String {
        format!("{:.0}%", self.integrity_percent)
    }

    /// Check if hull is critical.
    #[must_use]
    pub fn is_critical(&self) -> bool {
        matches!(self.damage_level, HullDamageLevel::Severe | HullDamageLevel::Breached)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hull_damage_level_display_names() {
        assert_eq!(HullDamageLevel::Intact.display_name(), "Intact");
        assert_eq!(HullDamageLevel::Minor.display_name(), "Minor Damage");
        assert_eq!(HullDamageLevel::Breached.display_name(), "Breached");
    }

    #[test]
    fn test_hull_damage_level_from_integrity() {
        assert_eq!(HullDamageLevel::from_integrity(100.0), HullDamageLevel::Intact);
        assert_eq!(HullDamageLevel::from_integrity(80.0), HullDamageLevel::Minor);
        assert_eq!(HullDamageLevel::from_integrity(60.0), HullDamageLevel::Moderate);
        assert_eq!(HullDamageLevel::from_integrity(30.0), HullDamageLevel::Severe);
        assert_eq!(HullDamageLevel::from_integrity(10.0), HullDamageLevel::Breached);
    }

    #[test]
    fn test_breach_indicator_new() {
        let indicator = BreachIndicator::new(0, "Bridge".to_string(), 0.5);
        assert_eq!(indicator.room_id(), 0);
        assert_eq!(indicator.room_name(), "Bridge");
        assert!((indicator.severity() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hull_integrity_display_new() {
        let display = HullIntegrityDisplay::new();
        assert!((display.integrity_percent() - 100.0).abs() < f32::EPSILON);
        assert_eq!(display.damage_level(), HullDamageLevel::Intact);
        assert_eq!(display.breach_count(), 0);
    }

    #[test]
    fn test_hull_integrity_display_update() {
        let mut display = HullIntegrityDisplay::new();
        display.update(60.0);

        assert!((display.integrity_percent() - 60.0).abs() < f32::EPSILON);
        assert_eq!(display.damage_level(), HullDamageLevel::Moderate);
    }

    #[test]
    fn test_hull_integrity_display_breaches() {
        let mut display = HullIntegrityDisplay::new();
        display.add_breach(BreachIndicator::new(0, "Bridge".to_string(), 0.5));
        display.add_breach(BreachIndicator::new(1, "Engineering".to_string(), 0.8));

        assert!(display.has_breaches());
        assert_eq!(display.breach_count(), 2);
    }

    #[test]
    fn test_hull_integrity_display_no_duplicate_breaches() {
        let mut display = HullIntegrityDisplay::new();
        display.add_breach(BreachIndicator::new(0, "Bridge".to_string(), 0.5));
        display.add_breach(BreachIndicator::new(0, "Bridge".to_string(), 0.8));

        assert_eq!(display.breach_count(), 1);
    }

    #[test]
    fn test_hull_integrity_display_remove_breach() {
        let mut display = HullIntegrityDisplay::new();
        display.add_breach(BreachIndicator::new(0, "Bridge".to_string(), 0.5));
        display.add_breach(BreachIndicator::new(1, "Engineering".to_string(), 0.8));
        display.remove_breach(0);

        assert_eq!(display.breach_count(), 1);
        assert_eq!(display.breaches()[0].room_id(), 1);
    }

    #[test]
    fn test_hull_integrity_display_clear_breaches() {
        let mut display = HullIntegrityDisplay::new();
        display.add_breach(BreachIndicator::new(0, "Bridge".to_string(), 0.5));
        display.clear_breaches();

        assert!(!display.has_breaches());
    }

    #[test]
    fn test_hull_integrity_display_is_critical() {
        let mut display = HullIntegrityDisplay::new();
        assert!(!display.is_critical());

        display.update(20.0);
        assert!(display.is_critical());
    }

    #[test]
    fn test_hull_integrity_display_formatted_string() {
        let mut display = HullIntegrityDisplay::new();
        display.update(85.0);
        assert_eq!(display.integrity_string(), "85%");
    }

    #[test]
    fn test_hull_integrity_display_default() {
        let display = HullIntegrityDisplay::default();
        assert_eq!(display.damage_level(), HullDamageLevel::Intact);
    }
}
