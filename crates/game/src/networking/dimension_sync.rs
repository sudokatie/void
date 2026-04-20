//! Dimension state synchronization.
//!
//! Serialization and deserialization of dimension chunk states
//! for network transmission.

use std::collections::HashMap;

use glam::IVec3;

/// Dimension state synchronization handler.
#[derive(Clone, Debug, Default)]
pub struct DimensionSync;

impl DimensionSync {
    /// Create a new dimension sync handler.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Serialize dimension state for network transmission.
    ///
    /// Format: count (4 bytes) + entries (x, y, z as i32 + dimension as u8)
    #[must_use]
    pub fn serialize_dimension_state(&self, chunks: &HashMap<IVec3, u8>) -> Vec<u8> {
        let mut data = Vec::new();

        // Write count
        let count = chunks.len() as u32;
        data.extend_from_slice(&count.to_le_bytes());

        // Write entries
        for (pos, dim) in chunks {
            data.extend_from_slice(&pos.x.to_le_bytes());
            data.extend_from_slice(&pos.y.to_le_bytes());
            data.extend_from_slice(&pos.z.to_le_bytes());
            data.push(*dim);
        }

        data
    }

    /// Deserialize dimension state from network data.
    ///
    /// Returns the chunk dimension map.
    #[must_use]
    pub fn deserialize_dimension_state(&self, data: &[u8]) -> HashMap<IVec3, u8> {
        let mut chunks = HashMap::new();

        if data.len() < 4 {
            return chunks;
        }

        let count = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
        let entry_size = 13; // 3 * 4 bytes (i32) + 1 byte (u8)

        let expected_len = 4 + count * entry_size;
        if data.len() < expected_len {
            return chunks;
        }

        let mut offset = 4;
        for _ in 0..count {
            if offset + entry_size > data.len() {
                break;
            }

            let x = i32::from_le_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            ]);
            let y = i32::from_le_bytes([
                data[offset + 4],
                data[offset + 5],
                data[offset + 6],
                data[offset + 7],
            ]);
            let z = i32::from_le_bytes([
                data[offset + 8],
                data[offset + 9],
                data[offset + 10],
                data[offset + 11],
            ]);
            let dim = data[offset + 12];

            chunks.insert(IVec3::new(x, y, z), dim);
            offset += entry_size;
        }

        chunks
    }

    /// Serialize a single chunk update.
    #[must_use]
    pub fn serialize_chunk_update(&self, pos: IVec3, dimension: u8) -> Vec<u8> {
        let mut data = Vec::with_capacity(13);
        data.extend_from_slice(&pos.x.to_le_bytes());
        data.extend_from_slice(&pos.y.to_le_bytes());
        data.extend_from_slice(&pos.z.to_le_bytes());
        data.push(dimension);
        data
    }

    /// Deserialize a single chunk update.
    #[must_use]
    pub fn deserialize_chunk_update(&self, data: &[u8]) -> Option<(IVec3, u8)> {
        if data.len() < 13 {
            return None;
        }

        let x = i32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        let y = i32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        let z = i32::from_le_bytes([data[8], data[9], data[10], data[11]]);
        let dim = data[12];

        Some((IVec3::new(x, y, z), dim))
    }
}

/// Serialize dimension state (standalone function).
#[must_use]
pub fn serialize_dimension_state(chunks: &HashMap<IVec3, u8>) -> Vec<u8> {
    DimensionSync::new().serialize_dimension_state(chunks)
}

/// Deserialize dimension state (standalone function).
#[must_use]
pub fn deserialize_dimension_state(data: &[u8]) -> HashMap<IVec3, u8> {
    DimensionSync::new().deserialize_dimension_state(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimension_sync_new() {
        let _sync = DimensionSync::new();
    }

    #[test]
    fn test_serialize_empty() {
        let sync = DimensionSync::new();
        let chunks = HashMap::new();

        let data = sync.serialize_dimension_state(&chunks);
        assert_eq!(data.len(), 4); // Just the count

        let decoded = sync.deserialize_dimension_state(&data);
        assert!(decoded.is_empty());
    }

    #[test]
    fn test_serialize_single_chunk() {
        let sync = DimensionSync::new();
        let mut chunks = HashMap::new();
        chunks.insert(IVec3::new(1, 2, 3), 1u8);

        let data = sync.serialize_dimension_state(&chunks);
        let decoded = sync.deserialize_dimension_state(&data);

        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded.get(&IVec3::new(1, 2, 3)), Some(&1u8));
    }

    #[test]
    fn test_serialize_multiple_chunks() {
        let sync = DimensionSync::new();
        let mut chunks = HashMap::new();
        chunks.insert(IVec3::new(0, 0, 0), 0u8);
        chunks.insert(IVec3::new(1, 1, 1), 1u8);
        chunks.insert(IVec3::new(-5, 10, 20), 2u8);

        let data = sync.serialize_dimension_state(&chunks);
        let decoded = sync.deserialize_dimension_state(&data);

        assert_eq!(decoded.len(), 3);
        assert_eq!(decoded.get(&IVec3::new(0, 0, 0)), Some(&0u8));
        assert_eq!(decoded.get(&IVec3::new(1, 1, 1)), Some(&1u8));
        assert_eq!(decoded.get(&IVec3::new(-5, 10, 20)), Some(&2u8));
    }

    #[test]
    fn test_serialize_negative_coords() {
        let sync = DimensionSync::new();
        let mut chunks = HashMap::new();
        chunks.insert(IVec3::new(-100, -200, -300), 3u8);

        let data = sync.serialize_dimension_state(&chunks);
        let decoded = sync.deserialize_dimension_state(&data);

        assert_eq!(decoded.get(&IVec3::new(-100, -200, -300)), Some(&3u8));
    }

    #[test]
    fn test_deserialize_invalid_data() {
        let sync = DimensionSync::new();

        // Too short
        let decoded = sync.deserialize_dimension_state(&[0, 1]);
        assert!(decoded.is_empty());

        // Count but no data
        let decoded = sync.deserialize_dimension_state(&[1, 0, 0, 0]);
        assert!(decoded.is_empty());
    }

    #[test]
    fn test_serialize_chunk_update() {
        let sync = DimensionSync::new();
        let pos = IVec3::new(10, 20, 30);

        let data = sync.serialize_chunk_update(pos, 2);
        let decoded = sync.deserialize_chunk_update(&data);

        assert!(decoded.is_some());
        let (dec_pos, dec_dim) = decoded.unwrap();
        assert_eq!(dec_pos, pos);
        assert_eq!(dec_dim, 2);
    }

    #[test]
    fn test_deserialize_chunk_update_invalid() {
        let sync = DimensionSync::new();

        let decoded = sync.deserialize_chunk_update(&[0, 1, 2]);
        assert!(decoded.is_none());
    }

    #[test]
    fn test_standalone_functions() {
        let mut chunks = HashMap::new();
        chunks.insert(IVec3::new(5, 5, 5), 1u8);

        let data = serialize_dimension_state(&chunks);
        let decoded = deserialize_dimension_state(&data);

        assert_eq!(decoded.get(&IVec3::new(5, 5, 5)), Some(&1u8));
    }
}
