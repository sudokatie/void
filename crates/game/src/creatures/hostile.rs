//! Hostile creature types for space station survival.
//!
//! Space-themed hostile creatures that threaten station operations.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Types of hostile creatures in the void.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HostileType {
    /// Creature that damages electrical systems.
    VoidCrawler,
    /// Creature that widens hull breaches.
    PressureLeech,
    /// Spectral entity that irradiates areas.
    RadiationWraith,
    /// Rogue repair drone gone hostile.
    DebrisDrone,
    /// Tiny creature that weakens hull integrity.
    HullMite,
}

impl HostileType {
    /// Get base HP for this creature type.
    #[must_use]
    pub fn base_hp(&self) -> u32 {
        match self {
            HostileType::VoidCrawler => 50,
            HostileType::PressureLeech => 30,
            HostileType::RadiationWraith => 70,
            HostileType::DebrisDrone => 60,
            HostileType::HullMite => 20,
        }
    }

    /// Get base damage for this creature type.
    #[must_use]
    pub fn base_damage(&self) -> u32 {
        match self {
            HostileType::VoidCrawler => 10,
            HostileType::PressureLeech => 5,
            HostileType::RadiationWraith => 15,
            HostileType::DebrisDrone => 12,
            HostileType::HullMite => 3,
        }
    }

    /// Get base movement speed for this creature type.
    #[must_use]
    pub fn base_speed(&self) -> f32 {
        match self {
            HostileType::VoidCrawler => 1.0,
            HostileType::PressureLeech => 1.5,
            HostileType::RadiationWraith => 0.8,
            HostileType::DebrisDrone => 1.2,
            HostileType::HullMite => 2.0,
        }
    }

    /// Get the special ability name for this creature type.
    #[must_use]
    pub fn special_ability_name(&self) -> &'static str {
        match self {
            HostileType::VoidCrawler => "short_circuit",
            HostileType::PressureLeech => "widen",
            HostileType::RadiationWraith => "irradiate",
            HostileType::DebrisDrone => "weld",
            HostileType::HullMite => "weaken",
        }
    }

    /// Get display name for this creature type.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            HostileType::VoidCrawler => "Void Crawler",
            HostileType::PressureLeech => "Pressure Leech",
            HostileType::RadiationWraith => "Radiation Wraith",
            HostileType::DebrisDrone => "Debris Drone",
            HostileType::HullMite => "Hull Mite",
        }
    }

    /// Get all hostile types.
    #[must_use]
    pub fn all() -> &'static [HostileType] {
        &[
            HostileType::VoidCrawler,
            HostileType::PressureLeech,
            HostileType::RadiationWraith,
            HostileType::DebrisDrone,
            HostileType::HullMite,
        ]
    }

    /// Get the habitat for this creature type.
    #[must_use]
    pub fn habitat(&self) -> &'static str {
        match self {
            HostileType::VoidCrawler => "electrical_systems",
            HostileType::PressureLeech => "hull_breaches",
            HostileType::RadiationWraith => "reactor_areas",
            HostileType::DebrisDrone => "debris_fields",
            HostileType::HullMite => "hull_surfaces",
        }
    }
}

impl fmt::Display for HostileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Special ability info with name and effect description.
#[derive(Clone, Debug, PartialEq)]
pub struct SpecialAbilityInfo {
    /// Name of the ability.
    pub name: &'static str,
    /// Description of the effect.
    pub description: &'static str,
    /// Base effect value.
    pub effect_value: f32,
}

/// Result of using a special ability.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AbilityResult {
    /// Whether the ability was successfully used.
    pub success: bool,
    /// Damage dealt by the ability.
    pub damage: u32,
    /// Duration of any status effect in ticks.
    pub effect_duration: u32,
    /// Description of the effect.
    pub effect: String,
}

impl AbilityResult {
    /// Create a new ability result.
    #[must_use]
    pub fn new(success: bool, damage: u32, effect_duration: u32, effect: String) -> Self {
        Self {
            success,
            damage,
            effect_duration,
            effect,
        }
    }

    /// Create a failed ability result.
    #[must_use]
    pub fn failed() -> Self {
        Self {
            success: false,
            damage: 0,
            effect_duration: 0,
            effect: String::new(),
        }
    }
}

/// A hostile creature instance.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HostileCreature {
    /// Type of this creature.
    creature_type: HostileType,
    /// Current HP.
    hp: u32,
    /// Maximum HP.
    max_hp: u32,
    /// Attack damage.
    damage: u32,
    /// Movement speed.
    speed: f32,
    /// Special ability name.
    special_ability: String,
    /// Whether the creature is active (not stunned/disabled).
    active: bool,
}

impl HostileCreature {
    /// Create a new hostile creature of the given type.
    #[must_use]
    pub fn new(creature_type: HostileType) -> Self {
        let max_hp = creature_type.base_hp();
        Self {
            creature_type,
            hp: max_hp,
            max_hp,
            damage: creature_type.base_damage(),
            speed: creature_type.base_speed(),
            special_ability: creature_type.special_ability_name().to_string(),
            active: true,
        }
    }

