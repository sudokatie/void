//! Hostile creature types for time-loop survival.
//!
//! Loop-aware hostile creatures that become more dangerous as loops progress.

use glam::IVec3;
use serde::{Deserialize, Serialize};

/// Time phase for creature spawning.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LoopPhaseSpawn {
    /// Spawns during dawn.
    Dawn,
    /// Spawns during day.
    Day,
    /// Spawns during dusk.
    Dusk,
    /// Spawns during night.
    Night,
    /// Spawns during midnight.
    Midnight,
    /// Spawns at any phase.
    Any,
}

/// Spawn condition for hostile creatures.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HostileSpawnCondition {
    /// Loop phase for spawning.
    pub phase: LoopPhaseSpawn,
    /// Minimum loop count for spawning.
    pub min_loop: u32,
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

/// Types of hostile creatures in the time loop.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HostileType {
    /// Ghostly entity that feeds on temporal energy.
    TimeWraith,
    /// Predator that tracks players across loop iterations.
    LoopStalker,
    /// Beast whose attacks grow stronger with each death at the same location.
    EchoBeast,
    /// Creature that feeds on temporal anomalies.
    TemporalParasite,
    /// Spider that weaves time-binding webs.
    ChronoSpider,
}

impl HostileType {
    /// Get base HP for this creature type.
    #[must_use]
    pub fn base_hp(&self) -> u32 {
        match self {
            HostileType::TimeWraith => 60,
            HostileType::LoopStalker => 80,
            HostileType::EchoBeast => 100,
            HostileType::TemporalParasite => 40,
            HostileType::ChronoSpider => 50,
        }
    }

    /// Get base damage for this creature type.
    #[must_use]
    pub fn base_damage(&self) -> u32 {
        match self {
            HostileType::TimeWraith => 12,
            HostileType::LoopStalker => 15,
            HostileType::EchoBeast => 20,
            HostileType::TemporalParasite => 8,
            HostileType::ChronoSpider => 10,
        }
    }

    /// Get base movement speed for this creature type.
    #[must_use]
    pub fn base_speed(&self) -> f32 {
        match self {
            HostileType::TimeWraith => 1.0,
            HostileType::LoopStalker => 1.3,
            HostileType::EchoBeast => 0.8,
            HostileType::TemporalParasite => 1.5,
            HostileType::ChronoSpider => 1.2,
        }
    }

