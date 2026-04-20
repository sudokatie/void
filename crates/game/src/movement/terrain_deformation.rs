//! Terrain deformation and stability systems.
//!
//! Manages how structures respond to Titan movement.

use std::collections::HashMap;
use std::fmt;

use glam::IVec3;

/// Type of foundation for structures.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FoundationType {
    /// Fixed foundation, minimal movement tolerance.
    Fixed,
    /// Sliding foundation, moderate movement tolerance.
    Sliding,
    /// Mobile foundation, high movement tolerance.
    Mobile,
}

impl fmt::Display for FoundationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FoundationType::Fixed => write!(f, "Fixed"),
            FoundationType::Sliding => write!(f, "Sliding"),
            FoundationType::Mobile => write!(f, "Mobile"),
        }
    }
}

/// Properties of a foundation type.
#[derive(Clone, Debug)]
pub struct FoundationProperties {
    /// The foundation type.
    pub foundation: FoundationType,
    /// Maximum shift the foundation can absorb.
    pub max_shift: i32,
    /// Durability of the foundation.
    pub durability: f32,
}

impl FoundationProperties {
    /// Create foundation properties for a given type.
    #[must_use]
    pub fn new(foundation: FoundationType) -> Self {
        let (max_shift, durability) = match foundation {
            FoundationType::Fixed => (2, 100.0),
            FoundationType::Sliding => (3, 200.0),
            FoundationType::Mobile => (10, 300.0),
        };
        Self {
            foundation,
            max_shift,
            durability,
        }
    }

    /// Attempt to absorb a shift amount.
    ///
    /// Returns `false` if the shift exceeds the maximum.
    #[must_use]
    pub fn absorb_shift(&self, shift_amount: i32) -> bool {
        shift_amount <= self.max_shift
    }
}

/// Tracks terrain stability across the world.
#[derive(Clone, Debug)]
pub struct TerrainStability {
    /// Stability values for tracked blocks.
    block_stability: HashMap<IVec3, f32>,
}

impl TerrainStability {
    /// Create a new terrain stability tracker.
    #[must_use]
    pub fn new() -> Self {
        Self {
            block_stability: HashMap::new(),
        }
    }

    /// Get the stability at a position.
    ///
    /// Returns 1.0 (fully stable) if not tracked.
    #[must_use]
    pub fn stability_at(&self, pos: IVec3) -> f32 {
        self.block_stability.get(&pos).copied().unwrap_or(1.0)
    }

    /// Apply a shift to an area, reducing stability.
    pub fn apply_shift(&mut self, shift: IVec3, affected_area_radius: i32) {
        let shift_magnitude =
            ((shift.x * shift.x + shift.y * shift.y + shift.z * shift.z) as f32).sqrt();
        let stability_reduction = (shift_magnitude * 0.1).min(0.5);

        for x in -affected_area_radius..=affected_area_radius {
            for y in -affected_area_radius..=affected_area_radius {
                for z in -affected_area_radius..=affected_area_radius {
                    let pos = shift + IVec3::new(x, y, z);
                    let current = self.stability_at(pos);
                    let new_stability = (current - stability_reduction).max(0.0);
                    self.block_stability.insert(pos, new_stability);
                }
            }
        }
    }

    /// Set the stability at a specific position.
    pub fn set_stability(&mut self, pos: IVec3, stability: f32) {
        self.block_stability.insert(pos, stability.clamp(0.0, 1.0));
    }

    /// Check if a position is unstable.
    #[must_use]
    pub fn is_unstable(&self, pos: IVec3) -> bool {
        self.stability_at(pos) < 0.3
    }
}

impl Default for TerrainStability {
    fn default() -> Self {
        Self::new()
    }
}

/// A flexible joint that can bend under stress.
#[derive(Clone, Debug)]
pub struct FlexibleJoint {
    /// Position of the joint.
    pub position: IVec3,
    /// Maximum bend angle before breaking.
    max_bend: f32,
    /// Current bend amount.
    current_bend: f32,
}

