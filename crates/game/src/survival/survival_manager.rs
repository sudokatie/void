//! Combined survival manager for time-loop gameplay.
//!
//! Manages hunger, thirst, and HP together with damage from
//! starvation and dehydration.

use serde::{Deserialize, Serialize};

use super::loop_hunger::LoopHunger;
use super::thirst::Thirst;

/// Default maximum HP.
pub const DEFAULT_MAX_HP: f32 = 100.0;

/// HP damage per tick when starving or dehydrated.
pub const DEPRIVATION_DAMAGE: f32 = 1.0;

/// Combined survival state manager.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SurvivalManager {
    /// Hunger state.
    hunger: LoopHunger,
    /// Thirst state.
    thirst: Thirst,
    /// Current HP.
    hp: f32,
}

impl SurvivalManager {
    /// Create a new survival manager with full stats.
    #[must_use]
    pub fn new() -> Self {
        Self {
            hunger: LoopHunger::new(),
            thirst: Thirst::new(),
            hp: DEFAULT_MAX_HP,
        }
    }

    /// Update all survival stats over time.
    ///
    /// Returns HP change (negative if taking damage from deprivation).
    pub fn tick(&mut self, dt: f32) -> f32 {
        self.hunger.tick(dt);
        self.thirst.tick(dt);

        let mut hp_change = 0.0;

        // Apply damage for starvation
        if self.hunger.is_starving() {
            let damage = DEPRIVATION_DAMAGE * dt;
            self.hp = (self.hp - damage).max(0.0);
            hp_change -= damage;
        }

        // Apply damage for dehydration
        if self.thirst.is_dehydrated() {
            let damage = DEPRIVATION_DAMAGE * dt;
            self.hp = (self.hp - damage).max(0.0);
            hp_change -= damage;
        }

        hp_change
    }

    /// Eat food to restore hunger.
    pub fn eat(&mut self, amount: f32) {
        self.hunger.eat(amount);
    }

    /// Drink to restore thirst.
    pub fn drink(&mut self, amount: f32) {
        self.thirst.drink(amount);
    }

    /// Get current HP.
    #[must_use]
    pub fn hp(&self) -> f32 {
        self.hp
    }

    /// Check if the player is alive.
    #[must_use]
    pub fn is_alive(&self) -> bool {
        self.hp > 0.0
    }

    /// Get hunger component.
    #[must_use]
    pub fn hunger(&self) -> &LoopHunger {
        &self.hunger
    }

    /// Get thirst component.
    #[must_use]
    pub fn thirst(&self) -> &Thirst {
        &self.thirst
    }

    /// Fully restore all survival stats.
    pub fn restore(&mut self) {
        self.hunger.restore();
        self.thirst.restore();
        self.hp = DEFAULT_MAX_HP;
    }

    /// Take direct damage.
    pub fn damage(&mut self, amount: f32) {
        self.hp = (self.hp - amount).max(0.0);
    }

    /// Heal HP.
    pub fn heal(&mut self, amount: f32) {
        self.hp = (self.hp + amount).min(DEFAULT_MAX_HP);
    }

    /// Check if player is in critical condition.
    #[must_use]
    pub fn is_critical(&self) -> bool {
        self.hp <= 20.0 || self.hunger.is_low() || self.thirst.is_low()
    }
}

impl Default for SurvivalManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_survival_manager_new() {
        let manager = SurvivalManager::new();
        assert!((manager.hp() - 100.0).abs() < f32::EPSILON);
        assert!(manager.is_alive());
        assert!(!manager.is_critical());
    }

    #[test]
    fn test_survival_manager_tick_no_damage() {
        let mut manager = SurvivalManager::new();
        let hp_change = manager.tick(1.0);
        assert!((hp_change - 0.0).abs() < f32::EPSILON);
        assert!((manager.hp() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_survival_manager_tick_starvation_damage() {
        let mut manager = SurvivalManager::new();
        // Drain hunger completely
        manager.tick(100.0);
        assert!(manager.hunger.is_starving());

        // Now tick should cause damage
        let hp_change = manager.tick(1.0);
        assert!(hp_change < 0.0);
    }

    #[test]
    fn test_survival_manager_tick_dehydration_damage() {
        let mut manager = SurvivalManager::new();
        // Drain thirst completely (faster than hunger)
        manager.tick(70.0); // 100 / 1.5 ~= 66.67, so 70 should empty it
        assert!(manager.thirst.is_dehydrated());
    }

    #[test]
    fn test_survival_manager_eat() {
        let mut manager = SurvivalManager::new();
        manager.tick(50.0);
        let before = manager.hunger().value();
        manager.eat(20.0);
        assert!(manager.hunger().value() > before);
    }

    #[test]
    fn test_survival_manager_drink() {
        let mut manager = SurvivalManager::new();
        manager.tick(30.0);
        let before = manager.thirst().value();
        manager.drink(20.0);
        assert!(manager.thirst().value() > before);
    }

    #[test]
    fn test_survival_manager_is_alive() {
        let mut manager = SurvivalManager::new();
        assert!(manager.is_alive());

        manager.hp = 0.0;
        assert!(!manager.is_alive());
    }

    #[test]
    fn test_survival_manager_restore() {
        let mut manager = SurvivalManager::new();
        manager.tick(50.0);
        manager.damage(30.0);

        manager.restore();
        assert!((manager.hp() - 100.0).abs() < f32::EPSILON);
        assert!((manager.hunger().value() - 100.0).abs() < f32::EPSILON);
        assert!((manager.thirst().value() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_survival_manager_damage() {
        let mut manager = SurvivalManager::new();
        manager.damage(25.0);
        assert!((manager.hp() - 75.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_survival_manager_heal() {
        let mut manager = SurvivalManager::new();
        manager.damage(30.0);
        manager.heal(15.0);
        assert!((manager.hp() - 85.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_survival_manager_heal_caps_at_max() {
        let mut manager = SurvivalManager::new();
        manager.damage(10.0);
        manager.heal(100.0);
        assert!((manager.hp() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_survival_manager_is_critical() {
        let mut manager = SurvivalManager::new();
        assert!(!manager.is_critical());

        manager.hp = 20.0;
        assert!(manager.is_critical());
    }
}