    /// Get the creature type.
    #[must_use]
    pub fn creature_type(&self) -> HostileType {
        self.creature_type
    }

    /// Get current HP.
    #[must_use]
    pub fn hp(&self) -> u32 {
        self.hp
    }

    /// Get maximum HP.
    #[must_use]
    pub fn max_hp(&self) -> u32 {
        self.max_hp
    }

    /// Get attack damage.
    #[must_use]
    pub fn damage(&self) -> u32 {
        self.damage
    }

    /// Get movement speed.
    #[must_use]
    pub fn speed(&self) -> f32 {
        self.speed
    }

    /// Get the special ability name.
    #[must_use]
    pub fn special_ability(&self) -> &str {
        &self.special_ability
    }

    /// Check if the creature is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Set active state.
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    /// Check if the creature is alive.
    #[must_use]
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    /// Apply damage to the creature.
    ///
    /// Returns the actual damage dealt.
    pub fn take_damage(&mut self, amount: u32) -> u32 {
        let actual = amount.min(self.hp);
        self.hp = self.hp.saturating_sub(amount);
        actual
    }

    /// Perform a basic attack.
    ///
    /// Returns the damage dealt (0 if inactive or dead).
    #[must_use]
    pub fn attack(&self) -> u32 {
        if self.is_alive() && self.active {
            self.damage
        } else {
            0
        }
    }

    /// Use the creature's special ability.
    #[must_use]
    pub fn use_ability(&self) -> AbilityResult {
        if !self.is_alive() || !self.active {
            return AbilityResult::failed();
        }

        match self.creature_type {
            HostileType::VoidCrawler => AbilityResult::new(
                true,
                0,
                3,
                "Short circuits nearby electrical systems".to_string(),
            ),
            HostileType::PressureLeech => AbilityResult::new(
                true,
                0,
                0,
                "Widens hull breach, increasing pressure loss".to_string(),
            ),
            HostileType::RadiationWraith => AbilityResult::new(
                true,
                self.damage,
                5,
                "Irradiates the area, causing damage over time".to_string(),
            ),
            HostileType::DebrisDrone => AbilityResult::new(
                true,
                self.damage * 2,
                0,
                "Welds debris to target, dealing double damage".to_string(),
            ),
            HostileType::HullMite => AbilityResult::new(
                true,
                0,
                0,
                "Weakens hull integrity in the area".to_string(),
            ),
        }
    }

