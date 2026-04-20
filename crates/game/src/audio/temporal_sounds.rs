//! Temporal audio for time-loop survival.
//!
//! Provides sounds for clocks, resets, paradox, and temporal chests.

use serde::{Deserialize, Serialize};

/// Temporal sound categories.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TemporalSoundType {
    /// Clock ticking sounds.
    ClockTick,
    /// Loop reset sounds.
    LoopReset,
    /// Paradox warning sounds.
    ParadoxWarning,
    /// Temporal chest interaction.
    ChestInteraction,
    /// Time dilation effect.
    TimeDilation,
    /// Creature temporal ability.
    TemporalAbility,
}

/// Handler for temporal audio.
#[derive(Clone, Debug, Default)]
pub struct TemporalSounds {
    /// Current loop count (affects sound intensity).
    loop_count: u32,
    /// Current paradox level (affects warning sounds).
    paradox_level: f32,
    /// Whether time is dilated.
    time_dilated: bool,
}

impl TemporalSounds {
    /// Create a new temporal sounds handler.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the current loop count.
    pub fn set_loop_count(&mut self, count: u32) {
        self.loop_count = count;
    }

    /// Set the current paradox level.
    pub fn set_paradox_level(&mut self, level: f32) {
        self.paradox_level = level.clamp(0.0, 1.0);
    }

    /// Set time dilation state.
    pub fn set_time_dilated(&mut self, dilated: bool) {
        self.time_dilated = dilated;
    }

