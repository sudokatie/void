//! Loop state synchronization for multiplayer.
//!
//! Serializes and deserializes loop state (current loop, phase, time remaining)
//! for network transmission between server and clients.

use engine_physics::temporal::LoopPhase;
use serde::{Deserialize, Serialize};

/// Loop state data for network synchronization.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoopStatePacket {
    /// Current loop number.
    pub loop_count: u32,
    /// Current phase (as u8).
    pub phase: u8,
    /// Time remaining in current phase.
    pub time_remaining: f32,
    /// Difficulty modifier.
    pub difficulty: f32,
}

impl LoopStatePacket {
    /// Create a new loop state packet.
    #[must_use]
    pub fn new(loop_count: u32, phase: LoopPhase, time_remaining: f32, difficulty: f32) -> Self {
        Self {
            loop_count,
            phase: phase_to_u8(phase),
            time_remaining,
            difficulty,
        }
    }

    /// Get the phase as enum.
    #[must_use]
    pub fn phase(&self) -> LoopPhase {
        u8_to_phase(self.phase)
    }
}

/// Serialize loop state to bytes.
#[must_use]
pub fn serialize_loop_state(
    loop_count: u32,
    phase: LoopPhase,
    time_remaining: f32,
    difficulty: f32,
) -> Vec<u8> {
    let packet = LoopStatePacket::new(loop_count, phase, time_remaining, difficulty);
    bincode::serialize(&packet).unwrap_or_default()
}

/// Deserialize loop state from bytes.
#[must_use]
pub fn deserialize_loop_state(data: &[u8]) -> Option<LoopStatePacket> {
    bincode::deserialize(data).ok()
}

/// Synchronization handler for loop state.
#[derive(Clone, Debug, Default)]
pub struct LoopSync {
    /// Current loop count.
    loop_count: u32,
    /// Current phase.
    phase: u8,
    /// Time remaining.
    time_remaining: f32,
    /// Difficulty modifier.
    difficulty: f32,
    /// Whether state has been updated since last check.
    dirty: bool,
}

impl LoopSync {
    /// Create a new loop sync handler.
    #[must_use]
    pub fn new() -> Self {
        Self {
            loop_count: 1,
            phase: 0,
            time_remaining: 30.0,
            difficulty: 1.0,
            dirty: false,
        }
    }

    /// Create a packet from current state.
    #[must_use]
    pub fn create_packet(&self) -> LoopStatePacket {
        LoopStatePacket {
            loop_count: self.loop_count,
            phase: self.phase,
            time_remaining: self.time_remaining,
            difficulty: self.difficulty,
        }
    }

    /// Update state from a network packet.
    ///
    /// Returns true if state was updated.
    pub fn update_from_network(&mut self, packet: &LoopStatePacket) -> bool {
        let changed = self.loop_count != packet.loop_count
            || self.phase != packet.phase
            || (self.time_remaining - packet.time_remaining).abs() > 0.1;

        if changed {
            self.loop_count = packet.loop_count;
            self.phase = packet.phase;
            self.time_remaining = packet.time_remaining;
            self.difficulty = packet.difficulty;
            self.dirty = true;
        }
        changed
    }

    /// Get current loop count.
    #[must_use]
    pub fn loop_count(&self) -> u32 {
        self.loop_count
    }

    /// Get current phase.
    #[must_use]
    pub fn phase(&self) -> LoopPhase {
        u8_to_phase(self.phase)
    }

    /// Get time remaining.
    #[must_use]
    pub fn time_remaining(&self) -> f32 {
        self.time_remaining
    }

    /// Get difficulty modifier.
    #[must_use]
    pub fn difficulty(&self) -> f32 {
        self.difficulty
    }

    /// Check and clear dirty flag.
    pub fn take_dirty(&mut self) -> bool {
        let was_dirty = self.dirty;
        self.dirty = false;
        was_dirty
    }
}

/// Convert phase enum to u8.
#[must_use]
pub fn phase_to_u8(phase: LoopPhase) -> u8 {
    match phase {
        LoopPhase::Dawn => 0,
        LoopPhase::Day => 1,
        LoopPhase::Dusk => 2,
        LoopPhase::Midnight => 3,
    }
}

/// Convert u8 to phase enum.
#[must_use]
pub fn u8_to_phase(value: u8) -> LoopPhase {
    match value {
        0 => LoopPhase::Dawn,
        1 => LoopPhase::Day,
        2 => LoopPhase::Dusk,
        3 => LoopPhase::Midnight,
        _ => LoopPhase::Dawn,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_state_packet_new() {
        let packet = LoopStatePacket::new(5, LoopPhase::Dusk, 15.0, 1.4);
        assert_eq!(packet.loop_count, 5);
        assert_eq!(packet.phase(), LoopPhase::Dusk);
        assert!((packet.time_remaining - 15.0).abs() < f32::EPSILON);
        assert!((packet.difficulty - 1.4).abs() < f32::EPSILON);
    }

    #[test]
    fn test_serialize_deserialize_roundtrip() {
        let data = serialize_loop_state(10, LoopPhase::Midnight, 5.5, 2.0);
        let packet = deserialize_loop_state(&data).unwrap();

        assert_eq!(packet.loop_count, 10);
        assert_eq!(packet.phase(), LoopPhase::Midnight);
        assert!((packet.time_remaining - 5.5).abs() < f32::EPSILON);
        assert!((packet.difficulty - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_loop_sync_new() {
        let sync = LoopSync::new();
        assert_eq!(sync.loop_count(), 1);
        assert_eq!(sync.phase(), LoopPhase::Dawn);
    }

    #[test]
    fn test_loop_sync_update_from_network() {
        let mut sync = LoopSync::new();
        let packet = LoopStatePacket::new(3, LoopPhase::Day, 20.0, 1.2);

        let updated = sync.update_from_network(&packet);
        assert!(updated);
        assert_eq!(sync.loop_count(), 3);
        assert_eq!(sync.phase(), LoopPhase::Day);
    }

    #[test]
    fn test_loop_sync_dirty_flag() {
        let mut sync = LoopSync::new();
        let packet = LoopStatePacket::new(2, LoopPhase::Dusk, 10.0, 1.1);

        sync.update_from_network(&packet);
        assert!(sync.take_dirty());
        assert!(!sync.take_dirty());
    }

    #[test]
    fn test_phase_conversion() {
        assert_eq!(u8_to_phase(phase_to_u8(LoopPhase::Dawn)), LoopPhase::Dawn);
        assert_eq!(u8_to_phase(phase_to_u8(LoopPhase::Day)), LoopPhase::Day);
        assert_eq!(u8_to_phase(phase_to_u8(LoopPhase::Dusk)), LoopPhase::Dusk);
        assert_eq!(
            u8_to_phase(phase_to_u8(LoopPhase::Midnight)),
            LoopPhase::Midnight
        );
    }
}
