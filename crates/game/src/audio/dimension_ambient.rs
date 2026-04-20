//! Dimension ambient audio.
//!
//! Provides ambient sound configurations for each dimension.

use engine_physics::dimension::Dimension;

/// Handler for dimension-specific ambient sounds.
#[derive(Clone, Debug, Default)]
pub struct DimensionAmbient;

impl DimensionAmbient {
    /// Create a new dimension ambient handler.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Get the primary ambient sound for a dimension.
    #[must_use]
    pub fn get_ambient_sound(&self, dim: Dimension) -> &'static str {
        match dim {
            Dimension::Prime => "ambient_nature",
            Dimension::Inverted => "ambient_fire_crackle",
            Dimension::Void => "ambient_void_whispers",
            Dimension::Nexus => "ambient_nexus_hum",
        }
    }

    /// Get the volume modifier for a dimension.
    ///
    /// Returns 0.0 to 1.0 volume multiplier.
    #[must_use]
    pub fn volume_modifier(&self, dim: Dimension) -> f32 {
        match dim {
            Dimension::Prime => 1.0,
            Dimension::Inverted => 0.8,
            Dimension::Void => 0.5,     // Quieter, eerie
            Dimension::Nexus => 0.9,
        }
    }

    /// Get the secondary ambient layer for a dimension.
    #[must_use]
    pub fn get_secondary_ambient(&self, dim: Dimension) -> Option<&'static str> {
        match dim {
            Dimension::Prime => Some("ambient_wind"),
            Dimension::Inverted => Some("ambient_lava_bubble"),
            Dimension::Void => None,    // Silence adds to eeriness
            Dimension::Nexus => Some("ambient_dimensional_shift"),
        }
    }

    /// Get the reverb amount for a dimension.
    ///
    /// Returns 0.0 (dry) to 1.0 (full reverb).
    #[must_use]
    pub fn get_reverb(&self, dim: Dimension) -> f32 {
        match dim {
            Dimension::Prime => 0.2,
            Dimension::Inverted => 0.4,
            Dimension::Void => 0.9,     // Echo in the void
            Dimension::Nexus => 0.6,
        }
    }

    /// Get the low-pass filter frequency for a dimension.
    ///
    /// Higher values = more high frequencies allowed.
    #[must_use]
    pub fn get_lowpass_freq(&self, dim: Dimension) -> f32 {
        match dim {
            Dimension::Prime => 20000.0, // No filtering
            Dimension::Inverted => 18000.0,
            Dimension::Void => 8000.0,   // Muffled
            Dimension::Nexus => 15000.0,
        }
    }

    /// Check if dimension has random ambient events.
    #[must_use]
    pub fn has_random_sounds(&self, dim: Dimension) -> bool {
        !matches!(dim, Dimension::Void)
    }

    /// Get random ambient sound events for a dimension.
    #[must_use]
    pub fn get_random_sounds(&self, dim: Dimension) -> Vec<&'static str> {
        match dim {
            Dimension::Prime => vec![
                "bird_chirp",
                "wind_gust",
                "leaves_rustle",
                "distant_animal",
            ],
            Dimension::Inverted => vec![
                "fire_flare",
                "rock_crack",
                "heat_sizzle",
                "ember_pop",
            ],
            Dimension::Void => vec![], // No random sounds, just silence
            Dimension::Nexus => vec![
                "reality_ripple",
                "crystal_chime",
                "dimensional_echo",
                "nexus_pulse",
            ],
        }
    }

    /// Get the pitch modifier for a dimension.
    ///
    /// Returns multiplier (0.5 = octave down, 2.0 = octave up).
    #[must_use]
    pub fn get_pitch_modifier(&self, dim: Dimension) -> f32 {
        match dim {
            Dimension::Prime => 1.0,
            Dimension::Inverted => 0.95,  // Slightly lower
            Dimension::Void => 0.8,       // Deeper tones
            Dimension::Nexus => 1.05,     // Slightly higher
        }
    }

    /// Get the music track for a dimension.
    #[must_use]
    pub fn get_music_track(&self, dim: Dimension) -> &'static str {
        match dim {
            Dimension::Prime => "music_prime_peaceful",
            Dimension::Inverted => "music_inverted_tense",
            Dimension::Void => "music_void_dread",
            Dimension::Nexus => "music_nexus_mysterious",
        }
    }
}

/// Get ambient sound for a dimension (standalone function).
#[must_use]
pub fn get_ambient_sound(dim: Dimension) -> &'static str {
    DimensionAmbient::new().get_ambient_sound(dim)
}

/// Get volume modifier for a dimension (standalone function).
#[must_use]
pub fn volume_modifier(dim: Dimension) -> f32 {
    DimensionAmbient::new().volume_modifier(dim)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ambient_sound() {
        let ambient = DimensionAmbient::new();

        assert_eq!(ambient.get_ambient_sound(Dimension::Prime), "ambient_nature");
        assert_eq!(ambient.get_ambient_sound(Dimension::Void), "ambient_void_whispers");
    }

    #[test]
    fn test_volume_modifier() {
        let ambient = DimensionAmbient::new();

        assert!((ambient.volume_modifier(Dimension::Prime) - 1.0).abs() < f32::EPSILON);
        assert!(ambient.volume_modifier(Dimension::Void) < ambient.volume_modifier(Dimension::Prime));
    }

    #[test]
    fn test_secondary_ambient() {
        let ambient = DimensionAmbient::new();

        assert!(ambient.get_secondary_ambient(Dimension::Prime).is_some());
        assert!(ambient.get_secondary_ambient(Dimension::Void).is_none());
    }

    #[test]
    fn test_reverb() {
        let ambient = DimensionAmbient::new();

        assert!(ambient.get_reverb(Dimension::Void) > ambient.get_reverb(Dimension::Prime));
    }

    #[test]
    fn test_lowpass_freq() {
        let ambient = DimensionAmbient::new();

        assert!(ambient.get_lowpass_freq(Dimension::Prime) > ambient.get_lowpass_freq(Dimension::Void));
    }

    #[test]
    fn test_random_sounds() {
        let ambient = DimensionAmbient::new();

        assert!(ambient.has_random_sounds(Dimension::Prime));
        assert!(!ambient.has_random_sounds(Dimension::Void));

        let prime_sounds = ambient.get_random_sounds(Dimension::Prime);
        assert!(!prime_sounds.is_empty());

        let void_sounds = ambient.get_random_sounds(Dimension::Void);
        assert!(void_sounds.is_empty());
    }

    #[test]
    fn test_pitch_modifier() {
        let ambient = DimensionAmbient::new();

        assert!((ambient.get_pitch_modifier(Dimension::Prime) - 1.0).abs() < f32::EPSILON);
        assert!(ambient.get_pitch_modifier(Dimension::Void) < 1.0);
    }

    #[test]
    fn test_music_track() {
        let ambient = DimensionAmbient::new();

        assert!(ambient.get_music_track(Dimension::Prime).contains("prime"));
        assert!(ambient.get_music_track(Dimension::Void).contains("void"));
    }

    #[test]
    fn test_standalone_functions() {
        assert_eq!(get_ambient_sound(Dimension::Prime), "ambient_nature");
        assert!((volume_modifier(Dimension::Prime) - 1.0).abs() < f32::EPSILON);
    }
}
