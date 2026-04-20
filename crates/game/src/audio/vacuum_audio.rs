//! Vacuum audio effects.
//!
//! Provides muffled sounds, radio chatter, and silence for vacuum environments.

use serde::{Deserialize, Serialize};

/// Audio state for vacuum environments.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum VacuumAudioState {
    /// Normal audio propagation.
    Normal,
    /// Muffled through suit/walls.
    Muffled,
    /// Complete silence (hard vacuum).
    Silent,
}

impl VacuumAudioState {
    /// Get display name.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            VacuumAudioState::Normal => "Normal",
            VacuumAudioState::Muffled => "Muffled",
            VacuumAudioState::Silent => "Silent",
        }
    }

    /// Get volume multiplier.
    #[must_use]
    pub fn volume_multiplier(&self) -> f32 {
        match self {
            VacuumAudioState::Normal => 1.0,
            VacuumAudioState::Muffled => 0.3,
            VacuumAudioState::Silent => 0.0,
        }
    }

    /// Get low-pass filter cutoff (Hz).
    #[must_use]
    pub fn lowpass_cutoff(&self) -> f32 {
        match self {
            VacuumAudioState::Normal => 20000.0,
            VacuumAudioState::Muffled => 800.0,
            VacuumAudioState::Silent => 0.0,
        }
    }

    /// Check if radio can be heard.
    #[must_use]
    pub fn radio_audible(&self) -> bool {
        true // Radio always works
    }
}

/// A radio chatter message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RadioChatter {
    /// Message content.
    message: String,
    /// Speaker name.
    speaker: String,
    /// Duration remaining.
    duration: f32,
    /// Volume.
    volume: f32,
    /// Static/interference level (0-1).
    static_level: f32,
}

impl RadioChatter {
    /// Create new radio chatter.
    #[must_use]
    pub fn new(speaker: String, message: String, duration: f32) -> Self {
        Self {
            message,
            speaker,
            duration,
            volume: 1.0,
            static_level: 0.1,
        }
    }

    /// Create with static interference.
    #[must_use]
    pub fn with_static(mut self, static_level: f32) -> Self {
        self.static_level = static_level.clamp(0.0, 1.0);
        self
    }

    /// Get message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Get speaker.
    #[must_use]
    pub fn speaker(&self) -> &str {
        &self.speaker
    }

    /// Get duration remaining.
    #[must_use]
    pub fn duration(&self) -> f32 {
        self.duration
    }

    /// Get volume.
    #[must_use]
    pub fn volume(&self) -> f32 {
        self.volume
    }

    /// Get static level.
    #[must_use]
    pub fn static_level(&self) -> f32 {
        self.static_level
    }

    /// Check if still playing.
    #[must_use]
    pub fn is_playing(&self) -> bool {
        self.duration > 0.0
    }

    /// Update chatter.
    pub fn tick(&mut self, dt: f32) {
        self.duration -= dt;
    }
}

/// Manages vacuum audio state and effects.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VacuumAudio {
    /// Current audio state.
    state: VacuumAudioState,
    /// Active radio chatter.
    radio_queue: Vec<RadioChatter>,
    /// Suit breathing sound volume.
    breathing_volume: f32,
    /// Helmet ringing volume (after impacts).
    ringing_volume: f32,
    /// Ringing decay rate.
    ringing_decay: f32,
}

impl Default for VacuumAudio {
    fn default() -> Self {
        Self::new()
    }
}

