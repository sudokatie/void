//! Vacuum rendering state and effects.
//!
//! Provides visual effects for vacuum environments and hull breaches.

use glam::Vec3;
use serde::{Deserialize, Serialize};

/// Render state for vacuum environments.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum VacuumRenderState {
    /// Normal atmosphere with particles.
    Atmosphere,
    /// Partial vacuum (low pressure).
    PartialVacuum,
    /// Complete vacuum (no particles).
    Vacuum,
}

impl VacuumRenderState {
    /// Get display name.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            VacuumRenderState::Atmosphere => "Atmosphere",
            VacuumRenderState::PartialVacuum => "Partial Vacuum",
            VacuumRenderState::Vacuum => "Vacuum",
        }
    }

    /// Get particle density multiplier (0-1).
    #[must_use]
    pub fn particle_density(&self) -> f32 {
        match self {
            VacuumRenderState::Atmosphere => 1.0,
            VacuumRenderState::PartialVacuum => 0.3,
            VacuumRenderState::Vacuum => 0.0,
        }
    }

    /// Get ambient occlusion strength.
    #[must_use]
    pub fn ao_strength(&self) -> f32 {
        match self {
            VacuumRenderState::Atmosphere => 0.5,
            VacuumRenderState::PartialVacuum => 0.3,
            VacuumRenderState::Vacuum => 0.1,
        }
    }

    /// Check if sound propagates normally.
    #[must_use]
    pub fn sound_propagates(&self) -> bool {
        matches!(self, VacuumRenderState::Atmosphere)
    }

    /// Get fog density.
    #[must_use]
    pub fn fog_density(&self) -> f32 {
        match self {
            VacuumRenderState::Atmosphere => 0.02,
            VacuumRenderState::PartialVacuum => 0.005,
            VacuumRenderState::Vacuum => 0.0,
        }
    }
}

/// A glowing breach effect.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BreachGlow {
    /// Position of the breach.
    position: Vec3,
    /// Glow intensity (0-1).
    intensity: f32,
    /// Glow radius.
    radius: f32,
    /// Color (RGB).
    color: [f32; 3],
    /// Pulse phase.
    pulse_phase: f32,
    /// Whether the breach is active.
    active: bool,
}

impl BreachGlow {
    /// Create a new breach glow.
    #[must_use]
    pub fn new(position: Vec3, radius: f32) -> Self {
        Self {
            position,
            intensity: 1.0,
            radius,
            color: [0.6, 0.8, 1.0], // Cyan-ish for vacuum
            pulse_phase: 0.0,
            active: true,
        }
    }

    /// Create with custom color.
    #[must_use]
    pub fn with_color(mut self, color: [f32; 3]) -> Self {
        self.color = color;
        self
    }

    /// Get position.
    #[must_use]
    pub fn position(&self) -> Vec3 {
        self.position
    }

    /// Get base intensity.
    #[must_use]
    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    /// Get current pulsing intensity.
    #[must_use]
    pub fn current_intensity(&self) -> f32 {
        let pulse = (self.pulse_phase.sin() * 0.2 + 0.8).clamp(0.0, 1.0);
        self.intensity * pulse
    }

    /// Get radius.
    #[must_use]
    pub fn radius(&self) -> f32 {
        self.radius
    }

    /// Get color.
    #[must_use]
    pub fn color(&self) -> [f32; 3] {
        self.color
    }

    /// Check if active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Set intensity.
    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity.clamp(0.0, 1.0);
    }

    /// Deactivate the glow.
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Update glow state.
    pub fn tick(&mut self, dt: f32) {
        self.pulse_phase += dt * 3.0; // Pulse frequency
        if self.pulse_phase > std::f32::consts::TAU {
            self.pulse_phase -= std::f32::consts::TAU;
        }
    }
}

/// Manages vacuum rendering state for a room.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VacuumRendering {
    /// Current render state.
    state: VacuumRenderState,
    /// Active breach glows.
    breach_glows: Vec<BreachGlow>,
    /// Transition progress (0-1).
    transition_progress: f32,
    /// Target state for transition.
    target_state: VacuumRenderState,
    /// Transition speed.
    transition_speed: f32,
}

impl Default for VacuumRendering {
    fn default() -> Self {
        Self::new()
    }
}

