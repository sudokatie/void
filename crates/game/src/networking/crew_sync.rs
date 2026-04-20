//! Crew state synchronization for multiplayer.
//!
//! Serializes and deserializes crew member state for network transmission.

use glam::Vec3;
use serde::{Deserialize, Serialize};

/// Crew role assignment.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrewRole {
    /// Captain - coordinates crew, can override bulkhead seals.
    Captain,
    /// Engineer - repairs hull breaches, manages power grid.
    Engineer,
    /// Medic - treats injuries, manages medical bay.
    Medic,
    /// Pilot - navigates station, manages EVA operations.
    Pilot,
}

impl CrewRole {
    /// Get the role name as a string.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            CrewRole::Captain => "Captain",
            CrewRole::Engineer => "Engineer",
            CrewRole::Medic => "Medic",
            CrewRole::Pilot => "Pilot",
        }
    }

    /// Get the priority level for role-based UI ordering.
    #[must_use]
    pub fn priority(&self) -> u8 {
        match self {
            CrewRole::Captain => 0,
            CrewRole::Engineer => 1,
            CrewRole::Medic => 2,
            CrewRole::Pilot => 3,
        }
    }

    /// Check if this role can override bulkhead seals.
    #[must_use]
    pub fn can_seal_bulkhead(&self) -> bool {
        matches!(self, CrewRole::Captain | CrewRole::Engineer)
    }

    /// Check if this role can operate the medical bay.
    #[must_use]
    pub fn can_operate_medbay(&self) -> bool {
        matches!(self, CrewRole::Medic | CrewRole::Captain)
    }

    /// Check if this role can authorize EVA.
    #[must_use]
    pub fn can_authorize_eva(&self) -> bool {
        matches!(self, CrewRole::Captain | CrewRole::Pilot)
    }

    /// Check if this role can reroute power.
    #[must_use]
    pub fn can_reroute_power(&self) -> bool {
        matches!(self, CrewRole::Engineer | CrewRole::Captain)
    }
}

impl std::fmt::Display for CrewRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Default for CrewRole {
    fn default() -> Self {
        CrewRole::Pilot
    }
}

/// Crew member position and orientation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CrewPosition {
    /// Position in world space.
    pub position: Vec3,
    /// Yaw rotation (radians).
    pub yaw: f32,
    /// Pitch rotation (radians).
    pub pitch: f32,
}

impl Default for CrewPosition {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

/// Crew member vital signs.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CrewVitals {
    /// Health (0-100).
    pub health: f32,
    /// Suit O2 (0-100).
    pub suit_o2: f32,
    /// Hunger (0-100).
    pub hunger: f32,
    /// Thirst (0-100).
    pub thirst: f32,
}

impl Default for CrewVitals {
    fn default() -> Self {
        Self {
            health: 100.0,
            suit_o2: 100.0,
            hunger: 100.0,
            thirst: 100.0,
        }
    }
}

/// Crew member equipment state.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CrewEquipment {
    /// Held item ID (if any).
    pub held_item: Option<u32>,
    /// Helmet equipped.
    pub helmet_on: bool,
    /// EVA suit equipped.
    pub suit_equipped: bool,
    /// Tether deployed.
    pub tether_deployed: bool,
}

/// Full crew member state for synchronization.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CrewState {
    /// Player ID.
    pub player_id: u64,
    /// Player name.
    pub name: String,
    /// Crew role.
    pub role: CrewRole,
    /// Position and orientation.
    pub position: CrewPosition,
    /// Vital signs.
    pub vitals: CrewVitals,
    /// Equipment state.
    pub equipment: CrewEquipment,
    /// Currently in EVA.
    pub in_eva: bool,
    /// Current room ID (None if in vacuum).
    pub current_room: Option<usize>,
    /// Sequence number for ordering.
    pub sequence: u32,
}

impl CrewState {
    /// Create a new crew state.
    #[must_use]
    pub fn new(player_id: u64, name: String) -> Self {
        Self {
            player_id,
            name,
            role: CrewRole::Pilot,
            position: CrewPosition::default(),
            vitals: CrewVitals::default(),
            equipment: CrewEquipment::default(),
            in_eva: false,
            current_room: None,
            sequence: 0,
        }
    }

