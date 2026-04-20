//! Position synchronization for entities.
//!
//! Serialization and deserialization for entity positions across network.

use glam::IVec3;

/// Serialize a list of entity positions.
///
/// Format: [count: u16] [id: u64, x: i32, y: i32, z: i32]...
#[must_use]
pub fn serialize_positions(positions: &[(u64, IVec3)]) -> Vec<u8> {
    // Calculate size: 2 (count) + entries * (8 id + 12 pos)
    let entry_size = 20;
    let count = positions.len().min(u16::MAX as usize);
    let mut data = Vec::with_capacity(2 + count * entry_size);

    // Write count
    data.extend_from_slice(&(count as u16).to_le_bytes());

    // Write each entry
    for (id, pos) in positions.iter().take(count) {
        data.extend_from_slice(&id.to_le_bytes());
        data.extend_from_slice(&pos.x.to_le_bytes());
        data.extend_from_slice(&pos.y.to_le_bytes());
        data.extend_from_slice(&pos.z.to_le_bytes());
    }

    data
}

/// Deserialize entity positions from network data.
#[must_use]
pub fn deserialize_positions(data: &[u8]) -> Vec<(u64, IVec3)> {
    if data.len() < 2 {
        return Vec::new();
    }

    let count = u16::from_le_bytes([data[0], data[1]]) as usize;
    let entry_size = 20;

    // Validate data length
    let expected_len = 2 + count * entry_size;
    if data.len() < expected_len {
        return Vec::new();
    }

    let mut positions = Vec::with_capacity(count);
    let mut offset = 2;

    for _ in 0..count {
        // Read entity ID
        let id = u64::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ]);
        offset += 8;

        // Read position
        let x = i32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]);
        offset += 4;

        let y = i32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]);
        offset += 4;

        let z = i32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]);
        offset += 4;

        positions.push((id, IVec3::new(x, y, z)));
    }

    positions
}

/// Position sync handler.
#[derive(Clone, Debug, Default)]
pub struct PositionSync {
    /// Cached positions.
    positions: Vec<(u64, IVec3)>,
    /// Sequence number for ordering.
    sequence: u32,
}

