//! Time Altar crafting station for time-loop survival.
//!
//! Stabilizes paradoxes and creates temporal keys from fragments.

use serde::{Deserialize, Serialize};

/// Result of a stabilization attempt.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StabilizationResult {
    /// Whether the stabilization was successful.
    pub success: bool,
    /// Amount of paradox reduced.
    pub paradox_reduced: f32,
    /// Stability gained.
    pub stability_gained: f32,
    /// Message describing the result.
    pub message: String,
}

impl StabilizationResult {
    /// Create a successful result.
    #[must_use]
    pub fn success(paradox_reduced: f32, stability_gained: f32) -> Self {
        Self {
            success: true,
            paradox_reduced,
            stability_gained,
            message: format!(
                "Stabilized: -{:.1} paradox, +{:.1} stability",
                paradox_reduced, stability_gained
            ),
        }
    }

    /// Create a failed result.
    #[must_use]
    pub fn failure(message: &str) -> Self {
        Self {
            success: false,
            paradox_reduced: 0.0,
            stability_gained: 0.0,
            message: message.to_string(),
        }
    }
}

/// Temporal Key created from fragments.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TemporalKey {
    /// Unique identifier for this key.
    pub id: u32,
    /// Power level of the key.
    pub power: u32,
    /// What this key unlocks.
    pub unlocks: String,
}

impl TemporalKey {
    /// Create a new temporal key.
    #[must_use]
    pub fn new(id: u32, power: u32, unlocks: &str) -> Self {
        Self {
            id,
            power,
            unlocks: unlocks.to_string(),
        }
    }
}

/// Time Altar crafting station.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimeAltar {
    /// Whether the altar is active.
    active: bool,
    /// Current temporal fragments stored.
    fragments: u32,
    /// Fragments required for a key.
    fragments_for_key: u32,
    /// Stabilization power (affects paradox reduction).
    stabilization_power: f32,
    /// Total paradox stabilized.
    total_paradox_stabilized: f32,
    /// Keys created.
    keys_created: u32,
    /// Cooldown ticks remaining.
    cooldown: u32,
}

impl TimeAltar {
    /// Fragments required to create a temporal key.
    pub const FRAGMENTS_FOR_KEY: u32 = 3;

    /// Create a new Time Altar.
    #[must_use]
    pub fn new() -> Self {
        Self {
            active: true,
            fragments: 0,
            fragments_for_key: Self::FRAGMENTS_FOR_KEY,
            stabilization_power: 1.0,
            total_paradox_stabilized: 0.0,
            keys_created: 0,
            cooldown: 0,
        }
    }

    /// Check if the altar is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Set active state.
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    /// Get current fragment count.
    #[must_use]
    pub fn fragments(&self) -> u32 {
        self.fragments
    }

    /// Get fragments required for a key.
    #[must_use]
    pub fn fragments_for_key(&self) -> u32 {
        self.fragments_for_key
    }

    /// Get stabilization power.
    #[must_use]
    pub fn stabilization_power(&self) -> f32 {
        self.stabilization_power
    }

    /// Get total paradox stabilized.
    #[must_use]
    pub fn total_paradox_stabilized(&self) -> f32 {
        self.total_paradox_stabilized
    }

    /// Get number of keys created.
    #[must_use]
    pub fn keys_created(&self) -> u32 {
        self.keys_created
    }

    /// Get remaining cooldown.
    #[must_use]
    pub fn cooldown(&self) -> u32 {
        self.cooldown
    }

    /// Check if on cooldown.
    #[must_use]
    pub fn is_on_cooldown(&self) -> bool {
        self.cooldown > 0
    }

    /// Add fragments to the altar.
    pub fn add_fragments(&mut self, count: u32) {
        self.fragments += count;
    }

    /// Remove fragments from the altar.
    pub fn remove_fragments(&mut self, count: u32) -> u32 {
        let removed = count.min(self.fragments);
        self.fragments -= removed;
        removed
    }

    /// Check if enough fragments for a key.
    #[must_use]
    pub fn can_create_key(&self) -> bool {
        self.active && self.fragments >= self.fragments_for_key && !self.is_on_cooldown()
    }

