//! Titan-specific sound effects.
//!
//! Provides audio for the Titan's biological sounds including
//! heartbeat, breathing, movement, and vocalizations.

use crate::titan::TitanMood;
use engine_physics::titan::TitanPhase;

/// Handler for Titan-related audio.
#[derive(Clone, Debug, Default)]
pub struct TitanSounds;

impl TitanSounds {
    /// Create a new Titan sounds handler.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Get the heartbeat sound effect based on mood.
    #[must_use]
    pub fn get_heartbeat_sound(&self, mood: TitanMood) -> &'static str {
        match mood {
            TitanMood::Calm => "titan_heartbeat_calm",
            TitanMood::Agitated => "titan_heartbeat_agitated",
            TitanMood::Enraged => "titan_heartbeat_enraged",
        }
    }

    /// Get the breathing sound effect based on mood.
    #[must_use]
    pub fn get_breathing_sound(&self, mood: TitanMood) -> &'static str {
        match mood {
            TitanMood::Calm => "titan_breathing_slow",
            TitanMood::Agitated => "titan_breathing_heavy",
            TitanMood::Enraged => "titan_breathing_ragged",
        }
    }

    /// Get the movement sound effect based on phase.
    #[must_use]
    pub fn get_movement_sound(&self, phase: TitanPhase) -> &'static str {
        match phase {
            TitanPhase::Resting => "titan_idle_settling",
            TitanPhase::Walking => "titan_walking_rumble",
            TitanPhase::Running => "titan_running_thunder",
            TitanPhase::Scratching => "titan_scratching_tremor",
        }
    }

    /// Get the scratching sound effect.
    #[must_use]
    pub fn get_scratching_sound(&self) -> &'static str {
        "titan_scratch_deep"
    }

    /// Get the roar/vocalization sound effect.
    #[must_use]
    pub fn get_roar_sound(&self) -> &'static str {
        "titan_roar_distant"
    }

    /// Get the heartbeat rate in BPM based on mood.
    #[must_use]
    pub fn get_heartbeat_bpm(&self, mood: TitanMood) -> f32 {
        match mood {
            TitanMood::Calm => 30.0,
            TitanMood::Agitated => 50.0,
            TitanMood::Enraged => 80.0,
        }
    }

    /// Get the volume level for heartbeat based on mood.
    #[must_use]
    pub fn get_heartbeat_volume(&self, mood: TitanMood) -> f32 {
        match mood {
            TitanMood::Calm => 0.3,
            TitanMood::Agitated => 0.6,
            TitanMood::Enraged => 1.0,
        }
    }

    /// Get the breathing rate (breaths per minute) based on mood.
    #[must_use]
    pub fn get_breathing_rate(&self, mood: TitanMood) -> f32 {
        match mood {
            TitanMood::Calm => 4.0,
            TitanMood::Agitated => 8.0,
            TitanMood::Enraged => 15.0,
        }
    }

    /// Get ground rumble sound for movement.
    #[must_use]
    pub fn get_ground_rumble(&self, phase: TitanPhase) -> &'static str {
        match phase {
            TitanPhase::Resting => "silence",
            TitanPhase::Walking => "titan_ground_shake_light",
            TitanPhase::Running => "titan_ground_shake_heavy",
            TitanPhase::Scratching => "titan_ground_shake_localized",
        }
    }

    /// Get muscle/tendon creaking sound.
    #[must_use]
    pub fn get_muscle_sound(&self, phase: TitanPhase) -> &'static str {
        match phase {
            TitanPhase::Resting => "titan_muscles_settling",
            TitanPhase::Walking => "titan_muscles_working",
            TitanPhase::Running => "titan_muscles_straining",
            TitanPhase::Scratching => "titan_muscles_flexing",
        }
    }

    /// Get the digestion/stomach rumble sound.
    #[must_use]
    pub fn get_digestion_sound(&self) -> &'static str {
        "titan_digestion_rumble"
    }

    /// Get joint cracking/popping sound.
    #[must_use]
    pub fn get_joint_sound(&self, phase: TitanPhase) -> &'static str {
        match phase {
            TitanPhase::Resting => "titan_joints_settling",
            TitanPhase::Walking | TitanPhase::Running | TitanPhase::Scratching => {
                "titan_joints_cracking"
            }
        }
    }

    /// Get alert/warning vocalization.
    #[must_use]
    pub fn get_warning_sound(&self, mood: TitanMood) -> Option<&'static str> {
        match mood {
            TitanMood::Calm => None,
            TitanMood::Agitated => Some("titan_groan_warning"),
            TitanMood::Enraged => Some("titan_bellow_rage"),
        }
    }

    /// Get the immune response sound (when fighting parasites internally).
    #[must_use]
    pub fn get_immune_sound(&self) -> &'static str {
        "titan_immune_pulse"
    }

    /// Get the healing/regeneration sound for wound sites.
    #[must_use]
    pub fn get_healing_sound(&self) -> &'static str {
        "titan_tissue_regeneration"
    }
}

