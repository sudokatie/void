//! Environmental audio for Titan zones.
//!
//! Provides ambient sounds for different Titan zones, weather effects,
//! and creature-specific audio.

use engine_physics::titan::TitanPhase;

/// Handler for environmental audio.
#[derive(Clone, Debug, Default)]
pub struct EnvironmentAudio;

impl EnvironmentAudio {
    /// Create a new environment audio handler.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Get wind sound based on Titan movement phase.
    #[must_use]
    pub fn get_wind_sound(&self, phase: TitanPhase) -> &'static str {
        match phase {
            TitanPhase::Resting => "wind_calm_ambient",
            TitanPhase::Walking => "wind_light_breeze",
            TitanPhase::Running => "wind_strong_gust",
            TitanPhase::Scratching => "wind_turbulent",
        }
    }

    /// Get settling/shifting sound for terrain.
    #[must_use]
    pub fn get_settling_sound(&self) -> &'static str {
        "terrain_settling_creak"
    }

    /// Get vent eruption/hissing sound.
    #[must_use]
    pub fn get_vent_sound(&self) -> &'static str {
        "vent_steam_hiss"
    }

    /// Get parasite/creature ambient sound.
    #[must_use]
    pub fn get_parasite_sound(&self, creature: &str) -> &'static str {
        match creature.to_lowercase().as_str() {
            "scale_tick" => "creature_tick_clicking",
            "shell_borer" => "creature_borer_drilling",
            "blood_leech" => "creature_leech_squelch",
            "neural_wasp" => "creature_wasp_buzzing",
            "mouth_crawler" => "creature_crawler_chittering",
            "scale_moth" => "creature_moth_flutter",
            "vent_shrimp" => "creature_shrimp_scuttle",
            "blood_fish" => "creature_fish_splash",
            "neural_butterfly" => "creature_butterfly_soft",
            "shell_crab" => "creature_crab_click",
            _ => "creature_generic_ambient",
        }
    }

    /// Get zone-specific ambient sound.
    #[must_use]
    pub fn get_zone_ambient(&self, zone: &str) -> &'static str {
        match zone.to_lowercase().as_str() {
            "shell_ridge" | "shellridge" => "zone_shell_wind_whistle",
            "scale_valley" | "scalevalley" => "zone_valley_echo",
            "parasite_forest" | "parasiteforest" => "zone_forest_crawling",
            "breathing_vent" | "breathingvent" => "zone_vent_rumble",
            "wound_site" | "woundsite" => "zone_wound_pulse",
            "neural_node" | "neuralnode" => "zone_neural_hum",
            _ => "zone_generic_ambient",
        }
    }

    /// Get wind volume based on phase.
    #[must_use]
    pub fn get_wind_volume(&self, phase: TitanPhase) -> f32 {
        match phase {
            TitanPhase::Resting => 0.2,
            TitanPhase::Walking => 0.4,
            TitanPhase::Running => 0.8,
            TitanPhase::Scratching => 0.6,
        }
    }

    /// Get creature attack sound.
    #[must_use]
    pub fn get_creature_attack_sound(&self, creature: &str) -> &'static str {
        match creature.to_lowercase().as_str() {
            "scale_tick" => "creature_tick_bite",
            "shell_borer" => "creature_borer_charge",
            "blood_leech" => "creature_leech_latch",
            "neural_wasp" => "creature_wasp_sting",
            "mouth_crawler" => "creature_crawler_chomp",
            _ => "creature_generic_attack",
        }
    }

    /// Get creature death sound.
    #[must_use]
    pub fn get_creature_death_sound(&self, creature: &str) -> &'static str {
        match creature.to_lowercase().as_str() {
            "scale_tick" => "creature_tick_pop",
            "shell_borer" => "creature_borer_crunch",
            "blood_leech" => "creature_leech_splat",
            "neural_wasp" => "creature_wasp_buzz_die",
            "mouth_crawler" => "creature_crawler_screech",
            "scale_moth" => "creature_moth_puff",
            "vent_shrimp" => "creature_shrimp_crack",
            "blood_fish" => "creature_fish_flop",
            "neural_butterfly" => "creature_butterfly_dissolve",
            "shell_crab" => "creature_crab_crunch",
            _ => "creature_generic_death",
        }
    }

    /// Get liquid/fluid sounds for wound sites.
    #[must_use]
    pub fn get_fluid_sound(&self) -> &'static str {
        "fluid_blood_drip"
    }

    /// Get thermal/heat sounds for vents.
    #[must_use]
    pub fn get_thermal_sound(&self) -> &'static str {
        "thermal_heat_crackle"
    }

    /// Get neural/electrical sounds for neural nodes.
    #[must_use]
    pub fn get_neural_sound(&self) -> &'static str {
        "neural_electric_pulse"
    }

    /// Get footstep sound for a zone surface.
    #[must_use]
    pub fn get_footstep_sound(&self, zone: &str) -> &'static str {
        match zone.to_lowercase().as_str() {
            "shell_ridge" | "shellridge" => "footstep_shell_hard",
            "scale_valley" | "scalevalley" => "footstep_scale_crunch",
            "parasite_forest" | "parasiteforest" => "footstep_organic_squish",
            "breathing_vent" | "breathingvent" => "footstep_grate_metal",
            "wound_site" | "woundsite" => "footstep_flesh_wet",
            "neural_node" | "neuralnode" => "footstep_membrane_soft",
            _ => "footstep_generic",
        }
    }

    /// Get ambient danger alert sound.
    #[must_use]
    pub fn get_danger_ambient(&self, danger_level: f32) -> Option<&'static str> {
        if danger_level < 0.3 {
            None
        } else if danger_level < 0.6 {
            Some("ambient_tension_low")
        } else if danger_level < 0.8 {
            Some("ambient_tension_medium")
        } else {
            Some("ambient_tension_high")
        }
    }

    /// Get weather/atmospheric sound.
    #[must_use]
    pub fn get_atmospheric_sound(&self, phase: TitanPhase) -> &'static str {
        match phase {
            TitanPhase::Resting => "atmosphere_calm_drone",
            TitanPhase::Walking => "atmosphere_movement_whoosh",
            TitanPhase::Running => "atmosphere_rushing_air",
            TitanPhase::Scratching => "atmosphere_localized_disturbance",
        }
    }
}

