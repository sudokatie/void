//! Cross-dimension synchronization for multiplayer.
//!
//! Serializes and deserializes player dimension states and ghost positions
//! for players in different dimensions to be visible as "ghosts" to each other.

use engine_physics::dimension::Dimension;
use glam::IVec3;

/// Cross-dimension synchronization handler.
///
/// Provides serialization and deserialization for player dimension states
/// and ghost positions when players are in different dimensions.
#[derive(Clone, Debug, Default)]
pub struct CrossDimensionSync;

impl CrossDimensionSync {
    /// Create a new cross-dimension sync handler.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Serialize a player's dimension for network transmission.
    ///
    /// Format: [player_id as 8 bytes BE] [dim as u8]
    #[must_use]
    pub fn serialize_player_dimension(&self, player_id: u64, dim: Dimension) -> Vec<u8> {
        let mut data = Vec::with_capacity(9);
        data.extend_from_slice(&player_id.to_be_bytes());
        data.push(dimension_to_u8(dim));
        data
    }

    /// Deserialize a player's dimension from network data.
    ///
    /// Returns None if data is too short or contains an invalid dimension.
    #[must_use]
    pub fn deserialize_player_dimension(&self, data: &[u8]) -> Option<(u64, Dimension)> {
        if data.len() < 9 {
            return None;
        }

        let player_id = u64::from_be_bytes([
            data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
        ]);
        let dim = u8_to_dimension(data[8])?;

        Some((player_id, dim))
    }

    /// Serialize ghost positions for players in different dimensions.
    ///
    /// Format: [count as u8] [for each: player_id 8B, x 4B, y 4B, z 4B, dim 1B]
    #[must_use]
    pub fn serialize_ghost_positions(&self, players: &[(u64, IVec3, Dimension)]) -> Vec<u8> {
        let count = players.len().min(255) as u8;
        let mut data = Vec::with_capacity(1 + count as usize * 21);
        data.push(count);

        for &(player_id, pos, dim) in players.iter().take(255) {
            data.extend_from_slice(&player_id.to_be_bytes());
            data.extend_from_slice(&pos.x.to_be_bytes());
            data.extend_from_slice(&pos.y.to_be_bytes());
            data.extend_from_slice(&pos.z.to_be_bytes());
            data.push(dimension_to_u8(dim));
        }

        data
    }

    /// Deserialize ghost positions from network data.
    ///
    /// Returns an empty vector if data is invalid.
    #[must_use]
    pub fn deserialize_ghost_positions(&self, data: &[u8]) -> Vec<(u64, IVec3, Dimension)> {
        let mut result = Vec::new();

        if data.is_empty() {
            return result;
        }

        let count = data[0] as usize;
        let entry_size = 21; // 8 + 4 + 4 + 4 + 1

        if data.len() < 1 + count * entry_size {
            return result;
        }

        let mut offset = 1;
        for _ in 0..count {
            if offset + entry_size > data.len() {
                break;
            }

            let player_id = u64::from_be_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
                data[offset + 4],
                data[offset + 5],
                data[offset + 6],
                data[offset + 7],
            ]);

            let x = i32::from_be_bytes([
                data[offset + 8],
                data[offset + 9],
                data[offset + 10],
                data[offset + 11],
            ]);
            let y = i32::from_be_bytes([
                data[offset + 12],
                data[offset + 13],
                data[offset + 14],
                data[offset + 15],
            ]);
            let z = i32::from_be_bytes([
                data[offset + 16],
                data[offset + 17],
                data[offset + 18],
                data[offset + 19],
            ]);

            let dim_byte = data[offset + 20];
            if let Some(dim) = u8_to_dimension(dim_byte) {
                result.push((player_id, IVec3::new(x, y, z), dim));
            }

            offset += entry_size;
        }

        result
    }
}

/// Convert a Dimension to its u8 representation.
#[must_use]
fn dimension_to_u8(dim: Dimension) -> u8 {
    match dim {
        Dimension::Prime => 0,
        Dimension::Inverted => 1,
        Dimension::Void => 2,
        Dimension::Nexus => 3,
    }
}