    /// Create a temporal key from fragments.
    pub fn create_key(&mut self, unlock_target: &str) -> Option<TemporalKey> {
        if !self.can_create_key() {
            return None;
        }

        self.fragments -= self.fragments_for_key;
        self.keys_created += 1;
        self.cooldown = 100; // 100 tick cooldown

        Some(TemporalKey::new(
            self.keys_created,
            self.stabilization_power as u32 + 1,
            unlock_target,
        ))
    }

    /// Stabilize paradox at the altar.
    pub fn stabilize_paradox(&mut self, current_paradox: f32) -> StabilizationResult {
        if !self.active {
            return StabilizationResult::failure("Altar is not active");
        }

        if self.is_on_cooldown() {
            return StabilizationResult::failure("Altar is on cooldown");
        }

        if current_paradox < 5.0 {
            return StabilizationResult::failure("Paradox level too low to stabilize");
        }

        // Calculate reduction based on stabilization power
        let reduction = (current_paradox * 0.2 * self.stabilization_power).min(current_paradox);
        let stability_gain = reduction * 0.5;

        self.total_paradox_stabilized += reduction;
        self.cooldown = 60; // 60 tick cooldown

        StabilizationResult::success(reduction, stability_gain)
    }

    /// Upgrade stabilization power.
    pub fn upgrade_power(&mut self, amount: f32) {
        self.stabilization_power += amount;
    }

    /// Update the altar (call each tick).
    pub fn update(&mut self) {
        if self.cooldown > 0 {
            self.cooldown -= 1;
        }
    }

    /// Reset for a new loop.
    pub fn reset_for_loop(&mut self) {
        self.cooldown = 0;
        self.active = true;
        // Keep fragments, keys_created, and upgrades
    }

    /// Get progress toward next key (0.0 - 1.0).
    #[must_use]
    pub fn key_progress(&self) -> f32 {
        if self.fragments_for_key == 0 {
            return 1.0;
        }
        (self.fragments as f32 / self.fragments_for_key as f32).min(1.0)
    }
}

