//! Room types and properties for space station.
//!
//! Defines different room types and their characteristics.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::Hash;

/// Types of rooms in the space station.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RoomType {
    /// Command center - controls station operations.
    Command,
    /// Life support - maintains atmosphere.
    LifeSupport,
    /// Power core - generates electricity.
    PowerCore,
    /// Medical bay - treats injuries.
    Medical,
    /// Engineering - repairs and maintenance.
    Engineering,
    /// Cargo bay - storage for goods.
    Cargo,
    /// Hydroponics - grows food.
    Hydroponics,
    /// Airlock - entry/exit to space.
    Airlock,
    /// Storage - general storage.
    Storage,
    /// Crew quarters - living spaces.
    Quarters,
}

impl RoomType {
    /// Check if this room type is critical for station operation.
    #[must_use]
    pub fn is_critical(&self) -> bool {
        matches!(self, RoomType::Command | RoomType::LifeSupport | RoomType::PowerCore)
    }

    /// Get the base volume for this room type.
    #[must_use]
    pub fn base_volume(&self) -> f32 {
        match self {
            RoomType::Command => 200.0,
            RoomType::LifeSupport => 150.0,
            RoomType::PowerCore => 300.0,
            RoomType::Medical => 100.0,
            RoomType::Engineering => 250.0,
            RoomType::Cargo => 400.0,
            RoomType::Hydroponics => 200.0,
            RoomType::Airlock => 50.0,
            RoomType::Storage => 150.0,
            RoomType::Quarters => 80.0,
        }
    }

    /// Get the base power draw for this room type.
    #[must_use]
    pub fn base_power_draw(&self) -> f32 {
        match self {
            RoomType::Command => 15.0,
            RoomType::LifeSupport => 25.0,
            RoomType::PowerCore => 5.0,
            RoomType::Medical => 10.0,
            RoomType::Engineering => 12.0,
            RoomType::Cargo => 3.0,
            RoomType::Hydroponics => 8.0,
            RoomType::Airlock => 5.0,
            RoomType::Storage => 2.0,
            RoomType::Quarters => 4.0,
        }
    }

    /// Get all room types.
    #[must_use]
    pub fn all() -> &'static [RoomType] {
        &[
            RoomType::Command,
            RoomType::LifeSupport,
            RoomType::PowerCore,
            RoomType::Medical,
            RoomType::Engineering,
            RoomType::Cargo,
            RoomType::Hydroponics,
            RoomType::Airlock,
            RoomType::Storage,
            RoomType::Quarters,
        ]
    }
}

impl fmt::Display for RoomType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RoomType::Command => write!(f, "Command"),
            RoomType::LifeSupport => write!(f, "Life Support"),
            RoomType::PowerCore => write!(f, "Power Core"),
            RoomType::Medical => write!(f, "Medical"),
            RoomType::Engineering => write!(f, "Engineering"),
            RoomType::Cargo => write!(f, "Cargo"),
            RoomType::Hydroponics => write!(f, "Hydroponics"),
            RoomType::Airlock => write!(f, "Airlock"),
            RoomType::Storage => write!(f, "Storage"),
            RoomType::Quarters => write!(f, "Quarters"),
        }
    }
}

/// Properties for a room type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoomProperties {
    /// Type of room.
    pub room_type: RoomType,
    /// Base volume in cubic meters.
    pub base_volume: f32,
    /// Power draw in units.
    pub power_draw: f32,
    /// Whether this room is critical for station operation.
    pub is_critical: bool,
}

impl RoomProperties {
    /// Create properties from a room type.
    #[must_use]
    pub fn from_type(room_type: RoomType) -> Self {
        Self {
            room_type,
            base_volume: room_type.base_volume(),
            power_draw: room_type.base_power_draw(),
            is_critical: room_type.is_critical(),
        }
    }
}

/// A room in the space station.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StationRoom {
    /// Unique room ID.
    id: usize,
    /// Type of room.
    room_type: RoomType,
    /// Name of the room.
    name: String,
    /// Volume in cubic meters.
    volume: f32,
    /// Hull integrity (0-100).
    hull_integrity: f32,
    /// Whether the room is powered.
    powered: bool,
}

impl StationRoom {
    /// Create a new station room.
    #[must_use]
    pub fn new(id: usize, room_type: RoomType, name: String) -> Self {
        Self {
            id,
            room_type,
            name,
            volume: room_type.base_volume(),
            hull_integrity: 100.0,
            powered: true,
        }
    }

    /// Get the room ID.
    #[must_use]
    pub fn id(&self) -> usize {
        self.id
    }