    /// Get clock tick sound based on time remaining.
    #[must_use]
    pub fn get_clock_tick(&self, time_remaining_percent: f32) -> &'static str {
        if time_remaining_percent > 0.75 {
            "clock_tick_slow"
        } else if time_remaining_percent > 0.5 {
            "clock_tick_normal"
        } else if time_remaining_percent > 0.25 {
            "clock_tick_fast"
        } else {
            "clock_tick_urgent"
        }
    }

    /// Get clock tick volume based on time remaining.
    #[must_use]
    pub fn get_clock_volume(&self, time_remaining_percent: f32) -> f32 {
        if time_remaining_percent > 0.75 {
            0.2
        } else if time_remaining_percent > 0.5 {
            0.4
        } else if time_remaining_percent > 0.25 {
            0.6
        } else {
            0.9
        }
    }

    /// Get loop reset sound.
    #[must_use]
    pub fn get_reset_sound(&self) -> &'static str {
        if self.loop_count >= 10 {
            "reset_dramatic_thunder"
        } else if self.loop_count >= 5 {
            "reset_intense_whoosh"
        } else {
            "reset_soft_wind"
        }
    }

    /// Get paradox warning sound based on level.
    #[must_use]
    pub fn get_paradox_sound(&self) -> Option<&'static str> {
        if self.paradox_level < 0.2 {
            None
        } else if self.paradox_level < 0.4 {
            Some("paradox_hum_low")
        } else if self.paradox_level < 0.6 {
            Some("paradox_pulse_medium")
        } else if self.paradox_level < 0.8 {
            Some("paradox_static_high")
        } else {
            Some("paradox_alarm_critical")
        }
    }

    /// Get paradox warning volume.
    #[must_use]
    pub fn get_paradox_volume(&self) -> f32 {
        if self.paradox_level < 0.2 {
            0.0
        } else {
            0.2 + (self.paradox_level - 0.2) * 0.8
        }
    }

    /// Get temporal chest interaction sound.
    #[must_use]
    pub fn get_chest_sound(&self, action: &str) -> &'static str {
        match action.to_lowercase().as_str() {
            "open" => "chest_temporal_open",
            "close" => "chest_temporal_close",
            "deposit" => "chest_item_deposit_shimmer",
            "withdraw" => "chest_item_withdraw_echo",
            "lock" => "chest_temporal_lock",
            "unlock" => "chest_temporal_unlock",
            "full" => "chest_full_warning",
            "persist" => "chest_persist_through_loop",
            _ => "chest_interact_generic",
        }
    }

    /// Get chest ambient sound based on contents.
    #[must_use]
    pub fn get_chest_ambient(&self, has_items: bool) -> &'static str {
        if has_items {
            "chest_ambient_hum_active"
        } else {
            "chest_ambient_hum_idle"
        }
    }

    /// Get time dilation sound.
    #[must_use]
    pub fn get_dilation_sound(&self, dilation_factor: f32) -> &'static str {
        if dilation_factor < 0.5 {
            "dilation_slow_deep"
        } else if dilation_factor < 1.0 {
            "dilation_slow_mild"
        } else if dilation_factor < 1.5 {
            "dilation_normal"
        } else {
            "dilation_fast_high"
        }
    }

    /// Get creature temporal ability sound.
    #[must_use]
    pub fn get_creature_ability_sound(&self, creature: &str, ability: &str) -> &'static str {
        match (creature.to_lowercase().as_str(), ability.to_lowercase().as_str()) {
            ("time_wraith", "chronal_drain") => "ability_chronal_drain_wisp",
            ("loop_stalker", "deja_vu") => "ability_deja_vu_echo",
            ("echo_beast", "resonance") => "ability_resonance_boom",
            ("temporal_parasite", "phase") => "ability_phase_shift",
            ("chrono_spider", "snare") => "ability_time_snare_weave",
            _ => "ability_temporal_generic",
        }
    }

    /// Get dawn transition sound.
    #[must_use]
    pub fn get_dawn_sound(&self) -> &'static str {
        "phase_dawn_chime"
    }

    /// Get midnight transition sound.
    #[must_use]
    pub fn get_midnight_sound(&self) -> &'static str {
        "phase_midnight_toll"
    }

    /// Get phase transition sound.
    #[must_use]
    pub fn get_phase_transition_sound(&self, phase: &str) -> &'static str {
        match phase.to_lowercase().as_str() {
            "dawn" => "phase_dawn_chime",
            "day" => "phase_day_brightness",
            "dusk" => "phase_dusk_fade",
            "midnight" => "phase_midnight_toll",
            _ => "phase_transition_generic",
        }
    }

    /// Get loop milestone sound.
    #[must_use]
    pub fn get_milestone_sound(&self, milestone: u32) -> Option<&'static str> {
        match milestone {
            5 => Some("milestone_loop_5_bell"),
            10 => Some("milestone_loop_10_gong"),
            20 => Some("milestone_loop_20_fanfare"),
            50 => Some("milestone_loop_50_epic"),
            100 => Some("milestone_loop_100_legendary"),
            _ => None,
        }
    }

    /// Get death sound (with loop awareness).
    #[must_use]
    pub fn get_death_sound(&self, deaths_at_location: u32) -> &'static str {
        if deaths_at_location >= 5 {
            "death_familiar_sigh"
        } else if deaths_at_location >= 2 {
            "death_echo_memory"
        } else {
            "death_first_time"
        }
    }

    /// Get trap discovery sound.
    #[must_use]
    pub fn get_trap_discovery_sound(&self, known_from_loop: bool) -> &'static str {
        if known_from_loop {
            "trap_recognition_click"
        } else {
            "trap_discovery_surprise"
        }
    }

    /// Get knowledge unlock sound.
    #[must_use]
    pub fn get_knowledge_unlock_sound(&self) -> &'static str {
        "knowledge_unlock_revelation"
    }

    /// Get memory persistence sound (item survives loop).
    #[must_use]
    pub fn get_persistence_sound(&self) -> &'static str {
        "persistence_temporal_anchor"
    }
}

/// Get clock tick sound (standalone function).
#[must_use]
pub fn get_clock_sound(time_remaining_percent: f32) -> &'static str {
    TemporalSounds::new().get_clock_tick(time_remaining_percent)
}