impl Default for TimeAltar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stabilization_result_success() {
        let result = StabilizationResult::success(10.0, 5.0);
        assert!(result.success);
        assert!((result.paradox_reduced - 10.0).abs() < f32::EPSILON);
        assert!((result.stability_gained - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_stabilization_result_failure() {
        let result = StabilizationResult::failure("Test failure");
        assert!(!result.success);
        assert_eq!(result.message, "Test failure");
    }

    #[test]
    fn test_temporal_key_new() {
        let key = TemporalKey::new(1, 5, "secret_door");
        assert_eq!(key.id, 1);
        assert_eq!(key.power, 5);
        assert_eq!(key.unlocks, "secret_door");
    }

    #[test]
    fn test_time_altar_new() {
        let altar = TimeAltar::new();

        assert!(altar.is_active());
        assert_eq!(altar.fragments(), 0);
        assert_eq!(altar.fragments_for_key(), 3);
        assert!((altar.stabilization_power() - 1.0).abs() < f32::EPSILON);
        assert_eq!(altar.keys_created(), 0);
        assert!(!altar.is_on_cooldown());
    }

    #[test]
    fn test_time_altar_add_fragments() {
        let mut altar = TimeAltar::new();

        altar.add_fragments(5);
        assert_eq!(altar.fragments(), 5);

        altar.add_fragments(3);
        assert_eq!(altar.fragments(), 8);
    }

    #[test]
    fn test_time_altar_remove_fragments() {
        let mut altar = TimeAltar::new();
        altar.add_fragments(5);

        let removed = altar.remove_fragments(3);
        assert_eq!(removed, 3);
        assert_eq!(altar.fragments(), 2);

        let removed = altar.remove_fragments(10);
        assert_eq!(removed, 2);
        assert_eq!(altar.fragments(), 0);
    }

    #[test]
    fn test_time_altar_can_create_key() {
        let mut altar = TimeAltar::new();
        assert!(!altar.can_create_key());

        altar.add_fragments(3);
        assert!(altar.can_create_key());
    }

    #[test]
    fn test_time_altar_can_create_key_inactive() {
        let mut altar = TimeAltar::new();
        altar.add_fragments(3);
        altar.set_active(false);

        assert!(!altar.can_create_key());
    }

    #[test]
    fn test_time_altar_can_create_key_on_cooldown() {
        let mut altar = TimeAltar::new();
        altar.add_fragments(6);
        altar.create_key("test");

        assert!(!altar.can_create_key()); // On cooldown
    }

    #[test]
    fn test_time_altar_create_key() {
        let mut altar = TimeAltar::new();
        altar.add_fragments(5);

        let key = altar.create_key("treasure_room");
        assert!(key.is_some());

        let key = key.unwrap();
        assert_eq!(key.id, 1);
        assert_eq!(key.unlocks, "treasure_room");

        assert_eq!(altar.fragments(), 2);
        assert_eq!(altar.keys_created(), 1);
        assert!(altar.is_on_cooldown());
    }

    #[test]
    fn test_time_altar_create_key_insufficient_fragments() {
        let mut altar = TimeAltar::new();
        altar.add_fragments(2);

        let key = altar.create_key("test");
        assert!(key.is_none());
    }

    #[test]
    fn test_time_altar_stabilize_paradox() {
        let mut altar = TimeAltar::new();

        let result = altar.stabilize_paradox(50.0);
        assert!(result.success);
        assert!(result.paradox_reduced > 0.0);
        assert!(result.stability_gained > 0.0);
        assert!(altar.total_paradox_stabilized() > 0.0);
        assert!(altar.is_on_cooldown());
    }

    #[test]
    fn test_time_altar_stabilize_inactive() {
        let mut altar = TimeAltar::new();
        altar.set_active(false);

        let result = altar.stabilize_paradox(50.0);
        assert!(!result.success);
    }

    #[test]
    fn test_time_altar_stabilize_on_cooldown() {
        let mut altar = TimeAltar::new();
        altar.stabilize_paradox(50.0);

        let result = altar.stabilize_paradox(50.0);
        assert!(!result.success);
        assert!(result.message.contains("cooldown"));
    }

    #[test]
    fn test_time_altar_stabilize_low_paradox() {
        let mut altar = TimeAltar::new();

        let result = altar.stabilize_paradox(2.0);
        assert!(!result.success);
        assert!(result.message.contains("too low"));
    }

    #[test]
    fn test_time_altar_upgrade_power() {
        let mut altar = TimeAltar::new();
        altar.upgrade_power(0.5);

        assert!((altar.stabilization_power() - 1.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_time_altar_update() {
        let mut altar = TimeAltar::new();
        altar.stabilize_paradox(50.0);
        let initial_cooldown = altar.cooldown();

        altar.update();
        assert_eq!(altar.cooldown(), initial_cooldown - 1);
    }

    #[test]
    fn test_time_altar_reset_for_loop() {
        let mut altar = TimeAltar::new();
        altar.add_fragments(5);
        altar.create_key("test");
        altar.upgrade_power(0.5);
        altar.set_active(false);

        altar.reset_for_loop();

        assert!(altar.is_active());
        assert!(!altar.is_on_cooldown());
        assert_eq!(altar.fragments(), 2); // Kept
        assert_eq!(altar.keys_created(), 1); // Kept
        assert!((altar.stabilization_power() - 1.5).abs() < f32::EPSILON); // Kept
    }

    #[test]
    fn test_time_altar_key_progress() {
        let mut altar = TimeAltar::new();
        assert!((altar.key_progress() - 0.0).abs() < f32::EPSILON);

        altar.add_fragments(1);
        assert!((altar.key_progress() - 0.333).abs() < 0.01);

        altar.add_fragments(2);
        assert!((altar.key_progress() - 1.0).abs() < f32::EPSILON);

        altar.add_fragments(5);
        assert!((altar.key_progress() - 1.0).abs() < f32::EPSILON); // Capped
    }
}
