//! Random event system for space station survival.
//!
//! Provides various random events that can occur during gameplay.

use serde::{Deserialize, Serialize};

/// Types of random events that can occur.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RandomEventType {
    /// Small meteorites striking the station hull.
    MicrometeoriteShower,
    /// Intense solar radiation affecting electronics.
    SolarFlare,
    /// Electrical surge damaging systems.
    PowerSurge,
    /// Debris field approaching the station.
    DebrisField,
    /// Random system failure or malfunction.
    SystemMalfunction,
}

impl RandomEventType {
    /// Get display name for this event type.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            RandomEventType::MicrometeoriteShower => "Micrometeorite Shower",
            RandomEventType::SolarFlare => "Solar Flare",
            RandomEventType::PowerSurge => "Power Surge",
            RandomEventType::DebrisField => "Debris Field",
            RandomEventType::SystemMalfunction => "System Malfunction",
        }
    }

    /// Get description for this event type.
    #[must_use]
    pub fn description(&self) -> &'static str {
        match self {
            RandomEventType::MicrometeoriteShower => {
                "Small meteorites are impacting the station hull"
            }
            RandomEventType::SolarFlare => {
                "Intense solar radiation is affecting electronic systems"
            }
            RandomEventType::PowerSurge => {
                "An electrical surge is damaging power systems"
            }
            RandomEventType::DebrisField => {
                "A field of debris is approaching the station"
            }
            RandomEventType::SystemMalfunction => {
                "A critical system has malfunctioned"
            }
        }
    }

    /// Get severity level (1-5).
    #[must_use]
    pub fn severity(&self) -> u32 {
        match self {
            RandomEventType::MicrometeoriteShower => 3,
            RandomEventType::SolarFlare => 4,
            RandomEventType::PowerSurge => 2,
            RandomEventType::DebrisField => 4,
            RandomEventType::SystemMalfunction => 2,
        }
    }

    /// Get base duration in seconds.
    #[must_use]
    pub fn base_duration(&self) -> f32 {
        match self {
            RandomEventType::MicrometeoriteShower => 30.0,
            RandomEventType::SolarFlare => 60.0,
            RandomEventType::PowerSurge => 5.0,
            RandomEventType::DebrisField => 45.0,
            RandomEventType::SystemMalfunction => 0.0, // Instant
        }
    }

    /// Get warning time before event starts.
    #[must_use]
    pub fn warning_time(&self) -> f32 {
        match self {
            RandomEventType::MicrometeoriteShower => 10.0,
            RandomEventType::SolarFlare => 30.0,
            RandomEventType::PowerSurge => 2.0,
            RandomEventType::DebrisField => 60.0,
            RandomEventType::SystemMalfunction => 0.0,
        }
    }

    /// Check if this event can cause hull damage.
    #[must_use]
    pub fn can_damage_hull(&self) -> bool {
        matches!(
            self,
            RandomEventType::MicrometeoriteShower | RandomEventType::DebrisField
        )
    }

    /// Check if this event affects power systems.
    #[must_use]
    pub fn affects_power(&self) -> bool {
        matches!(
            self,
            RandomEventType::SolarFlare | RandomEventType::PowerSurge
        )
    }

    /// Get all random event types.
    #[must_use]
    pub fn all() -> &'static [RandomEventType] {
        &[
            RandomEventType::MicrometeoriteShower,
            RandomEventType::SolarFlare,
            RandomEventType::PowerSurge,
            RandomEventType::DebrisField,
            RandomEventType::SystemMalfunction,
        ]
    }
}

/// A random event instance.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RandomEvent {
    /// Type of event.
    event_type: RandomEventType,
    /// Remaining duration in seconds.
    remaining_duration: f32,
    /// Whether the event is currently active.
    active: bool,
    /// Intensity multiplier (0.5-2.0).
    intensity: f32,
    /// Affected room IDs.
    affected_rooms: Vec<usize>,
}

