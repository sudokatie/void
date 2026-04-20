//! Titan state synchronization.
//!
//! Serialization and deserialization for Titan state across network.

/// Serialize Titan state for network transmission.
///
/// Packs HP (f32), mood (u8), phase (u8), and day (u32) into bytes.
#[must_use]
pub fn serialize_titan_state(hp: f32, mood: u8, phase: u8, day: u32) -> Vec<u8> {
    let mut data = Vec::with_capacity(10);

    // HP as 4 bytes
    data.extend_from_slice(&hp.to_le_bytes());

    // Mood as 1 byte
    data.push(mood);

    // Phase as 1 byte
    data.push(phase);

    // Day as 4 bytes
    data.extend_from_slice(&day.to_le_bytes());

    data
}

/// Deserialize Titan state from network data.
///
/// Returns (hp, mood, phase, day) if valid, None otherwise.
#[must_use]
pub fn deserialize_titan_state(data: &[u8]) -> Option<(f32, u8, u8, u32)> {
    if data.len() < 10 {
        return None;
    }

    // HP from first 4 bytes
    let hp = f32::from_le_bytes([data[0], data[1], data[2], data[3]]);

    // Mood from byte 4
    let mood = data[4];

    // Phase from byte 5
    let phase = data[5];

    // Day from bytes 6-9
    let day = u32::from_le_bytes([data[6], data[7], data[8], data[9]]);

    Some((hp, mood, phase, day))
}

/// Titan state sync handler.
#[derive(Clone, Debug)]
pub struct TitanSync {
    /// Last received HP.
    last_hp: f32,
    /// Last received mood.
    last_mood: u8,
    /// Last received phase.
    last_phase: u8,
    /// Last received day.
    last_day: u32,
    /// Sequence number for ordering.
    sequence: u32,
}

impl Default for TitanSync {
    fn default() -> Self {
        Self::new()
    }
}

impl TitanSync {
    /// Create a new Titan sync handler.
    #[must_use]
    pub fn new() -> Self {
        Self {
            last_hp: 10000.0,
            last_mood: 0,
            last_phase: 0,
            last_day: 1,
            sequence: 0,
        }
    }

    /// Update from received state.
    ///
    /// Returns true if state was updated.
    pub fn update_from_network(&mut self, data: &[u8]) -> bool {
        if let Some((hp, mood, phase, day)) = deserialize_titan_state(data) {
            self.last_hp = hp;
            self.last_mood = mood;
            self.last_phase = phase;
            self.last_day = day;
            self.sequence = self.sequence.wrapping_add(1);
            true
        } else {
            false
        }
    }

    /// Create network packet from current state.
    #[must_use]
    pub fn create_packet(&self, hp: f32, mood: u8, phase: u8, day: u32) -> Vec<u8> {
        serialize_titan_state(hp, mood, phase, day)
    }

    /// Get last received HP.
    #[must_use]
    pub fn hp(&self) -> f32 {
        self.last_hp
    }

    /// Get last received mood.
    #[must_use]
    pub fn mood(&self) -> u8 {
        self.last_mood
    }

    /// Get last received phase.
    #[must_use]
    pub fn phase(&self) -> u8 {
        self.last_phase
    }

    /// Get last received day.
    #[must_use]
    pub fn day(&self) -> u32 {
        self.last_day
    }

    /// Get sequence number.
    #[must_use]
    pub fn sequence(&self) -> u32 {
        self.sequence
    }

    /// Check if Titan is alive based on last state.
    #[must_use]
    pub fn is_alive(&self) -> bool {
        self.last_hp > 0.0
    }

