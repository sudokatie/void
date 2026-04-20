//! Gas flow model for connected rooms.
//!
//! Simulates gas flow between connected rooms and vacuum breaches.

use engine_physics::vacuum::RoomAtmosphereSim;
use serde::{Deserialize, Serialize};

use super::{DecompressionEvent, DecompressionType};

/// Critical pressure threshold for generating events.
const CRITICAL_PRESSURE: f32 = 50.0;

/// Model for gas flow between rooms.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GasModel {
    /// Room atmospheres.
    rooms: Vec<RoomAtmosphereSim>,
    /// Connections between rooms: (room_a, room_b, opening_size).
    connections: Vec<(usize, usize, f32)>,
    /// Active breaches: (room_id, breach_type).
    breaches: Vec<(usize, DecompressionType)>,
}

impl GasModel {
    /// Create a new empty gas model.
    #[must_use]
    pub fn new() -> Self {
        Self {
            rooms: Vec::new(),
            connections: Vec::new(),
            breaches: Vec::new(),
        }
    }

    /// Add a room and return its ID.
    pub fn add_room(&mut self, atmosphere: RoomAtmosphereSim) -> usize {
        let id = self.rooms.len();
        self.rooms.push(atmosphere);
        id
    }

    /// Connect two rooms with an opening.
    pub fn connect_rooms(&mut self, a: usize, b: usize, opening: f32) {
        if a < self.rooms.len() && b < self.rooms.len() && a != b {
            self.connections.push((a, b, opening));
        }
    }

    /// Get a reference to a room's atmosphere.
    #[must_use]
    pub fn get_room(&self, id: usize) -> Option<&RoomAtmosphereSim> {
        self.rooms.get(id)
    }

    /// Get a mutable reference to a room's atmosphere.
    pub fn get_room_mut(&mut self, id: usize) -> Option<&mut RoomAtmosphereSim> {
        self.rooms.get_mut(id)
    }

    /// Get the number of rooms.
    #[must_use]
    pub fn room_count(&self) -> usize {
        self.rooms.len()
    }

    /// Trigger a breach in a room.
    pub fn trigger_breach(&mut self, room_id: usize, breach_type: DecompressionType) -> DecompressionEvent {
        if room_id < self.rooms.len() {
            self.breaches.push((room_id, breach_type));
        }
        DecompressionEvent::new(room_id, breach_type)
    }

    /// Seal a breach in a room.
    pub fn seal_breach(&mut self, room_id: usize) -> bool {
        let initial_len = self.breaches.len();
        self.breaches.retain(|(id, _)| *id != room_id);
        self.breaches.len() < initial_len
    }