    /// Get the room type.
    #[must_use]
    pub fn room_type(&self) -> RoomType {
        self.room_type
    }

    /// Get the room name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the room volume.
    #[must_use]
    pub fn volume(&self) -> f32 {
        self.volume
    }

    /// Get the hull integrity.
    #[must_use]
    pub fn hull_integrity(&self) -> f32 {
        self.hull_integrity
    }

    /// Check if the room is powered.
    #[must_use]
    pub fn is_powered(&self) -> bool {
        self.powered
    }

    /// Set the powered state.
    pub fn set_powered(&mut self, powered: bool) {
        self.powered = powered;
    }

    /// Check if the hull is breached (integrity below 50%).
    #[must_use]
    pub fn is_breached(&self) -> bool {
        self.hull_integrity < 50.0
    }

    /// Damage the hull.
    pub fn damage_hull(&mut self, amount: f32) {
        self.hull_integrity = (self.hull_integrity - amount).max(0.0);
    }

    /// Patch the hull (restore integrity).
    pub fn patch_hull(&mut self, amount: f32) {
        self.hull_integrity = (self.hull_integrity + amount).min(100.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_type_is_critical() {
        assert!(RoomType::Command.is_critical());
        assert!(RoomType::LifeSupport.is_critical());
        assert!(RoomType::PowerCore.is_critical());
        assert!(!RoomType::Medical.is_critical());
        assert!(!RoomType::Cargo.is_critical());
    }

    #[test]
    fn test_room_type_base_volume() {
        assert!((RoomType::Command.base_volume() - 200.0).abs() < f32::EPSILON);
        assert!((RoomType::Airlock.base_volume() - 50.0).abs() < f32::EPSILON);
        assert!((RoomType::Cargo.base_volume() - 400.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_room_type_base_power_draw() {
        assert!((RoomType::LifeSupport.base_power_draw() - 25.0).abs() < f32::EPSILON);
        assert!((RoomType::Storage.base_power_draw() - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_room_type_display() {
        assert_eq!(format!("{}", RoomType::Command), "Command");
        assert_eq!(format!("{}", RoomType::LifeSupport), "Life Support");
        assert_eq!(format!("{}", RoomType::PowerCore), "Power Core");
    }

    #[test]
    fn test_room_type_all() {
        let all = RoomType::all();
        assert_eq!(all.len(), 10);
        assert!(all.contains(&RoomType::Command));
        assert!(all.contains(&RoomType::Quarters));
    }

    #[test]
    fn test_room_properties_from_type() {
        let props = RoomProperties::from_type(RoomType::Medical);
        assert_eq!(props.room_type, RoomType::Medical);
        assert!((props.base_volume - 100.0).abs() < f32::EPSILON);
        assert!((props.power_draw - 10.0).abs() < f32::EPSILON);
        assert!(!props.is_critical);
    }

    #[test]
    fn test_station_room_new() {
        let room = StationRoom::new(0, RoomType::Engineering, "Main Engineering".to_string());
        assert_eq!(room.id(), 0);
        assert_eq!(room.room_type(), RoomType::Engineering);
        assert_eq!(room.name(), "Main Engineering");
        assert!((room.volume() - 250.0).abs() < f32::EPSILON);
        assert!((room.hull_integrity() - 100.0).abs() < f32::EPSILON);
        assert!(room.is_powered());
    }

    #[test]
    fn test_station_room_is_breached() {
        let mut room = StationRoom::new(0, RoomType::Cargo, "Cargo Bay".to_string());
        assert!(!room.is_breached());

        room.damage_hull(60.0);
        assert!(room.is_breached());
    }

    #[test]
    fn test_station_room_damage_hull() {
        let mut room = StationRoom::new(0, RoomType::Storage, "Storage A".to_string());
        room.damage_hull(30.0);
        assert!((room.hull_integrity() - 70.0).abs() < f32::EPSILON);

        room.damage_hull(100.0);
        assert!((room.hull_integrity() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_station_room_patch_hull() {
        let mut room = StationRoom::new(0, RoomType::Quarters, "Crew Quarters".to_string());
        room.damage_hull(50.0);
        room.patch_hull(30.0);
        assert!((room.hull_integrity() - 80.0).abs() < f32::EPSILON);

        room.patch_hull(100.0);
        assert!((room.hull_integrity() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_station_room_set_powered() {
        let mut room = StationRoom::new(0, RoomType::Medical, "Medbay".to_string());
        assert!(room.is_powered());

        room.set_powered(false);
        assert!(!room.is_powered());

        room.set_powered(true);
        assert!(room.is_powered());
    }
}