    /// Create a new crew state with a specific role.
    #[must_use]
    pub fn with_role(player_id: u64, name: String, role: CrewRole) -> Self {
        Self {
            player_id,
            name,
            role,
            position: CrewPosition::default(),
            vitals: CrewVitals::default(),
            equipment: CrewEquipment::default(),
            in_eva: false,
            current_room: None,
            sequence: 0,
        }
    }

    /// Assign a new role.
    pub fn assign_role(&mut self, role: CrewRole) {
        self.role = role;
        self.sequence += 1;
    }

    /// Update position.
    pub fn update_position(&mut self, pos: Vec3, yaw: f32, pitch: f32) {
        self.position.position = pos;
        self.position.yaw = yaw;
        self.position.pitch = pitch;
        self.sequence += 1;
    }

    /// Update vitals.
    pub fn update_vitals(&mut self, health: f32, suit_o2: f32, hunger: f32, thirst: f32) {
        self.vitals.health = health.clamp(0.0, 100.0);
        self.vitals.suit_o2 = suit_o2.clamp(0.0, 100.0);
        self.vitals.hunger = hunger.clamp(0.0, 100.0);
        self.vitals.thirst = thirst.clamp(0.0, 100.0);
    }

    /// Check if crew member is alive.
    #[must_use]
    pub fn is_alive(&self) -> bool {
        self.vitals.health > 0.0
    }

    /// Check if crew member needs help.
    #[must_use]
    pub fn needs_help(&self) -> bool {
        self.vitals.health < 25.0 || self.vitals.suit_o2 < 10.0
    }
}

/// Manages crew state synchronization.
#[derive(Clone, Debug, Default)]
pub struct CrewSync {
    /// Local crew states by player ID.
    crew_states: std::collections::HashMap<u64, CrewState>,
    /// Last sent sequence number.
    last_sent_sequence: u32,
}

