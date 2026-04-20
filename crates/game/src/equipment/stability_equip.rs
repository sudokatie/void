//! Stability equipment for dimensional operations.
//!
//! Provides detectors, builders, and tethers for interacting with
//! dimensional weak points and anchors.

use glam::IVec3;
use serde::{Deserialize, Serialize};

use crate::dimension::{AnchorTier, DimensionalAnchor};

/// Default detection range in blocks.
pub const DEFAULT_DETECTION_RANGE: f32 = 10.0;

/// Default tether maximum length.
pub const DEFAULT_TETHER_LENGTH: f32 = 20.0;

/// A detector for locating dimensional weak points.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StabilityDetector {
    /// Detection range in blocks.
    range: f32,
    /// Whether the detector is active.
    active: bool,
}

impl StabilityDetector {
    /// Create a new stability detector.
    #[must_use]
    pub fn new() -> Self {
        Self {
            range: DEFAULT_DETECTION_RANGE,
            active: true,
        }
    }

    /// Create a detector with custom range.
    #[must_use]
    pub fn with_range(range: f32) -> Self {
        Self {
            range: range.max(1.0),
            active: true,
        }
    }

    /// Detect weak points within range of the player position.
    ///
    /// Returns positions of weak points that are within detection range.
    #[must_use]
    pub fn detect_weak_points(&self, positions: Vec<IVec3>, player_pos: IVec3) -> Vec<IVec3> {
        if !self.active {
            return Vec::new();
        }

        let range_sq = self.range * self.range;
        positions
            .into_iter()
            .filter(|&pos| {
                let diff = pos - player_pos;
                let dist_sq = (diff.x * diff.x + diff.y * diff.y + diff.z * diff.z) as f32;
                dist_sq <= range_sq
            })
            .collect()
    }

    /// Toggle the detector on/off.
    pub fn toggle(&mut self) {
        self.active = !self.active;
    }

    /// Set the detector active state.
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    /// Check if the detector is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Get the detection range.
    #[must_use]
    pub fn range(&self) -> f32 {
        self.range
    }

    /// Set the detection range.
    pub fn set_range(&mut self, range: f32) {
        self.range = range.max(1.0);
    }
}

impl Default for StabilityDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Default build cost for anchors.
pub const DEFAULT_BUILD_COST: f32 = 50.0;

/// A builder for creating dimensional anchors.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnchorBuilder {
    /// Whether the builder can currently build.
    can_build: bool,
    /// Cost to build an anchor.
    build_cost: f32,
}

impl AnchorBuilder {
    /// Create a new anchor builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            can_build: true,
            build_cost: DEFAULT_BUILD_COST,
        }
    }

    /// Create an anchor builder with custom cost.
    #[must_use]
    pub fn with_cost(cost: f32) -> Self {
        Self {
            can_build: true,
            build_cost: cost.max(0.0),
        }
    }

    /// Attempt to build an anchor at the given position.
    ///
    /// Returns the created anchor if successful, None otherwise.
    #[must_use]
    pub fn build_anchor(&self, tier: &str, pos: IVec3) -> Option<DimensionalAnchor> {
        if !self.can_build {
            return None;
        }

        let anchor_tier = match tier.to_lowercase().as_str() {
            "basic" => AnchorTier::Basic,
            "standard" => AnchorTier::Standard,
            "military" => AnchorTier::Military,
            _ => return None,
        };

        Some(DimensionalAnchor::new(pos, anchor_tier))
    }

    /// Check if the builder can build.
    #[must_use]
    pub fn can_build(&self) -> bool {
        self.can_build
    }

    /// Set whether the builder can build.
    pub fn set_can_build(&mut self, can_build: bool) {
        self.can_build = can_build;
    }

    /// Get the build cost.
    #[must_use]
    pub fn build_cost(&self) -> f32 {
        self.build_cost
    }
}

impl Default for AnchorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// A tether for maintaining connection to dimensional anchors.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VoidTether {
    /// Maximum tether length.
    max_length: f32,
    /// Whether the tether is attached.
    attached: bool,
    /// Position of the anchor the tether is attached to.
    anchor_pos: Option<IVec3>,
}

impl VoidTether {
    /// Create a new void tether.
    #[must_use]
    pub fn new() -> Self {
        Self {
            max_length: DEFAULT_TETHER_LENGTH,
            attached: false,
            anchor_pos: None,
        }
    }

    /// Create a tether with custom maximum length.
    #[must_use]
    pub fn with_length(max_length: f32) -> Self {
        Self {
            max_length: max_length.max(1.0),
            attached: false,
            anchor_pos: None,
        }
    }

