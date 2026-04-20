//! Persistent state synchronization for multiplayer.
//!
//! Serializes and deserializes persistent game state (chest contents,
//! knowledge, messages) for network transmission between server and clients.

use serde::{Deserialize, Serialize};

use crate::knowledge::discoveries::{DiscoveryID, KnowledgeCategory};
use crate::temporal::state_persistence::StateCategory;
use crate::temporal_chest::chest::ItemStack;

/// A serializable chest state for network sync.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChestStatePacket {
    /// Chest ID (position hash or unique ID).
    pub chest_id: u64,
    /// Slot contents (slot index, item type, quantity).
    pub slots: Vec<(usize, String, u32)>,
}

impl ChestStatePacket {
    /// Create a new chest state packet.
    #[must_use]
    pub fn new(chest_id: u64) -> Self {
        Self {
            chest_id,
            slots: Vec::new(),
        }
    }

    /// Add a slot to the packet.
    pub fn add_slot(&mut self, slot: usize, item_type: String, quantity: u32) {
        self.slots.push((slot, item_type, quantity));
    }

    /// Convert slots to item stacks.
    #[must_use]
    pub fn to_item_stacks(&self) -> Vec<(usize, ItemStack)> {
        self.slots
            .iter()
            .map(|(slot, item_type, quantity)| (*slot, ItemStack::new(item_type.clone(), *quantity)))
            .collect()
    }
}

/// Knowledge state packet for network sync.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeStatePacket {
    /// List of discovered items (category as u8, id).
    pub discoveries: Vec<(u8, u32)>,
}

impl KnowledgeStatePacket {
    /// Create a new knowledge state packet.
    #[must_use]
    pub fn new() -> Self {
        Self {
            discoveries: Vec::new(),
        }
    }

    /// Add a discovery to the packet.
    pub fn add_discovery(&mut self, category: KnowledgeCategory, id: u32) {
        self.discoveries.push((category_to_u8(category), id));
    }

    /// Convert to discovery IDs.
    #[must_use]
    pub fn to_discovery_ids(&self) -> Vec<DiscoveryID> {
        self.discoveries
            .iter()
            .map(|(cat, id)| DiscoveryID::new(u8_to_category(*cat), *id))
            .collect()
    }
}

impl Default for KnowledgeStatePacket {
    fn default() -> Self {
        Self::new()
    }
}

/// Full persistent state packet for network sync.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PersistentStatePacket {
    /// Chest states.
    pub chests: Vec<ChestStatePacket>,
    /// Knowledge state.
    pub knowledge: KnowledgeStatePacket,
    /// Message count (messages stored persistently).
    pub message_count: u32,
}

impl PersistentStatePacket {
    /// Create a new persistent state packet.
    #[must_use]
    pub fn new() -> Self {
        Self {
            chests: Vec::new(),
            knowledge: KnowledgeStatePacket::new(),
            message_count: 0,
        }
    }

    /// Add a chest state.
    pub fn add_chest(&mut self, chest: ChestStatePacket) {
        self.chests.push(chest);
    }
}

/// Serialize persistent state to bytes.
#[must_use]
pub fn serialize_persistent_state(packet: &PersistentStatePacket) -> Vec<u8> {
    bincode::serialize(packet).unwrap_or_default()
}

/// Deserialize persistent state from bytes.
#[must_use]
pub fn deserialize_persistent_state(data: &[u8]) -> Option<PersistentStatePacket> {
    bincode::deserialize(data).ok()
}

/// Synchronization handler for persistent state.
#[derive(Clone, Debug, Default)]
pub struct PersistentSync {
    /// Current state packet.
    current: PersistentStatePacket,
    /// Sequence number for ordering.
    sequence: u32,
    /// Whether state has changed.
    dirty: bool,
}

impl PersistentSync {
    /// Create a new persistent sync handler.
    #[must_use]
    pub fn new() -> Self {
        Self {
            current: PersistentStatePacket::new(),
            sequence: 0,
            dirty: false,
        }
    }

    /// Get the current state packet.
    #[must_use]
    pub fn current(&self) -> &PersistentStatePacket {
        &self.current
    }

    /// Update from a network packet.
    ///
    /// Returns true if state was updated.
    pub fn update_from_network(&mut self, packet: PersistentStatePacket, sequence: u32) -> bool {
        if sequence > self.sequence {
            self.current = packet;
            self.sequence = sequence;
            self.dirty = true;
            true
        } else {
            false
        }
    }

    /// Set local state (for server).
    pub fn set_state(&mut self, packet: PersistentStatePacket) {
        self.current = packet;
        self.sequence += 1;
        self.dirty = true;
    }

    /// Get the current sequence number.
    #[must_use]
    pub fn sequence(&self) -> u32 {
        self.sequence
    }

    /// Check and clear dirty flag.
    pub fn take_dirty(&mut self) -> bool {
        let was_dirty = self.dirty;
        self.dirty = false;
        was_dirty
    }

    /// Get chest count.
    #[must_use]
    pub fn chest_count(&self) -> usize {
        self.current.chests.len()
    }

    /// Get discovery count.
    #[must_use]
    pub fn discovery_count(&self) -> usize {
        self.current.knowledge.discoveries.len()
    }
}

/// Convert knowledge category to u8.
#[must_use]
pub fn category_to_u8(category: KnowledgeCategory) -> u8 {
    match category {
        KnowledgeCategory::Map => 0,
        KnowledgeCategory::Recipe => 1,
        KnowledgeCategory::Trap => 2,
        KnowledgeCategory::Creature => 3,
        KnowledgeCategory::Route => 4,
    }
}

