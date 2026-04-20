//! Foundation systems for building on the Titan.
//!
//! Different foundation types that handle Titan movement.

use std::fmt;

use glam::IVec3;
use serde::{Deserialize, Serialize};

/// Types of foundations for structures.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FoundationType {
    /// Fixed foundation with minimal movement tolerance.
    Fixed,
    /// Sliding foundation with moderate movement tolerance.
    Sliding,
    /// Mobile foundation with high movement tolerance.
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

impl FoundationType {
    /// Get the maximum shift this foundation can absorb.
    #[must_use]
    pub fn max_shift(&self) -> i32 {
        match self {
            FoundationType::Fixed => 2,
            FoundationType::Sliding => 3,
            FoundationType::Mobile => 10,
        }
    }

    /// Get the base durability for this foundation type.
    #[must_use]
    pub fn base_durability(&self) -> f32 {
        match self {
            FoundationType::Fixed => 100.0,
            FoundationType::Sliding => 200.0,
            FoundationType::Mobile => 300.0,
        }
    }
}

/// A foundation structure.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Foundation {
    /// Type of foundation.
    pub foundation: FoundationType,
    /// Position of the foundation.
    pub position: IVec3,
    /// Maximum shift the foundation can absorb.
    max_shift: i32,
    /// Current durability.
    pub durability: f32,
    /// Maximum durability.
    max_durability: f32,
}

impl Foundation {
    /// Create a new foundation.
    #[must_use]
    pub fn new(foundation: FoundationType, position: IVec3) -> Self {
        let max_shift = foundation.max_shift();
        let durability = foundation.base_durability();
        Self {
            foundation,
            position,
            max_shift,
            durability,
            max_durability: durability,
        }
    }

    /// Attempt to absorb a shift amount.
    ///
    /// Returns `true` if the shift was absorbed, `false` if it exceeded capacity.
    /// Reduces durability on successful absorption.
    pub fn absorb_shift(&mut self, amount: i32) -> bool {
        if amount > self.max_shift {
            return false;
        }
        // Each absorbed shift reduces durability slightly
        self.durability = (self.durability - amount as f32).max(0.0);
        true
    }

    /// Check if the foundation is broken.
    #[must_use]
    pub fn is_broken(&self) -> bool {
        self.durability <= 0.0
    }

    /// Get the maximum shift capacity.
    #[must_use]
    pub fn max_shift(&self) -> i32 {
        self.max_shift
    }

    /// Get durability as a percentage.
    #[must_use]
    pub fn durability_percent(&self) -> f32 {
        self.durability / self.max_durability
    }

    /// Repair the foundation.
    pub fn repair(&mut self, amount: f32) {
        self.durability = (self.durability + amount).min(self.max_durability);
    }
}

/// A flexible joint that can bend under stress.
#[derive(Clone, Debug, Serialize, Deserialize)]
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

    /// Get the current bend amount.
    #[must_use]
    pub fn current_bend(&self) -> f32 {
        self.current_bend
    }

    /// Get the maximum bend capacity.
    #[must_use]
    pub fn max_bend(&self) -> f32 {
        self.max_bend
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
    fn test_foundation_type_max_shift() {
        assert_eq!(FoundationType::Fixed.max_shift(), 2);
        assert_eq!(FoundationType::Sliding.max_shift(), 3);
        assert_eq!(FoundationType::Mobile.max_shift(), 10);
    }

    #[test]
    fn test_foundation_type_durability() {
        assert!((FoundationType::Fixed.base_durability() - 100.0).abs() < f32::EPSILON);
        assert!((FoundationType::Sliding.base_durability() - 200.0).abs() < f32::EPSILON);
        assert!((FoundationType::Mobile.base_durability() - 300.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_foundation_new_fixed() {
        let foundation = Foundation::new(FoundationType::Fixed, IVec3::ZERO);
        assert_eq!(foundation.foundation, FoundationType::Fixed);
        assert_eq!(foundation.max_shift(), 2);
        assert!((foundation.durability - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_foundation_new_sliding() {
        let foundation = Foundation::new(FoundationType::Sliding, IVec3::new(1, 2, 3));
        assert_eq!(foundation.foundation, FoundationType::Sliding);
        assert_eq!(foundation.max_shift(), 3);
        assert!((foundation.durability - 200.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_foundation_new_mobile() {
        let foundation = Foundation::new(FoundationType::Mobile, IVec3::ONE);
        assert_eq!(foundation.foundation, FoundationType::Mobile);
        assert_eq!(foundation.max_shift(), 10);
        assert!((foundation.durability - 300.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_foundation_absorb_shift_success() {
        let mut foundation = Foundation::new(FoundationType::Fixed, IVec3::ZERO);
        assert!(foundation.absorb_shift(2));
    }

    #[test]
    fn test_foundation_absorb_shift_failure() {
        let mut foundation = Foundation::new(FoundationType::Fixed, IVec3::ZERO);
        assert!(!foundation.absorb_shift(3));
    }

    #[test]
    fn test_foundation_absorb_shift_reduces_durability() {
        let mut foundation = Foundation::new(FoundationType::Fixed, IVec3::ZERO);
        foundation.absorb_shift(2);
        assert!((foundation.durability - 98.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_foundation_is_broken() {
        let mut foundation = Foundation::new(FoundationType::Fixed, IVec3::ZERO);
        assert!(!foundation.is_broken());
        foundation.durability = 0.0;
        assert!(foundation.is_broken());
    }

    #[test]
    fn test_foundation_repair() {
        let mut foundation = Foundation::new(FoundationType::Fixed, IVec3::ZERO);
        foundation.durability = 50.0;
        foundation.repair(30.0);
        assert!((foundation.durability - 80.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_flexible_joint_new() {
        let joint = FlexibleJoint::new(IVec3::ZERO, 45.0);
        assert!(!joint.is_broken());
        assert!((joint.current_bend() - 0.0).abs() < f32::EPSILON);
        assert!((joint.max_bend() - 45.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_flexible_joint_bend() {
        let mut joint = FlexibleJoint::new(IVec3::ZERO, 45.0);
        let applied = joint.bend(20.0);
        assert!((applied - 20.0).abs() < f32::EPSILON);
        assert!((joint.current_bend() - 20.0).abs() < f32::EPSILON);
        assert!(!joint.is_broken());
    }

    #[test]
    fn test_flexible_joint_bend_capped() {
        let mut joint = FlexibleJoint::new(IVec3::ZERO, 45.0);
        joint.bend(30.0);
        let applied = joint.bend(20.0);
        assert!((applied - 15.0).abs() < f32::EPSILON);
        assert!(joint.is_broken());
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
        assert!((joint.current_bend() - 0.0).abs() < f32::EPSILON);
    }
}
