//! Player balance meter system.
//!
//! Tracks player stability based on Titan movement phases.

use engine_physics::titan::TitanPhase;

/// Tracks player balance on the Titan's surface.
#[derive(Clone, Debug)]
pub struct BalanceMeter {
    /// Current balance value (0-100).
    value: f32,
    /// Equipment bonus that reduces negative effects.
    equipment_bonus: f32,
}

impl BalanceMeter {
    /// Create a new balance meter at full stability.
    #[must_use]
    pub fn new() -> Self {
        Self {
            value: 100.0,
            equipment_bonus: 0.0,
        }
    }

    /// Create a balance meter with equipment bonus.
    #[must_use]
    pub fn with_equipment(bonus: f32) -> Self {
        Self {
            value: 100.0,
            equipment_bonus: bonus,
        }
    }

    /// Update balance based on Titan's current phase.
    ///
    /// Returns the current balance value after the tick.
    pub fn tick(&mut self, _dt: f32, phase: TitanPhase) -> f32 {
        let change = match phase {
            TitanPhase::Resting => 5.0,
            TitanPhase::Walking => -2.0,
            TitanPhase::Running => -10.0,
            TitanPhase::Scratching => -5.0,
        };

        // Apply equipment bonus to reduce negative effects
        let adjusted_change = if change < 0.0 {
            change * (1.0 - self.equipment_bonus.min(0.9))
        } else {
            change
        };

        self.value = (self.value + adjusted_change).clamp(0.0, 100.0);
        self.value
    }

    /// Directly modify balance by a given amount.
    pub fn modify(&mut self, amount: f32) {
        self.value = (self.value + amount).clamp(0.0, 100.0);
    }

    /// Get the current balance value.
    #[must_use]
    pub fn balance(&self) -> f32 {
        self.value
    }

    /// Check if the player is falling (balance depleted).
    #[must_use]
    pub fn is_falling(&self) -> bool {
        self.value <= 0.0
    }

    /// Reset balance to maximum.
    pub fn reset(&mut self) {
        self.value = 100.0;
    }
}

impl Default for BalanceMeter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balance_meter_new() {
        let meter = BalanceMeter::new();
        assert!((meter.balance() - 100.0).abs() < f32::EPSILON);
        assert!(!meter.is_falling());
    }

    #[test]
    fn test_balance_meter_with_equipment() {
        let meter = BalanceMeter::with_equipment(0.5);
        assert!((meter.balance() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_tick_resting_increases() {
        let mut meter = BalanceMeter::new();
        meter.modify(-20.0);
        let balance = meter.tick(1.0, TitanPhase::Resting);
        assert!((balance - 85.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_tick_walking_decreases() {
        let mut meter = BalanceMeter::new();
        let balance = meter.tick(1.0, TitanPhase::Walking);
        assert!((balance - 98.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_tick_running_decreases_fast() {
        let mut meter = BalanceMeter::new();
        let balance = meter.tick(1.0, TitanPhase::Running);
        assert!((balance - 90.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_tick_scratching_decreases() {
        let mut meter = BalanceMeter::new();
        let balance = meter.tick(1.0, TitanPhase::Scratching);
        assert!((balance - 95.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_equipment_reduces_negative() {
        let mut meter = BalanceMeter::with_equipment(0.5);
        let balance = meter.tick(1.0, TitanPhase::Running);
        // -10 * 0.5 = -5
        assert!((balance - 95.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_modify_positive() {
        let mut meter = BalanceMeter::new();
        meter.modify(-50.0);
        meter.modify(20.0);
        assert!((meter.balance() - 70.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_modify_clamps_at_zero() {
        let mut meter = BalanceMeter::new();
        meter.modify(-150.0);
        assert!((meter.balance() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_modify_clamps_at_max() {
        let mut meter = BalanceMeter::new();
        meter.modify(50.0);
        assert!((meter.balance() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_is_falling_when_zero() {
        let mut meter = BalanceMeter::new();
        meter.modify(-100.0);
        assert!(meter.is_falling());
    }

    #[test]
    fn test_balance_not_falling_when_positive() {
        let mut meter = BalanceMeter::new();
        meter.modify(-99.0);
        assert!(!meter.is_falling());
    }

    #[test]
    fn test_balance_reset() {
        let mut meter = BalanceMeter::new();
        meter.modify(-80.0);
        meter.reset();
        assert!((meter.balance() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_default() {
        let meter = BalanceMeter::default();
        assert!((meter.balance() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_equipment_bonus_capped() {
        let mut meter = BalanceMeter::with_equipment(1.0);
        // Bonus capped at 0.9, so -10 * 0.1 = -1
        let balance = meter.tick(1.0, TitanPhase::Running);
        assert!((balance - 99.0).abs() < f32::EPSILON);
    }
}