    /// Get the special ability name for this creature type.
    #[must_use]
    pub fn special_ability_name(&self) -> &'static str {
        match self {
            HostileType::TimeWraith => "chronal_drain",
            HostileType::LoopStalker => "deja_vu",
            HostileType::EchoBeast => "resonance",
            HostileType::TemporalParasite => "phase",
            HostileType::ChronoSpider => "snare",
        }
    }

    /// Get the special ability info including name, description, and effect value.
    #[must_use]
    pub fn special_ability(&self) -> SpecialAbilityInfo {
        match self {
            HostileType::TimeWraith => SpecialAbilityInfo {
                name: "chronal_drain",
                description: "Reduces the player's loop counter by 1",
                effect_value: 1.0,
            },
            HostileType::LoopStalker => SpecialAbilityInfo {
                name: "deja_vu",
                description: "Player cannot move for 1 tick",
                effect_value: 1.0,
            },
            HostileType::EchoBeast => SpecialAbilityInfo {
                name: "resonance",
                description: "Damage increases per death at same location",
                effect_value: 1.5,
            },
            HostileType::TemporalParasite => SpecialAbilityInfo {
                name: "phase",
                description: "Teleports to nearest temporal chest",
                effect_value: 0.0,
            },
            HostileType::ChronoSpider => SpecialAbilityInfo {
                name: "snare",
                description: "Immobilizes target, damage scales with loop count",
                effect_value: 0.1,
            },
        }
    }

    /// Get the spawn condition for this creature type.
    #[must_use]
    pub fn spawn_condition(&self) -> HostileSpawnCondition {
        match self {
            HostileType::TimeWraith => HostileSpawnCondition {
                phase: LoopPhaseSpawn::Midnight,
                min_loop: 1,
            },
            HostileType::LoopStalker => HostileSpawnCondition {
                phase: LoopPhaseSpawn::Any,
                min_loop: 3,
            },
            HostileType::EchoBeast => HostileSpawnCondition {
                phase: LoopPhaseSpawn::Dusk,
                min_loop: 2,
            },
            HostileType::TemporalParasite => HostileSpawnCondition {
                phase: LoopPhaseSpawn::Any,
                min_loop: 1,
            },
            HostileType::ChronoSpider => HostileSpawnCondition {
                phase: LoopPhaseSpawn::Day,
                min_loop: 1,
            },
        }
    }

    /// Get loop-aware spawn condition as (LoopPhase, min_loop_number).
    ///
    /// Returns the phase when this creature spawns and the minimum loop required.
    /// - TimeWraith: (Midnight, loop >= 3)
    /// - LoopStalker: (Dusk, loop >= 5)
    /// - EchoBeast: (Day, any loop)
    /// - TemporalParasite: (Any, loop >= 2)
    /// - ChronoSpider: (Night, any loop)
    #[must_use]
    pub fn loop_aware_spawn(&self) -> (LoopPhaseSpawn, u32) {
        match self {
            HostileType::TimeWraith => (LoopPhaseSpawn::Midnight, 3),
            HostileType::LoopStalker => (LoopPhaseSpawn::Dusk, 5),
            HostileType::EchoBeast => (LoopPhaseSpawn::Day, 1),
            HostileType::TemporalParasite => (LoopPhaseSpawn::Any, 2),
            HostileType::ChronoSpider => (LoopPhaseSpawn::Midnight, 1),
        }
    }

    /// Check if this creature can spawn given the current phase and loop number.
    #[must_use]
    pub fn can_spawn(&self, current_phase: LoopPhaseSpawn, current_loop: u32) -> bool {
        let (required_phase, min_loop) = self.loop_aware_spawn();
        if current_loop < min_loop {
            return false;
        }
        required_phase == LoopPhaseSpawn::Any || required_phase == current_phase
    }

    /// Get display name for this creature type.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            HostileType::TimeWraith => "Time Wraith",
            HostileType::LoopStalker => "Loop Stalker",
            HostileType::EchoBeast => "Echo Beast",
            HostileType::TemporalParasite => "Temporal Parasite",
            HostileType::ChronoSpider => "Chrono Spider",
        }
    }

    /// Get all hostile types.
    #[must_use]
    pub fn all() -> &'static [HostileType] {
        &[
            HostileType::TimeWraith,
            HostileType::LoopStalker,
            HostileType::EchoBeast,
            HostileType::TemporalParasite,
            HostileType::ChronoSpider,
        ]
    }
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
    /// Current loop count (for scaling effects).
    loop_count: u32,
    /// Death count at current position (for EchoBeast).
    death_resonance: u32,
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
            loop_count: 1,
            death_resonance: 0,
        }
    }

    /// Create a hostile creature with loop awareness.
    #[must_use]
    pub fn new_with_loop(creature_type: HostileType, loop_count: u32) -> Self {
        let mut creature = Self::new(creature_type);
        creature.loop_count = loop_count;
        // Scale HP and damage with loop count
        let scale = 1.0 + (loop_count.saturating_sub(1) as f32 * 0.1);
        creature.max_hp = (creature.max_hp as f32 * scale) as u32;
        creature.hp = creature.max_hp;
        creature.damage = (creature.damage as f32 * scale) as u32;
        creature
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

    /// Register a player death at this creature's location (for EchoBeast).
    pub fn register_death_at_location(&mut self) {
        self.death_resonance += 1;
    }

    /// Get the death resonance count.
    #[must_use]
    pub fn death_resonance(&self) -> u32 {
        self.death_resonance
    }

    /// Use the creature's special ability.
    #[must_use]
    pub fn use_ability(&self) -> AbilityResult {
        if !self.is_alive() || !self.active {
            return AbilityResult::failed();
        }

        match self.creature_type {
            HostileType::TimeWraith => AbilityResult::new(
                true,
                0,
                0,
                "Drains temporal energy, reducing loop counter by 1".to_string(),
            ),
            HostileType::LoopStalker => AbilityResult::new(
                true,
                0,
                1,
                "Deja vu effect: player cannot move for 1 tick".to_string(),
            ),
            HostileType::EchoBeast => {
                let resonance_damage =
                    self.damage + (self.death_resonance * self.damage / 2);
                AbilityResult::new(
                    true,
                    resonance_damage,
                    0,
                    format!(
                        "Resonance strike with {} deaths at location, dealing {} damage",
                        self.death_resonance, resonance_damage
                    ),
                )
            }
            HostileType::TemporalParasite => AbilityResult::new(
                true,
                0,
                0,
                "Phases to nearest temporal chest location".to_string(),
            ),
            HostileType::ChronoSpider => {
                let snare_damage = self.damage + (self.loop_count * 2);
                AbilityResult::new(
                    true,
                    snare_damage,
                    3,
                    format!(
                        "Snare immobilizes for 3 ticks, damage scales with loop count: {}",
                        snare_damage
                    ),
                )
            }
        }
    }

    /// Get target position for phase ability (TemporalParasite).
    #[must_use]
    pub fn get_phase_target(&self, chest_positions: &[IVec3], current_pos: IVec3) -> Option<IVec3> {
        if self.creature_type != HostileType::TemporalParasite {
            return None;
        }

        chest_positions
            .iter()
            .min_by_key(|pos| {
                let diff = **pos - current_pos;
                diff.x.abs() + diff.y.abs() + diff.z.abs()
            })
            .copied()
    }

    /// Heal the creature.
    pub fn heal(&mut self, amount: u32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }

    /// Update loop count (for scaling effects).
    pub fn set_loop_count(&mut self, loop_count: u32) {
        self.loop_count = loop_count;
    }

    /// Get current loop count.
    #[must_use]
    pub fn loop_count(&self) -> u32 {
        self.loop_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hostile_type_time_wraith_stats() {
        assert_eq!(HostileType::TimeWraith.base_hp(), 60);
        assert_eq!(HostileType::TimeWraith.base_damage(), 12);
        assert!((HostileType::TimeWraith.base_speed() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hostile_type_loop_stalker_stats() {
        assert_eq!(HostileType::LoopStalker.base_hp(), 80);
        assert_eq!(HostileType::LoopStalker.base_damage(), 15);
        assert!((HostileType::LoopStalker.base_speed() - 1.3).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hostile_type_echo_beast_stats() {
        assert_eq!(HostileType::EchoBeast.base_hp(), 100);
        assert_eq!(HostileType::EchoBeast.base_damage(), 20);
        assert!((HostileType::EchoBeast.base_speed() - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hostile_type_temporal_parasite_stats() {
        assert_eq!(HostileType::TemporalParasite.base_hp(), 40);
        assert_eq!(HostileType::TemporalParasite.base_damage(), 8);
        assert!((HostileType::TemporalParasite.base_speed() - 1.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hostile_type_chrono_spider_stats() {
        assert_eq!(HostileType::ChronoSpider.base_hp(), 50);
        assert_eq!(HostileType::ChronoSpider.base_damage(), 10);
        assert!((HostileType::ChronoSpider.base_speed() - 1.2).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hostile_type_special_ability_names() {
        assert_eq!(HostileType::TimeWraith.special_ability_name(), "chronal_drain");
        assert_eq!(HostileType::LoopStalker.special_ability_name(), "deja_vu");
        assert_eq!(HostileType::EchoBeast.special_ability_name(), "resonance");
        assert_eq!(HostileType::TemporalParasite.special_ability_name(), "phase");
        assert_eq!(HostileType::ChronoSpider.special_ability_name(), "snare");
    }

    #[test]
    fn test_hostile_type_special_ability_info() {
        let ability = HostileType::TimeWraith.special_ability();
        assert_eq!(ability.name, "chronal_drain");
        assert!(ability.description.contains("loop counter"));

        let ability = HostileType::LoopStalker.special_ability();
        assert_eq!(ability.name, "deja_vu");
        assert!(ability.description.contains("cannot move"));

        let ability = HostileType::EchoBeast.special_ability();
        assert_eq!(ability.name, "resonance");
        assert!(ability.description.contains("death"));

        let ability = HostileType::TemporalParasite.special_ability();
        assert_eq!(ability.name, "phase");
        assert!(ability.description.contains("chest"));

        let ability = HostileType::ChronoSpider.special_ability();
        assert_eq!(ability.name, "snare");
        assert!(ability.description.contains("loop count"));
    }

    #[test]
    fn test_hostile_type_spawn_condition_time_wraith() {
        let cond = HostileType::TimeWraith.spawn_condition();
        assert_eq!(cond.phase, LoopPhaseSpawn::Midnight);
        assert_eq!(cond.min_loop, 1);
    }

    #[test]
    fn test_hostile_type_spawn_condition_loop_stalker() {
        let cond = HostileType::LoopStalker.spawn_condition();
        assert_eq!(cond.phase, LoopPhaseSpawn::Any);
        assert_eq!(cond.min_loop, 3);
    }

    #[test]
    fn test_hostile_type_spawn_condition_echo_beast() {
        let cond = HostileType::EchoBeast.spawn_condition();
        assert_eq!(cond.phase, LoopPhaseSpawn::Dusk);
        assert_eq!(cond.min_loop, 2);
    }

    #[test]
    fn test_hostile_type_spawn_condition_temporal_parasite() {
        let cond = HostileType::TemporalParasite.spawn_condition();
        assert_eq!(cond.phase, LoopPhaseSpawn::Any);
        assert_eq!(cond.min_loop, 1);
    }

    #[test]
    fn test_hostile_type_spawn_condition_chrono_spider() {
        let cond = HostileType::ChronoSpider.spawn_condition();
        assert_eq!(cond.phase, LoopPhaseSpawn::Day);
        assert_eq!(cond.min_loop, 1);
    }

    #[test]
    fn test_hostile_type_display_names() {
        assert_eq!(HostileType::TimeWraith.display_name(), "Time Wraith");
        assert_eq!(HostileType::LoopStalker.display_name(), "Loop Stalker");
        assert_eq!(HostileType::EchoBeast.display_name(), "Echo Beast");
        assert_eq!(HostileType::TemporalParasite.display_name(), "Temporal Parasite");
        assert_eq!(HostileType::ChronoSpider.display_name(), "Chrono Spider");
    }

    #[test]
    fn test_hostile_type_all() {
        let all = HostileType::all();
        assert_eq!(all.len(), 5);
        assert!(all.contains(&HostileType::TimeWraith));
        assert!(all.contains(&HostileType::LoopStalker));
        assert!(all.contains(&HostileType::EchoBeast));
        assert!(all.contains(&HostileType::TemporalParasite));
        assert!(all.contains(&HostileType::ChronoSpider));
    }

    #[test]
    fn test_hostile_creature_new() {
        let creature = HostileCreature::new(HostileType::LoopStalker);

        assert_eq!(creature.creature_type(), HostileType::LoopStalker);
        assert_eq!(creature.hp(), 80);
        assert_eq!(creature.max_hp(), 80);
        assert_eq!(creature.damage(), 15);
        assert!((creature.speed() - 1.3).abs() < f32::EPSILON);
        assert!(creature.is_alive());
        assert!(creature.is_active());
        assert_eq!(creature.loop_count(), 1);
    }

    #[test]
    fn test_hostile_creature_new_with_loop() {
        let creature = HostileCreature::new_with_loop(HostileType::TimeWraith, 5);

        assert_eq!(creature.loop_count(), 5);
        // HP should be scaled: 60 * (1.0 + 0.4) = 84
        assert_eq!(creature.max_hp(), 84);
        assert_eq!(creature.hp(), 84);
        // Damage should be scaled: 12 * 1.4 = 16
        assert_eq!(creature.damage(), 16);
    }

    #[test]
    fn test_hostile_creature_take_damage() {
        let mut creature = HostileCreature::new(HostileType::EchoBeast);
        assert_eq!(creature.hp(), 100);

        let dealt = creature.take_damage(30);
        assert_eq!(dealt, 30);
        assert_eq!(creature.hp(), 70);
        assert!(creature.is_alive());
    }

    #[test]
    fn test_hostile_creature_death() {
        let mut creature = HostileCreature::new(HostileType::TemporalParasite);
        creature.take_damage(50);

        assert_eq!(creature.hp(), 0);
        assert!(!creature.is_alive());
    }

    #[test]
    fn test_hostile_creature_overkill() {
        let mut creature = HostileCreature::new(HostileType::TemporalParasite);
        let dealt = creature.take_damage(1000);

        assert_eq!(dealt, 40);
        assert_eq!(creature.hp(), 0);
    }

    #[test]
    fn test_hostile_creature_attack() {
        let creature = HostileCreature::new(HostileType::ChronoSpider);
        assert_eq!(creature.attack(), 10);
    }

    #[test]
    fn test_hostile_creature_attack_when_dead() {
        let mut creature = HostileCreature::new(HostileType::ChronoSpider);
        creature.take_damage(100);
        assert_eq!(creature.attack(), 0);
    }

    #[test]
    fn test_hostile_creature_attack_when_inactive() {
        let mut creature = HostileCreature::new(HostileType::ChronoSpider);
        creature.set_active(false);
        assert_eq!(creature.attack(), 0);
    }

    #[test]
    fn test_hostile_creature_use_ability_chronal_drain() {
        let creature = HostileCreature::new(HostileType::TimeWraith);
        let result = creature.use_ability();

        assert!(result.success);
        assert_eq!(result.damage, 0);
        assert!(result.effect.contains("loop counter"));
    }

    #[test]
    fn test_hostile_creature_use_ability_deja_vu() {
        let creature = HostileCreature::new(HostileType::LoopStalker);
        let result = creature.use_ability();

        assert!(result.success);
        assert_eq!(result.effect_duration, 1);
        assert!(result.effect.contains("cannot move"));
    }

    #[test]
    fn test_hostile_creature_use_ability_resonance() {
        let mut creature = HostileCreature::new(HostileType::EchoBeast);
        creature.register_death_at_location();
        creature.register_death_at_location();
        let result = creature.use_ability();

        assert!(result.success);
        // Base damage 20 + (2 deaths * 10) = 40
        assert_eq!(result.damage, 40);
        assert!(result.effect.contains("2 deaths"));
    }

    #[test]
    fn test_hostile_creature_use_ability_phase() {
        let creature = HostileCreature::new(HostileType::TemporalParasite);
        let result = creature.use_ability();

        assert!(result.success);
        assert!(result.effect.contains("chest"));
    }

    #[test]
    fn test_hostile_creature_use_ability_snare() {
        let creature = HostileCreature::new_with_loop(HostileType::ChronoSpider, 5);
        let result = creature.use_ability();

        assert!(result.success);
        assert_eq!(result.effect_duration, 3);
        // Damage: scaled base (14) + (5 loops * 2) = 24
        assert_eq!(result.damage, 24);
    }

    #[test]
    fn test_hostile_creature_ability_when_dead() {
        let mut creature = HostileCreature::new(HostileType::TimeWraith);
        creature.take_damage(100);
        let result = creature.use_ability();

        assert!(!result.success);
    }

    #[test]
    fn test_hostile_creature_heal() {
        let mut creature = HostileCreature::new(HostileType::EchoBeast);
        creature.take_damage(50);
        assert_eq!(creature.hp(), 50);

        creature.heal(30);
        assert_eq!(creature.hp(), 80);

        creature.heal(100);
        assert_eq!(creature.hp(), 100);
    }

    #[test]
    fn test_ability_result_failed() {
        let result = AbilityResult::failed();
        assert!(!result.success);
        assert_eq!(result.damage, 0);
        assert_eq!(result.effect_duration, 0);
    }

    #[test]
    fn test_hostile_creature_death_resonance() {
        let mut creature = HostileCreature::new(HostileType::EchoBeast);
        assert_eq!(creature.death_resonance(), 0);

        creature.register_death_at_location();
        assert_eq!(creature.death_resonance(), 1);

        creature.register_death_at_location();
        creature.register_death_at_location();
        assert_eq!(creature.death_resonance(), 3);
    }

    #[test]
    fn test_hostile_creature_phase_target() {
        let creature = HostileCreature::new(HostileType::TemporalParasite);
        let current_pos = IVec3::new(0, 0, 0);
        let chests = vec![
            IVec3::new(10, 0, 0),
            IVec3::new(5, 0, 0),
            IVec3::new(20, 0, 0),
        ];

        let target = creature.get_phase_target(&chests, current_pos);
        assert_eq!(target, Some(IVec3::new(5, 0, 0)));
    }

    #[test]
    fn test_hostile_creature_phase_target_wrong_type() {
        let creature = HostileCreature::new(HostileType::TimeWraith);
        let chests = vec![IVec3::new(10, 0, 0)];

        let target = creature.get_phase_target(&chests, IVec3::ZERO);
        assert!(target.is_none());
    }

    #[test]
    fn test_hostile_creature_set_loop_count() {
        let mut creature = HostileCreature::new(HostileType::ChronoSpider);
        assert_eq!(creature.loop_count(), 1);

        creature.set_loop_count(10);
        assert_eq!(creature.loop_count(), 10);
    }

    #[test]
    fn test_loop_phase_spawn_enum() {
        assert_ne!(LoopPhaseSpawn::Dawn, LoopPhaseSpawn::Day);
        assert_ne!(LoopPhaseSpawn::Dusk, LoopPhaseSpawn::Midnight);
        assert_ne!(LoopPhaseSpawn::Any, LoopPhaseSpawn::Dawn);
    }

    #[test]
    fn test_loop_aware_spawn_time_wraith() {
        let (phase, min_loop) = HostileType::TimeWraith.loop_aware_spawn();
        assert_eq!(phase, LoopPhaseSpawn::Midnight);
        assert_eq!(min_loop, 3);
    }

    #[test]
    fn test_loop_aware_spawn_loop_stalker() {
        let (phase, min_loop) = HostileType::LoopStalker.loop_aware_spawn();
        assert_eq!(phase, LoopPhaseSpawn::Dusk);
        assert_eq!(min_loop, 5);
    }

    #[test]
    fn test_loop_aware_spawn_echo_beast() {
        let (phase, min_loop) = HostileType::EchoBeast.loop_aware_spawn();
        assert_eq!(phase, LoopPhaseSpawn::Day);
        assert_eq!(min_loop, 1);
    }

    #[test]
    fn test_loop_aware_spawn_temporal_parasite() {
        let (phase, min_loop) = HostileType::TemporalParasite.loop_aware_spawn();
        assert_eq!(phase, LoopPhaseSpawn::Any);
        assert_eq!(min_loop, 2);
    }

    #[test]
    fn test_loop_aware_spawn_chrono_spider() {
        let (phase, min_loop) = HostileType::ChronoSpider.loop_aware_spawn();
        assert_eq!(phase, LoopPhaseSpawn::Midnight);
        assert_eq!(min_loop, 1);
    }

    #[test]
    fn test_can_spawn_time_wraith_correct_conditions() {
        assert!(HostileType::TimeWraith.can_spawn(LoopPhaseSpawn::Midnight, 3));
        assert!(HostileType::TimeWraith.can_spawn(LoopPhaseSpawn::Midnight, 10));
    }

    #[test]
    fn test_can_spawn_time_wraith_wrong_phase() {
        assert!(!HostileType::TimeWraith.can_spawn(LoopPhaseSpawn::Day, 5));
        assert!(!HostileType::TimeWraith.can_spawn(LoopPhaseSpawn::Dawn, 3));
    }

    #[test]
    fn test_can_spawn_time_wraith_insufficient_loop() {
        assert!(!HostileType::TimeWraith.can_spawn(LoopPhaseSpawn::Midnight, 1));
        assert!(!HostileType::TimeWraith.can_spawn(LoopPhaseSpawn::Midnight, 2));
    }

    #[test]
    fn test_can_spawn_temporal_parasite_any_phase() {
        assert!(HostileType::TemporalParasite.can_spawn(LoopPhaseSpawn::Dawn, 2));
        assert!(HostileType::TemporalParasite.can_spawn(LoopPhaseSpawn::Day, 2));
        assert!(HostileType::TemporalParasite.can_spawn(LoopPhaseSpawn::Dusk, 2));
        assert!(HostileType::TemporalParasite.can_spawn(LoopPhaseSpawn::Night, 2));
        assert!(HostileType::TemporalParasite.can_spawn(LoopPhaseSpawn::Midnight, 2));
    }

    #[test]
    fn test_can_spawn_temporal_parasite_insufficient_loop() {
        assert!(!HostileType::TemporalParasite.can_spawn(LoopPhaseSpawn::Any, 1));
    }

    #[test]
    fn test_can_spawn_echo_beast_any_loop() {
        assert!(HostileType::EchoBeast.can_spawn(LoopPhaseSpawn::Day, 1));
        assert!(HostileType::EchoBeast.can_spawn(LoopPhaseSpawn::Day, 100));
    }

    #[test]
    fn test_can_spawn_loop_stalker() {
        assert!(!HostileType::LoopStalker.can_spawn(LoopPhaseSpawn::Dusk, 4));
        assert!(HostileType::LoopStalker.can_spawn(LoopPhaseSpawn::Dusk, 5));
        assert!(HostileType::LoopStalker.can_spawn(LoopPhaseSpawn::Dusk, 10));
    }

    #[test]
    fn test_can_spawn_chrono_spider() {
        assert!(HostileType::ChronoSpider.can_spawn(LoopPhaseSpawn::Midnight, 1));
        assert!(!HostileType::ChronoSpider.can_spawn(LoopPhaseSpawn::Day, 1));
    }
}
