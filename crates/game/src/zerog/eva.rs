//! EVA (Extra-Vehicular Activity) operations.
//!
//! Handles spacewalk mechanics including suit oxygen and tethers.

use serde::{Deserialize, Serialize};

/// Maximum suit oxygen in minutes.
const MAX_SUIT_O2: f32 = 30.0;

/// Maximum tether length in meters.
const MAX_TETHER_LENGTH: f32 = 50.0;

/// O2 consumption rate per tick.
const O2_CONSUMPTION_RATE: f32 = 0.1;

/// EVA suit state.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EVAState {
    /// Suit oxygen remaining in minutes.
    suit_o2: f32,
    /// Tether length in meters.
    tether_length: f32,
    /// Whether the tether is deployed.
    tethered: bool,
    /// Whether currently outside the station.
    outside: bool,
}

impl Default for EVAState {
    fn default() -> Self {
        Self::new()
    }
}

impl EVAState {
    /// Create a new EVA state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            suit_o2: MAX_SUIT_O2,
            tether_length: MAX_TETHER_LENGTH,
            tethered: false,
            outside: false,
        }
    }

    /// Get the remaining suit oxygen.
    #[must_use]
    pub fn suit_o2(&self) -> f32 {
        self.suit_o2
    }

    /// Get the suit oxygen as a percentage.
    #[must_use]
    pub fn suit_o2_percentage(&self) -> f32 {
        (self.suit_o2 / MAX_SUIT_O2) * 100.0
    }

    /// Get the tether length.
    #[must_use]
    pub fn tether_length(&self) -> f32 {
        self.tether_length
    }

    /// Check if tethered.
    #[must_use]
    pub fn is_tethered(&self) -> bool {
        self.tethered
    }

    /// Check if outside the station.
    #[must_use]
    pub fn is_outside(&self) -> bool {
        self.outside
    }

    /// Enter vacuum (go outside).
    ///
    /// Returns true if successful, false if already outside.
    pub fn enter_vacuum(&mut self) -> bool {
        if self.outside {
            return false;
        }
        self.outside = true;
        true
    }

    /// Return inside the station.
    ///
    /// Returns true if successful, false if already inside.
    pub fn return_inside(&mut self) -> bool {
        if !self.outside {
            return false;
        }
        self.outside = false;
        self.tethered = false;
        true
    }

    /// Use oxygen while in EVA.
    ///
    /// Returns the remaining oxygen.
    pub fn use_o2(&mut self, dt: f32) -> f32 {
        if self.outside {
            self.suit_o2 = (self.suit_o2 - O2_CONSUMPTION_RATE * dt).max(0.0);
        }
        self.suit_o2
    }

    /// Deploy the tether.
    ///
    /// Returns true if successful, false if not outside.
    pub fn deploy_tether(&mut self) -> bool {
        if !self.outside {
            return false;
        }
        self.tethered = true;
        true
    }

    /// Retract the tether.
    pub fn retract_tether(&mut self) {
        self.tethered = false;
    }

    /// Extend the tether by a certain amount.
    pub fn extend_tether(&mut self, amount: f32) {
        self.tether_length = (self.tether_length + amount).min(MAX_TETHER_LENGTH);
    }

    /// Shorten the tether.
    pub fn shorten_tether(&mut self, amount: f32) {
        self.tether_length = (self.tether_length - amount).max(0.0);
    }

    /// Refill suit oxygen.
    pub fn refill_o2(&mut self, amount: f32) {
        self.suit_o2 = (self.suit_o2 + amount).min(MAX_SUIT_O2);
    }

    /// Check if oxygen is critically low (below 20%).
    #[must_use]
    pub fn is_o2_critical(&self) -> bool {
        self.suit_o2_percentage() < 20.0
    }

    /// Check if oxygen is depleted.
    #[must_use]
    pub fn is_o2_depleted(&self) -> bool {
        self.suit_o2 <= 0.0
    }

    /// Get maximum tether length.
    #[must_use]
    pub fn max_tether_length(&self) -> f32 {
        MAX_TETHER_LENGTH
    }

    /// Get maximum suit oxygen.
    #[must_use]
    pub fn max_suit_o2(&self) -> f32 {
        MAX_SUIT_O2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eva_state_new() {
        let state = EVAState::new();
        assert!((state.suit_o2() - 30.0).abs() < f32::EPSILON);
        assert!((state.tether_length() - 50.0).abs() < f32::EPSILON);
        assert!(!state.is_tethered());
        assert!(!state.is_outside());
    }

    #[test]
    fn test_eva_state_suit_o2_percentage() {
        let state = EVAState::new();
        assert!((state.suit_o2_percentage() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_eva_enter_vacuum() {
        let mut state = EVAState::new();
        assert!(state.enter_vacuum());
        assert!(state.is_outside());

        // Can't enter vacuum if already outside
        assert!(!state.enter_vacuum());
    }

    #[test]
    fn test_eva_return_inside() {
        let mut state = EVAState::new();
        state.enter_vacuum();
        state.deploy_tether();

        assert!(state.return_inside());
        assert!(!state.is_outside());
        assert!(!state.is_tethered());

        // Can't return if already inside
        assert!(!state.return_inside());
    }

    #[test]
    fn test_eva_use_o2() {
        let mut state = EVAState::new();
        state.enter_vacuum();

        let remaining = state.use_o2(1.0);
        assert!(remaining < 30.0);
    }

    #[test]
    fn test_eva_use_o2_inside() {
        let mut state = EVAState::new();
        let remaining = state.use_o2(1.0);
        // Should not consume O2 when inside
        assert!((remaining - 30.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_eva_use_o2_depletes() {
        let mut state = EVAState::new();
        state.suit_o2 = 0.05;
        state.enter_vacuum();

        state.use_o2(1.0);
        assert!(state.is_o2_depleted());
    }

    #[test]
    fn test_eva_deploy_tether() {
        let mut state = EVAState::new();

        // Can't deploy if inside
        assert!(!state.deploy_tether());

        state.enter_vacuum();
        assert!(state.deploy_tether());
        assert!(state.is_tethered());
    }

    #[test]
    fn test_eva_retract_tether() {
        let mut state = EVAState::new();
        state.enter_vacuum();
        state.deploy_tether();
        assert!(state.is_tethered());

        state.retract_tether();
        assert!(!state.is_tethered());
    }

    #[test]
    fn test_eva_extend_tether() {
        let mut state = EVAState::new();
        state.tether_length = 30.0;
        state.extend_tether(10.0);
        assert!((state.tether_length() - 40.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_eva_extend_tether_max() {
        let mut state = EVAState::new();
        state.extend_tether(20.0);
        assert!((state.tether_length() - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_eva_shorten_tether() {
        let mut state = EVAState::new();
        state.shorten_tether(20.0);
        assert!((state.tether_length() - 30.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_eva_shorten_tether_min() {
        let mut state = EVAState::new();
        state.shorten_tether(100.0);
        assert!((state.tether_length() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_eva_refill_o2() {
        let mut state = EVAState::new();
        state.suit_o2 = 10.0;
        state.refill_o2(15.0);
        assert!((state.suit_o2() - 25.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_eva_refill_o2_max() {
        let mut state = EVAState::new();
        state.refill_o2(10.0);
        assert!((state.suit_o2() - 30.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_eva_is_o2_critical() {
        let mut state = EVAState::new();
        assert!(!state.is_o2_critical());

        state.suit_o2 = 5.0;
        assert!(state.is_o2_critical());
    }

    #[test]
    fn test_eva_max_values() {
        let state = EVAState::new();
        assert!((state.max_tether_length() - 50.0).abs() < f32::EPSILON);
        assert!((state.max_suit_o2() - 30.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_eva_default() {
        let state = EVAState::default();
        assert!(!state.is_outside());
    }
}
