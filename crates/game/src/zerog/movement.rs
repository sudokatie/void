//! Zero-gravity movement mechanics.
//!
//! Handles thruster-based movement and magnetic boots in zero-g.

use glam::IVec3;
use serde::{Deserialize, Serialize};

/// Maximum thruster fuel.
const MAX_FUEL: f32 = 100.0;

/// Fuel cost per thrust unit.
const FUEL_COST_MULTIPLIER: f32 = 1.0;

/// Velocity dampening when grabbing a surface.
const GRAB_DAMPENING: f32 = 0.8;

/// Zero-gravity movement state.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZeroGMovement {
    /// Current position.
    position: IVec3,
    /// Current velocity.
    velocity: IVec3,
    /// Thruster fuel (0-100).
    thruster_fuel: f32,
    /// Whether magnetic boots are active.
    magnetic_boots: bool,
    /// Whether attached to a surface.
    attached: bool,
}

impl Default for ZeroGMovement {
    fn default() -> Self {
        Self::new()
    }
}

impl ZeroGMovement {
    /// Create a new zero-g movement state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            position: IVec3::ZERO,
            velocity: IVec3::ZERO,
            thruster_fuel: MAX_FUEL,
            magnetic_boots: false,
            attached: false,
        }
    }

    /// Create with a specific position.
    #[must_use]
    pub fn at_position(position: IVec3) -> Self {
        Self {
            position,
            ..Self::new()
        }
    }

    /// Get the current position.
    #[must_use]
    pub fn position(&self) -> IVec3 {
        self.position
    }

    /// Get the current velocity.
    #[must_use]
    pub fn velocity(&self) -> IVec3 {
        self.velocity
    }

    /// Get the thruster fuel level.
    #[must_use]
    pub fn thruster_fuel(&self) -> f32 {
        self.thruster_fuel
    }

    /// Check if magnetic boots are active.
    #[must_use]
    pub fn magnetic_boots(&self) -> bool {
        self.magnetic_boots
    }

    /// Check if attached to a surface.
    #[must_use]
    pub fn is_attached(&self) -> bool {
        self.attached
    }

    /// Apply thrust in a direction.
    ///
    /// Returns true if thrust was applied, false if insufficient fuel.
    pub fn thrust(&mut self, direction: IVec3, fuel_cost: f32) -> bool {
        let actual_cost = fuel_cost * FUEL_COST_MULTIPLIER;
        if self.thruster_fuel < actual_cost {
            return false;
        }

        self.thruster_fuel -= actual_cost;
        self.velocity += direction;
        self.attached = false;
        true
    }

    /// Grab a surface to stop movement.
    pub fn grab_surface(&mut self) {
        // Dampen velocity
        self.velocity = IVec3::new(
            (self.velocity.x as f32 * (1.0 - GRAB_DAMPENING)) as i32,
            (self.velocity.y as f32 * (1.0 - GRAB_DAMPENING)) as i32,
            (self.velocity.z as f32 * (1.0 - GRAB_DAMPENING)) as i32,
        );
        self.attached = true;
    }

    /// Toggle magnetic boots.
    pub fn toggle_boots(&mut self) {
        self.magnetic_boots = !self.magnetic_boots;
    }

    /// Enable magnetic boots.
    pub fn enable_boots(&mut self) {
        self.magnetic_boots = true;
    }

    /// Disable magnetic boots.
    pub fn disable_boots(&mut self) {
        self.magnetic_boots = false;
    }

    /// Tick the movement state.
    pub fn tick(&mut self, dt: f32) {
        if self.attached && self.magnetic_boots {
            // Attached with boots - no movement
            self.velocity = IVec3::ZERO;
            return;
        }

        // Apply velocity to position
        let dt_int = dt as i32;
        self.position += self.velocity * dt_int.max(1);

        // Very slight velocity decay in space (for gameplay)
        if !self.attached {
            self.velocity = IVec3::new(
                (self.velocity.x as f32 * 0.99) as i32,
                (self.velocity.y as f32 * 0.99) as i32,
                (self.velocity.z as f32 * 0.99) as i32,
            );
        }
    }

    /// Set position directly.
    pub fn set_position(&mut self, position: IVec3) {
        self.position = position;
    }

    /// Set velocity directly.
    pub fn set_velocity(&mut self, velocity: IVec3) {
        self.velocity = velocity;
    }

    /// Refuel the thrusters.
    pub fn refuel(&mut self, amount: f32) {
        self.thruster_fuel = (self.thruster_fuel + amount).min(MAX_FUEL);
    }

    /// Get the speed (magnitude of velocity).
    #[must_use]
    pub fn speed(&self) -> f32 {
        let v = self.velocity.as_vec3();
        v.length()
    }

    /// Apply an external force (e.g., from an explosion).
    pub fn apply_force(&mut self, force: IVec3) {
        self.velocity += force;
        self.attached = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_g_movement_new() {
        let movement = ZeroGMovement::new();
        assert_eq!(movement.position(), IVec3::ZERO);
        assert_eq!(movement.velocity(), IVec3::ZERO);
        assert!((movement.thruster_fuel() - 100.0).abs() < f32::EPSILON);
        assert!(!movement.magnetic_boots());
        assert!(!movement.is_attached());
    }

    #[test]
    fn test_zero_g_movement_at_position() {
        let pos = IVec3::new(10, 20, 30);
        let movement = ZeroGMovement::at_position(pos);
        assert_eq!(movement.position(), pos);
    }

    #[test]
    fn test_zero_g_thrust() {
        let mut movement = ZeroGMovement::new();
        let result = movement.thrust(IVec3::new(1, 0, 0), 10.0);

        assert!(result);
        assert_eq!(movement.velocity(), IVec3::new(1, 0, 0));
        assert!((movement.thruster_fuel() - 90.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_zero_g_thrust_no_fuel() {
        let mut movement = ZeroGMovement::new();
        movement.thruster_fuel = 5.0;

        let result = movement.thrust(IVec3::new(1, 0, 0), 10.0);

        assert!(!result);
        assert_eq!(movement.velocity(), IVec3::ZERO);
    }

    #[test]
    fn test_zero_g_thrust_detaches() {
        let mut movement = ZeroGMovement::new();
        movement.grab_surface();
        assert!(movement.is_attached());

        movement.thrust(IVec3::new(1, 0, 0), 5.0);
        assert!(!movement.is_attached());
    }

    #[test]
    fn test_zero_g_grab_surface() {
        let mut movement = ZeroGMovement::new();
        movement.velocity = IVec3::new(10, 10, 10);
        movement.grab_surface();

        assert!(movement.is_attached());
        // Velocity should be dampened
        assert!(movement.velocity().x < 10);
    }

    #[test]
    fn test_zero_g_toggle_boots() {
        let mut movement = ZeroGMovement::new();
        assert!(!movement.magnetic_boots());

        movement.toggle_boots();
        assert!(movement.magnetic_boots());

        movement.toggle_boots();
        assert!(!movement.magnetic_boots());
    }

    #[test]
    fn test_zero_g_enable_disable_boots() {
        let mut movement = ZeroGMovement::new();
        movement.enable_boots();
        assert!(movement.magnetic_boots());

        movement.disable_boots();
        assert!(!movement.magnetic_boots());
    }

    #[test]
    fn test_zero_g_tick_movement() {
        let mut movement = ZeroGMovement::new();
        movement.velocity = IVec3::new(5, 0, 0);

        movement.tick(1.0);

        assert!(movement.position().x > 0);
    }

    #[test]
    fn test_zero_g_tick_attached_with_boots() {
        let mut movement = ZeroGMovement::new();
        movement.velocity = IVec3::new(10, 0, 0);
        movement.grab_surface();
        movement.enable_boots();

        movement.tick(1.0);

        assert_eq!(movement.velocity(), IVec3::ZERO);
    }

    #[test]
    fn test_zero_g_set_position() {
        let mut movement = ZeroGMovement::new();
        movement.set_position(IVec3::new(100, 200, 300));
        assert_eq!(movement.position(), IVec3::new(100, 200, 300));
    }

    #[test]
    fn test_zero_g_set_velocity() {
        let mut movement = ZeroGMovement::new();
        movement.set_velocity(IVec3::new(5, 5, 5));
        assert_eq!(movement.velocity(), IVec3::new(5, 5, 5));
    }

    #[test]
    fn test_zero_g_refuel() {
        let mut movement = ZeroGMovement::new();
        movement.thruster_fuel = 50.0;
        movement.refuel(30.0);
        assert!((movement.thruster_fuel() - 80.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_zero_g_refuel_max() {
        let mut movement = ZeroGMovement::new();
        movement.refuel(50.0);
        assert!((movement.thruster_fuel() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_zero_g_speed() {
        let mut movement = ZeroGMovement::new();
        movement.velocity = IVec3::new(3, 4, 0);
        assert!((movement.speed() - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_zero_g_apply_force() {
        let mut movement = ZeroGMovement::new();
        movement.grab_surface();
        movement.apply_force(IVec3::new(10, 0, 0));

        assert_eq!(movement.velocity(), IVec3::new(10, 0, 0));
        assert!(!movement.is_attached());
    }

    #[test]
    fn test_zero_g_default() {
        let movement = ZeroGMovement::default();
        assert_eq!(movement.position(), IVec3::ZERO);
    }
}