/// Get heartbeat sound (standalone function).
#[must_use]
pub fn get_heartbeat_sound(mood: TitanMood) -> &'static str {
    TitanSounds::new().get_heartbeat_sound(mood)
}

/// Get breathing sound (standalone function).
#[must_use]
pub fn get_breathing_sound(mood: TitanMood) -> &'static str {
    TitanSounds::new().get_breathing_sound(mood)
}

/// Get movement sound (standalone function).
#[must_use]
pub fn get_movement_sound(phase: TitanPhase) -> &'static str {
    TitanSounds::new().get_movement_sound(phase)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_titan_sounds_new() {
        let sounds = TitanSounds::new();
        assert_eq!(sounds.get_roar_sound(), "titan_roar_distant");
    }

    #[test]
    fn test_heartbeat_sound_calm() {
        let sounds = TitanSounds::new();
        assert_eq!(sounds.get_heartbeat_sound(TitanMood::Calm), "titan_heartbeat_calm");
    }

    #[test]
    fn test_heartbeat_sound_agitated() {
        let sounds = TitanSounds::new();
        assert_eq!(sounds.get_heartbeat_sound(TitanMood::Agitated), "titan_heartbeat_agitated");
    }

    #[test]
    fn test_heartbeat_sound_enraged() {
        let sounds = TitanSounds::new();
        assert_eq!(sounds.get_heartbeat_sound(TitanMood::Enraged), "titan_heartbeat_enraged");
    }

    #[test]
    fn test_breathing_sound_calm() {
        let sounds = TitanSounds::new();
        assert_eq!(sounds.get_breathing_sound(TitanMood::Calm), "titan_breathing_slow");
    }

    #[test]
    fn test_breathing_sound_agitated() {
        let sounds = TitanSounds::new();
        assert_eq!(sounds.get_breathing_sound(TitanMood::Agitated), "titan_breathing_heavy");
    }

    #[test]
    fn test_breathing_sound_enraged() {
        let sounds = TitanSounds::new();
        assert_eq!(sounds.get_breathing_sound(TitanMood::Enraged), "titan_breathing_ragged");
    }

    #[test]
    fn test_movement_sound_resting() {
        let sounds = TitanSounds::new();
        assert_eq!(sounds.get_movement_sound(TitanPhase::Resting), "titan_idle_settling");
    }

    #[test]
    fn test_movement_sound_walking() {
        let sounds = TitanSounds::new();
        assert_eq!(sounds.get_movement_sound(TitanPhase::Walking), "titan_walking_rumble");
    }

    #[test]
    fn test_movement_sound_running() {
        let sounds = TitanSounds::new();
        assert_eq!(sounds.get_movement_sound(TitanPhase::Running), "titan_running_thunder");
    }

    #[test]
    fn test_movement_sound_scratching() {
        let sounds = TitanSounds::new();
        assert_eq!(sounds.get_movement_sound(TitanPhase::Scratching), "titan_scratching_tremor");
    }

    #[test]
    fn test_scratching_sound() {
        let sounds = TitanSounds::new();
        assert_eq!(sounds.get_scratching_sound(), "titan_scratch_deep");
    }

    #[test]
    fn test_roar_sound() {
        let sounds = TitanSounds::new();
        assert_eq!(sounds.get_roar_sound(), "titan_roar_distant");
    }

    #[test]
    fn test_heartbeat_bpm() {
        let sounds = TitanSounds::new();
        assert!((sounds.get_heartbeat_bpm(TitanMood::Calm) - 30.0).abs() < f32::EPSILON);
        assert!((sounds.get_heartbeat_bpm(TitanMood::Agitated) - 50.0).abs() < f32::EPSILON);
        assert!((sounds.get_heartbeat_bpm(TitanMood::Enraged) - 80.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_heartbeat_volume() {
        let sounds = TitanSounds::new();
        assert!(sounds.get_heartbeat_volume(TitanMood::Calm) < sounds.get_heartbeat_volume(TitanMood::Enraged));
    }

    #[test]
    fn test_breathing_rate() {
        let sounds = TitanSounds::new();
        assert!(sounds.get_breathing_rate(TitanMood::Calm) < sounds.get_breathing_rate(TitanMood::Enraged));
    }

    #[test]
    fn test_ground_rumble() {
        let sounds = TitanSounds::new();
        assert_eq!(sounds.get_ground_rumble(TitanPhase::Resting), "silence");
        assert!(sounds.get_ground_rumble(TitanPhase::Running).contains("heavy"));
    }

    #[test]
    fn test_muscle_sound() {
        let sounds = TitanSounds::new();
        assert!(sounds.get_muscle_sound(TitanPhase::Running).contains("straining"));
    }

    #[test]
    fn test_digestion_sound() {
        let sounds = TitanSounds::new();
        assert!(sounds.get_digestion_sound().contains("digestion"));
    }

    #[test]
    fn test_joint_sound() {
        let sounds = TitanSounds::new();
        assert!(sounds.get_joint_sound(TitanPhase::Resting).contains("settling"));
        assert!(sounds.get_joint_sound(TitanPhase::Walking).contains("cracking"));
    }

    #[test]
    fn test_warning_sound_calm() {
        let sounds = TitanSounds::new();
        assert!(sounds.get_warning_sound(TitanMood::Calm).is_none());
    }

    #[test]
    fn test_warning_sound_agitated() {
        let sounds = TitanSounds::new();
        assert!(sounds.get_warning_sound(TitanMood::Agitated).is_some());
        assert!(sounds.get_warning_sound(TitanMood::Agitated).unwrap().contains("warning"));
    }

    #[test]
    fn test_warning_sound_enraged() {
        let sounds = TitanSounds::new();
        assert!(sounds.get_warning_sound(TitanMood::Enraged).is_some());
        assert!(sounds.get_warning_sound(TitanMood::Enraged).unwrap().contains("rage"));
    }

    #[test]
    fn test_immune_sound() {
        let sounds = TitanSounds::new();
        assert!(sounds.get_immune_sound().contains("immune"));
    }

    #[test]
    fn test_healing_sound() {
        let sounds = TitanSounds::new();
        assert!(sounds.get_healing_sound().contains("regeneration"));
    }

    #[test]
    fn test_standalone_get_heartbeat_sound() {
        assert_eq!(get_heartbeat_sound(TitanMood::Calm), "titan_heartbeat_calm");
    }

    #[test]
    fn test_standalone_get_breathing_sound() {
        assert_eq!(get_breathing_sound(TitanMood::Agitated), "titan_breathing_heavy");
    }

    #[test]
    fn test_standalone_get_movement_sound() {
        assert_eq!(get_movement_sound(TitanPhase::Running), "titan_running_thunder");
    }

    #[test]
    fn test_titan_sounds_default() {
        let sounds = TitanSounds::default();
        assert_eq!(sounds.get_roar_sound(), "titan_roar_distant");
    }
}