/// Convert u8 to knowledge category.
#[must_use]
pub fn u8_to_category(value: u8) -> KnowledgeCategory {
    match value {
        0 => KnowledgeCategory::Map,
        1 => KnowledgeCategory::Recipe,
        2 => KnowledgeCategory::Trap,
        3 => KnowledgeCategory::Creature,
        4 => KnowledgeCategory::Route,
        _ => KnowledgeCategory::Map,
    }
}

/// Convert state category to u8.
#[must_use]
pub fn state_category_to_u8(category: StateCategory) -> u8 {
    match category {
        StateCategory::Persistent => 0,
        StateCategory::SemiPersistent => 1,
        StateCategory::Volatile => 2,
    }
}

/// Convert u8 to state category.
#[must_use]
pub fn u8_to_state_category(value: u8) -> StateCategory {
    match value {
        0 => StateCategory::Persistent,
        1 => StateCategory::SemiPersistent,
        2 => StateCategory::Volatile,
        _ => StateCategory::Volatile,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chest_state_packet_new() {
        let packet = ChestStatePacket::new(123);
        assert_eq!(packet.chest_id, 123);
        assert!(packet.slots.is_empty());
    }

    #[test]
    fn test_chest_state_packet_add_slot() {
        let mut packet = ChestStatePacket::new(1);
        packet.add_slot(0, "diamond".to_string(), 5);
        packet.add_slot(3, "gold".to_string(), 10);

        assert_eq!(packet.slots.len(), 2);
        assert_eq!(packet.slots[0], (0, "diamond".to_string(), 5));
    }

    #[test]
    fn test_chest_state_packet_to_item_stacks() {
        let mut packet = ChestStatePacket::new(1);
        packet.add_slot(0, "iron".to_string(), 20);

        let stacks = packet.to_item_stacks();
        assert_eq!(stacks.len(), 1);
        assert_eq!(stacks[0].0, 0);
        assert_eq!(stacks[0].1.item_type, "iron");
        assert_eq!(stacks[0].1.quantity, 20);
    }

    #[test]
    fn test_knowledge_state_packet_new() {
        let packet = KnowledgeStatePacket::new();
        assert!(packet.discoveries.is_empty());
    }

    #[test]
    fn test_knowledge_state_packet_add_discovery() {
        let mut packet = KnowledgeStatePacket::new();
        packet.add_discovery(KnowledgeCategory::Recipe, 42);
        packet.add_discovery(KnowledgeCategory::Trap, 5);

        assert_eq!(packet.discoveries.len(), 2);
    }

    #[test]
    fn test_knowledge_state_packet_to_discovery_ids() {
        let mut packet = KnowledgeStatePacket::new();
        packet.add_discovery(KnowledgeCategory::Creature, 10);

        let ids = packet.to_discovery_ids();
        assert_eq!(ids.len(), 1);
        assert_eq!(ids[0].category, KnowledgeCategory::Creature);
        assert_eq!(ids[0].id, 10);
    }

    #[test]
    fn test_persistent_state_packet_new() {
        let packet = PersistentStatePacket::new();
        assert!(packet.chests.is_empty());
        assert!(packet.knowledge.discoveries.is_empty());
        assert_eq!(packet.message_count, 0);
    }

    #[test]
    fn test_serialize_deserialize_roundtrip() {
        let mut packet = PersistentStatePacket::new();
        let mut chest = ChestStatePacket::new(1);
        chest.add_slot(0, "key".to_string(), 1);
        packet.add_chest(chest);
        packet.knowledge.add_discovery(KnowledgeCategory::Map, 1);
        packet.message_count = 5;

        let data = serialize_persistent_state(&packet);
        let decoded = deserialize_persistent_state(&data).unwrap();

        assert_eq!(decoded.chests.len(), 1);
        assert_eq!(decoded.chests[0].chest_id, 1);
        assert_eq!(decoded.knowledge.discoveries.len(), 1);
        assert_eq!(decoded.message_count, 5);
    }

    #[test]
    fn test_persistent_sync_new() {
        let sync = PersistentSync::new();
        assert_eq!(sync.chest_count(), 0);
        assert_eq!(sync.discovery_count(), 0);
        assert_eq!(sync.sequence(), 0);
    }

    #[test]
    fn test_persistent_sync_update_from_network() {
        let mut sync = PersistentSync::new();
        let packet = PersistentStatePacket::new();

        let updated = sync.update_from_network(packet, 1);
        assert!(updated);
        assert_eq!(sync.sequence(), 1);
    }

    #[test]
    fn test_persistent_sync_ignores_old_sequence() {
        let mut sync = PersistentSync::new();
        sync.update_from_network(PersistentStatePacket::new(), 5);

        let updated = sync.update_from_network(PersistentStatePacket::new(), 3);
        assert!(!updated);
        assert_eq!(sync.sequence(), 5);
    }

    #[test]
    fn test_persistent_sync_dirty_flag() {
        let mut sync = PersistentSync::new();
        sync.update_from_network(PersistentStatePacket::new(), 1);

        assert!(sync.take_dirty());
        assert!(!sync.take_dirty());
    }

    #[test]
    fn test_category_conversion() {
        for cat in KnowledgeCategory::all() {
            assert_eq!(u8_to_category(category_to_u8(*cat)), *cat);
        }
    }

    #[test]
    fn test_state_category_conversion() {
        assert_eq!(
            u8_to_state_category(state_category_to_u8(StateCategory::Persistent)),
            StateCategory::Persistent
        );
        assert_eq!(
            u8_to_state_category(state_category_to_u8(StateCategory::SemiPersistent)),
            StateCategory::SemiPersistent
        );
        assert_eq!(
            u8_to_state_category(state_category_to_u8(StateCategory::Volatile)),
            StateCategory::Volatile
        );
    }
}
