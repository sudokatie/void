//! Relative physics system for entities on the Titan.
//!
//! Handles gravity, wind, and fall detection in the Titan's reference frame.

use glam::IVec3;
use serde::{Deserialize, Serialize};

/// Default gravity strength in blocks per second squared.
pub const DEFAULT_GRAVITY: i32 = 10;

/// Manages physics calculations relative to the Titan's surface.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RelativePhysics {
    /// Direction of gravity (always down relative to Titan surface).
    gravity_direction: IVec3,
    /// Current wind force from Titan movement.
    wind_force: f32,
}

impl RelativePhysics {
    /// Create a new relative physics system.
    #[must_use]
    pub fn new() -> Self {
        Self {
            gravity_direction: IVec3::new(0, -1, 0),
            wind_force: 0.0,
        }
    }

    /// Apply gravity to a velocity vector.
    ///
    /// Returns the new velocity after gravity is applied.
    #[must_use]
    pub fn apply_gravity(&self, velocity: IVec3, dt: f32) -> IVec3 {
        let gravity_delta = self.gravity_direction * (DEFAULT_GRAVITY as f32 * dt) as i32;
        velocity + gravity_delta
    }

    /// Apply wind force to a position.
    ///
    /// Returns the new position after wind displacement.
    #[must_use]
    pub fn apply_wind(&self, position: IVec3, dt: f32) -> IVec3 {
        if self.wind_force.abs() < f32::EPSILON {
            return position;
        }
        // Wind pushes horizontally in the X direction (Titan's movement direction)
        let wind_delta = (self.wind_force * dt * 10.0) as i32;
        IVec3::new(position.x + wind_delta, position.y, position.z)
    }

    /// Set the current wind force.
    pub fn set_wind_force(&mut self, force: f32) {
        self.wind_force = force;
    }

    /// Get the current wind force.
    #[must_use]
    pub fn wind_force(&self) -> f32 {
        self.wind_force
    }

    /// Check if an entity has fallen off the Titan.
    ///
    /// Returns true if the position is below the Titan's minimum height.
    #[must_use]
    pub fn check_fall(&self, position: IVec3, titan_height: i32) -> bool {
        position.y < -titan_height
    }

    /// Get the gravity direction.
    #[must_use]
    pub fn gravity_direction(&self) -> IVec3 {
        self.gravity_direction
    }
}

impl Default for RelativePhysics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative_physics_new() {
        let physics = RelativePhysics::new();
        assert_eq!(physics.gravity_direction(), IVec3::new(0, -1, 0));
        assert!((physics.wind_force() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_apply_gravity() {
        let physics = RelativePhysics::new();
        let velocity = IVec3::new(5, 0, 0);
        let new_velocity = physics.apply_gravity(velocity, 1.0);
        assert_eq!(new_velocity.x, 5);
        assert!(new_velocity.y < 0); // Gravity pulled down
    }

    #[test]
    fn test_apply_gravity_accumulates() {
        let physics = RelativePhysics::new();
        let velocity = IVec3::ZERO;
        let v1 = physics.apply_gravity(velocity, 1.0);
        let v2 = physics.apply_gravity(v1, 1.0);
        assert!(v2.y < v1.y); // More downward velocity
    }

    #[test]
    fn test_apply_wind_no_force() {
        let physics = RelativePhysics::new();
        let position = IVec3::new(100, 50, 25);
        let new_position = physics.apply_wind(position, 1.0);
        assert_eq!(new_position, position);
    }

    #[test]
    fn test_apply_wind_with_force() {
        let mut physics = RelativePhysics::new();
        physics.set_wind_force(0.5);
        let position = IVec3::new(100, 50, 25);
        let new_position = physics.apply_wind(position, 1.0);
        assert!(new_position.x > position.x); // Wind pushed in X direction
        assert_eq!(new_position.y, position.y);
        assert_eq!(new_position.z, position.z);
    }

    #[test]
    fn test_set_wind_force() {
        let mut physics = RelativePhysics::new();
        physics.set_wind_force(0.8);
        assert!((physics.wind_force() - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn test_check_fall_safe() {
        let physics = RelativePhysics::new();
        let position = IVec3::new(0, 10, 0);
        assert!(!physics.check_fall(position, 100));
    }

    #[test]
    fn test_check_fall_fallen() {
        let physics = RelativePhysics::new();
        let position = IVec3::new(0, -150, 0);
        assert!(physics.check_fall(position, 100));
    }

    #[test]
    fn test_check_fall_edge() {
        let physics = RelativePhysics::new();
        let position = IVec3::new(0, -100, 0);
        assert!(!physics.check_fall(position, 100)); // Exactly at edge is safe
        let fallen = IVec3::new(0, -101, 0);
        assert!(physics.check_fall(fallen, 100));
    }
}