/// Get reset sound (standalone function).
#[must_use]
pub fn get_reset_sound(loop_count: u32) -> &'static str {
    let mut sounds = TemporalSounds::new();
    sounds.set_loop_count(loop_count);
    sounds.get_reset_sound()
}

/// Get paradox sound (standalone function).
#[must_use]
pub fn get_paradox_sound(level: f32) -> Option<&'static str> {
    let mut sounds = TemporalSounds::new();
    sounds.set_paradox_level(level);
    sounds.get_paradox_sound()
}

/// Get chest sound (standalone function).
#[must_use]
pub fn get_chest_sound(action: &str) -> &'static str {
    TemporalSounds::new().get_chest_sound(action)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temporal_sounds_new() {
        let sounds = TemporalSounds::new();
        assert!(!sounds.time_dilated);
    }

    #[test]
    fn test_clock_tick_slow() {
        let sounds = TemporalSounds::new();
        assert_eq!(sounds.get_clock_tick(0.9), "clock_tick_slow");
    }

    #[test]
    fn test_clock_tick_normal() {
        let sounds = TemporalSounds::new();
        assert_eq!(sounds.get_clock_tick(0.6), "clock_tick_normal");
    }

    #[test]
    fn test_clock_tick_fast() {
        let sounds = TemporalSounds::new();
        assert_eq!(sounds.get_clock_tick(0.3), "clock_tick_fast");
    }

    #[test]
    fn test_clock_tick_urgent() {
        let sounds = TemporalSounds::new();
        assert_eq!(sounds.get_clock_tick(0.1), "clock_tick_urgent");
    }

    #[test]
    fn test_clock_volume_increases_with_urgency() {
        let sounds = TemporalSounds::new();
        assert!(sounds.get_clock_volume(0.1) > sounds.get_clock_volume(0.9));
    }

    #[test]
    fn test_reset_sound_low_loop() {
        let mut sounds = TemporalSounds::new();
        sounds.set_loop_count(2);
        assert_eq!(sounds.get_reset_sound(), "reset_soft_wind");
    }

    #[test]
    fn test_reset_sound_medium_loop() {
        let mut sounds = TemporalSounds::new();
        sounds.set_loop_count(7);
        assert_eq!(sounds.get_reset_sound(), "reset_intense_whoosh");
    }

    #[test]
    fn test_reset_sound_high_loop() {
        let mut sounds = TemporalSounds::new();
        sounds.set_loop_count(15);
        assert_eq!(sounds.get_reset_sound(), "reset_dramatic_thunder");
    }

    #[test]
    fn test_paradox_sound_none() {
        let mut sounds = TemporalSounds::new();
        sounds.set_paradox_level(0.1);
        assert!(sounds.get_paradox_sound().is_none());
    }

    #[test]
    fn test_paradox_sound_low() {
        let mut sounds = TemporalSounds::new();
        sounds.set_paradox_level(0.3);
        assert_eq!(sounds.get_paradox_sound(), Some("paradox_hum_low"));
    }

    #[test]
    fn test_paradox_sound_critical() {
        let mut sounds = TemporalSounds::new();
        sounds.set_paradox_level(0.9);
        assert_eq!(sounds.get_paradox_sound(), Some("paradox_alarm_critical"));
    }

    #[test]
    fn test_paradox_volume() {
        let mut sounds = TemporalSounds::new();
        sounds.set_paradox_level(0.1);
        assert!((sounds.get_paradox_volume() - 0.0).abs() < f32::EPSILON);

        sounds.set_paradox_level(0.5);
        assert!(sounds.get_paradox_volume() > 0.0);
    }

    #[test]
    fn test_chest_sound_open() {
        let sounds = TemporalSounds::new();
        assert_eq!(sounds.get_chest_sound("open"), "chest_temporal_open");
    }

    #[test]
    fn test_chest_sound_deposit() {
        let sounds = TemporalSounds::new();
        assert!(sounds.get_chest_sound("deposit").contains("deposit"));
    }

    #[test]
    fn test_chest_sound_persist() {
        let sounds = TemporalSounds::new();
        assert!(sounds.get_chest_sound("persist").contains("persist"));
    }

    #[test]
    fn test_chest_ambient_with_items() {
        let sounds = TemporalSounds::new();
        assert!(sounds.get_chest_ambient(true).contains("active"));
    }

    #[test]
    fn test_chest_ambient_empty() {
        let sounds = TemporalSounds::new();
        assert!(sounds.get_chest_ambient(false).contains("idle"));
    }

    #[test]
    fn test_dilation_sound_slow() {
        let sounds = TemporalSounds::new();
        assert!(sounds.get_dilation_sound(0.3).contains("slow"));
    }

    #[test]
    fn test_dilation_sound_fast() {
        let sounds = TemporalSounds::new();
        assert!(sounds.get_dilation_sound(2.0).contains("fast"));
    }

    #[test]
    fn test_creature_ability_sound() {
        let sounds = TemporalSounds::new();
        assert!(sounds.get_creature_ability_sound("time_wraith", "chronal_drain").contains("chronal"));
        assert!(sounds.get_creature_ability_sound("echo_beast", "resonance").contains("resonance"));
    }

    #[test]
    fn test_dawn_sound() {
        let sounds = TemporalSounds::new();
        assert!(sounds.get_dawn_sound().contains("dawn"));
    }

    #[test]
    fn test_midnight_sound() {
        let sounds = TemporalSounds::new();
        assert!(sounds.get_midnight_sound().contains("midnight"));
    }

    #[test]
    fn test_phase_transition_sound() {
        let sounds = TemporalSounds::new();
        assert!(sounds.get_phase_transition_sound("dusk").contains("dusk"));
    }

    #[test]
    fn test_milestone_sound() {
        let sounds = TemporalSounds::new();
        assert!(sounds.get_milestone_sound(5).is_some());
        assert!(sounds.get_milestone_sound(3).is_none());
    }

    #[test]
    fn test_death_sound_first() {
        let sounds = TemporalSounds::new();
        assert!(sounds.get_death_sound(0).contains("first"));
    }

    #[test]
    fn test_death_sound_familiar() {
        let sounds = TemporalSounds::new();
        assert!(sounds.get_death_sound(10).contains("familiar"));
    }

    #[test]
    fn test_trap_discovery_known() {
        let sounds = TemporalSounds::new();
        assert!(sounds.get_trap_discovery_sound(true).contains("recognition"));
    }

    #[test]
    fn test_trap_discovery_new() {
        let sounds = TemporalSounds::new();
        assert!(sounds.get_trap_discovery_sound(false).contains("surprise"));
    }

    #[test]
    fn test_knowledge_unlock_sound() {
        let sounds = TemporalSounds::new();
        assert!(sounds.get_knowledge_unlock_sound().contains("unlock") || sounds.get_knowledge_unlock_sound().contains("revelation"));
    }

    #[test]
    fn test_persistence_sound() {
        let sounds = TemporalSounds::new();
        assert!(sounds.get_persistence_sound().contains("persistence") || sounds.get_persistence_sound().contains("anchor"));
    }

    #[test]
    fn test_standalone_get_clock_sound() {
        assert_eq!(get_clock_sound(0.9), "clock_tick_slow");
    }

    #[test]
    fn test_standalone_get_reset_sound() {
        assert!(get_reset_sound(15).contains("dramatic") || get_reset_sound(15).contains("thunder"));
    }

    #[test]
    fn test_standalone_get_paradox_sound() {
        assert!(get_paradox_sound(0.5).is_some());
        assert!(get_paradox_sound(0.1).is_none());
    }

    #[test]
    fn test_standalone_get_chest_sound() {
        assert!(get_chest_sound("open").contains("open"));
    }
}