impl CrewSync {
    /// Create a new crew sync manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            crew_states: std::collections::HashMap::new(),
            last_sent_sequence: 0,
        }
    }

    /// Register a new crew member.
    pub fn register(&mut self, player_id: u64, name: String) {
        self.crew_states
            .insert(player_id, CrewState::new(player_id, name));
    }

    /// Unregister a crew member.
    pub fn unregister(&mut self, player_id: u64) -> bool {
        self.crew_states.remove(&player_id).is_some()
    }

    /// Get a crew member's state.
    #[must_use]
    pub fn get(&self, player_id: u64) -> Option<&CrewState> {
        self.crew_states.get(&player_id)
    }

    /// Get a mutable crew member's state.
    pub fn get_mut(&mut self, player_id: u64) -> Option<&mut CrewState> {
        self.crew_states.get_mut(&player_id)
    }

    /// Serialize a crew state for transmission.
    #[must_use]
    pub fn serialize(state: &CrewState) -> Vec<u8> {
        bincode::serialize(state).unwrap_or_default()
    }

    /// Deserialize a crew state from received data.
    #[must_use]
    pub fn deserialize(data: &[u8]) -> Option<CrewState> {
        bincode::deserialize(data).ok()
    }

    /// Apply a received crew state update.
    pub fn apply_update(&mut self, state: CrewState) {
        if let Some(existing) = self.crew_states.get_mut(&state.player_id) {
            // Only apply if newer
            if state.sequence > existing.sequence {
                *existing = state;
            }
        } else {
            self.crew_states.insert(state.player_id, state);
        }
    }

    /// Get all crew states.
    #[must_use]
    pub fn all_crew(&self) -> Vec<&CrewState> {
        self.crew_states.values().collect()
    }

    /// Get crew count.
    #[must_use]
    pub fn crew_count(&self) -> usize {
        self.crew_states.len()
    }

    /// Get crew members in a specific room.
    #[must_use]
    pub fn crew_in_room(&self, room_id: usize) -> Vec<&CrewState> {
        self.crew_states
            .values()
            .filter(|s| s.current_room == Some(room_id))
            .collect()
    }

    /// Get crew members in EVA.
    #[must_use]
    pub fn crew_in_eva(&self) -> Vec<&CrewState> {
        self.crew_states.values().filter(|s| s.in_eva).collect()
    }

    /// Get crew members needing help.
    #[must_use]
    pub fn crew_needing_help(&self) -> Vec<&CrewState> {
        self.crew_states
            .values()
            .filter(|s| s.needs_help())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crew_position_default() {
        let pos = CrewPosition::default();
        assert_eq!(pos.position, Vec3::ZERO);
        assert!((pos.yaw - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_crew_vitals_default() {
        let vitals = CrewVitals::default();
        assert!((vitals.health - 100.0).abs() < f32::EPSILON);
        assert!((vitals.suit_o2 - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_crew_state_new() {
        let state = CrewState::new(1, "Player1".to_string());
        assert_eq!(state.player_id, 1);
        assert_eq!(state.name, "Player1");
        assert!(state.is_alive());
    }

    #[test]
    fn test_crew_state_update_position() {
        let mut state = CrewState::new(1, "Player1".to_string());
        state.update_position(Vec3::new(10.0, 5.0, 3.0), 1.5, 0.5);

        assert_eq!(state.position.position, Vec3::new(10.0, 5.0, 3.0));
        assert!((state.position.yaw - 1.5).abs() < f32::EPSILON);
        assert_eq!(state.sequence, 1);
    }

    #[test]
    fn test_crew_state_update_vitals() {
        let mut state = CrewState::new(1, "Player1".to_string());
        state.update_vitals(80.0, 50.0, 70.0, 60.0);

        assert!((state.vitals.health - 80.0).abs() < f32::EPSILON);
        assert!((state.vitals.suit_o2 - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_crew_state_vitals_clamped() {
        let mut state = CrewState::new(1, "Player1".to_string());
        state.update_vitals(150.0, -10.0, 100.0, 100.0);

        assert!((state.vitals.health - 100.0).abs() < f32::EPSILON);
        assert!((state.vitals.suit_o2 - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_crew_state_is_alive() {
        let mut state = CrewState::new(1, "Player1".to_string());
        assert!(state.is_alive());

        state.update_vitals(0.0, 100.0, 100.0, 100.0);
        assert!(!state.is_alive());
    }

    #[test]
    fn test_crew_state_needs_help() {
        let mut state = CrewState::new(1, "Player1".to_string());
        assert!(!state.needs_help());

        state.update_vitals(20.0, 100.0, 100.0, 100.0);
        assert!(state.needs_help());

        state.update_vitals(100.0, 5.0, 100.0, 100.0);
        assert!(state.needs_help());
    }

    #[test]
    fn test_crew_sync_new() {
        let sync = CrewSync::new();
        assert_eq!(sync.crew_count(), 0);
    }

    #[test]
    fn test_crew_sync_register() {
        let mut sync = CrewSync::new();
        sync.register(1, "Player1".to_string());

        assert_eq!(sync.crew_count(), 1);
        assert!(sync.get(1).is_some());
    }

    #[test]
    fn test_crew_sync_unregister() {
        let mut sync = CrewSync::new();
        sync.register(1, "Player1".to_string());
        assert!(sync.unregister(1));
        assert_eq!(sync.crew_count(), 0);
    }

    #[test]
    fn test_crew_sync_get_mut() {
        let mut sync = CrewSync::new();
        sync.register(1, "Player1".to_string());

        if let Some(state) = sync.get_mut(1) {
            state.in_eva = true;
        }

        assert!(sync.get(1).unwrap().in_eva);
    }

    #[test]
    fn test_crew_sync_serialize_deserialize() {
        let state = CrewState::new(1, "Player1".to_string());
        let data = CrewSync::serialize(&state);
        let restored = CrewSync::deserialize(&data).unwrap();

        assert_eq!(restored.player_id, 1);
        assert_eq!(restored.name, "Player1");
    }

    #[test]
    fn test_crew_sync_apply_update() {
        let mut sync = CrewSync::new();
        sync.register(1, "Player1".to_string());

        let mut update = CrewState::new(1, "Player1".to_string());
        update.sequence = 5;
        update.position.position = Vec3::new(10.0, 0.0, 0.0);

        sync.apply_update(update);
        assert_eq!(sync.get(1).unwrap().position.position, Vec3::new(10.0, 0.0, 0.0));
    }

    #[test]
    fn test_crew_sync_apply_update_ignores_old() {
        let mut sync = CrewSync::new();
        sync.register(1, "Player1".to_string());
        sync.get_mut(1).unwrap().sequence = 10;

        let mut update = CrewState::new(1, "Player1".to_string());
        update.sequence = 5; // Older than current
        update.position.position = Vec3::new(10.0, 0.0, 0.0);

        sync.apply_update(update);
        assert_eq!(sync.get(1).unwrap().position.position, Vec3::ZERO); // Not updated
    }

    #[test]
    fn test_crew_sync_all_crew() {
        let mut sync = CrewSync::new();
        sync.register(1, "Player1".to_string());
        sync.register(2, "Player2".to_string());

        assert_eq!(sync.all_crew().len(), 2);
    }

    #[test]
    fn test_crew_sync_crew_in_room() {
        let mut sync = CrewSync::new();
        sync.register(1, "Player1".to_string());
        sync.register(2, "Player2".to_string());

        sync.get_mut(1).unwrap().current_room = Some(0);
        sync.get_mut(2).unwrap().current_room = Some(1);

        assert_eq!(sync.crew_in_room(0).len(), 1);
        assert_eq!(sync.crew_in_room(0)[0].player_id, 1);
    }

    #[test]
    fn test_crew_sync_crew_in_eva() {
        let mut sync = CrewSync::new();
        sync.register(1, "Player1".to_string());
        sync.register(2, "Player2".to_string());

        sync.get_mut(1).unwrap().in_eva = true;

        assert_eq!(sync.crew_in_eva().len(), 1);
    }

    #[test]
    fn test_crew_sync_crew_needing_help() {
        let mut sync = CrewSync::new();
        sync.register(1, "Player1".to_string());
        sync.register(2, "Player2".to_string());

        sync.get_mut(1).unwrap().vitals.health = 20.0;

        assert_eq!(sync.crew_needing_help().len(), 1);
    }

    #[test]
    fn test_crew_role_name() {
        assert_eq!(CrewRole::Captain.name(), "Captain");
        assert_eq!(CrewRole::Engineer.name(), "Engineer");
        assert_eq!(CrewRole::Medic.name(), "Medic");
        assert_eq!(CrewRole::Pilot.name(), "Pilot");
    }

    #[test]
    fn test_crew_role_display() {
        assert_eq!(format!("{}", CrewRole::Captain), "Captain");
    }

    #[test]
    fn test_crew_role_priority() {
        assert!(CrewRole::Captain.priority() < CrewRole::Engineer.priority());
        assert!(CrewRole::Engineer.priority() < CrewRole::Medic.priority());
        assert!(CrewRole::Medic.priority() < CrewRole::Pilot.priority());
    }

    #[test]
    fn test_crew_role_permissions() {
        assert!(CrewRole::Captain.can_seal_bulkhead());
        assert!(CrewRole::Engineer.can_seal_bulkhead());
        assert!(!CrewRole::Medic.can_seal_bulkhead());
        assert!(!CrewRole::Pilot.can_seal_bulkhead());

        assert!(CrewRole::Captain.can_operate_medbay());
        assert!(CrewRole::Medic.can_operate_medbay());
        assert!(!CrewRole::Engineer.can_operate_medbay());

        assert!(CrewRole::Captain.can_authorize_eva());
        assert!(CrewRole::Pilot.can_authorize_eva());
        assert!(!CrewRole::Engineer.can_authorize_eva());

        assert!(CrewRole::Captain.can_reroute_power());
        assert!(CrewRole::Engineer.can_reroute_power());
        assert!(!CrewRole::Medic.can_reroute_power());
    }

    #[test]
    fn test_crew_state_with_role() {
        let state = CrewState::with_role(1, "Engineer1".to_string(), CrewRole::Engineer);
        assert_eq!(state.role, CrewRole::Engineer);
    }

    #[test]
    fn test_crew_state_assign_role() {
        let mut state = CrewState::new(1, "Player1".to_string());
        assert_eq!(state.role, CrewRole::Pilot); // default

        state.assign_role(CrewRole::Captain);
        assert_eq!(state.role, CrewRole::Captain);
        assert_eq!(state.sequence, 1);
    }

    #[test]
    fn test_crew_role_default() {
        assert_eq!(CrewRole::default(), CrewRole::Pilot);
    }

    #[test]
    fn test_crew_role_serialization() {
        let state = CrewState::with_role(1, "Captain1".to_string(), CrewRole::Captain);
        let data = CrewSync::serialize(&state);
        let restored = CrewSync::deserialize(&data).unwrap();
        assert_eq!(restored.role, CrewRole::Captain);
    }

    #[test]
    fn test_crew_role_equality() {
        assert_eq!(CrewRole::Captain, CrewRole::Captain);
        assert_ne!(CrewRole::Captain, CrewRole::Engineer);
    }
}