/// Get wind sound (standalone function).
#[must_use]
pub fn get_wind_sound(phase: TitanPhase) -> &'static str {
    EnvironmentAudio::new().get_wind_sound(phase)
}

/// Get settling sound (standalone function).
#[must_use]
pub fn get_settling_sound() -> &'static str {
    EnvironmentAudio::new().get_settling_sound()
}

/// Get vent sound (standalone function).
#[must_use]
pub fn get_vent_sound() -> &'static str {
    EnvironmentAudio::new().get_vent_sound()
}

/// Get parasite sound (standalone function).
#[must_use]
pub fn get_parasite_sound(creature: &str) -> &'static str {
    EnvironmentAudio::new().get_parasite_sound(creature)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_audio_new() {
        let audio = EnvironmentAudio::new();
        assert!(audio.get_settling_sound().contains("settling"));
    }

    #[test]
    fn test_wind_sound_resting() {
        let audio = EnvironmentAudio::new();
        assert_eq!(audio.get_wind_sound(TitanPhase::Resting), "wind_calm_ambient");
    }

    #[test]
    fn test_wind_sound_walking() {
        let audio = EnvironmentAudio::new();
        assert_eq!(audio.get_wind_sound(TitanPhase::Walking), "wind_light_breeze");
    }

    #[test]
    fn test_wind_sound_running() {
        let audio = EnvironmentAudio::new();
        assert_eq!(audio.get_wind_sound(TitanPhase::Running), "wind_strong_gust");
    }

    #[test]
    fn test_wind_sound_scratching() {
        let audio = EnvironmentAudio::new();
        assert_eq!(audio.get_wind_sound(TitanPhase::Scratching), "wind_turbulent");
    }

    #[test]
    fn test_settling_sound() {
        let audio = EnvironmentAudio::new();
        assert!(audio.get_settling_sound().contains("settling"));
    }

    #[test]
    fn test_vent_sound() {
        let audio = EnvironmentAudio::new();
        assert!(audio.get_vent_sound().contains("vent") || audio.get_vent_sound().contains("steam"));
    }

    #[test]
    fn test_parasite_sound_scale_tick() {
        let audio = EnvironmentAudio::new();
        assert!(audio.get_parasite_sound("scale_tick").contains("tick"));
    }

    #[test]
    fn test_parasite_sound_blood_leech() {
        let audio = EnvironmentAudio::new();
        assert!(audio.get_parasite_sound("blood_leech").contains("leech"));
    }

    #[test]
    fn test_parasite_sound_neural_wasp() {
        let audio = EnvironmentAudio::new();
        assert!(audio.get_parasite_sound("neural_wasp").contains("wasp") || audio.get_parasite_sound("neural_wasp").contains("buzzing"));
    }

    #[test]
    fn test_parasite_sound_unknown() {
        let audio = EnvironmentAudio::new();
        assert!(audio.get_parasite_sound("unknown_creature").contains("generic"));
    }

    #[test]
    fn test_zone_ambient_shell_ridge() {
        let audio = EnvironmentAudio::new();
        assert!(audio.get_zone_ambient("shell_ridge").contains("shell") || audio.get_zone_ambient("shell_ridge").contains("wind"));
    }

    #[test]
    fn test_zone_ambient_breathing_vent() {
        let audio = EnvironmentAudio::new();
        assert!(audio.get_zone_ambient("breathing_vent").contains("vent") || audio.get_zone_ambient("breathing_vent").contains("rumble"));
    }

    #[test]
    fn test_zone_ambient_wound_site() {
        let audio = EnvironmentAudio::new();
        assert!(audio.get_zone_ambient("wound_site").contains("wound") || audio.get_zone_ambient("wound_site").contains("pulse"));
    }

    #[test]
    fn test_zone_ambient_unknown() {
        let audio = EnvironmentAudio::new();
        assert!(audio.get_zone_ambient("unknown_zone").contains("generic"));
    }

    #[test]
    fn test_wind_volume() {
        let audio = EnvironmentAudio::new();
        assert!(audio.get_wind_volume(TitanPhase::Resting) < audio.get_wind_volume(TitanPhase::Running));
    }

    #[test]
    fn test_creature_attack_sound() {
        let audio = EnvironmentAudio::new();
        assert!(audio.get_creature_attack_sound("scale_tick").contains("tick"));
        assert!(audio.get_creature_attack_sound("mouth_crawler").contains("crawler") || audio.get_creature_attack_sound("mouth_crawler").contains("chomp"));
    }

    #[test]
    fn test_creature_death_sound() {
        let audio = EnvironmentAudio::new();
        assert!(audio.get_creature_death_sound("blood_leech").contains("leech") || audio.get_creature_death_sound("blood_leech").contains("splat"));
    }

    #[test]
    fn test_fluid_sound() {
        let audio = EnvironmentAudio::new();
        assert!(audio.get_fluid_sound().contains("blood") || audio.get_fluid_sound().contains("fluid"));
    }

    #[test]
    fn test_thermal_sound() {
        let audio = EnvironmentAudio::new();
        assert!(audio.get_thermal_sound().contains("thermal") || audio.get_thermal_sound().contains("heat"));
    }

    #[test]
    fn test_neural_sound() {
        let audio = EnvironmentAudio::new();
        assert!(audio.get_neural_sound().contains("neural") || audio.get_neural_sound().contains("electric"));
    }

    #[test]
    fn test_footstep_sound() {
        let audio = EnvironmentAudio::new();
        assert!(audio.get_footstep_sound("shell_ridge").contains("shell") || audio.get_footstep_sound("shell_ridge").contains("hard"));
        assert!(audio.get_footstep_sound("wound_site").contains("flesh") || audio.get_footstep_sound("wound_site").contains("wet"));
    }

    #[test]
    fn test_danger_ambient_low() {
        let audio = EnvironmentAudio::new();
        assert!(audio.get_danger_ambient(0.1).is_none());
    }

    #[test]
    fn test_danger_ambient_medium() {
        let audio = EnvironmentAudio::new();
        assert!(audio.get_danger_ambient(0.5).is_some());
        assert!(audio.get_danger_ambient(0.5).unwrap().contains("tension"));
    }

    #[test]
    fn test_danger_ambient_high() {
        let audio = EnvironmentAudio::new();
        assert!(audio.get_danger_ambient(0.9).is_some());
        assert!(audio.get_danger_ambient(0.9).unwrap().contains("high"));
    }

    #[test]
    fn test_atmospheric_sound() {
        let audio = EnvironmentAudio::new();
        assert!(audio.get_atmospheric_sound(TitanPhase::Resting).contains("calm") || audio.get_atmospheric_sound(TitanPhase::Resting).contains("atmosphere"));
    }

    #[test]
    fn test_standalone_get_wind_sound() {
        assert_eq!(get_wind_sound(TitanPhase::Running), "wind_strong_gust");
    }

    #[test]
    fn test_standalone_get_settling_sound() {
        assert!(get_settling_sound().contains("settling"));
    }

    #[test]
    fn test_standalone_get_vent_sound() {
        assert!(get_vent_sound().contains("vent") || get_vent_sound().contains("steam"));
    }

    #[test]
    fn test_standalone_get_parasite_sound() {
        assert!(get_parasite_sound("shell_borer").contains("borer") || get_parasite_sound("shell_borer").contains("drilling"));
    }

    #[test]
    fn test_environment_audio_default() {
        let audio = EnvironmentAudio::default();
        assert!(audio.get_settling_sound().contains("settling"));
    }
}