    /// Attach the tether to an anchor position.
    pub fn attach(&mut self, pos: IVec3) {
        self.attached = true;
        self.anchor_pos = Some(pos);
    }

    /// Detach the tether.
    pub fn detach(&mut self) {
        self.attached = false;
        self.anchor_pos = None;
    }

    /// Check if the player is within safe tether range.
    ///
    /// Returns true if the tether is attached and the player is within range.
    #[must_use]
    pub fn is_safe(&self, player_pos: IVec3) -> bool {
        if !self.attached {
            return false;
        }

        let Some(anchor_pos) = self.anchor_pos else {
            return false;
        };

        let diff = player_pos - anchor_pos;
        let dist_sq = (diff.x * diff.x + diff.y * diff.y + diff.z * diff.z) as f32;
        let max_sq = self.max_length * self.max_length;

        dist_sq <= max_sq
    }

    /// Get the distance to the anchor (if attached).
    #[must_use]
    pub fn distance_to_anchor(&self, player_pos: IVec3) -> Option<f32> {
        self.anchor_pos.map(|anchor_pos| {
            let diff = player_pos - anchor_pos;
            ((diff.x * diff.x + diff.y * diff.y + diff.z * diff.z) as f32).sqrt()
        })
    }

    /// Check if the tether is attached.
    #[must_use]
    pub fn is_attached(&self) -> bool {
        self.attached
    }

    /// Get the anchor position (if attached).
    #[must_use]
    pub fn anchor_pos(&self) -> Option<IVec3> {
        self.anchor_pos
    }

    /// Get the maximum tether length.
    #[must_use]
    pub fn max_length(&self) -> f32 {
        self.max_length
    }

    /// Get remaining safe distance from current position.
    #[must_use]
    pub fn remaining_length(&self, player_pos: IVec3) -> Option<f32> {
        self.distance_to_anchor(player_pos)
            .map(|dist| (self.max_length - dist).max(0.0))
    }
}

impl Default for VoidTether {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // StabilityDetector tests
    #[test]
    fn test_stability_detector_new() {
        let detector = StabilityDetector::new();
        assert!((detector.range() - DEFAULT_DETECTION_RANGE).abs() < f32::EPSILON);
        assert!(detector.is_active());
    }