impl FlexibleJoint {
    /// Create a new flexible joint.
    #[must_use]
    pub fn new(position: IVec3, max_bend: f32) -> Self {
        Self {
            position,
            max_bend,
            current_bend: 0.0,
        }
    }

    /// Apply a bend to the joint.
    ///
    /// Returns the actual bend applied (capped at max).
    pub fn bend(&mut self, amount: f32) -> f32 {
        let new_bend = self.current_bend + amount;
        let actual_bend = new_bend.min(self.max_bend);
        let applied = actual_bend - self.current_bend;
        self.current_bend = actual_bend;
        applied
    }

    /// Check if the joint is broken.
    #[must_use]
    pub fn is_broken(&self) -> bool {
        self.current_bend >= self.max_bend
    }

    /// Reset the joint to its original state.
    pub fn reset(&mut self) {
        self.current_bend = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foundation_type_display() {
        assert_eq!(format!("{}", FoundationType::Fixed), "Fixed");
        assert_eq!(format!("{}", FoundationType::Sliding), "Sliding");
        assert_eq!(format!("{}", FoundationType::Mobile), "Mobile");
    }

    #[test]
    fn test_foundation_properties_fixed() {
        let props = FoundationProperties::new(FoundationType::Fixed);
        assert_eq!(props.max_shift, 2);
        assert!((props.durability - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_foundation_properties_sliding() {
        let props = FoundationProperties::new(FoundationType::Sliding);
        assert_eq!(props.max_shift, 3);
        assert!((props.durability - 200.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_foundation_properties_mobile() {
        let props = FoundationProperties::new(FoundationType::Mobile);
        assert_eq!(props.max_shift, 10);
        assert!((props.durability - 300.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_foundation_absorb_shift_success() {
        let props = FoundationProperties::new(FoundationType::Fixed);
        assert!(props.absorb_shift(2));
    }

    #[test]
    fn test_foundation_absorb_shift_failure() {
        let props = FoundationProperties::new(FoundationType::Fixed);
        assert!(!props.absorb_shift(3));
    }

    #[test]
    fn test_terrain_stability_new() {
        let stability = TerrainStability::new();
        assert!((stability.stability_at(IVec3::ZERO) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_terrain_stability_set_and_get() {
        let mut stability = TerrainStability::new();
        stability.set_stability(IVec3::new(1, 2, 3), 0.5);
        assert!((stability.stability_at(IVec3::new(1, 2, 3)) - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_terrain_stability_is_unstable() {
        let mut stability = TerrainStability::new();
        stability.set_stability(IVec3::ZERO, 0.2);
        assert!(stability.is_unstable(IVec3::ZERO));
    }

    #[test]
    fn test_terrain_stability_not_unstable() {
        let mut stability = TerrainStability::new();
        stability.set_stability(IVec3::ZERO, 0.5);
        assert!(!stability.is_unstable(IVec3::ZERO));
    }

    #[test]
    fn test_flexible_joint_new() {
        let joint = FlexibleJoint::new(IVec3::ZERO, 45.0);
        assert!(!joint.is_broken());
    }

    #[test]
    fn test_flexible_joint_bend() {
        let mut joint = FlexibleJoint::new(IVec3::ZERO, 45.0);
        let applied = joint.bend(20.0);
        assert!((applied - 20.0).abs() < f32::EPSILON);
        assert!(!joint.is_broken());
    }

    #[test]
    fn test_flexible_joint_break() {
        let mut joint = FlexibleJoint::new(IVec3::ZERO, 45.0);
        joint.bend(45.0);
        assert!(joint.is_broken());
    }

    #[test]
    fn test_flexible_joint_reset() {
        let mut joint = FlexibleJoint::new(IVec3::ZERO, 45.0);
        joint.bend(30.0);
        joint.reset();
        assert!(!joint.is_broken());
    }
}
