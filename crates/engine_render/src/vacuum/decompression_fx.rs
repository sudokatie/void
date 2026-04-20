//! Decompression visual effects.
//!
//! Provides air venting particles and frost effects for decompression events.

use glam::Vec3;
use serde::{Deserialize, Serialize};

/// A single venting particle for air escape visualization.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VentingParticle {
    /// Position in world space.
    position: Vec3,
    /// Velocity vector.
    velocity: Vec3,
    /// Particle lifetime remaining.
    lifetime: f32,
    /// Maximum lifetime.
    max_lifetime: f32,
    /// Particle size.
    size: f32,
    /// Opacity (0-1).
    opacity: f32,
}

impl VentingParticle {
    /// Create a new venting particle.
    #[must_use]
    pub fn new(position: Vec3, velocity: Vec3, size: f32) -> Self {
        Self {
            position,
            velocity,
            lifetime: 2.0,
            max_lifetime: 2.0,
            size,
            opacity: 1.0,
        }
    }

    /// Create a particle with custom lifetime.
    #[must_use]
    pub fn with_lifetime(mut self, lifetime: f32) -> Self {
        self.lifetime = lifetime;
        self.max_lifetime = lifetime;
        self
    }

    /// Get position.
    #[must_use]
    pub fn position(&self) -> Vec3 {
        self.position
    }

    /// Get velocity.
    #[must_use]
    pub fn velocity(&self) -> Vec3 {
        self.velocity
    }

    /// Get remaining lifetime.
    #[must_use]
    pub fn lifetime(&self) -> f32 {
        self.lifetime
    }

    /// Get particle size.
    #[must_use]
    pub fn size(&self) -> f32 {
        self.size
    }

    /// Get current opacity.
    #[must_use]
    pub fn opacity(&self) -> f32 {
        self.opacity
    }

    /// Check if particle is alive.
    #[must_use]
    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }

    /// Get life progress (0 = just spawned, 1 = dead).
    #[must_use]
    pub fn life_progress(&self) -> f32 {
        1.0 - (self.lifetime / self.max_lifetime)
    }

    /// Update particle state.
    pub fn tick(&mut self, dt: f32) {
        self.lifetime -= dt;
        self.position += self.velocity * dt;

        // Fade out over time
        self.opacity = (self.lifetime / self.max_lifetime).max(0.0);

        // Particles expand as they dissipate
        let expansion_rate = 1.0 + self.life_progress() * 2.0;
        self.size *= expansion_rate.powf(dt);
    }
}

/// A frost effect on surfaces near decompression.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FrostEffect {
    /// Center position.
    position: Vec3,
    /// Current radius.
    radius: f32,
    /// Maximum radius.
    max_radius: f32,
    /// Intensity (0-1).
    intensity: f32,
    /// Duration remaining.
    duration: f32,
}

impl FrostEffect {
    /// Create a new frost effect.
    #[must_use]
    pub fn new(position: Vec3, max_radius: f32) -> Self {
        Self {
            position,
            radius: 0.0,
            max_radius,
            intensity: 1.0,
            duration: 5.0,
        }
    }

    /// Create frost with custom duration.
    #[must_use]
    pub fn with_duration(mut self, duration: f32) -> Self {
        self.duration = duration;
        self
    }

    /// Get position.
    #[must_use]
    pub fn position(&self) -> Vec3 {
        self.position
    }

    /// Get current radius.
    #[must_use]
    pub fn radius(&self) -> f32 {
        self.radius
    }

    /// Get intensity.
    #[must_use]
    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    /// Get duration remaining.
    #[must_use]
    pub fn duration(&self) -> f32 {
        self.duration
    }

    /// Check if effect is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.duration > 0.0
    }

    /// Update frost effect.
    pub fn tick(&mut self, dt: f32) {
        self.duration -= dt;

        // Frost spreads quickly then fades
        if self.radius < self.max_radius {
            self.radius = (self.radius + dt * 2.0).min(self.max_radius);
        }

        // Intensity fades over time
        if self.duration < 2.0 {
            self.intensity = (self.duration / 2.0).max(0.0);
        }
    }
}

/// Manages decompression visual effects.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DecompressionFX {
    /// Active venting particles.
    particles: Vec<VentingParticle>,
    /// Active frost effects.
    frost_effects: Vec<FrostEffect>,
    /// Maximum particles allowed.
    max_particles: usize,
}

