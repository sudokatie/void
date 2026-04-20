//! Station state synchronization for multiplayer.
//!
//! Serializes and deserializes atmosphere, hull, and power state for network transmission.

use serde::{Deserialize, Serialize};

/// Room atmosphere state for sync.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoomAtmosphereState {
    /// Room ID.
    pub room_id: usize,
    /// Oxygen percentage.
    pub o2_percent: f32,
    /// Pressure (kPa).
    pub pressure: f32,
    /// Temperature (Celsius).
    pub temperature: f32,
    /// Whether room is sealed.
    pub sealed: bool,
    /// Whether life support is active.
    pub life_support_active: bool,
}

impl RoomAtmosphereState {
    /// Create a new room atmosphere state.
    #[must_use]
    pub fn new(room_id: usize) -> Self {
        Self {
            room_id,
            o2_percent: 21.0,
            pressure: 101.3,
            temperature: 20.0,
            sealed: true,
            life_support_active: true,
        }
    }

    /// Check if atmosphere is breathable.
    #[must_use]
    pub fn is_breathable(&self) -> bool {
        self.o2_percent >= 16.0 && self.pressure >= 50.0 && self.sealed
    }
}

/// Hull segment state for sync.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HullSegmentState {
    /// Segment ID.
    pub segment_id: usize,
    /// Integrity percentage.
    pub integrity: f32,
    /// Whether segment is breached.
    pub breached: bool,
}

impl HullSegmentState {
    /// Create a new hull segment state.
    #[must_use]
    pub fn new(segment_id: usize) -> Self {
        Self {
            segment_id,
            integrity: 100.0,
            breached: false,
        }
    }

    /// Get damage level string.
    #[must_use]
    pub fn damage_level(&self) -> &'static str {
        if self.integrity >= 75.0 {
            "Intact"
        } else if self.integrity >= 50.0 {
            "Damaged"
        } else if self.integrity >= 25.0 {
            "Compromised"
        } else {
            "Critical"
        }
    }
}

/// Power system state for sync.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PowerSystemState {
    /// Reactor output.
    pub reactor_output: f32,
    /// Total consumption.
    pub total_consumption: f32,
    /// Battery charge percentage.
    pub battery_percent: f32,
    /// Whether reactor is online.
    pub reactor_online: bool,
    /// Systems that are powered off due to shortage.
    pub unpowered_systems: Vec<usize>,
}

impl PowerSystemState {
    /// Create a new power system state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            reactor_output: 100.0,
            total_consumption: 0.0,
            battery_percent: 100.0,
            reactor_online: true,
            unpowered_systems: Vec::new(),
        }
    }

    /// Check if power is sufficient.
    #[must_use]
    pub fn is_sufficient(&self) -> bool {
        self.reactor_output >= self.total_consumption || self.battery_percent > 0.0
    }

    /// Get power surplus/deficit.
    #[must_use]
    pub fn power_balance(&self) -> f32 {
        self.reactor_output - self.total_consumption
    }
}

impl Default for PowerSystemState {
    fn default() -> Self {
        Self::new()
    }
}

/// Full station state for synchronization.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StationState {
    /// Room atmosphere states.
    pub atmospheres: Vec<RoomAtmosphereState>,
    /// Hull segment states.
    pub hull_segments: Vec<HullSegmentState>,
    /// Power system state.
    pub power: PowerSystemState,
    /// Sequence number for ordering.
    pub sequence: u32,
    /// Server tick number.
    pub tick: u64,
}

impl StationState {
    /// Create a new station state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            atmospheres: Vec::new(),
            hull_segments: Vec::new(),
            power: PowerSystemState::new(),
            sequence: 0,
            tick: 0,
        }
    }

    /// Add a room atmosphere state.
    pub fn add_room(&mut self, state: RoomAtmosphereState) {
        self.atmospheres.push(state);
    }

    /// Add a hull segment state.
    pub fn add_hull_segment(&mut self, state: HullSegmentState) {
        self.hull_segments.push(state);
    }

    /// Get room count.
    #[must_use]
    pub fn room_count(&self) -> usize {
        self.atmospheres.len()
    }

    /// Get hull segment count.
    #[must_use]
    pub fn hull_segment_count(&self) -> usize {
        self.hull_segments.len()
    }

    /// Get a room's atmosphere state.
    #[must_use]
    pub fn get_room(&self, room_id: usize) -> Option<&RoomAtmosphereState> {
        self.atmospheres.iter().find(|r| r.room_id == room_id)
    }

    /// Get a hull segment's state.
    #[must_use]
    pub fn get_hull_segment(&self, segment_id: usize) -> Option<&HullSegmentState> {
        self.hull_segments.iter().find(|s| s.segment_id == segment_id)
    }

    /// Get breached rooms.
    #[must_use]
    pub fn breached_rooms(&self) -> Vec<&RoomAtmosphereState> {
        self.atmospheres.iter().filter(|r| !r.sealed).collect()
    }

    /// Get breached hull segments.
    #[must_use]
    pub fn breached_segments(&self) -> Vec<&HullSegmentState> {
        self.hull_segments.iter().filter(|s| s.breached).collect()
    }
}

