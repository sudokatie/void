//! World origin shifting system.
//!
//! Manages coordinate transformation as the Titan moves through the world,
//! preventing floating-point precision issues at large distances.

use engine_physics::titan::TitanPhase;
use glam::IVec3;
use serde::{Deserialize, Serialize};

/// Maximum number of shifts to keep in history.
const MAX_SHIFT_HISTORY: usize = 100;

/// Manages world origin shifting to maintain coordinate precision.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldOriginManager {
    /// Current accumulated offset from true origin.
    current_offset: IVec3,
    /// History of recent shifts with their associated phase.
    shift_history: Vec<(IVec3, TitanPhase)>,
}

impl WorldOriginManager {
    /// Create a new world origin manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            current_offset: IVec3::ZERO,
            shift_history: Vec::new(),
        }
    }

    /// Apply a shift to the world origin.
    ///
    /// Updates the offset and records the shift in history.
    pub fn apply_shift(&mut self, shift: IVec3, phase: TitanPhase) {
        self.current_offset += shift;
        self.shift_history.push((shift, phase));

        // Trim history if needed
        if self.shift_history.len() > MAX_SHIFT_HISTORY {
            self.shift_history.remove(0);
        }
    }

    /// Get the current world offset.
    #[must_use]
    pub fn world_offset(&self) -> IVec3 {
        self.current_offset
    }

    /// Convert an absolute position to a relative position.
    ///
    /// Relative positions are offset from the current world origin.
    #[must_use]
    pub fn relative_position(&self, absolute_pos: IVec3) -> IVec3 {
        absolute_pos - self.current_offset
    }

    /// Convert a relative position to an absolute position.
    ///
    /// Absolute positions are in true world coordinates.
    #[must_use]
    pub fn absolute_position(&self, relative_pos: IVec3) -> IVec3 {
        relative_pos + self.current_offset
    }

    /// Get the total number of shifts recorded.
    #[must_use]
    pub fn shift_count(&self) -> usize {
        self.shift_history.len()
    }

    /// Get the most recent shifts.
    #[must_use]
    pub fn recent_shifts(&self, count: usize) -> &[(IVec3, TitanPhase)] {
        let len = self.shift_history.len();
        let start = len.saturating_sub(count);
        &self.shift_history[start..]
    }

    /// Reset the origin manager to initial state.
    pub fn reset(&mut self) {
        self.current_offset = IVec3::ZERO;
        self.shift_history.clear();
    }

    /// Get total distance shifted.
    #[must_use]
    pub fn total_distance(&self) -> i32 {
        self.current_offset.x.abs() + self.current_offset.y.abs() + self.current_offset.z.abs()
    }
}

impl Default for WorldOriginManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_origin_new() {
        let manager = WorldOriginManager::new();
        assert_eq!(manager.world_offset(), IVec3::ZERO);
        assert_eq!(manager.shift_count(), 0);
    }

    #[test]
    fn test_world_origin_apply_shift() {
        let mut manager = WorldOriginManager::new();
        manager.apply_shift(IVec3::new(10, 0, 0), TitanPhase::Walking);
        assert_eq!(manager.world_offset(), IVec3::new(10, 0, 0));
        assert_eq!(manager.shift_count(), 1);
    }

    #[test]
    fn test_world_origin_multiple_shifts() {
        let mut manager = WorldOriginManager::new();
        manager.apply_shift(IVec3::new(10, 0, 0), TitanPhase::Walking);
        manager.apply_shift(IVec3::new(5, 0, 0), TitanPhase::Running);
        manager.apply_shift(IVec3::new(-3, 0, 0), TitanPhase::Scratching);
        assert_eq!(manager.world_offset(), IVec3::new(12, 0, 0));
        assert_eq!(manager.shift_count(), 3);
    }

    #[test]
    fn test_relative_position() {
        let mut manager = WorldOriginManager::new();
        manager.apply_shift(IVec3::new(100, 0, 0), TitanPhase::Walking);

        let absolute = IVec3::new(150, 10, 20);
        let relative = manager.relative_position(absolute);
        assert_eq!(relative, IVec3::new(50, 10, 20));
    }

    #[test]
    fn test_absolute_position() {
        let mut manager = WorldOriginManager::new();
        manager.apply_shift(IVec3::new(100, 0, 0), TitanPhase::Walking);

        let relative = IVec3::new(50, 10, 20);
        let absolute = manager.absolute_position(relative);
        assert_eq!(absolute, IVec3::new(150, 10, 20));
    }

    #[test]
    fn test_position_roundtrip() {
        let mut manager = WorldOriginManager::new();
        manager.apply_shift(IVec3::new(100, 50, -25), TitanPhase::Running);

        let original = IVec3::new(200, 100, 50);
        let relative = manager.relative_position(original);
        let back = manager.absolute_position(relative);
        assert_eq!(original, back);
    }

    #[test]
    fn test_recent_shifts() {
        let mut manager = WorldOriginManager::new();
        manager.apply_shift(IVec3::new(1, 0, 0), TitanPhase::Walking);
        manager.apply_shift(IVec3::new(2, 0, 0), TitanPhase::Running);
        manager.apply_shift(IVec3::new(3, 0, 0), TitanPhase::Scratching);

        let recent = manager.recent_shifts(2);
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0].0, IVec3::new(2, 0, 0));
        assert_eq!(recent[1].0, IVec3::new(3, 0, 0));
    }

    #[test]
    fn test_recent_shifts_all() {
        let mut manager = WorldOriginManager::new();
        manager.apply_shift(IVec3::new(1, 0, 0), TitanPhase::Walking);
        manager.apply_shift(IVec3::new(2, 0, 0), TitanPhase::Running);

        let recent = manager.recent_shifts(10);
        assert_eq!(recent.len(), 2);
    }

    #[test]
    fn test_total_distance() {
        let mut manager = WorldOriginManager::new();
        manager.apply_shift(IVec3::new(10, 5, -3), TitanPhase::Walking);
        assert_eq!(manager.total_distance(), 18);
    }

    #[test]
    fn test_reset() {
        let mut manager = WorldOriginManager::new();
        manager.apply_shift(IVec3::new(100, 0, 0), TitanPhase::Walking);
        manager.reset();
        assert_eq!(manager.world_offset(), IVec3::ZERO);
        assert_eq!(manager.shift_count(), 0);
    }
}