/// Convert a u8 to a Dimension, returning None for invalid values.
#[must_use]
fn u8_to_dimension(value: u8) -> Option<Dimension> {
    match value {
        0 => Some(Dimension::Prime),
        1 => Some(Dimension::Inverted),
        2 => Some(Dimension::Void),
        3 => Some(Dimension::Nexus),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _sync = CrossDimensionSync::new();
    }

    #[test]
    fn test_serialize_player_dimension_format() {
        let sync = CrossDimensionSync::new();
        let data = sync.serialize_player_dimension(12345, Dimension::Prime);

        // Format: [player_id as 8 bytes BE] [dim as u8]
        assert_eq!(data.len(), 9);
        assert_eq!(
            u64::from_be_bytes([data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]]),
            12345
        );
        assert_eq!(data[8], 0); // Prime = 0
    }

    #[test]
    fn test_serialize_player_dimension_all_dimensions() {
        let sync = CrossDimensionSync::new();

        assert_eq!(sync.serialize_player_dimension(1, Dimension::Prime)[8], 0);
        assert_eq!(sync.serialize_player_dimension(1, Dimension::Inverted)[8], 1);
        assert_eq!(sync.serialize_player_dimension(1, Dimension::Void)[8], 2);
        assert_eq!(sync.serialize_player_dimension(1, Dimension::Nexus)[8], 3);
    }

    #[test]
    fn test_deserialize_player_dimension_roundtrip() {
        let sync = CrossDimensionSync::new();
        let data = sync.serialize_player_dimension(9999, Dimension::Void);

        let result = sync.deserialize_player_dimension(&data);
        assert!(result.is_some());

        let (player_id, dim) = result.unwrap();
        assert_eq!(player_id, 9999);
        assert_eq!(dim, Dimension::Void);
    }

    #[test]
    fn test_deserialize_player_dimension_errors() {
        let sync = CrossDimensionSync::new();

        // Too short (5 bytes, need 9)
        assert!(sync.deserialize_player_dimension(&[0u8, 0, 0, 0, 0]).is_none());

        // Invalid dimension byte (99)
        let mut data = vec![0u8; 9];
        data[8] = 99;
        assert!(sync.deserialize_player_dimension(&data).is_none());
    }

    #[test]
    fn test_serialize_ghost_positions_format() {
        let sync = CrossDimensionSync::new();

        // Empty list
        let data = sync.serialize_ghost_positions(&[]);
        assert_eq!(data.len(), 1);
        assert_eq!(data[0], 0);

        // Single player: 1 + 21 bytes
        let players = vec![(100u64, IVec3::new(10, 20, 30), Dimension::Inverted)];
        let data = sync.serialize_ghost_positions(&players);
        assert_eq!(data.len(), 22);
        assert_eq!(data[0], 1);

        // Three players: 1 + 3*21 = 64 bytes
        let players = vec![
            (1u64, IVec3::new(0, 0, 0), Dimension::Prime),
            (2u64, IVec3::new(100, 200, 300), Dimension::Void),
            (3u64, IVec3::new(-50, -100, -150), Dimension::Nexus),
        ];
        let data = sync.serialize_ghost_positions(&players);
        assert_eq!(data.len(), 64);
        assert_eq!(data[0], 3);
    }

    #[test]
    fn test_deserialize_ghost_positions_roundtrip() {
        let sync = CrossDimensionSync::new();
        let players = vec![
            (42u64, IVec3::new(100, 64, -200), Dimension::Inverted),
            (99u64, IVec3::new(-1000, 128, 500), Dimension::Void),
        ];

        let data = sync.serialize_ghost_positions(&players);
        let decoded = sync.deserialize_ghost_positions(&data);

        assert_eq!(decoded.len(), 2);
        assert_eq!(decoded[0], (42, IVec3::new(100, 64, -200), Dimension::Inverted));
        assert_eq!(decoded[1], (99, IVec3::new(-1000, 128, 500), Dimension::Void));
    }

    #[test]
    fn test_deserialize_ghost_positions_errors() {
        let sync = CrossDimensionSync::new();

        // Empty data
        assert!(sync.deserialize_ghost_positions(&[]).is_empty());

        // Truncated: count=2 but only 1 entry
        let mut data = vec![2u8];
        data.extend_from_slice(&[0u8; 21]);
        assert!(sync.deserialize_ghost_positions(&data).is_empty());
    }

    #[test]
    fn test_dimension_to_u8_conversion() {
        assert_eq!(dimension_to_u8(Dimension::Prime), 0);
        assert_eq!(dimension_to_u8(Dimension::Inverted), 1);
        assert_eq!(dimension_to_u8(Dimension::Void), 2);
        assert_eq!(dimension_to_u8(Dimension::Nexus), 3);
    }

    #[test]
    fn test_u8_to_dimension_conversion() {
        assert_eq!(u8_to_dimension(0), Some(Dimension::Prime));
        assert_eq!(u8_to_dimension(1), Some(Dimension::Inverted));
        assert_eq!(u8_to_dimension(2), Some(Dimension::Void));
        assert_eq!(u8_to_dimension(3), Some(Dimension::Nexus));
        assert_eq!(u8_to_dimension(4), None);
        assert_eq!(u8_to_dimension(255), None);
    }
}