impl VacuumRendering {
    /// Create a new vacuum rendering manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: VacuumRenderState::Atmosphere,
            breach_glows: Vec::new(),
            transition_progress: 1.0,
            target_state: VacuumRenderState::Atmosphere,
            transition_speed: 1.0,
        }
    }

    /// Get current state.
    #[must_use]
    pub fn state(&self) -> VacuumRenderState {
        self.state
    }

    /// Set state immediately.
    pub fn set_state(&mut self, state: VacuumRenderState) {
        self.state = state;
        self.target_state = state;
        self.transition_progress = 1.0;
    }

    /// Start transitioning to a new state.
    pub fn transition_to(&mut self, state: VacuumRenderState, speed: f32) {
        if self.state != state {
            self.target_state = state;
            self.transition_progress = 0.0;
            self.transition_speed = speed.max(0.1);
        }
    }

    /// Check if transitioning.
    #[must_use]
    pub fn is_transitioning(&self) -> bool {
        self.transition_progress < 1.0
    }

    /// Get transition progress (0-1).
    #[must_use]
    pub fn transition_progress(&self) -> f32 {
        self.transition_progress
    }

    /// Add a breach glow.
    pub fn add_breach_glow(&mut self, glow: BreachGlow) {
        self.breach_glows.push(glow);
    }

    /// Remove a breach glow at position.
    pub fn remove_breach_glow(&mut self, position: Vec3) {
        self.breach_glows.retain(|g| (g.position - position).length() > 0.1);
    }

    /// Get all breach glows.
    #[must_use]
    pub fn breach_glows(&self) -> &[BreachGlow] {
        &self.breach_glows
    }

    /// Get breach glow count.
    #[must_use]
    pub fn breach_count(&self) -> usize {
        self.breach_glows.len()
    }

    /// Get interpolated particle density during transition.
    #[must_use]
    pub fn current_particle_density(&self) -> f32 {
        if self.is_transitioning() {
            let from = self.state.particle_density();
            let to = self.target_state.particle_density();
            from + (to - from) * self.transition_progress
        } else {
            self.state.particle_density()
        }
    }

    /// Get interpolated fog density during transition.
    #[must_use]
    pub fn current_fog_density(&self) -> f32 {
        if self.is_transitioning() {
            let from = self.state.fog_density();
            let to = self.target_state.fog_density();
            from + (to - from) * self.transition_progress
        } else {
            self.state.fog_density()
        }
    }

    /// Update rendering state.
    pub fn tick(&mut self, dt: f32) {
        // Update transition
        if self.is_transitioning() {
            self.transition_progress += dt * self.transition_speed;
            if self.transition_progress >= 1.0 {
                self.transition_progress = 1.0;
                self.state = self.target_state;
            }
        }

        // Update breach glows
        for glow in &mut self.breach_glows {
            glow.tick(dt);
        }

        // Remove inactive glows
        self.breach_glows.retain(|g| g.is_active());
    }

    /// Clear all breach glows.
    pub fn clear_breaches(&mut self) {
        self.breach_glows.clear();
    }

    /// Check if in vacuum state.
    #[must_use]
    pub fn is_vacuum(&self) -> bool {
        self.state == VacuumRenderState::Vacuum
    }

    /// Check if in atmosphere state.
    #[must_use]
    pub fn is_atmosphere(&self) -> bool {
        self.state == VacuumRenderState::Atmosphere
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vacuum_render_state_display_names() {
        assert_eq!(VacuumRenderState::Atmosphere.display_name(), "Atmosphere");
        assert_eq!(VacuumRenderState::PartialVacuum.display_name(), "Partial Vacuum");
        assert_eq!(VacuumRenderState::Vacuum.display_name(), "Vacuum");
    }

    #[test]
    fn test_vacuum_render_state_particle_density() {
        assert!((VacuumRenderState::Atmosphere.particle_density() - 1.0).abs() < f32::EPSILON);
        assert!((VacuumRenderState::PartialVacuum.particle_density() - 0.3).abs() < f32::EPSILON);
        assert!((VacuumRenderState::Vacuum.particle_density() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_vacuum_render_state_sound() {
        assert!(VacuumRenderState::Atmosphere.sound_propagates());
        assert!(!VacuumRenderState::PartialVacuum.sound_propagates());
        assert!(!VacuumRenderState::Vacuum.sound_propagates());
    }

    #[test]
    fn test_vacuum_render_state_fog() {
        assert!(VacuumRenderState::Atmosphere.fog_density() > VacuumRenderState::PartialVacuum.fog_density());
        assert!(VacuumRenderState::PartialVacuum.fog_density() > VacuumRenderState::Vacuum.fog_density());
    }

    #[test]
    fn test_breach_glow_new() {
        let glow = BreachGlow::new(Vec3::ZERO, 5.0);
        assert_eq!(glow.position(), Vec3::ZERO);
        assert!((glow.radius() - 5.0).abs() < f32::EPSILON);
        assert!(glow.is_active());
    }

    #[test]
    fn test_breach_glow_with_color() {
        let glow = BreachGlow::new(Vec3::ZERO, 5.0).with_color([1.0, 0.0, 0.0]);
        assert_eq!(glow.color(), [1.0, 0.0, 0.0]);
    }

    #[test]
    fn test_breach_glow_tick() {
        let mut glow = BreachGlow::new(Vec3::ZERO, 5.0);
        let initial_phase = glow.pulse_phase;
        glow.tick(1.0);
        assert!(glow.pulse_phase > initial_phase);
    }

    #[test]
    fn test_breach_glow_intensity() {
        let mut glow = BreachGlow::new(Vec3::ZERO, 5.0);
        glow.set_intensity(0.5);
        assert!((glow.intensity() - 0.5).abs() < f32::EPSILON);

        glow.set_intensity(2.0); // Clamped
        assert!((glow.intensity() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_breach_glow_deactivate() {
        let mut glow = BreachGlow::new(Vec3::ZERO, 5.0);
        glow.deactivate();
        assert!(!glow.is_active());
    }

    #[test]
    fn test_breach_glow_current_intensity() {
        let glow = BreachGlow::new(Vec3::ZERO, 5.0);
        // Current intensity varies with pulse
        let intensity = glow.current_intensity();
        assert!(intensity > 0.0 && intensity <= 1.0);
    }

    #[test]
    fn test_vacuum_rendering_new() {
        let vr = VacuumRendering::new();
        assert_eq!(vr.state(), VacuumRenderState::Atmosphere);
        assert!(!vr.is_transitioning());
        assert!(vr.is_atmosphere());
    }

    #[test]
    fn test_vacuum_rendering_set_state() {
        let mut vr = VacuumRendering::new();
        vr.set_state(VacuumRenderState::Vacuum);

        assert_eq!(vr.state(), VacuumRenderState::Vacuum);
        assert!(vr.is_vacuum());
        assert!(!vr.is_transitioning());
    }

    #[test]
    fn test_vacuum_rendering_transition() {
        let mut vr = VacuumRendering::new();
        vr.transition_to(VacuumRenderState::Vacuum, 1.0);

        assert!(vr.is_transitioning());
        assert_eq!(vr.state(), VacuumRenderState::Atmosphere); // Still in old state

        vr.tick(0.5);
        assert!(vr.is_transitioning());
        assert!((vr.transition_progress() - 0.5).abs() < f32::EPSILON);

        vr.tick(0.6);
        assert!(!vr.is_transitioning());
        assert_eq!(vr.state(), VacuumRenderState::Vacuum);
    }

    #[test]
    fn test_vacuum_rendering_add_breach() {
        let mut vr = VacuumRendering::new();
        vr.add_breach_glow(BreachGlow::new(Vec3::ZERO, 5.0));

        assert_eq!(vr.breach_count(), 1);
        assert_eq!(vr.breach_glows().len(), 1);
    }

    #[test]
    fn test_vacuum_rendering_remove_breach() {
        let mut vr = VacuumRendering::new();
        vr.add_breach_glow(BreachGlow::new(Vec3::ZERO, 5.0));
        vr.add_breach_glow(BreachGlow::new(Vec3::ONE, 3.0));

        vr.remove_breach_glow(Vec3::ZERO);
        assert_eq!(vr.breach_count(), 1);
    }

    #[test]
    fn test_vacuum_rendering_clear_breaches() {
        let mut vr = VacuumRendering::new();
        vr.add_breach_glow(BreachGlow::new(Vec3::ZERO, 5.0));
        vr.add_breach_glow(BreachGlow::new(Vec3::ONE, 3.0));

        vr.clear_breaches();
        assert_eq!(vr.breach_count(), 0);
    }

    #[test]
    fn test_vacuum_rendering_interpolated_density() {
        let mut vr = VacuumRendering::new();
        vr.transition_to(VacuumRenderState::Vacuum, 1.0);
        vr.tick(0.5); // 50% through transition

        let density = vr.current_particle_density();
        assert!(density > 0.0 && density < 1.0); // Between atmosphere and vacuum
    }

    #[test]
    fn test_vacuum_rendering_interpolated_fog() {
        let mut vr = VacuumRendering::new();
        vr.transition_to(VacuumRenderState::Vacuum, 1.0);
        vr.tick(0.5);

        let fog = vr.current_fog_density();
        assert!(fog > 0.0 && fog < 0.02); // Between atmosphere and vacuum fog
    }

    #[test]
    fn test_vacuum_rendering_tick_cleans_inactive() {
        let mut vr = VacuumRendering::new();
        let mut glow = BreachGlow::new(Vec3::ZERO, 5.0);
        glow.deactivate();
        vr.breach_glows.push(glow);

        vr.tick(0.1);
        assert_eq!(vr.breach_count(), 0);
    }

    #[test]
    fn test_vacuum_rendering_default() {
        let vr = VacuumRendering::default();
        assert!(vr.is_atmosphere());
    }

    #[test]
    fn test_vacuum_rendering_no_transition_same_state() {
        let mut vr = VacuumRendering::new();
        vr.transition_to(VacuumRenderState::Atmosphere, 1.0);
        assert!(!vr.is_transitioning()); // Already in that state
    }
}