    /// Heal the creature.
    pub fn heal(&mut self, amount: u32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hostile_type_void_crawler_stats() {
        assert_eq!(HostileType::VoidCrawler.base_hp(), 50);
        assert_eq!(HostileType::VoidCrawler.base_damage(), 10);
        assert!((HostileType::VoidCrawler.base_speed() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hostile_type_pressure_leech_stats() {
        assert_eq!(HostileType::PressureLeech.base_hp(), 30);
        assert_eq!(HostileType::PressureLeech.base_damage(), 5);
        assert!((HostileType::PressureLeech.base_speed() - 1.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hostile_type_radiation_wraith_stats() {
        assert_eq!(HostileType::RadiationWraith.base_hp(), 70);
        assert_eq!(HostileType::RadiationWraith.base_damage(), 15);
        assert!((HostileType::RadiationWraith.base_speed() - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hostile_type_debris_drone_stats() {
        assert_eq!(HostileType::DebrisDrone.base_hp(), 60);
        assert_eq!(HostileType::DebrisDrone.base_damage(), 12);
        assert!((HostileType::DebrisDrone.base_speed() - 1.2).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hostile_type_hull_mite_stats() {
        assert_eq!(HostileType::HullMite.base_hp(), 20);
        assert_eq!(HostileType::HullMite.base_damage(), 3);
        assert!((HostileType::HullMite.base_speed() - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hostile_type_special_ability_names() {
        assert_eq!(HostileType::VoidCrawler.special_ability_name(), "short_circuit");
        assert_eq!(HostileType::PressureLeech.special_ability_name(), "widen");
        assert_eq!(HostileType::RadiationWraith.special_ability_name(), "irradiate");
        assert_eq!(HostileType::DebrisDrone.special_ability_name(), "weld");
        assert_eq!(HostileType::HullMite.special_ability_name(), "weaken");
    }

    #[test]
    fn test_hostile_type_display_names() {
        assert_eq!(HostileType::VoidCrawler.display_name(), "Void Crawler");
        assert_eq!(HostileType::PressureLeech.display_name(), "Pressure Leech");
        assert_eq!(HostileType::RadiationWraith.display_name(), "Radiation Wraith");
        assert_eq!(HostileType::DebrisDrone.display_name(), "Debris Drone");
        assert_eq!(HostileType::HullMite.display_name(), "Hull Mite");
    }

    #[test]
    fn test_hostile_type_display_trait() {
        assert_eq!(format!("{}", HostileType::VoidCrawler), "Void Crawler");
    }

    #[test]
    fn test_hostile_type_habitats() {
        assert_eq!(HostileType::VoidCrawler.habitat(), "electrical_systems");
        assert_eq!(HostileType::PressureLeech.habitat(), "hull_breaches");
        assert_eq!(HostileType::RadiationWraith.habitat(), "reactor_areas");
        assert_eq!(HostileType::DebrisDrone.habitat(), "debris_fields");
        assert_eq!(HostileType::HullMite.habitat(), "hull_surfaces");
    }

    #[test]
    fn test_hostile_type_all() {
        let all = HostileType::all();
        assert_eq!(all.len(), 5);
        assert!(all.contains(&HostileType::VoidCrawler));
        assert!(all.contains(&HostileType::PressureLeech));
        assert!(all.contains(&HostileType::RadiationWraith));
        assert!(all.contains(&HostileType::DebrisDrone));
        assert!(all.contains(&HostileType::HullMite));
    }

    #[test]
    fn test_hostile_creature_new() {
        let creature = HostileCreature::new(HostileType::VoidCrawler);
        assert_eq!(creature.creature_type(), HostileType::VoidCrawler);
        assert_eq!(creature.hp(), 50);
        assert_eq!(creature.max_hp(), 50);
        assert_eq!(creature.damage(), 10);
        assert!((creature.speed() - 1.0).abs() < f32::EPSILON);
        assert!(creature.is_alive());
        assert!(creature.is_active());
    }

    #[test]
    fn test_hostile_creature_take_damage() {
        let mut creature = HostileCreature::new(HostileType::RadiationWraith);
        assert_eq!(creature.hp(), 70);

        let dealt = creature.take_damage(30);
        assert_eq!(dealt, 30);
        assert_eq!(creature.hp(), 40);
        assert!(creature.is_alive());
    }

    #[test]
    fn test_hostile_creature_death() {
        let mut creature = HostileCreature::new(HostileType::HullMite);
        creature.take_damage(30);
        assert_eq!(creature.hp(), 0);
        assert!(!creature.is_alive());
    }

    #[test]
    fn test_hostile_creature_overkill() {
        let mut creature = HostileCreature::new(HostileType::HullMite);
        let dealt = creature.take_damage(100);
        assert_eq!(dealt, 20);
        assert_eq!(creature.hp(), 0);
    }

    #[test]
    fn test_hostile_creature_attack() {
        let creature = HostileCreature::new(HostileType::DebrisDrone);
        assert_eq!(creature.attack(), 12);
    }

    #[test]
    fn test_hostile_creature_attack_when_dead() {
        let mut creature = HostileCreature::new(HostileType::DebrisDrone);
        creature.take_damage(100);
        assert_eq!(creature.attack(), 0);
    }

    #[test]
    fn test_hostile_creature_attack_when_inactive() {
        let mut creature = HostileCreature::new(HostileType::DebrisDrone);
        creature.set_active(false);
        assert_eq!(creature.attack(), 0);
    }

    #[test]
    fn test_hostile_creature_use_ability_void_crawler() {
        let creature = HostileCreature::new(HostileType::VoidCrawler);
        let result = creature.use_ability();
        assert!(result.success);
        assert_eq!(result.effect_duration, 3);
        assert!(result.effect.contains("electrical"));
    }

    #[test]
    fn test_hostile_creature_use_ability_pressure_leech() {
        let creature = HostileCreature::new(HostileType::PressureLeech);
        let result = creature.use_ability();
        assert!(result.success);
        assert!(result.effect.contains("breach"));
    }

    #[test]
    fn test_hostile_creature_use_ability_radiation_wraith() {
        let creature = HostileCreature::new(HostileType::RadiationWraith);
        let result = creature.use_ability();
        assert!(result.success);
        assert_eq!(result.damage, 15);
        assert_eq!(result.effect_duration, 5);
    }

    #[test]
    fn test_hostile_creature_use_ability_debris_drone() {
        let creature = HostileCreature::new(HostileType::DebrisDrone);
        let result = creature.use_ability();
        assert!(result.success);
        assert_eq!(result.damage, 24); // double damage
    }

    #[test]
    fn test_hostile_creature_use_ability_hull_mite() {
        let creature = HostileCreature::new(HostileType::HullMite);
        let result = creature.use_ability();
        assert!(result.success);
        assert!(result.effect.contains("hull"));
    }

    #[test]
    fn test_hostile_creature_ability_when_dead() {
        let mut creature = HostileCreature::new(HostileType::VoidCrawler);
        creature.take_damage(100);
        let result = creature.use_ability();
        assert!(!result.success);
    }

    #[test]
    fn test_hostile_creature_heal() {
        let mut creature = HostileCreature::new(HostileType::RadiationWraith);
        creature.take_damage(50);
        assert_eq!(creature.hp(), 20);

        creature.heal(30);
        assert_eq!(creature.hp(), 50);

        creature.heal(100);
        assert_eq!(creature.hp(), 70);
    }

    #[test]
    fn test_ability_result_failed() {
        let result = AbilityResult::failed();
        assert!(!result.success);
        assert_eq!(result.damage, 0);
        assert_eq!(result.effect_duration, 0);
    }
}
