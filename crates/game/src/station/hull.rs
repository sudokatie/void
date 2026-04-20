//! Hull segment integrity tracking.
//!
//! Tracks damage and breaches in hull segments.

use serde::{Deserialize, Serialize};

/// A segment of the station hull.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HullSegment {
    /// ID of the room this segment belongs to.
    room_id: usize,
    /// Integrity percentage (0-100).
    integrity: f32,
    /// Whether this segment is breached.
    breached: bool,
}

impl HullSegment {
    /// Create a new hull segment for a room.
    #[must_use]
    pub fn new(room_id: usize) -> Self {
        Self {
            room_id,
            integrity: 100.0,
            breached: false,
        }
    }

    /// Get the room ID.
    #[must_use]
    pub fn room_id(&self) -> usize {
        self.room_id
    }

    /// Get the integrity percentage.
    #[must_use]
    pub fn integrity(&self) -> f32 {
        self.integrity
    }

    /// Check if the segment is breached.
    #[must_use]
    pub fn is_breached(&self) -> bool {
        self.breached
    }

    /// Damage the hull segment.
    ///
    /// If integrity drops below a threshold, the segment becomes breached.
    pub fn damage(&mut self, amount: f32) {
        self.integrity = (self.integrity - amount).max(0.0);
        if self.integrity < 25.0 {
            self.breached = true;
        }
    }

    /// Patch the hull segment.
    ///
    /// Restores integrity and seals the breach if integrity is sufficient.
    pub fn patch(&mut self, amount: f32) {
        self.integrity = (self.integrity + amount).min(100.0);
        if self.integrity >= 50.0 {
            self.breached = false;
        }
    }

    /// Force seal the breach without restoring integrity.
    pub fn emergency_seal(&mut self) {
        self.breached = false;
    }

    /// Get the damage level as a category.
    #[must_use]
    pub fn damage_level(&self) -> &'static str {
        if self.integrity >= 75.0 {
            "Intact"
        } else if self.integrity >= 50.0 {
            "Damaged"
        } else if self.integrity >= 25.0 {
            "Critical"
        } else {
            "Compromised"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hull_segment_new() {
        let segment = HullSegment::new(5);
        assert_eq!(segment.room_id(), 5);
        assert!((segment.integrity() - 100.0).abs() < f32::EPSILON);
        assert!(!segment.is_breached());
    }

    #[test]
    fn test_hull_segment_damage() {
        let mut segment = HullSegment::new(0);
        segment.damage(30.0);
        assert!((segment.integrity() - 70.0).abs() < f32::EPSILON);
        assert!(!segment.is_breached());
    }

    #[test]
    fn test_hull_segment_damage_breach() {
        let mut segment = HullSegment::new(0);
        segment.damage(80.0);
        assert!((segment.integrity() - 20.0).abs() < f32::EPSILON);
        assert!(segment.is_breached());
    }

    #[test]
    fn test_hull_segment_damage_minimum() {
        let mut segment = HullSegment::new(0);
        segment.damage(200.0);
        assert!((segment.integrity() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hull_segment_patch() {
        let mut segment = HullSegment::new(0);
        segment.damage(50.0);
        segment.patch(30.0);
        assert!((segment.integrity() - 80.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hull_segment_patch_seals_breach() {
        let mut segment = HullSegment::new(0);
        segment.damage(80.0);
        assert!(segment.is_breached());

        segment.patch(40.0);
        assert!((segment.integrity() - 60.0).abs() < f32::EPSILON);
        assert!(!segment.is_breached());
    }

    #[test]
    fn test_hull_segment_patch_maximum() {
        let mut segment = HullSegment::new(0);
        segment.patch(50.0);
        assert!((segment.integrity() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hull_segment_emergency_seal() {
        let mut segment = HullSegment::new(0);
        segment.damage(80.0);
        assert!(segment.is_breached());

        segment.emergency_seal();
        assert!(!segment.is_breached());
        assert!((segment.integrity() - 20.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hull_segment_damage_level_intact() {
        let segment = HullSegment::new(0);
        assert_eq!(segment.damage_level(), "Intact");
    }

    #[test]
    fn test_hull_segment_damage_level_damaged() {
        let mut segment = HullSegment::new(0);
        segment.damage(30.0);
        assert_eq!(segment.damage_level(), "Damaged");
    }

    #[test]
    fn test_hull_segment_damage_level_critical() {
        let mut segment = HullSegment::new(0);
        segment.damage(60.0);
        assert_eq!(segment.damage_level(), "Critical");
    }

    #[test]
    fn test_hull_segment_damage_level_compromised() {
        let mut segment = HullSegment::new(0);
        segment.damage(85.0);
        assert_eq!(segment.damage_level(), "Compromised");
    }
}
