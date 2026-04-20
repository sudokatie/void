//! Bulkhead doors between rooms.
//!
//! Controls access and atmosphere containment between rooms.

use serde::{Deserialize, Serialize};
use std::fmt;

/// State of a bulkhead door.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BulkheadState {
    /// Door is open, allows passage and gas flow.
    Open,
    /// Door is sealed, blocks passage and gas flow.
    Sealed,
    /// Door is jammed, cannot be opened or sealed.
    Jammed,
}

impl BulkheadState {
    /// Check if this state allows passage.
    #[must_use]
    pub fn allows_passage(&self) -> bool {
        matches!(self, BulkheadState::Open)
    }

    /// Check if this state allows gas flow.
    #[must_use]
    pub fn allows_gas_flow(&self) -> bool {
        matches!(self, BulkheadState::Open)
    }
}

impl fmt::Display for BulkheadState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BulkheadState::Open => write!(f, "Open"),
            BulkheadState::Sealed => write!(f, "Sealed"),
            BulkheadState::Jammed => write!(f, "Jammed"),
        }
    }
}

/// A bulkhead door connecting two rooms.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bulkhead {
    /// ID of the first connected room.
    room_a: usize,
    /// ID of the second connected room.
    room_b: usize,
    /// Current state of the bulkhead.
    state: BulkheadState,
}

impl Bulkhead {
    /// Create a new bulkhead between two rooms.
    #[must_use]
    pub fn new(room_a: usize, room_b: usize) -> Self {
        Self {
            room_a,
            room_b,
            state: BulkheadState::Open,
        }
    }

    /// Get the first room ID.
    #[must_use]
    pub fn room_a(&self) -> usize {
        self.room_a
    }

    /// Get the second room ID.
    #[must_use]
    pub fn room_b(&self) -> usize {
        self.room_b
    }

    /// Get the current state.
    #[must_use]
    pub fn state(&self) -> BulkheadState {
        self.state
    }

    /// Seal the bulkhead.
    ///
    /// Returns true if successful, false if jammed.
    pub fn seal(&mut self) -> bool {
        if self.state == BulkheadState::Jammed {
            return false;
        }
        self.state = BulkheadState::Sealed;
        true
    }

    /// Open the bulkhead.
    ///
    /// Returns true if successful, false if jammed.
    pub fn open(&mut self) -> bool {
        if self.state == BulkheadState::Jammed {
            return false;
        }
        self.state = BulkheadState::Open;
        true
    }

    /// Check if the bulkhead is passable.
    #[must_use]
    pub fn is_passable(&self) -> bool {
        self.state.allows_passage()
    }

    /// Jam the bulkhead in its current state.
    pub fn jam(&mut self) {
        self.state = BulkheadState::Jammed;
    }

    /// Repair a jammed bulkhead.
    pub fn repair(&mut self) {
        if self.state == BulkheadState::Jammed {
            self.state = BulkheadState::Sealed;
        }
    }

    /// Check if this bulkhead connects the given room.
    #[must_use]
    pub fn connects(&self, room_id: usize) -> bool {
        self.room_a == room_id || self.room_b == room_id
    }

    /// Get the other room connected by this bulkhead.
    #[must_use]
    pub fn other_room(&self, room_id: usize) -> Option<usize> {
        if self.room_a == room_id {
            Some(self.room_b)
        } else if self.room_b == room_id {
            Some(self.room_a)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bulkhead_state_allows_passage() {
        assert!(BulkheadState::Open.allows_passage());
        assert!(!BulkheadState::Sealed.allows_passage());
        assert!(!BulkheadState::Jammed.allows_passage());
    }

    #[test]
    fn test_bulkhead_state_allows_gas_flow() {
        assert!(BulkheadState::Open.allows_gas_flow());
        assert!(!BulkheadState::Sealed.allows_gas_flow());
        assert!(!BulkheadState::Jammed.allows_gas_flow());
    }

    #[test]
    fn test_bulkhead_state_display() {
        assert_eq!(format!("{}", BulkheadState::Open), "Open");
        assert_eq!(format!("{}", BulkheadState::Sealed), "Sealed");
        assert_eq!(format!("{}", BulkheadState::Jammed), "Jammed");
    }

    #[test]
    fn test_bulkhead_new() {
        let bulkhead = Bulkhead::new(0, 1);
        assert_eq!(bulkhead.room_a(), 0);
        assert_eq!(bulkhead.room_b(), 1);
        assert_eq!(bulkhead.state(), BulkheadState::Open);
    }

    #[test]
    fn test_bulkhead_seal() {
        let mut bulkhead = Bulkhead::new(0, 1);
        assert!(bulkhead.seal());
        assert_eq!(bulkhead.state(), BulkheadState::Sealed);
        assert!(!bulkhead.is_passable());
    }

    #[test]
    fn test_bulkhead_open() {
        let mut bulkhead = Bulkhead::new(0, 1);
        bulkhead.seal();
        assert!(bulkhead.open());
        assert_eq!(bulkhead.state(), BulkheadState::Open);
        assert!(bulkhead.is_passable());
    }

    #[test]
    fn test_bulkhead_jam_prevents_seal() {
        let mut bulkhead = Bulkhead::new(0, 1);
        bulkhead.jam();
        assert!(!bulkhead.seal());
        assert_eq!(bulkhead.state(), BulkheadState::Jammed);
    }

    #[test]
    fn test_bulkhead_jam_prevents_open() {
        let mut bulkhead = Bulkhead::new(0, 1);
        bulkhead.seal();
        bulkhead.jam();
        assert!(!bulkhead.open());
        assert_eq!(bulkhead.state(), BulkheadState::Jammed);
    }

    #[test]
    fn test_bulkhead_repair() {
        let mut bulkhead = Bulkhead::new(0, 1);
        bulkhead.jam();
        bulkhead.repair();
        assert_eq!(bulkhead.state(), BulkheadState::Sealed);
    }

    #[test]
    fn test_bulkhead_connects() {
        let bulkhead = Bulkhead::new(0, 1);
        assert!(bulkhead.connects(0));
        assert!(bulkhead.connects(1));
        assert!(!bulkhead.connects(2));
    }

    #[test]
    fn test_bulkhead_other_room() {
        let bulkhead = Bulkhead::new(0, 1);
        assert_eq!(bulkhead.other_room(0), Some(1));
        assert_eq!(bulkhead.other_room(1), Some(0));
        assert_eq!(bulkhead.other_room(2), None);
    }

    #[test]
    fn test_bulkhead_is_passable() {
        let mut bulkhead = Bulkhead::new(0, 1);
        assert!(bulkhead.is_passable());

        bulkhead.seal();
        assert!(!bulkhead.is_passable());

        bulkhead.open();
        assert!(bulkhead.is_passable());
    }
}