    /// Tick the gas model, processing flow and breaches.
    ///
    /// Returns any new decompression events (rooms becoming critical).
    pub fn tick(&mut self, dt: f32) -> Vec<DecompressionEvent> {
        let mut events = Vec::new();

        // Process gas flow between connected rooms
        for &(a, b, opening) in &self.connections.clone() {
            if a < self.rooms.len() && b < self.rooms.len() {
                // Split borrow workaround
                let (left, right) = self.rooms.split_at_mut(a.max(b));
                if a < b {
                    left[a].flow_from(&mut right[0], opening, dt);
                } else {
                    right[0].flow_from(&mut left[b], opening, dt);
                }
            }
        }

        // Process breaches
        for &(room_id, breach_type) in &self.breaches.clone() {
            if let Some(room) = self.rooms.get_mut(room_id) {
                let initial_pressure = room.pressure;
                room.vent_to_vacuum(breach_type.breach_size(), dt);

                // Generate event if room just became critical
                if initial_pressure >= CRITICAL_PRESSURE && room.pressure < CRITICAL_PRESSURE {
                    events.push(DecompressionEvent::new(room_id, breach_type));
                }
            }
        }

        events
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gas_model_new() {
        let model = GasModel::new();
        assert_eq!(model.room_count(), 0);
    }

    #[test]
    fn test_gas_model_add_room() {
        let mut model = GasModel::new();
        let id = model.add_room(RoomAtmosphereSim::new());
        assert_eq!(id, 0);
        assert_eq!(model.room_count(), 1);
    }

    #[test]
    fn test_gas_model_multiple_rooms() {
        let mut model = GasModel::new();
        let id1 = model.add_room(RoomAtmosphereSim::new());
        let id2 = model.add_room(RoomAtmosphereSim::with_volume(200.0));
        assert_eq!(id1, 0);
        assert_eq!(id2, 1);
        assert_eq!(model.room_count(), 2);
    }

    #[test]
    fn test_gas_model_get_room() {
        let mut model = GasModel::new();
        model.add_room(RoomAtmosphereSim::with_volume(150.0));

        let room = model.get_room(0);
        assert!(room.is_some());
        assert!((room.unwrap().volume - 150.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_gas_model_get_room_invalid() {
        let model = GasModel::new();
        assert!(model.get_room(0).is_none());
    }

    #[test]
    fn test_gas_model_get_room_mut() {
        let mut model = GasModel::new();
        model.add_room(RoomAtmosphereSim::new());

        if let Some(room) = model.get_room_mut(0) {
            room.pressure = 50.0;
        }

        assert!((model.get_room(0).unwrap().pressure - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_gas_model_connect_rooms() {
        let mut model = GasModel::new();
        model.add_room(RoomAtmosphereSim::new());
        model.add_room(RoomAtmosphereSim::new());
        model.connect_rooms(0, 1, 1.0);

        assert_eq!(model.connections.len(), 1);
    }

    #[test]
    fn test_gas_model_connect_invalid_rooms() {
        let mut model = GasModel::new();
        model.add_room(RoomAtmosphereSim::new());
        model.connect_rooms(0, 5, 1.0);
        model.connect_rooms(0, 0, 1.0);

        assert_eq!(model.connections.len(), 0);
    }

    #[test]
    fn test_gas_model_trigger_breach() {
        let mut model = GasModel::new();
        model.add_room(RoomAtmosphereSim::new());

        let event = model.trigger_breach(0, DecompressionType::Rapid);
        assert_eq!(event.room_id, 0);
        assert_eq!(event.event_type, DecompressionType::Rapid);
        assert_eq!(model.breaches.len(), 1);
    }

    #[test]
    fn test_gas_model_seal_breach() {
        let mut model = GasModel::new();
        model.add_room(RoomAtmosphereSim::new());
        model.trigger_breach(0, DecompressionType::Slow);

        assert!(model.seal_breach(0));
        assert_eq!(model.breaches.len(), 0);
    }

    #[test]
    fn test_gas_model_seal_nonexistent_breach() {
        let mut model = GasModel::new();
        model.add_room(RoomAtmosphereSim::new());

        assert!(!model.seal_breach(0));
    }

    #[test]
    fn test_gas_model_tick_breach_vents() {
        let mut model = GasModel::new();
        model.add_room(RoomAtmosphereSim::new());
        model.trigger_breach(0, DecompressionType::Rapid);

        let initial_pressure = model.get_room(0).unwrap().pressure;
        model.tick(1.0);

        assert!(model.get_room(0).unwrap().pressure < initial_pressure);
    }

    #[test]
    fn test_gas_model_tick_flow_between_rooms() {
        let mut model = GasModel::new();

        let mut room1 = RoomAtmosphereSim::new();
        room1.pressure = 120.0;
        model.add_room(room1);

        let mut room2 = RoomAtmosphereSim::new();
        room2.pressure = 80.0;
        model.add_room(room2);

        model.connect_rooms(0, 1, 2.0);
        model.tick(1.0);

        // Pressures should move toward equilibrium
        let p1 = model.get_room(0).unwrap().pressure;
        let p2 = model.get_room(1).unwrap().pressure;
        assert!(p1 < 120.0);
        assert!(p2 > 80.0);
    }

    #[test]
    fn test_gas_model_default() {
        let model = GasModel::default();
        assert_eq!(model.room_count(), 0);
    }
}