impl DecompressionFX {
    /// Create a new decompression FX manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            frost_effects: Vec::new(),
            max_particles: 1000,
        }
    }

    /// Create with custom particle limit.
    #[must_use]
    pub fn with_max_particles(max_particles: usize) -> Self {
        Self {
            particles: Vec::new(),
            frost_effects: Vec::new(),
            max_particles,
        }
    }

    /// Spawn venting particles at a breach.
    pub fn spawn_venting(&mut self, position: Vec3, direction: Vec3, count: usize, intensity: f32) {
        let available = self.max_particles.saturating_sub(self.particles.len());
        let to_spawn = count.min(available);

        for i in 0..to_spawn {
            // Add some variation to particle direction and speed
            let angle_offset = (i as f32 * 0.1).sin() * 0.3;
            let speed_variation = 1.0 + (i as f32 * 0.2).cos() * 0.2;

            let velocity = Vec3::new(
                direction.x + angle_offset,
                direction.y + angle_offset * 0.5,
                direction.z,
            )
            .normalize()
                * intensity
                * speed_variation
                * 5.0;

            let size = 0.1 + (i as f32 * 0.1).sin().abs() * 0.1;
            let particle = VentingParticle::new(position, velocity, size);
            self.particles.push(particle);
        }
    }

    /// Spawn a frost effect.
    pub fn spawn_frost(&mut self, position: Vec3, radius: f32, duration: f32) {
        let frost = FrostEffect::new(position, radius).with_duration(duration);
        self.frost_effects.push(frost);
    }

    /// Get active particles.
    #[must_use]
    pub fn particles(&self) -> &[VentingParticle] {
        &self.particles
    }

    /// Get active frost effects.
    #[must_use]
    pub fn frost_effects(&self) -> &[FrostEffect] {
        &self.frost_effects
    }

    /// Get particle count.
    #[must_use]
    pub fn particle_count(&self) -> usize {
        self.particles.len()
    }

    /// Get frost effect count.
    #[must_use]
    pub fn frost_count(&self) -> usize {
        self.frost_effects.len()
    }

    /// Update all effects.
    pub fn tick(&mut self, dt: f32) {
        // Update particles
        for particle in &mut self.particles {
            particle.tick(dt);
        }

        // Remove dead particles
        self.particles.retain(|p| p.is_alive());

        // Update frost effects
        for frost in &mut self.frost_effects {
            frost.tick(dt);
        }

        // Remove inactive frost
        self.frost_effects.retain(|f| f.is_active());
    }

    /// Clear all effects.
    pub fn clear(&mut self) {
        self.particles.clear();
        self.frost_effects.clear();
    }

    /// Check if any effects are active.
    #[must_use]
    pub fn has_active_effects(&self) -> bool {
        !self.particles.is_empty() || !self.frost_effects.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_venting_particle_new() {
        let p = VentingParticle::new(Vec3::ZERO, Vec3::X, 0.5);
        assert_eq!(p.position(), Vec3::ZERO);
        assert_eq!(p.velocity(), Vec3::X);
        assert!((p.size() - 0.5).abs() < f32::EPSILON);
        assert!(p.is_alive());
    }

    #[test]
    fn test_venting_particle_with_lifetime() {
        let p = VentingParticle::new(Vec3::ZERO, Vec3::X, 0.5).with_lifetime(5.0);
        assert!((p.lifetime() - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_venting_particle_tick() {
        let mut p = VentingParticle::new(Vec3::ZERO, Vec3::X, 0.5).with_lifetime(1.0);
        p.tick(0.5);

        assert!(p.position().x > 0.0);
        assert!((p.lifetime() - 0.5).abs() < f32::EPSILON);
        assert!(p.opacity() < 1.0);
    }

    #[test]
    fn test_venting_particle_death() {
        let mut p = VentingParticle::new(Vec3::ZERO, Vec3::X, 0.5).with_lifetime(1.0);
        p.tick(1.5);
        assert!(!p.is_alive());
    }

    #[test]
    fn test_venting_particle_life_progress() {
        let mut p = VentingParticle::new(Vec3::ZERO, Vec3::X, 0.5).with_lifetime(1.0);
        assert!((p.life_progress() - 0.0).abs() < f32::EPSILON);

        p.tick(0.5);
        assert!((p.life_progress() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_frost_effect_new() {
        let f = FrostEffect::new(Vec3::ZERO, 5.0);
        assert_eq!(f.position(), Vec3::ZERO);
        assert!((f.radius() - 0.0).abs() < f32::EPSILON);
        assert!((f.intensity() - 1.0).abs() < f32::EPSILON);
        assert!(f.is_active());
    }

    #[test]
    fn test_frost_effect_with_duration() {
        let f = FrostEffect::new(Vec3::ZERO, 5.0).with_duration(10.0);
        assert!((f.duration() - 10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_frost_effect_tick() {
        let mut f = FrostEffect::new(Vec3::ZERO, 5.0).with_duration(5.0);
        f.tick(1.0);

        assert!(f.radius() > 0.0);
        assert!((f.duration() - 4.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_frost_effect_spread() {
        let mut f = FrostEffect::new(Vec3::ZERO, 5.0);
        f.tick(2.0);
        assert!((f.radius() - 4.0).abs() < f32::EPSILON);

        f.tick(2.0);
        assert!((f.radius() - 5.0).abs() < f32::EPSILON); // Capped at max
    }

    #[test]
    fn test_frost_effect_fade() {
        let mut f = FrostEffect::new(Vec3::ZERO, 5.0).with_duration(3.0);
        f.tick(2.0); // Duration now 1.0, which is < 2.0
        assert!(f.intensity() < 1.0);
    }

    #[test]
    fn test_frost_effect_inactive() {
        let mut f = FrostEffect::new(Vec3::ZERO, 5.0).with_duration(1.0);
        f.tick(1.5);
        assert!(!f.is_active());
    }

    #[test]
    fn test_decompression_fx_new() {
        let fx = DecompressionFX::new();
        assert_eq!(fx.particle_count(), 0);
        assert_eq!(fx.frost_count(), 0);
        assert!(!fx.has_active_effects());
    }

    #[test]
    fn test_decompression_fx_with_max_particles() {
        let fx = DecompressionFX::with_max_particles(500);
        assert_eq!(fx.particle_count(), 0);
    }

    #[test]
    fn test_decompression_fx_spawn_venting() {
        let mut fx = DecompressionFX::new();
        fx.spawn_venting(Vec3::ZERO, Vec3::X, 10, 1.0);

        assert_eq!(fx.particle_count(), 10);
        assert!(fx.has_active_effects());
    }

    #[test]
    fn test_decompression_fx_spawn_venting_limit() {
        let mut fx = DecompressionFX::with_max_particles(5);
        fx.spawn_venting(Vec3::ZERO, Vec3::X, 10, 1.0);

        assert_eq!(fx.particle_count(), 5); // Limited to max
    }

    #[test]
    fn test_decompression_fx_spawn_frost() {
        let mut fx = DecompressionFX::new();
        fx.spawn_frost(Vec3::ZERO, 5.0, 3.0);

        assert_eq!(fx.frost_count(), 1);
        assert!(fx.has_active_effects());
    }

    #[test]
    fn test_decompression_fx_tick() {
        let mut fx = DecompressionFX::new();
        fx.spawn_venting(Vec3::ZERO, Vec3::X, 5, 1.0);
        fx.spawn_frost(Vec3::ZERO, 5.0, 1.0);

        fx.tick(0.5);

        // Particles should have moved
        for p in fx.particles() {
            assert!(p.position().length() > 0.0);
        }

        // Frost should have spread
        for f in fx.frost_effects() {
            assert!(f.radius() > 0.0);
        }
    }

    #[test]
    fn test_decompression_fx_particle_cleanup() {
        let mut fx = DecompressionFX::new();
        fx.spawn_venting(Vec3::ZERO, Vec3::X, 5, 1.0);

        fx.tick(3.0); // Particles have 2.0 lifetime by default
        assert_eq!(fx.particle_count(), 0);
    }

    #[test]
    fn test_decompression_fx_frost_cleanup() {
        let mut fx = DecompressionFX::new();
        fx.spawn_frost(Vec3::ZERO, 5.0, 1.0);

        fx.tick(2.0);
        assert_eq!(fx.frost_count(), 0);
    }

    #[test]
    fn test_decompression_fx_clear() {
        let mut fx = DecompressionFX::new();
        fx.spawn_venting(Vec3::ZERO, Vec3::X, 10, 1.0);
        fx.spawn_frost(Vec3::ZERO, 5.0, 3.0);

        fx.clear();
        assert!(!fx.has_active_effects());
    }

    #[test]
    fn test_decompression_fx_particles_accessor() {
        let mut fx = DecompressionFX::new();
        fx.spawn_venting(Vec3::ZERO, Vec3::X, 3, 1.0);

        let particles = fx.particles();
        assert_eq!(particles.len(), 3);
    }

    #[test]
    fn test_decompression_fx_frost_accessor() {
        let mut fx = DecompressionFX::new();
        fx.spawn_frost(Vec3::ZERO, 5.0, 3.0);
        fx.spawn_frost(Vec3::ONE, 3.0, 2.0);

        let frost = fx.frost_effects();
        assert_eq!(frost.len(), 2);
    }

    #[test]
    fn test_decompression_fx_default() {
        let fx = DecompressionFX::default();
        assert!(!fx.has_active_effects());
    }
}