impl RandomEvent {
    /// Create a new random event.
    #[must_use]
    pub fn new(event_type: RandomEventType) -> Self {
        Self {
            event_type,
            remaining_duration: event_type.base_duration(),
            active: false,
            intensity: 1.0,
            affected_rooms: Vec::new(),
        }
    }

    /// Create an event with custom intensity.
    #[must_use]
    pub fn with_intensity(event_type: RandomEventType, intensity: f32) -> Self {
        Self {
            event_type,
            remaining_duration: event_type.base_duration(),
            active: false,
            intensity: intensity.clamp(0.5, 2.0),
            affected_rooms: Vec::new(),
        }
    }

    /// Get the event type.
    #[must_use]
    pub fn event_type(&self) -> RandomEventType {
        self.event_type
    }

    /// Get remaining duration.
    #[must_use]
    pub fn remaining_duration(&self) -> f32 {
        self.remaining_duration
    }

    /// Check if the event is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Check if the event has finished.
    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.active && self.remaining_duration <= 0.0
    }

    /// Get intensity multiplier.
    #[must_use]
    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    /// Get affected rooms.
    #[must_use]
    pub fn affected_rooms(&self) -> &[usize] {
        &self.affected_rooms
    }

    /// Start the event.
    pub fn start(&mut self) {
        self.active = true;
    }

    /// Add an affected room.
    pub fn add_affected_room(&mut self, room_id: usize) {
        if !self.affected_rooms.contains(&room_id) {
            self.affected_rooms.push(room_id);
        }
    }

    /// Update the event, returns true if still active.
    pub fn tick(&mut self, dt: f32) -> bool {
        if !self.active {
            return false;
        }

        self.remaining_duration -= dt;
        if self.remaining_duration <= 0.0 {
            self.remaining_duration = 0.0;
            self.active = false;
            return false;
        }
        true
    }

    /// Calculate hull damage per tick.
    #[must_use]
    pub fn hull_damage_per_tick(&self) -> f32 {
        if !self.active || !self.event_type.can_damage_hull() {
            return 0.0;
        }
        match self.event_type {
            RandomEventType::MicrometeoriteShower => 0.5 * self.intensity,
            RandomEventType::DebrisField => 1.0 * self.intensity,
            _ => 0.0,
        }
    }

    /// Calculate power damage per tick.
    #[must_use]
    pub fn power_damage_per_tick(&self) -> f32 {
        if !self.active || !self.event_type.affects_power() {
            return 0.0;
        }
        match self.event_type {
            RandomEventType::SolarFlare => 2.0 * self.intensity,
            RandomEventType::PowerSurge => 10.0 * self.intensity,
            _ => 0.0,
        }
    }

    /// Get the effective severity considering intensity.
    #[must_use]
    pub fn effective_severity(&self) -> u32 {
        let base = self.event_type.severity() as f32;
        (base * self.intensity).round() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // RandomEventType tests
    #[test]
    fn test_random_event_type_display_names() {
        assert_eq!(
            RandomEventType::MicrometeoriteShower.display_name(),
            "Micrometeorite Shower"
        );
        assert_eq!(RandomEventType::SolarFlare.display_name(), "Solar Flare");
        assert_eq!(RandomEventType::PowerSurge.display_name(), "Power Surge");
        assert_eq!(RandomEventType::DebrisField.display_name(), "Debris Field");
        assert_eq!(
            RandomEventType::SystemMalfunction.display_name(),
            "System Malfunction"
        );
    }

    #[test]
    fn test_random_event_type_descriptions() {
        assert!(RandomEventType::MicrometeoriteShower
            .description()
            .contains("meteorite"));
        assert!(RandomEventType::SolarFlare.description().contains("solar"));
        assert!(RandomEventType::PowerSurge.description().contains("surge"));
        assert!(RandomEventType::DebrisField.description().contains("debris"));
        assert!(RandomEventType::SystemMalfunction
            .description()
            .contains("malfunction"));
    }

    #[test]
    fn test_random_event_type_severity() {
        assert_eq!(RandomEventType::MicrometeoriteShower.severity(), 3);
        assert_eq!(RandomEventType::SolarFlare.severity(), 4);
        assert_eq!(RandomEventType::PowerSurge.severity(), 2);
        assert_eq!(RandomEventType::DebrisField.severity(), 4);
        assert_eq!(RandomEventType::SystemMalfunction.severity(), 2);
    }

    #[test]
    fn test_random_event_type_base_duration() {
        assert!((RandomEventType::MicrometeoriteShower.base_duration() - 30.0).abs() < f32::EPSILON);
        assert!((RandomEventType::SolarFlare.base_duration() - 60.0).abs() < f32::EPSILON);
        assert!((RandomEventType::PowerSurge.base_duration() - 5.0).abs() < f32::EPSILON);
        assert!((RandomEventType::DebrisField.base_duration() - 45.0).abs() < f32::EPSILON);
        assert!((RandomEventType::SystemMalfunction.base_duration() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_random_event_type_warning_time() {
        assert!((RandomEventType::MicrometeoriteShower.warning_time() - 10.0).abs() < f32::EPSILON);
        assert!((RandomEventType::SolarFlare.warning_time() - 30.0).abs() < f32::EPSILON);
        assert!((RandomEventType::PowerSurge.warning_time() - 2.0).abs() < f32::EPSILON);
        assert!((RandomEventType::DebrisField.warning_time() - 60.0).abs() < f32::EPSILON);
        assert!((RandomEventType::SystemMalfunction.warning_time() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_random_event_type_can_damage_hull() {
        assert!(RandomEventType::MicrometeoriteShower.can_damage_hull());
        assert!(!RandomEventType::SolarFlare.can_damage_hull());
        assert!(!RandomEventType::PowerSurge.can_damage_hull());
        assert!(RandomEventType::DebrisField.can_damage_hull());
        assert!(!RandomEventType::SystemMalfunction.can_damage_hull());
    }

    #[test]
    fn test_random_event_type_affects_power() {
        assert!(!RandomEventType::MicrometeoriteShower.affects_power());
        assert!(RandomEventType::SolarFlare.affects_power());
        assert!(RandomEventType::PowerSurge.affects_power());
        assert!(!RandomEventType::DebrisField.affects_power());
        assert!(!RandomEventType::SystemMalfunction.affects_power());
    }

    #[test]
    fn test_random_event_type_all() {
        let all = RandomEventType::all();
        assert_eq!(all.len(), 5);
        assert!(all.contains(&RandomEventType::MicrometeoriteShower));
        assert!(all.contains(&RandomEventType::SolarFlare));
        assert!(all.contains(&RandomEventType::PowerSurge));
        assert!(all.contains(&RandomEventType::DebrisField));
        assert!(all.contains(&RandomEventType::SystemMalfunction));
    }

    // RandomEvent struct tests
    #[test]
    fn test_random_event_new() {
        let event = RandomEvent::new(RandomEventType::SolarFlare);
        assert_eq!(event.event_type(), RandomEventType::SolarFlare);
        assert!((event.remaining_duration() - 60.0).abs() < f32::EPSILON);
        assert!(!event.is_active());
        assert!((event.intensity() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_random_event_with_intensity() {
        let event = RandomEvent::with_intensity(RandomEventType::PowerSurge, 1.5);
        assert!((event.intensity() - 1.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_random_event_intensity_clamped() {
        let event = RandomEvent::with_intensity(RandomEventType::PowerSurge, 3.0);
        assert!((event.intensity() - 2.0).abs() < f32::EPSILON);

        let event2 = RandomEvent::with_intensity(RandomEventType::PowerSurge, 0.1);
        assert!((event2.intensity() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_random_event_start() {
        let mut event = RandomEvent::new(RandomEventType::DebrisField);
        assert!(!event.is_active());
        event.start();
        assert!(event.is_active());
    }

    #[test]
    fn test_random_event_tick() {
        let mut event = RandomEvent::new(RandomEventType::PowerSurge);
        event.start();

        assert!(event.tick(2.0));
        assert!((event.remaining_duration() - 3.0).abs() < f32::EPSILON);

        assert!(event.tick(2.0));
        assert!((event.remaining_duration() - 1.0).abs() < f32::EPSILON);

        assert!(!event.tick(2.0));
        assert!(event.is_finished());
    }

    #[test]
    fn test_random_event_tick_inactive() {
        let mut event = RandomEvent::new(RandomEventType::SolarFlare);
        assert!(!event.tick(1.0));
    }

    #[test]
    fn test_random_event_is_finished() {
        let mut event = RandomEvent::new(RandomEventType::PowerSurge);
        event.start();
        assert!(!event.is_finished());

        event.tick(10.0);
        assert!(event.is_finished());
    }

    #[test]
    fn test_random_event_add_affected_room() {
        let mut event = RandomEvent::new(RandomEventType::MicrometeoriteShower);
        assert!(event.affected_rooms().is_empty());

        event.add_affected_room(0);
        event.add_affected_room(1);
        event.add_affected_room(0); // Duplicate, should not add

        assert_eq!(event.affected_rooms().len(), 2);
        assert!(event.affected_rooms().contains(&0));
        assert!(event.affected_rooms().contains(&1));
    }

    #[test]
    fn test_random_event_hull_damage_per_tick() {
        let mut event = RandomEvent::new(RandomEventType::MicrometeoriteShower);
        assert!((event.hull_damage_per_tick() - 0.0).abs() < f32::EPSILON); // Not active

        event.start();
        assert!((event.hull_damage_per_tick() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_random_event_hull_damage_with_intensity() {
        let mut event = RandomEvent::with_intensity(RandomEventType::DebrisField, 2.0);
        event.start();
        assert!((event.hull_damage_per_tick() - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_random_event_no_hull_damage() {
        let mut event = RandomEvent::new(RandomEventType::SolarFlare);
        event.start();
        assert!((event.hull_damage_per_tick() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_random_event_power_damage_per_tick() {
        let mut event = RandomEvent::new(RandomEventType::SolarFlare);
        assert!((event.power_damage_per_tick() - 0.0).abs() < f32::EPSILON); // Not active

        event.start();
        assert!((event.power_damage_per_tick() - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_random_event_power_damage_surge() {
        let mut event = RandomEvent::new(RandomEventType::PowerSurge);
        event.start();
        assert!((event.power_damage_per_tick() - 10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_random_event_power_damage_with_intensity() {
        let mut event = RandomEvent::with_intensity(RandomEventType::SolarFlare, 1.5);
        event.start();
        assert!((event.power_damage_per_tick() - 3.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_random_event_no_power_damage() {
        let mut event = RandomEvent::new(RandomEventType::DebrisField);
        event.start();
        assert!((event.power_damage_per_tick() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_random_event_effective_severity() {
        let event = RandomEvent::new(RandomEventType::SolarFlare);
        assert_eq!(event.effective_severity(), 4);

        let event2 = RandomEvent::with_intensity(RandomEventType::SolarFlare, 1.5);
        assert_eq!(event2.effective_severity(), 6);
    }

    #[test]
    fn test_random_event_system_malfunction_instant() {
        let mut event = RandomEvent::new(RandomEventType::SystemMalfunction);
        event.start();
        // Duration is 0, so should be finished immediately
        assert!((event.remaining_duration() - 0.0).abs() < f32::EPSILON);
        event.tick(0.1);
        assert!(event.is_finished());
    }

    #[test]
    fn test_all_event_types_create_events() {
        for event_type in RandomEventType::all() {
            let event = RandomEvent::new(*event_type);
            assert_eq!(event.event_type(), *event_type);
            assert!(!event.is_active());
        }
    }
}