    /// Check if Titan is enraged based on last state.
    #[must_use]
    pub fn is_enraged(&self) -> bool {
        self.last_mood >= 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_titan_state() {
        let data = serialize_titan_state(10000.0, 1, 2, 5);
        assert_eq!(data.len(), 10);
    }

    #[test]
    fn test_deserialize_titan_state() {
        let data = serialize_titan_state(8000.0, 2, 1, 10);
        let result = deserialize_titan_state(&data);

        assert!(result.is_some());
        let (hp, mood, phase, day) = result.unwrap();
        assert!((hp - 8000.0).abs() < f32::EPSILON);
        assert_eq!(mood, 2);
        assert_eq!(phase, 1);
        assert_eq!(day, 10);
    }

    #[test]
    fn test_deserialize_titan_state_too_short() {
        let data = vec![0, 1, 2, 3, 4];
        let result = deserialize_titan_state(&data);
        assert!(result.is_none());
    }

    #[test]
    fn test_roundtrip() {
        let original_hp = 5000.5;
        let original_mood = 1;
        let original_phase = 3;
        let original_day = 42;

        let data = serialize_titan_state(original_hp, original_mood, original_phase, original_day);
        let (hp, mood, phase, day) = deserialize_titan_state(&data).unwrap();

        assert!((hp - original_hp).abs() < f32::EPSILON);
        assert_eq!(mood, original_mood);
        assert_eq!(phase, original_phase);
        assert_eq!(day, original_day);
    }

    #[test]
    fn test_titan_sync_new() {
        let sync = TitanSync::new();
        assert!((sync.hp() - 10000.0).abs() < f32::EPSILON);
        assert_eq!(sync.mood(), 0);
        assert_eq!(sync.phase(), 0);
        assert_eq!(sync.day(), 1);
    }

    #[test]
    fn test_titan_sync_update_from_network() {
        let mut sync = TitanSync::new();
        let data = serialize_titan_state(5000.0, 2, 1, 15);

        let updated = sync.update_from_network(&data);

        assert!(updated);
        assert!((sync.hp() - 5000.0).abs() < f32::EPSILON);
        assert_eq!(sync.mood(), 2);
        assert_eq!(sync.phase(), 1);
        assert_eq!(sync.day(), 15);
    }

    #[test]
    fn test_titan_sync_update_from_network_invalid() {
        let mut sync = TitanSync::new();
        let data = vec![0, 1, 2]; // Too short

        let updated = sync.update_from_network(&data);

        assert!(!updated);
        // State unchanged
        assert!((sync.hp() - 10000.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_titan_sync_create_packet() {
        let sync = TitanSync::new();
        let packet = sync.create_packet(7000.0, 1, 2, 8);

        assert_eq!(packet.len(), 10);

        let (hp, mood, phase, day) = deserialize_titan_state(&packet).unwrap();
        assert!((hp - 7000.0).abs() < f32::EPSILON);
        assert_eq!(mood, 1);
        assert_eq!(phase, 2);
        assert_eq!(day, 8);
    }

    #[test]
    fn test_titan_sync_sequence() {
        let mut sync = TitanSync::new();
        assert_eq!(sync.sequence(), 0);

        let data = serialize_titan_state(5000.0, 0, 0, 1);
        sync.update_from_network(&data);
        assert_eq!(sync.sequence(), 1);

        sync.update_from_network(&data);
        assert_eq!(sync.sequence(), 2);
    }

    #[test]
    fn test_titan_sync_is_alive() {
        let mut sync = TitanSync::new();
        assert!(sync.is_alive());

        let data = serialize_titan_state(0.0, 0, 0, 1);
        sync.update_from_network(&data);
        assert!(!sync.is_alive());
    }

    #[test]
    fn test_titan_sync_is_enraged() {
        let mut sync = TitanSync::new();
        assert!(!sync.is_enraged());

        let data = serialize_titan_state(10000.0, 2, 0, 1);
        sync.update_from_network(&data);
        assert!(sync.is_enraged());
    }

    #[test]
    fn test_titan_sync_default() {
        let sync = TitanSync::default();
        assert!((sync.hp() - 10000.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_serialize_edge_cases() {
        // Test with 0 values
        let data = serialize_titan_state(0.0, 0, 0, 0);
        let (hp, mood, phase, day) = deserialize_titan_state(&data).unwrap();
        assert!((hp - 0.0).abs() < f32::EPSILON);
        assert_eq!(mood, 0);
        assert_eq!(phase, 0);
        assert_eq!(day, 0);

        // Test with max values
        let data = serialize_titan_state(f32::MAX, 255, 255, u32::MAX);
        let (hp, mood, phase, day) = deserialize_titan_state(&data).unwrap();
        assert!((hp - f32::MAX).abs() < f32::EPSILON);
        assert_eq!(mood, 255);
        assert_eq!(phase, 255);
        assert_eq!(day, u32::MAX);
    }
}