impl VacuumAudio {
    /// Create new vacuum audio manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: VacuumAudioState::Normal,
            radio_queue: Vec::new(),
            breathing_volume: 0.0,
            ringing_volume: 0.0,
            ringing_decay: 2.0,
        }
    }

    /// Get current state.
    #[must_use]
    pub fn state(&self) -> VacuumAudioState {
        self.state
    }

    /// Set audio state.
    pub fn set_state(&mut self, state: VacuumAudioState) {
        self.state = state;
    }

    /// Enter vacuum (enable suit sounds).
    pub fn enter_vacuum(&mut self) {
        self.state = VacuumAudioState::Silent;
        self.breathing_volume = 0.5;
    }

    /// Exit vacuum (return to normal).
    pub fn exit_vacuum(&mut self) {
        self.state = VacuumAudioState::Normal;
        self.breathing_volume = 0.0;
    }

    /// Set muffled state (partial atmosphere).
    pub fn set_muffled(&mut self) {
        self.state = VacuumAudioState::Muffled;
    }

    /// Queue radio chatter.
    pub fn queue_radio(&mut self, chatter: RadioChatter) {
        self.radio_queue.push(chatter);
    }

    /// Get current radio message (if any).
    #[must_use]
    pub fn current_radio(&self) -> Option<&RadioChatter> {
        self.radio_queue.first()
    }

    /// Check if radio is playing.
    #[must_use]
    pub fn radio_playing(&self) -> bool {
        self.radio_queue.first().is_some_and(|r| r.is_playing())
    }

    /// Get breathing volume.
    #[must_use]
    pub fn breathing_volume(&self) -> f32 {
        self.breathing_volume
    }

    /// Set breathing volume (for exertion).
    pub fn set_breathing_volume(&mut self, volume: f32) {
        self.breathing_volume = volume.clamp(0.0, 1.0);
    }

    /// Get ringing volume.
    #[must_use]
    pub fn ringing_volume(&self) -> f32 {
        self.ringing_volume
    }

    /// Trigger helmet ringing (from impact).
    pub fn trigger_ringing(&mut self, intensity: f32) {
        self.ringing_volume = intensity.clamp(0.0, 1.0);
    }

    /// Get volume multiplier for external sounds.
    #[must_use]
    pub fn external_volume_multiplier(&self) -> f32 {
        self.state.volume_multiplier()
    }

    /// Get low-pass filter cutoff.
    #[must_use]
    pub fn lowpass_cutoff(&self) -> f32 {
        self.state.lowpass_cutoff()
    }

    /// Check if in silent state.
    #[must_use]
    pub fn is_silent(&self) -> bool {
        self.state == VacuumAudioState::Silent
    }

    /// Check if in muffled state.
    #[must_use]
    pub fn is_muffled(&self) -> bool {
        self.state == VacuumAudioState::Muffled
    }

    /// Update audio state.
    pub fn tick(&mut self, dt: f32) {
        // Update radio queue
        if let Some(chatter) = self.radio_queue.first_mut() {
            chatter.tick(dt);
            if !chatter.is_playing() {
                self.radio_queue.remove(0);
            }
        }

        // Decay ringing
        if self.ringing_volume > 0.0 {
            self.ringing_volume = (self.ringing_volume - dt * self.ringing_decay).max(0.0);
        }
    }

    /// Clear radio queue.
    pub fn clear_radio(&mut self) {
        self.radio_queue.clear();
    }

    /// Get radio queue length.
    #[must_use]
    pub fn radio_queue_length(&self) -> usize {
        self.radio_queue.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vacuum_audio_state_display_names() {
        assert_eq!(VacuumAudioState::Normal.display_name(), "Normal");
        assert_eq!(VacuumAudioState::Muffled.display_name(), "Muffled");
        assert_eq!(VacuumAudioState::Silent.display_name(), "Silent");
    }

    #[test]
    fn test_vacuum_audio_state_volume_multiplier() {
        assert!((VacuumAudioState::Normal.volume_multiplier() - 1.0).abs() < f32::EPSILON);
        assert!((VacuumAudioState::Muffled.volume_multiplier() - 0.3).abs() < f32::EPSILON);
        assert!((VacuumAudioState::Silent.volume_multiplier() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_vacuum_audio_state_lowpass() {
        assert!(VacuumAudioState::Normal.lowpass_cutoff() > VacuumAudioState::Muffled.lowpass_cutoff());
        assert!(VacuumAudioState::Muffled.lowpass_cutoff() > VacuumAudioState::Silent.lowpass_cutoff());
    }

    #[test]
    fn test_vacuum_audio_state_radio() {
        assert!(VacuumAudioState::Normal.radio_audible());
        assert!(VacuumAudioState::Muffled.radio_audible());
        assert!(VacuumAudioState::Silent.radio_audible());
    }

    #[test]
    fn test_radio_chatter_new() {
        let chatter = RadioChatter::new("Control".to_string(), "Status report".to_string(), 3.0);
        assert_eq!(chatter.speaker(), "Control");
        assert_eq!(chatter.message(), "Status report");
        assert!((chatter.duration() - 3.0).abs() < f32::EPSILON);
        assert!(chatter.is_playing());
    }

    #[test]
    fn test_radio_chatter_with_static() {
        let chatter = RadioChatter::new("Control".to_string(), "Test".to_string(), 1.0)
            .with_static(0.5);
        assert!((chatter.static_level() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_radio_chatter_tick() {
        let mut chatter = RadioChatter::new("Control".to_string(), "Test".to_string(), 1.0);
        chatter.tick(0.5);
        assert!((chatter.duration() - 0.5).abs() < f32::EPSILON);

        chatter.tick(1.0);
        assert!(!chatter.is_playing());
    }

    #[test]
    fn test_vacuum_audio_new() {
        let audio = VacuumAudio::new();
        assert_eq!(audio.state(), VacuumAudioState::Normal);
        assert!(!audio.is_silent());
        assert!(!audio.radio_playing());
    }

    #[test]
    fn test_vacuum_audio_enter_vacuum() {
        let mut audio = VacuumAudio::new();
        audio.enter_vacuum();

        assert!(audio.is_silent());
        assert!(audio.breathing_volume() > 0.0);
    }

    #[test]
    fn test_vacuum_audio_exit_vacuum() {
        let mut audio = VacuumAudio::new();
        audio.enter_vacuum();
        audio.exit_vacuum();

        assert!(!audio.is_silent());
        assert!((audio.breathing_volume() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_vacuum_audio_set_muffled() {
        let mut audio = VacuumAudio::new();
        audio.set_muffled();

        assert!(audio.is_muffled());
    }

    #[test]
    fn test_vacuum_audio_queue_radio() {
        let mut audio = VacuumAudio::new();
        audio.queue_radio(RadioChatter::new("A".to_string(), "Test".to_string(), 1.0));
        audio.queue_radio(RadioChatter::new("B".to_string(), "Test2".to_string(), 1.0));

        assert!(audio.radio_playing());
        assert_eq!(audio.radio_queue_length(), 2);
        assert_eq!(audio.current_radio().unwrap().speaker(), "A");
    }

    #[test]
    fn test_vacuum_audio_radio_progression() {
        let mut audio = VacuumAudio::new();
        audio.queue_radio(RadioChatter::new("A".to_string(), "Test".to_string(), 1.0));
        audio.queue_radio(RadioChatter::new("B".to_string(), "Test2".to_string(), 1.0));

        audio.tick(1.5); // First message done
        assert_eq!(audio.current_radio().unwrap().speaker(), "B");
        assert_eq!(audio.radio_queue_length(), 1);
    }

    #[test]
    fn test_vacuum_audio_breathing() {
        let mut audio = VacuumAudio::new();
        audio.set_breathing_volume(0.7);
        assert!((audio.breathing_volume() - 0.7).abs() < f32::EPSILON);

        audio.set_breathing_volume(2.0); // Clamped
        assert!((audio.breathing_volume() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_vacuum_audio_ringing() {
        let mut audio = VacuumAudio::new();
        audio.trigger_ringing(0.8);
        assert!((audio.ringing_volume() - 0.8).abs() < f32::EPSILON);

        audio.tick(0.2);
        assert!(audio.ringing_volume() < 0.8);
    }

    #[test]
    fn test_vacuum_audio_ringing_decay() {
        let mut audio = VacuumAudio::new();
        audio.trigger_ringing(1.0);
        audio.tick(1.0); // Decay for 1 second at rate 2.0
        assert!(audio.ringing_volume() <= 0.0);
    }

    #[test]
    fn test_vacuum_audio_external_volume() {
        let mut audio = VacuumAudio::new();
        assert!((audio.external_volume_multiplier() - 1.0).abs() < f32::EPSILON);

        audio.enter_vacuum();
        assert!((audio.external_volume_multiplier() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_vacuum_audio_clear_radio() {
        let mut audio = VacuumAudio::new();
        audio.queue_radio(RadioChatter::new("A".to_string(), "Test".to_string(), 1.0));
        audio.clear_radio();

        assert!(!audio.radio_playing());
        assert_eq!(audio.radio_queue_length(), 0);
    }

    #[test]
    fn test_vacuum_audio_default() {
        let audio = VacuumAudio::default();
        assert_eq!(audio.state(), VacuumAudioState::Normal);
    }
}