impl PositionSync {
    /// Create a new position sync handler.
    #[must_use]
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            sequence: 0,
        }
    }

    /// Update from received data.
    ///
    /// Returns true if updated successfully.
    pub fn update_from_network(&mut self, data: &[u8]) -> bool {
        let positions = deserialize_positions(data);
        if !positions.is_empty() || (data.len() >= 2 && data[0] == 0 && data[1] == 0) {
            self.positions = positions;
            self.sequence = self.sequence.wrapping_add(1);
            true
        } else {
            false
        }
    }

    /// Create network packet from positions.
    #[must_use]
    pub fn create_packet(&self, positions: &[(u64, IVec3)]) -> Vec<u8> {
        serialize_positions(positions)
    }

    /// Get cached positions.
    #[must_use]
    pub fn positions(&self) -> &[(u64, IVec3)] {
        &self.positions
    }

    /// Get position for a specific entity.
    #[must_use]
    pub fn get_position(&self, entity_id: u64) -> Option<IVec3> {
        self.positions
            .iter()
            .find(|(id, _)| *id == entity_id)
            .map(|(_, pos)| *pos)
    }

    /// Get sequence number.
    #[must_use]
    pub fn sequence(&self) -> u32 {
        self.sequence
    }

    /// Get number of tracked entities.
    #[must_use]
    pub fn count(&self) -> usize {
        self.positions.len()
    }

    /// Clear cached positions.
    pub fn clear(&mut self) {
        self.positions.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_positions_empty() {
        let data = serialize_positions(&[]);
        assert_eq!(data.len(), 2);
        assert_eq!(data[0], 0);
        assert_eq!(data[1], 0);
    }

    #[test]
    fn test_serialize_positions_single() {
        let positions = vec![(1u64, IVec3::new(10, 20, 30))];
        let data = serialize_positions(&positions);
        assert_eq!(data.len(), 22); // 2 + 20
    }

    #[test]
    fn test_serialize_positions_multiple() {
        let positions = vec![
            (1u64, IVec3::new(10, 20, 30)),
            (2u64, IVec3::new(40, 50, 60)),
            (3u64, IVec3::new(70, 80, 90)),
        ];
        let data = serialize_positions(&positions);
        assert_eq!(data.len(), 62); // 2 + 3 * 20
    }

    #[test]
    fn test_deserialize_positions_empty() {
        let data = vec![0, 0];
        let positions = deserialize_positions(&data);
        assert!(positions.is_empty());
    }

    #[test]
    fn test_deserialize_positions_too_short() {
        let data = vec![0];
        let positions = deserialize_positions(&data);
        assert!(positions.is_empty());
    }

    #[test]
    fn test_roundtrip_single() {
        let original = vec![(42u64, IVec3::new(100, -50, 200))];
        let data = serialize_positions(&original);
        let result = deserialize_positions(&data);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, 42);
        assert_eq!(result[0].1, IVec3::new(100, -50, 200));
    }

    #[test]
    fn test_roundtrip_multiple() {
        let original = vec![
            (1u64, IVec3::new(0, 0, 0)),
            (1000u64, IVec3::new(-100, 500, -999)),
            (u64::MAX, IVec3::new(i32::MAX, i32::MIN, 0)),
        ];
        let data = serialize_positions(&original);
        let result = deserialize_positions(&data);

        assert_eq!(result.len(), 3);

        assert_eq!(result[0].0, 1);
        assert_eq!(result[0].1, IVec3::new(0, 0, 0));

        assert_eq!(result[1].0, 1000);
        assert_eq!(result[1].1, IVec3::new(-100, 500, -999));

        assert_eq!(result[2].0, u64::MAX);
        assert_eq!(result[2].1, IVec3::new(i32::MAX, i32::MIN, 0));
    }

    #[test]
    fn test_position_sync_new() {
        let sync = PositionSync::new();
        assert!(sync.positions().is_empty());
        assert_eq!(sync.sequence(), 0);
    }

    #[test]
    fn test_position_sync_update_from_network() {
        let mut sync = PositionSync::new();
        let original = vec![(5u64, IVec3::new(1, 2, 3))];
        let data = serialize_positions(&original);

        let updated = sync.update_from_network(&data);

        assert!(updated);
        assert_eq!(sync.count(), 1);
        assert_eq!(sync.positions()[0].0, 5);
    }

    #[test]
    fn test_position_sync_update_from_network_empty() {
        let mut sync = PositionSync::new();
        let data = serialize_positions(&[]);

        let updated = sync.update_from_network(&data);

        assert!(updated);
        assert!(sync.positions().is_empty());
    }

    #[test]
    fn test_position_sync_create_packet() {
        let sync = PositionSync::new();
        let positions = vec![(10u64, IVec3::new(5, 6, 7))];
        let packet = sync.create_packet(&positions);

        let result = deserialize_positions(&packet);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, 10);
    }

    #[test]
    fn test_position_sync_get_position() {
        let mut sync = PositionSync::new();
        let positions = vec![
            (1u64, IVec3::new(10, 20, 30)),
            (2u64, IVec3::new(40, 50, 60)),
        ];
        let data = serialize_positions(&positions);
        sync.update_from_network(&data);

        assert_eq!(sync.get_position(1), Some(IVec3::new(10, 20, 30)));
        assert_eq!(sync.get_position(2), Some(IVec3::new(40, 50, 60)));
        assert_eq!(sync.get_position(999), None);
    }

    #[test]
    fn test_position_sync_sequence() {
        let mut sync = PositionSync::new();
        assert_eq!(sync.sequence(), 0);

        let data = serialize_positions(&[]);
        sync.update_from_network(&data);
        assert_eq!(sync.sequence(), 1);

        sync.update_from_network(&data);
        assert_eq!(sync.sequence(), 2);
    }

    #[test]
    fn test_position_sync_count() {
        let mut sync = PositionSync::new();
        assert_eq!(sync.count(), 0);

        let positions = vec![
            (1u64, IVec3::ZERO),
            (2u64, IVec3::ZERO),
            (3u64, IVec3::ZERO),
        ];
        let data = serialize_positions(&positions);
        sync.update_from_network(&data);
        assert_eq!(sync.count(), 3);
    }

    #[test]
    fn test_position_sync_clear() {
        let mut sync = PositionSync::new();
        let positions = vec![(1u64, IVec3::ZERO)];
        let data = serialize_positions(&positions);
        sync.update_from_network(&data);
        assert_eq!(sync.count(), 1);

        sync.clear();
        assert_eq!(sync.count(), 0);
    }

    #[test]
    fn test_position_sync_default() {
        let sync = PositionSync::default();
        assert!(sync.positions().is_empty());
    }

    #[test]
    fn test_deserialize_truncated_data() {
        // Create valid header but truncated entries
        let mut data = vec![0, 2]; // Says 2 entries
        data.extend_from_slice(&[0; 10]); // But only partial data

        let positions = deserialize_positions(&data);
        assert!(positions.is_empty()); // Should fail gracefully
    }

    #[test]
    fn test_negative_coordinates() {
        let original = vec![(1u64, IVec3::new(-100, -200, -300))];
        let data = serialize_positions(&original);
        let result = deserialize_positions(&data);

        assert_eq!(result[0].1, IVec3::new(-100, -200, -300));
    }
}
