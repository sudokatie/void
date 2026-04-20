//! Station ambient and event sounds.
//!
//! Provides audio for life support, venting, alarms, and reactor systems.

use serde::{Deserialize, Serialize};

/// Types of station sounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StationSoundType {
    /// Constant life support system hum.
    LifeSupportHum,
    /// Air venting through breach.
    Venting,
    /// General station alarm.
    Alarm,
    /// Hull breach alarm.
    BreachAlarm,
    /// Reactor hum.
    ReactorHum,
    /// Bulkhead seal sound.
    BulkheadSeal,
    /// Bulkhead open sound.
    BulkheadOpen,
    /// Power down sound.
    PowerDown,
    /// Power up sound.
    PowerUp,
    /// Emergency siren.
    EmergencySiren,
}

impl StationSoundType {
    /// Get display name.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            StationSoundType::LifeSupportHum => "Life Support Hum",
            StationSoundType::Venting => "Air Venting",
            StationSoundType::Alarm => "Station Alarm",
            StationSoundType::BreachAlarm => "Breach Alarm",
            StationSoundType::ReactorHum => "Reactor Hum",
            StationSoundType::BulkheadSeal => "Bulkhead Seal",
            StationSoundType::BulkheadOpen => "Bulkhead Open",
            StationSoundType::PowerDown => "Power Down",
            StationSoundType::PowerUp => "Power Up",
            StationSoundType::EmergencySiren => "Emergency Siren",
        }
    }

    /// Check if this sound loops.
    #[must_use]
    pub fn is_looping(&self) -> bool {
        matches!(
            self,
            StationSoundType::LifeSupportHum
                | StationSoundType::Venting
                | StationSoundType::Alarm
                | StationSoundType::BreachAlarm
                | StationSoundType::ReactorHum
                | StationSoundType::EmergencySiren
        )
    }

    /// Get base volume (0-1).
    #[must_use]
    pub fn base_volume(&self) -> f32 {
        match self {
            StationSoundType::LifeSupportHum => 0.3,
            StationSoundType::Venting => 0.7,
            StationSoundType::Alarm => 0.8,
            StationSoundType::BreachAlarm => 0.9,
            StationSoundType::ReactorHum => 0.4,
            StationSoundType::BulkheadSeal => 0.6,
            StationSoundType::BulkheadOpen => 0.6,
            StationSoundType::PowerDown => 0.5,
            StationSoundType::PowerUp => 0.5,
            StationSoundType::EmergencySiren => 1.0,
        }
    }

    /// Get priority level (higher = more important).
    #[must_use]
    pub fn priority(&self) -> u32 {
        match self {
            StationSoundType::EmergencySiren => 10,
            StationSoundType::BreachAlarm => 9,
            StationSoundType::Alarm => 8,
            StationSoundType::Venting => 7,
            StationSoundType::PowerDown => 6,
            StationSoundType::PowerUp => 5,
            StationSoundType::BulkheadSeal => 4,
            StationSoundType::BulkheadOpen => 4,
            StationSoundType::ReactorHum => 2,
            StationSoundType::LifeSupportHum => 1,
        }
    }

    /// Get all sound types.
    #[must_use]
    pub fn all() -> &'static [StationSoundType] {
        &[
            StationSoundType::LifeSupportHum,
            StationSoundType::Venting,
            StationSoundType::Alarm,
            StationSoundType::BreachAlarm,
            StationSoundType::ReactorHum,
            StationSoundType::BulkheadSeal,
            StationSoundType::BulkheadOpen,
            StationSoundType::PowerDown,
            StationSoundType::PowerUp,
            StationSoundType::EmergencySiren,
        ]
    }
}

/// A playing station sound instance.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StationSoundInstance {
    /// Sound type.
    sound_type: StationSoundType,
    /// Current volume.
    volume: f32,
    /// Whether playing.
    playing: bool,
    /// Source room ID.
    room_id: Option<usize>,
}

impl StationSoundInstance {
    /// Create a new sound instance.
    #[must_use]
    pub fn new(sound_type: StationSoundType) -> Self {
        Self {
            sound_type,
            volume: sound_type.base_volume(),
            playing: false,
            room_id: None,
        }
    }

    /// Create with room ID.
    #[must_use]
    pub fn in_room(mut self, room_id: usize) -> Self {
        self.room_id = Some(room_id);
        self
    }