impl Default for StationState {
    fn default() -> Self {
        Self::new()
    }
}

/// Manages station state synchronization.
#[derive(Clone, Debug, Default)]
pub struct StateSync {
    /// Current station state.
    state: StationState,
    /// Last received sequence.
    last_received_sequence: u32,
}

impl StateSync {
    /// Create a new state sync manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: StationState::new(),
            last_received_sequence: 0,
        }
    }

    /// Get current state.
    #[must_use]
    pub fn state(&self) -> &StationState {
        &self.state
    }

    /// Get mutable state.
    pub fn state_mut(&mut self) -> &mut StationState {
        &mut self.state
    }

    /// Serialize current state for transmission.
    #[must_use]
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self.state).unwrap_or_default()
    }

    /// Serialize a station state for transmission.
    #[must_use]
    pub fn serialize_state(state: &StationState) -> Vec<u8> {
        bincode::serialize(state).unwrap_or_default()
    }

    /// Deserialize a station state from received data.
    #[must_use]
    pub fn deserialize(data: &[u8]) -> Option<StationState> {
        bincode::deserialize(data).ok()
    }

    /// Apply a received state update.
    pub fn apply_update(&mut self, state: StationState) -> bool {
        if state.sequence > self.last_received_sequence {
            self.last_received_sequence = state.sequence;
            self.state = state;
            true
        } else {
            false
        }
    }

    /// Update room atmosphere state.
    pub fn update_room(&mut self, room_state: RoomAtmosphereState) {
        if let Some(existing) = self
            .state
            .atmospheres
            .iter_mut()
            .find(|r| r.room_id == room_state.room_id)
        {
            *existing = room_state;
        } else {
            self.state.atmospheres.push(room_state);
        }
        self.state.sequence += 1;
    }

    /// Update hull segment state.
    pub fn update_hull_segment(&mut self, segment_state: HullSegmentState) {
        if let Some(existing) = self
            .state
            .hull_segments
            .iter_mut()
            .find(|s| s.segment_id == segment_state.segment_id)
        {
            *existing = segment_state;
        } else {
            self.state.hull_segments.push(segment_state);
        }
        self.state.sequence += 1;
    }

    /// Update power state.
    pub fn update_power(&mut self, power_state: PowerSystemState) {
        self.state.power = power_state;
        self.state.sequence += 1;
    }

    /// Increment tick counter.
    pub fn tick(&mut self) {
        self.state.tick += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_atmosphere_state_new() {
        let state = RoomAtmosphereState::new(0);
        assert_eq!(state.room_id, 0);
        assert!((state.o2_percent - 21.0).abs() < f32::EPSILON);
        assert!(state.sealed);
    }

    #[test]
    fn test_room_atmosphere_state_breathable() {
        let state = RoomAtmosphereState::new(0);
        assert!(state.is_breathable());

        let mut low_o2 = RoomAtmosphereState::new(1);
        low_o2.o2_percent = 10.0;
        assert!(!low_o2.is_breathable());

        let mut unsealed = RoomAtmosphereState::new(2);
        unsealed.sealed = false;
        assert!(!unsealed.is_breathable());
    }

    #[test]
    fn test_hull_segment_state_new() {
        let state = HullSegmentState::new(0);
        assert_eq!(state.segment_id, 0);
        assert!((state.integrity - 100.0).abs() < f32::EPSILON);
        assert!(!state.breached);
    }

    #[test]
    fn test_hull_segment_state_damage_level() {
        let mut state = HullSegmentState::new(0);
        assert_eq!(state.damage_level(), "Intact");

        state.integrity = 60.0;
        assert_eq!(state.damage_level(), "Damaged");

        state.integrity = 40.0;
        assert_eq!(state.damage_level(), "Compromised");

        state.integrity = 10.0;
        assert_eq!(state.damage_level(), "Critical");
    }

    #[test]
    fn test_power_system_state_new() {
        let state = PowerSystemState::new();
        assert!((state.reactor_output - 100.0).abs() < f32::EPSILON);
        assert!(state.reactor_online);
        assert!(state.is_sufficient());
    }

    #[test]
    fn test_power_system_state_balance() {
        let mut state = PowerSystemState::new();
        state.total_consumption = 80.0;
        assert!((state.power_balance() - 20.0).abs() < f32::EPSILON);

        state.total_consumption = 120.0;
        assert!((state.power_balance() - -20.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_power_system_state_sufficient() {
        let mut state = PowerSystemState::new();
        state.total_consumption = 120.0;
        assert!(state.is_sufficient()); // Battery backup

        state.battery_percent = 0.0;
        assert!(!state.is_sufficient());
    }

    #[test]
    fn test_station_state_new() {
        let state = StationState::new();
        assert_eq!(state.room_count(), 0);
        assert_eq!(state.hull_segment_count(), 0);
    }

    #[test]
    fn test_station_state_add_room() {
        let mut state = StationState::new();
        state.add_room(RoomAtmosphereState::new(0));
        state.add_room(RoomAtmosphereState::new(1));

        assert_eq!(state.room_count(), 2);
        assert!(state.get_room(0).is_some());
        assert!(state.get_room(1).is_some());
    }

    #[test]
    fn test_station_state_add_hull_segment() {
        let mut state = StationState::new();
        state.add_hull_segment(HullSegmentState::new(0));

        assert_eq!(state.hull_segment_count(), 1);
        assert!(state.get_hull_segment(0).is_some());
    }

    #[test]
    fn test_station_state_breached_rooms() {
        let mut state = StationState::new();
        state.add_room(RoomAtmosphereState::new(0));
        let mut breached = RoomAtmosphereState::new(1);
        breached.sealed = false;
        state.add_room(breached);

        assert_eq!(state.breached_rooms().len(), 1);
    }

    #[test]
    fn test_station_state_breached_segments() {
        let mut state = StationState::new();
        state.add_hull_segment(HullSegmentState::new(0));
        let mut breached = HullSegmentState::new(1);
        breached.breached = true;
        state.add_hull_segment(breached);

        assert_eq!(state.breached_segments().len(), 1);
    }

    #[test]
    fn test_state_sync_new() {
        let sync = StateSync::new();
        assert_eq!(sync.state().room_count(), 0);
    }

    #[test]
    fn test_state_sync_serialize_deserialize() {
        let mut sync = StateSync::new();
        sync.state_mut().add_room(RoomAtmosphereState::new(0));

        let data = sync.serialize();
        let restored = StateSync::deserialize(&data).unwrap();

        assert_eq!(restored.room_count(), 1);
    }

    #[test]
    fn test_state_sync_apply_update() {
        let mut sync = StateSync::new();
        let mut state = StationState::new();
        state.sequence = 5;
        state.add_room(RoomAtmosphereState::new(0));

        assert!(sync.apply_update(state));
        assert_eq!(sync.state().room_count(), 1);
    }

    #[test]
    fn test_state_sync_apply_update_ignores_old() {
        let mut sync = StateSync::new();
        let mut state1 = StationState::new();
        state1.sequence = 10;
        sync.apply_update(state1);

        let mut state2 = StationState::new();
        state2.sequence = 5; // Older
        state2.add_room(RoomAtmosphereState::new(0));

        assert!(!sync.apply_update(state2));
        assert_eq!(sync.state().room_count(), 0); // Not updated
    }

    #[test]
    fn test_state_sync_update_room() {
        let mut sync = StateSync::new();
        sync.update_room(RoomAtmosphereState::new(0));
        assert_eq!(sync.state().room_count(), 1);

        // Update existing
        let mut updated = RoomAtmosphereState::new(0);
        updated.o2_percent = 18.0;
        sync.update_room(updated);

        assert_eq!(sync.state().room_count(), 1);
        assert!((sync.state().get_room(0).unwrap().o2_percent - 18.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_state_sync_update_hull_segment() {
        let mut sync = StateSync::new();
        sync.update_hull_segment(HullSegmentState::new(0));
        assert_eq!(sync.state().hull_segment_count(), 1);

        // Update existing
        let mut updated = HullSegmentState::new(0);
        updated.integrity = 80.0;
        sync.update_hull_segment(updated);

        assert_eq!(sync.state().hull_segment_count(), 1);
        assert!((sync.state().get_hull_segment(0).unwrap().integrity - 80.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_state_sync_update_power() {
        let mut sync = StateSync::new();
        let mut power = PowerSystemState::new();
        power.reactor_output = 80.0;
        sync.update_power(power);

        assert!((sync.state().power.reactor_output - 80.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_state_sync_tick() {
        let mut sync = StateSync::new();
        assert_eq!(sync.state().tick, 0);
        sync.tick();
        assert_eq!(sync.state().tick, 1);
    }

    #[test]
    fn test_state_sync_default() {
        let sync = StateSync::default();
        assert_eq!(sync.state().sequence, 0);
    }
}
