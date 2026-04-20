//! Audio systems for time-loop survival gameplay.
//!
//! Provides temporal sounds, environmental audio, and creature audio.

mod dimension_ambient;
mod environment;
mod fracture_audio;
mod temporal_sounds;
mod titan_sounds;

pub use dimension_ambient::{get_ambient_sound, volume_modifier, DimensionAmbient};
pub use environment::{
    get_parasite_sound, get_settling_sound, get_vent_sound, get_wind_sound, EnvironmentAudio,
};
pub use fracture_audio::{get_creature_sound, get_fracture_sound, FractureAudio};
pub use temporal_sounds::{
    get_chest_sound, get_clock_sound, get_paradox_sound, get_reset_sound, TemporalSounds,
};
pub use titan_sounds::{
    get_breathing_sound, get_heartbeat_sound, get_movement_sound, TitanSounds,
};