    #[test]
    fn test_stability_detector_with_range() {
        let detector = StabilityDetector::with_range(15.0);
        assert!((detector.range() - 15.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_stability_detector_with_range_minimum() {
        let detector = StabilityDetector::with_range(-5.0);
        assert!((detector.range() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_stability_detector_detect_weak_points() {
        let detector = StabilityDetector::with_range(5.0);
        let player_pos = IVec3::ZERO;
        let positions = vec![
            IVec3::new(2, 0, 0),  // distance 2, within range
            IVec3::new(10, 0, 0), // distance 10, outside range
            IVec3::new(3, 3, 0),  // distance ~4.24, within range
        ];

        let detected = detector.detect_weak_points(positions, player_pos);
        assert_eq!(detected.len(), 2);
        assert!(detected.contains(&IVec3::new(2, 0, 0)));
        assert!(detected.contains(&IVec3::new(3, 3, 0)));
    }

    #[test]
    fn test_stability_detector_detect_inactive() {
        let mut detector = StabilityDetector::new();
        detector.set_active(false);

        let positions = vec![IVec3::new(1, 0, 0)];
        let detected = detector.detect_weak_points(positions, IVec3::ZERO);
        assert!(detected.is_empty());
    }

    #[test]
    fn test_stability_detector_toggle() {
        let mut detector = StabilityDetector::new();
        assert!(detector.is_active());

        detector.toggle();
        assert!(!detector.is_active());

        detector.toggle();
        assert!(detector.is_active());
    }

    #[test]
    fn test_stability_detector_set_range() {
        let mut detector = StabilityDetector::new();
        detector.set_range(25.0);
        assert!((detector.range() - 25.0).abs() < f32::EPSILON);
    }

    // AnchorBuilder tests
    #[test]
    fn test_anchor_builder_new() {
        let builder = AnchorBuilder::new();
        assert!(builder.can_build());
        assert!((builder.build_cost() - DEFAULT_BUILD_COST).abs() < f32::EPSILON);
    }

    #[test]
    fn test_anchor_builder_with_cost() {
        let builder = AnchorBuilder::with_cost(100.0);
        assert!((builder.build_cost() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_anchor_builder_build_basic() {
        let builder = AnchorBuilder::new();
        let anchor = builder.build_anchor("basic", IVec3::ZERO);

        assert!(anchor.is_some());
        let anchor = anchor.unwrap();
        assert_eq!(anchor.tier(), AnchorTier::Basic);
        assert_eq!(anchor.position(), IVec3::ZERO);
    }

    #[test]
    fn test_anchor_builder_build_standard() {
        let builder = AnchorBuilder::new();
        let anchor = builder.build_anchor("standard", IVec3::new(5, 5, 5));

        assert!(anchor.is_some());
        assert_eq!(anchor.unwrap().tier(), AnchorTier::Standard);
    }

    #[test]
    fn test_anchor_builder_build_military() {
        let builder = AnchorBuilder::new();
        let anchor = builder.build_anchor("military", IVec3::ZERO);

        assert!(anchor.is_some());
        assert_eq!(anchor.unwrap().tier(), AnchorTier::Military);
    }

    #[test]
    fn test_anchor_builder_build_case_insensitive() {
        let builder = AnchorBuilder::new();

        assert!(builder.build_anchor("BASIC", IVec3::ZERO).is_some());
        assert!(builder.build_anchor("Standard", IVec3::ZERO).is_some());
        assert!(builder.build_anchor("MiLiTaRy", IVec3::ZERO).is_some());
    }

    #[test]
    fn test_anchor_builder_build_invalid_tier() {
        let builder = AnchorBuilder::new();
        let anchor = builder.build_anchor("invalid", IVec3::ZERO);
        assert!(anchor.is_none());
    }

    #[test]
    fn test_anchor_builder_cannot_build() {
        let mut builder = AnchorBuilder::new();
        builder.set_can_build(false);

        let anchor = builder.build_anchor("basic", IVec3::ZERO);
        assert!(anchor.is_none());
    }

    // VoidTether tests
    #[test]
    fn test_void_tether_new() {
        let tether = VoidTether::new();
        assert!((tether.max_length() - DEFAULT_TETHER_LENGTH).abs() < f32::EPSILON);
        assert!(!tether.is_attached());
        assert!(tether.anchor_pos().is_none());
    }

    #[test]
    fn test_void_tether_with_length() {
        let tether = VoidTether::with_length(30.0);
        assert!((tether.max_length() - 30.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_void_tether_with_length_minimum() {
        let tether = VoidTether::with_length(-10.0);
        assert!((tether.max_length() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_void_tether_attach() {
        let mut tether = VoidTether::new();
        let anchor_pos = IVec3::new(10, 0, 0);

        tether.attach(anchor_pos);
        assert!(tether.is_attached());
        assert_eq!(tether.anchor_pos(), Some(anchor_pos));
    }

    #[test]
    fn test_void_tether_detach() {
        let mut tether = VoidTether::new();
        tether.attach(IVec3::ZERO);
        tether.detach();

        assert!(!tether.is_attached());
        assert!(tether.anchor_pos().is_none());
    }

    #[test]
    fn test_void_tether_is_safe_within_range() {
        let mut tether = VoidTether::with_length(10.0);
        tether.attach(IVec3::ZERO);

        assert!(tether.is_safe(IVec3::new(5, 0, 0))); // distance 5
        assert!(tether.is_safe(IVec3::new(7, 7, 0))); // distance ~9.9
    }

    #[test]
    fn test_void_tether_is_safe_outside_range() {
        let mut tether = VoidTether::with_length(10.0);
        tether.attach(IVec3::ZERO);

        assert!(!tether.is_safe(IVec3::new(15, 0, 0))); // distance 15
    }

    #[test]
    fn test_void_tether_is_safe_not_attached() {
        let tether = VoidTether::new();
        assert!(!tether.is_safe(IVec3::ZERO));
    }

    #[test]
    fn test_void_tether_distance_to_anchor() {
        let mut tether = VoidTether::new();
        tether.attach(IVec3::ZERO);

        let distance = tether.distance_to_anchor(IVec3::new(3, 4, 0));
        assert!(distance.is_some());
        assert!((distance.unwrap() - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_void_tether_distance_not_attached() {
        let tether = VoidTether::new();
        assert!(tether.distance_to_anchor(IVec3::ZERO).is_none());
    }

    #[test]
    fn test_void_tether_remaining_length() {
        let mut tether = VoidTether::with_length(20.0);
        tether.attach(IVec3::ZERO);

        let remaining = tether.remaining_length(IVec3::new(5, 0, 0));
        assert!(remaining.is_some());
        assert!((remaining.unwrap() - 15.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_void_tether_remaining_length_exceeded() {
        let mut tether = VoidTether::with_length(10.0);
        tether.attach(IVec3::ZERO);

        let remaining = tether.remaining_length(IVec3::new(15, 0, 0));
        assert!(remaining.is_some());
        assert!((remaining.unwrap() - 0.0).abs() < f32::EPSILON);
    }
}
