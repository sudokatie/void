//! Recoil physics for zero-gravity.
//!
//! Handles weapon and tool recoil in zero-g environments.

use glam::IVec3;
use serde::{Deserialize, Serialize};

/// Recoil calculation system.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RecoilSystem;

impl RecoilSystem {
    /// Create a new recoil system.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Apply recoil to a velocity.
    ///
    /// Returns the new velocity after recoil is applied.
    #[must_use]
    pub fn apply_recoil(&self, velocity: IVec3, force: IVec3) -> IVec3 {
        velocity - force
    }

    /// Calculate recoil force from a tool's weight.
    ///
    /// Heavier tools produce more recoil when used.
    #[must_use]
    pub fn calculate_recoil(&self, tool_weight: f32) -> IVec3 {
        // Recoil is proportional to tool weight
        let magnitude = (tool_weight * 0.5) as i32;
        IVec3::new(magnitude, 0, 0)
    }

    /// Calculate directional recoil.
    ///
    /// Recoil is opposite to the action direction.
    #[must_use]
    pub fn calculate_directional_recoil(&self, direction: IVec3, force_magnitude: f32) -> IVec3 {
        let magnitude = force_magnitude as i32;
        IVec3::new(
            -direction.x.signum() * magnitude,
            -direction.y.signum() * magnitude,
            -direction.z.signum() * magnitude,
        )
    }

    /// Calculate weapon fire recoil.
    #[must_use]
    pub fn weapon_recoil(&self, weapon_power: f32) -> IVec3 {
        let magnitude = (weapon_power * 0.3) as i32;
        IVec3::new(magnitude, 0, 0)
    }

    /// Calculate push recoil (from pushing off a surface).
    #[must_use]
    pub fn push_recoil(&self, push_strength: f32, direction: IVec3) -> IVec3 {
        let magnitude = push_strength as i32;
        IVec3::new(
            direction.x * magnitude,
            direction.y * magnitude,
            direction.z * magnitude,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recoil_system_new() {
        let _system = RecoilSystem::new();
    }

    #[test]
    fn test_apply_recoil() {
        let system = RecoilSystem::new();
        let velocity = IVec3::new(10, 0, 0);
        let force = IVec3::new(3, 0, 0);

        let result = system.apply_recoil(velocity, force);
        assert_eq!(result, IVec3::new(7, 0, 0));
    }

    #[test]
    fn test_apply_recoil_negative() {
        let system = RecoilSystem::new();
        let velocity = IVec3::new(5, 0, 0);
        let force = IVec3::new(10, 0, 0);

        let result = system.apply_recoil(velocity, force);
        assert_eq!(result, IVec3::new(-5, 0, 0));
    }

    #[test]
    fn test_calculate_recoil() {
        let system = RecoilSystem::new();
        let recoil = system.calculate_recoil(10.0);
        assert_eq!(recoil.x, 5);
    }

    #[test]
    fn test_calculate_recoil_light_tool() {
        let system = RecoilSystem::new();
        let recoil = system.calculate_recoil(2.0);
        assert_eq!(recoil.x, 1);
    }

    #[test]
    fn test_calculate_directional_recoil() {
        let system = RecoilSystem::new();
        let direction = IVec3::new(1, 0, 0);
        let recoil = system.calculate_directional_recoil(direction, 5.0);

        // Recoil should be opposite to direction
        assert_eq!(recoil.x, -5);
    }

    #[test]
    fn test_calculate_directional_recoil_negative() {
        let system = RecoilSystem::new();
        let direction = IVec3::new(-1, 1, 0);
        let recoil = system.calculate_directional_recoil(direction, 3.0);

        assert_eq!(recoil.x, 3);
        assert_eq!(recoil.y, -3);
    }

    #[test]
    fn test_weapon_recoil() {
        let system = RecoilSystem::new();
        let recoil = system.weapon_recoil(10.0);
        assert_eq!(recoil.x, 3);
    }

    #[test]
    fn test_push_recoil() {
        let system = RecoilSystem::new();
        let direction = IVec3::new(1, 0, 0);
        let recoil = system.push_recoil(5.0, direction);

        assert_eq!(recoil.x, 5);
        assert_eq!(recoil.y, 0);
        assert_eq!(recoil.z, 0);
    }

    #[test]
    fn test_push_recoil_diagonal() {
        let system = RecoilSystem::new();
        let direction = IVec3::new(1, 1, 1);
        let recoil = system.push_recoil(2.0, direction);

        assert_eq!(recoil.x, 2);
        assert_eq!(recoil.y, 2);
        assert_eq!(recoil.z, 2);
    }

    #[test]
    fn test_recoil_system_default() {
        let _system = RecoilSystem::default();
    }
}