    /// Get sound type.
    #[must_use]
    pub fn sound_type(&self) -> StationSoundType {
        self.sound_type
    }

    /// Get current volume.
    #[must_use]
    pub fn volume(&self) -> f32 {
        self.volume
    }

    /// Set volume.
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    /// Check if playing.
    #[must_use]
    pub fn is_playing(&self) -> bool {
        self.playing
    }

    /// Get room ID.
    #[must_use]
    pub fn room_id(&self) -> Option<usize> {
        self.room_id
    }

    /// Start playing.
    pub fn play(&mut self) {
        self.playing = true;
    }

    /// Stop playing.
    pub fn stop(&mut self) {
        self.playing = false;
    }

    /// Check if this is a looping sound.
    #[must_use]
    pub fn is_looping(&self) -> bool {
        self.sound_type.is_looping()
    }
}

/// Manages station ambient and event sounds.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StationSounds {
    /// Active sound instances.
    sounds: Vec<StationSoundInstance>,
    /// Master volume.
    master_volume: f32,
    /// Whether audio is enabled.
    enabled: bool,
}

impl StationSounds {
    /// Create a new station sounds manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            sounds: Vec::new(),
            master_volume: 1.0,
            enabled: true,
        }
    }

    /// Play a sound.
    pub fn play(&mut self, sound_type: StationSoundType) {
        let mut instance = StationSoundInstance::new(sound_type);
        instance.play();
        self.sounds.push(instance);
    }

    /// Play a sound in a specific room.
    pub fn play_in_room(&mut self, sound_type: StationSoundType, room_id: usize) {
        let mut instance = StationSoundInstance::new(sound_type).in_room(room_id);
        instance.play();
        self.sounds.push(instance);
    }

    /// Stop all sounds of a type.
    pub fn stop(&mut self, sound_type: StationSoundType) {
        for sound in &mut self.sounds {
            if sound.sound_type() == sound_type {
                sound.stop();
            }
        }
    }

    /// Stop all sounds in a room.
    pub fn stop_in_room(&mut self, room_id: usize) {
        for sound in &mut self.sounds {
            if sound.room_id() == Some(room_id) {
                sound.stop();
            }
        }
    }

    /// Check if a sound type is playing.
    #[must_use]
    pub fn is_playing(&self, sound_type: StationSoundType) -> bool {
        self.sounds
            .iter()
            .any(|s| s.sound_type() == sound_type && s.is_playing())
    }

    /// Get active sound count.
    #[must_use]
    pub fn active_count(&self) -> usize {
        self.sounds.iter().filter(|s| s.is_playing()).count()
    }

    /// Set master volume.
    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
    }

    /// Get master volume.
    #[must_use]
    pub fn master_volume(&self) -> f32 {
        self.master_volume
    }

    /// Enable/disable audio.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Check if audio is enabled.
    #[must_use]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Update sounds, removing stopped non-looping sounds.
    pub fn tick(&mut self, _dt: f32) {
        // Remove stopped sounds
        self.sounds.retain(|s| s.is_playing() || s.is_looping());
    }

    /// Stop all sounds.
    pub fn stop_all(&mut self) {
        for sound in &mut self.sounds {
            sound.stop();
        }
    }

    /// Clear all sounds.
    pub fn clear(&mut self) {
        self.sounds.clear();
    }

    /// Get all active sounds.
    #[must_use]
    pub fn active_sounds(&self) -> Vec<&StationSoundInstance> {
        self.sounds.iter().filter(|s| s.is_playing()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_station_sound_type_display_names() {
        assert_eq!(StationSoundType::LifeSupportHum.display_name(), "Life Support Hum");
        assert_eq!(StationSoundType::Venting.display_name(), "Air Venting");
        assert_eq!(StationSoundType::BreachAlarm.display_name(), "Breach Alarm");
    }

    #[test]
    fn test_station_sound_type_is_looping() {
        assert!(StationSoundType::LifeSupportHum.is_looping());
        assert!(StationSoundType::Venting.is_looping());
        assert!(!StationSoundType::BulkheadSeal.is_looping());
        assert!(!StationSoundType::PowerDown.is_looping());
    }

    #[test]
    fn test_station_sound_type_base_volume() {
        assert!((StationSoundType::LifeSupportHum.base_volume() - 0.3).abs() < f32::EPSILON);
        assert!((StationSoundType::EmergencySiren.base_volume() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_station_sound_type_priority() {
        assert!(StationSoundType::EmergencySiren.priority() > StationSoundType::Alarm.priority());
        assert!(StationSoundType::BreachAlarm.priority() > StationSoundType::Venting.priority());
    }

    #[test]
    fn test_station_sound_type_all() {
        let all = StationSoundType::all();
        assert_eq!(all.len(), 10);
    }

    #[test]
    fn test_station_sound_instance_new() {
        let instance = StationSoundInstance::new(StationSoundType::Alarm);
        assert_eq!(instance.sound_type(), StationSoundType::Alarm);
        assert!(!instance.is_playing());
        assert!(instance.room_id().is_none());
    }

    #[test]
    fn test_station_sound_instance_in_room() {
        let instance = StationSoundInstance::new(StationSoundType::Venting).in_room(5);
        assert_eq!(instance.room_id(), Some(5));
    }

    #[test]
    fn test_station_sound_instance_play_stop() {
        let mut instance = StationSoundInstance::new(StationSoundType::Alarm);
        assert!(!instance.is_playing());

        instance.play();
        assert!(instance.is_playing());

        instance.stop();
        assert!(!instance.is_playing());
    }

    #[test]
    fn test_station_sound_instance_volume() {
        let mut instance = StationSoundInstance::new(StationSoundType::Alarm);
        instance.set_volume(0.5);
        assert!((instance.volume() - 0.5).abs() < f32::EPSILON);

        instance.set_volume(2.0); // Clamped
        assert!((instance.volume() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_station_sounds_new() {
        let sounds = StationSounds::new();
        assert!(sounds.is_enabled());
        assert_eq!(sounds.active_count(), 0);
    }

    #[test]
    fn test_station_sounds_play() {
        let mut sounds = StationSounds::new();
        sounds.play(StationSoundType::Alarm);

        assert!(sounds.is_playing(StationSoundType::Alarm));
        assert_eq!(sounds.active_count(), 1);
    }

    #[test]
    fn test_station_sounds_play_in_room() {
        let mut sounds = StationSounds::new();
        sounds.play_in_room(StationSoundType::Venting, 3);

        assert!(sounds.is_playing(StationSoundType::Venting));
    }

    #[test]
    fn test_station_sounds_stop() {
        let mut sounds = StationSounds::new();
        sounds.play(StationSoundType::Alarm);
        sounds.stop(StationSoundType::Alarm);

        assert!(!sounds.is_playing(StationSoundType::Alarm));
    }

    #[test]
    fn test_station_sounds_stop_in_room() {
        let mut sounds = StationSounds::new();
        sounds.play_in_room(StationSoundType::Venting, 3);
        sounds.play_in_room(StationSoundType::Alarm, 5);
        sounds.stop_in_room(3);

        assert!(!sounds.is_playing(StationSoundType::Venting));
        assert!(sounds.is_playing(StationSoundType::Alarm));
    }

    #[test]
    fn test_station_sounds_master_volume() {
        let mut sounds = StationSounds::new();
        sounds.set_master_volume(0.5);
        assert!((sounds.master_volume() - 0.5).abs() < f32::EPSILON);

        sounds.set_master_volume(2.0); // Clamped
        assert!((sounds.master_volume() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_station_sounds_enabled() {
        let mut sounds = StationSounds::new();
        assert!(sounds.is_enabled());

        sounds.set_enabled(false);
        assert!(!sounds.is_enabled());
    }

    #[test]
    fn test_station_sounds_stop_all() {
        let mut sounds = StationSounds::new();
        sounds.play(StationSoundType::Alarm);
        sounds.play(StationSoundType::Venting);

        sounds.stop_all();
        assert_eq!(sounds.active_count(), 0);
    }

    #[test]
    fn test_station_sounds_clear() {
        let mut sounds = StationSounds::new();
        sounds.play(StationSoundType::Alarm);
        sounds.clear();

        assert!(!sounds.is_playing(StationSoundType::Alarm));
    }

    #[test]
    fn test_station_sounds_active_sounds() {
        let mut sounds = StationSounds::new();
        sounds.play(StationSoundType::Alarm);
        sounds.play(StationSoundType::Venting);

        let active = sounds.active_sounds();
        assert_eq!(active.len(), 2);
    }

    #[test]
    fn test_station_sounds_default() {
        let sounds = StationSounds::default();
        assert!(sounds.is_enabled());
    }
}
