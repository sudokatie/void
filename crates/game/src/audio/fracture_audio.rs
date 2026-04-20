//! Fracture and creature audio.
//!
//! Provides sound effects for fracture events and dimensional creatures.

use engine_physics::dimension::FractureType;

/// Handler for fracture-related audio.
#[derive(Clone, Debug, Default)]
pub struct FractureAudio;

impl FractureAudio {
    /// Create a new fracture audio handler.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Get the sound effect for a fracture type.
    #[must_use]
    pub fn get_fracture_sound(&self, ftype: FractureType) -> &'static str {
        match ftype {
            FractureType::Micro => "fracture_micro_crack",
            FractureType::Meso => "fracture_meso_tear",
            FractureType::Macro => "fracture_macro_rip",
            FractureType::Cascade => "fracture_cascade_collapse",
        }
    }

    /// Get the volume for a fracture type.
    ///
    /// Returns 0.0 to 1.0 volume.
    #[must_use]
    pub fn get_fracture_volume(&self, ftype: FractureType) -> f32 {
        match ftype {
            FractureType::Micro => 0.4,
            FractureType::Meso => 0.6,
            FractureType::Macro => 0.8,
            FractureType::Cascade => 1.0,
        }
    }

    /// Get the sound radius for a fracture type.
    #[must_use]
    pub fn get_sound_radius(&self, ftype: FractureType) -> f32 {
        match ftype {
            FractureType::Micro => 10.0,
            FractureType::Meso => 25.0,
            FractureType::Macro => 50.0,
            FractureType::Cascade => 100.0,
        }
    }

    /// Get the warning sound before a fracture.
    #[must_use]
    pub fn get_warning_sound(&self, ftype: FractureType) -> Option<&'static str> {
        match ftype {
            FractureType::Micro => None, // No warning
            FractureType::Meso => Some("fracture_warning_low"),
            FractureType::Macro => Some("fracture_warning_medium"),
            FractureType::Cascade => Some("fracture_warning_high"),
        }
    }

    /// Get ambient rumble sound for unstable areas.
    #[must_use]
    pub fn get_instability_rumble(&self) -> &'static str {
        "fracture_instability_rumble"
    }

    /// Get sound for dimension boundary crossing.
    #[must_use]
    pub fn get_boundary_sound(&self) -> &'static str {
        "dimension_boundary_cross"
    }

    /// Get the sound effect for a creature type.
    #[must_use]
    pub fn get_creature_sound(&self, creature: &str) -> &'static str {
        match creature.to_lowercase().as_str() {
            "void_stalker" => "creature_void_stalker_growl",
            "void_leech" => "creature_void_leech_hiss",
            "phase_spider" => "creature_phase_spider_skitter",
            "inverted_wraith" => "creature_inverted_wraith_wail",
            "ember_elemental" => "creature_ember_elemental_crackle",
            "nexus_guardian" => "creature_nexus_guardian_resonance",
            "stability_golem" => "creature_stability_golem_hum",
            "chaos_wisp" => "creature_chaos_wisp_whisper",
            _ => "creature_generic_ambient",
        }
    }

    /// Get the attack sound for a creature type.
    #[must_use]
    pub fn get_creature_attack_sound(&self, creature: &str) -> &'static str {
        match creature.to_lowercase().as_str() {
            "void_stalker" => "creature_void_stalker_attack",
            "void_leech" => "creature_void_leech_attack",
            "phase_spider" => "creature_phase_spider_attack",
            "inverted_wraith" => "creature_inverted_wraith_attack",
            "ember_elemental" => "creature_ember_elemental_attack",
            "nexus_guardian" => "creature_nexus_guardian_attack",
            "stability_golem" => "creature_stability_golem_attack",
            "chaos_wisp" => "creature_chaos_wisp_attack",
            _ => "creature_generic_attack",
        }
    }

    /// Get the death sound for a creature type.
    #[must_use]
    pub fn get_creature_death_sound(&self, creature: &str) -> &'static str {
        match creature.to_lowercase().as_str() {
            "void_stalker" => "creature_void_stalker_death",
            "void_leech" => "creature_void_leech_death",
            "phase_spider" => "creature_phase_spider_death",
            "inverted_wraith" => "creature_inverted_wraith_death",
            "ember_elemental" => "creature_ember_elemental_death",
            "nexus_guardian" => "creature_nexus_guardian_death",
            "stability_golem" => "creature_stability_golem_death",
            "chaos_wisp" => "creature_chaos_wisp_death",
            _ => "creature_generic_death",
        }
    }

    /// Get the phase shift sound effect.
    #[must_use]
    pub fn get_phase_shift_sound(&self) -> &'static str {
        "player_phase_shift"
    }

    /// Get the sickness warning sound.
    #[must_use]
    pub fn get_sickness_sound(&self, level: u8) -> &'static str {
        match level {
            0 => "silence", // No sickness
            1 => "sickness_mild_heartbeat",
            2 => "sickness_moderate_distortion",
            3 => "sickness_severe_chaos",
            _ => "sickness_critical_collapse",
        }
    }

    /// Get the anchor activation sound.
    #[must_use]
    pub fn get_anchor_sound(&self, event: &str) -> &'static str {
        match event.to_lowercase().as_str() {
            "place" => "anchor_place",
            "activate" => "anchor_activate",
            "deactivate" => "anchor_deactivate",
            "low_fuel" => "anchor_low_fuel_warning",
            "depleted" => "anchor_depleted",
            _ => "anchor_generic",
        }
    }
}

/// Get fracture sound for a type (standalone function).
#[must_use]
pub fn get_fracture_sound(ftype: FractureType) -> &'static str {
    FractureAudio::new().get_fracture_sound(ftype)
}

/// Get creature sound (standalone function).
#[must_use]
pub fn get_creature_sound(creature: &str) -> &'static str {
    FractureAudio::new().get_creature_sound(creature)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fracture_sound() {
        let audio = FractureAudio::new();

        assert_eq!(audio.get_fracture_sound(FractureType::Micro), "fracture_micro_crack");
        assert_eq!(audio.get_fracture_sound(FractureType::Cascade), "fracture_cascade_collapse");
    }

    #[test]
    fn test_fracture_volume() {
        let audio = FractureAudio::new();

        assert!(audio.get_fracture_volume(FractureType::Micro) < audio.get_fracture_volume(FractureType::Cascade));
    }

    #[test]
    fn test_sound_radius() {
        let audio = FractureAudio::new();

        assert!(audio.get_sound_radius(FractureType::Micro) < audio.get_sound_radius(FractureType::Cascade));
    }

    #[test]
    fn test_warning_sound() {
        let audio = FractureAudio::new();

        assert!(audio.get_warning_sound(FractureType::Micro).is_none());
        assert!(audio.get_warning_sound(FractureType::Macro).is_some());
        assert!(audio.get_warning_sound(FractureType::Cascade).is_some());
    }

    #[test]
    fn test_creature_sound() {
        let audio = FractureAudio::new();

        assert!(audio.get_creature_sound("void_stalker").contains("void_stalker"));
        assert!(audio.get_creature_sound("unknown").contains("generic"));
    }

    #[test]
    fn test_creature_attack_sound() {
        let audio = FractureAudio::new();

        assert!(audio.get_creature_attack_sound("phase_spider").contains("attack"));
    }

    #[test]
    fn test_creature_death_sound() {
        let audio = FractureAudio::new();

        assert!(audio.get_creature_death_sound("ember_elemental").contains("death"));
    }

    #[test]
    fn test_phase_shift_sound() {
        let audio = FractureAudio::new();
        assert_eq!(audio.get_phase_shift_sound(), "player_phase_shift");
    }

    #[test]
    fn test_sickness_sound() {
        let audio = FractureAudio::new();

        assert_eq!(audio.get_sickness_sound(0), "silence");
        assert!(audio.get_sickness_sound(4).contains("critical"));
    }

    #[test]
    fn test_anchor_sound() {
        let audio = FractureAudio::new();

        assert_eq!(audio.get_anchor_sound("place"), "anchor_place");
        assert_eq!(audio.get_anchor_sound("activate"), "anchor_activate");
        assert!(audio.get_anchor_sound("unknown").contains("generic"));
    }

    #[test]
    fn test_standalone_functions() {
        assert_eq!(get_fracture_sound(FractureType::Micro), "fracture_micro_crack");
        assert!(get_creature_sound("void_stalker").contains("void_stalker"));
    }

    #[test]
    fn test_instability_rumble() {
        let audio = FractureAudio::new();
        assert!(audio.get_instability_rumble().contains("rumble"));
    }

    #[test]
    fn test_boundary_sound() {
        let audio = FractureAudio::new();
        assert!(audio.get_boundary_sound().contains("boundary"));
    }
}
