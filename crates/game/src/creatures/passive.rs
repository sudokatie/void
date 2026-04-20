//! Passive creature types for time-loop survival.
//!
//! Loop-aware passive creatures that provide temporal resources.

use serde::{Deserialize, Serialize};

/// Time phase for passive creature spawning.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PassiveLoopPhase {
    /// Spawns during dawn.
    Dawn,
    /// Spawns during day.
    Day,
    /// Spawns during dusk.
    Dusk,
    /// Spawns during night.
    Night,
    /// Spawns at any phase.
    Any,
}

/// Loop number parity for spawn conditions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LoopParity {
    /// Spawns on even-numbered loops.
    Even,
    /// Spawns on odd-numbered loops.
    Odd,
    /// Spawns on any loop number.
    Any,
}

impl LoopParity {
    /// Check if a loop number matches this parity.
    #[must_use]
    pub fn matches(&self, loop_number: u32) -> bool {
        match self {
            LoopParity::Even => loop_number % 2 == 0,
            LoopParity::Odd => loop_number % 2 == 1,
            LoopParity::Any => true,
        }
    }
}

/// Spawn condition for passive creatures.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PassiveSpawnCondition {
    /// Whether creature only appears on even-numbered loops.
    pub even_loops_only: bool,
    /// Minimum loop count for spawning.
    pub min_loop: u32,
}

/// Types of passive creatures in the time loop.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PassiveType {
    /// Moth that carries temporal dust, attracted to time anomalies.
    MemoryMoth,
    /// Fish that only appears on even-numbered loops.
    LoopFish,
    /// Rabbit that leaves behind loop fiber when caught.
    EchoRabbit,
    /// Deer that phases between timelines, drops phase antler.
    PhaseDeer,
    /// Turtle with a shell that anchors time, very durable.
    AnchorTurtle,
}

impl PassiveType {
    /// Get base HP for this creature type.
    #[must_use]
    pub fn base_hp(&self) -> u32 {
        match self {
            PassiveType::MemoryMoth => 8,
            PassiveType::LoopFish => 10,
            PassiveType::EchoRabbit => 6,
            PassiveType::PhaseDeer => 15,
            PassiveType::AnchorTurtle => 20,
        }
    }

    /// Get the drop item for this creature type.
    #[must_use]
    pub fn drop_item(&self) -> &'static str {
        match self {
            PassiveType::MemoryMoth => "temporal_dust",
            PassiveType::LoopFish => "time_scale",
            PassiveType::EchoRabbit => "loop_fiber",
            PassiveType::PhaseDeer => "phase_antler",
            PassiveType::AnchorTurtle => "anchor_shell",
        }
    }

    /// Get the special trait for this creature type.
    #[must_use]
    pub fn special_trait(&self) -> &'static str {
        match self {
            PassiveType::MemoryMoth => "temporal_attraction",
            PassiveType::LoopFish => "even_loop_spawn",
            PassiveType::EchoRabbit => "echo_trail",
            PassiveType::PhaseDeer => "phase_shift",
            PassiveType::AnchorTurtle => "time_anchor",
        }
    }

    /// Get display name for this creature type.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            PassiveType::MemoryMoth => "Memory Moth",
            PassiveType::LoopFish => "Loop Fish",
            PassiveType::EchoRabbit => "Echo Rabbit",
            PassiveType::PhaseDeer => "Phase Deer",
            PassiveType::AnchorTurtle => "Anchor Turtle",
        }
    }

    /// Get the habitat location for this creature.
    #[must_use]
    pub fn habitat(&self) -> &'static str {
        match self {
            PassiveType::MemoryMoth => "temporal_rifts",
            PassiveType::LoopFish => "time_pools",
            PassiveType::EchoRabbit => "echo_fields",
            PassiveType::PhaseDeer => "phase_boundaries",
            PassiveType::AnchorTurtle => "anchor_points",
        }
    }

    /// Get the spawn condition for this creature type.
    #[must_use]
    pub fn spawn_condition(&self) -> PassiveSpawnCondition {
        match self {
            PassiveType::MemoryMoth => PassiveSpawnCondition {
                even_loops_only: false,
                min_loop: 1,
            },
            PassiveType::LoopFish => PassiveSpawnCondition {
                even_loops_only: true,
                min_loop: 2,
            },
            PassiveType::EchoRabbit => PassiveSpawnCondition {
                even_loops_only: false,
                min_loop: 1,
            },
            PassiveType::PhaseDeer => PassiveSpawnCondition {
                even_loops_only: false,
                min_loop: 3,
            },
            PassiveType::AnchorTurtle => PassiveSpawnCondition {
                even_loops_only: false,
                min_loop: 1,
            },
        }
    }

    /// Check if this creature can spawn on the given loop.
    #[must_use]
    pub fn can_spawn_on_loop(&self, loop_count: u32) -> bool {
        let condition = self.spawn_condition();
        if loop_count < condition.min_loop {
            return false;
        }
        if condition.even_loops_only && loop_count % 2 != 0 {
            return false;
        }
        true
    }

    /// Get all passive types.
    #[must_use]
    pub fn all() -> &'static [PassiveType] {
        &[
            PassiveType::MemoryMoth,
            PassiveType::LoopFish,
            PassiveType::EchoRabbit,
            PassiveType::PhaseDeer,
            PassiveType::AnchorTurtle,
        ]
    }

    /// Get loop-aware spawn condition as (LoopPhase, LoopParity).
    ///
    /// Returns the phase when this creature spawns and the loop parity requirement.
    /// - MemoryMoth: (Dawn, Any)
    /// - LoopFish: (Day, Even)
    /// - EchoRabbit: (Day, Any)
    /// - PhaseDeer: (Dusk, Odd)
    /// - AnchorTurtle: (Any, Any)
    #[must_use]
    pub fn loop_spawn_condition(&self) -> (PassiveLoopPhase, LoopParity) {
        match self {
            PassiveType::MemoryMoth => (PassiveLoopPhase::Dawn, LoopParity::Any),
            PassiveType::LoopFish => (PassiveLoopPhase::Day, LoopParity::Even),
            PassiveType::EchoRabbit => (PassiveLoopPhase::Day, LoopParity::Any),
            PassiveType::PhaseDeer => (PassiveLoopPhase::Dusk, LoopParity::Odd),
            PassiveType::AnchorTurtle => (PassiveLoopPhase::Any, LoopParity::Any),
        }
    }

    /// Check if this creature can spawn given the current phase and loop number.
    #[must_use]
    pub fn can_spawn_at(&self, current_phase: PassiveLoopPhase, current_loop: u32) -> bool {
        let (required_phase, parity) = self.loop_spawn_condition();
        let phase_matches =
            required_phase == PassiveLoopPhase::Any || required_phase == current_phase;
        let parity_matches = parity.matches(current_loop);
        phase_matches && parity_matches
    }
}

/// A passive creature instance.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PassiveCreature {
    /// Type of this creature.
    creature_type: PassiveType,
    /// Current HP.
    hp: u32,
    /// Maximum HP.
    max_hp: u32,
    /// Item dropped when caught/killed.
    drop_item: String,
    /// Special trait identifier.
    special_trait: String,
    /// Current loop count when spawned.
    spawn_loop: u32,
    /// Whether the creature is phased (for PhaseDeer).
    phased: bool,
}

impl PassiveCreature {
    /// Create a new passive creature of the given type.
    #[must_use]
    pub fn new(creature_type: PassiveType) -> Self {
        let max_hp = creature_type.base_hp();
        Self {
            creature_type,
            hp: max_hp,
            max_hp,
            drop_item: creature_type.drop_item().to_string(),
            special_trait: creature_type.special_trait().to_string(),
            spawn_loop: 1,
            phased: false,
        }
    }

    /// Create a new passive creature with loop awareness.
    #[must_use]
    pub fn new_with_loop(creature_type: PassiveType, loop_count: u32) -> Self {
        let mut creature = Self::new(creature_type);
        creature.spawn_loop = loop_count;
        creature
    }

    /// Get the creature type.
    #[must_use]
    pub fn creature_type(&self) -> PassiveType {
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

    /// Get the drop item name.
    #[must_use]
    pub fn drop_item(&self) -> &str {
        &self.drop_item
    }

    /// Get the special trait name.
    #[must_use]
    pub fn special_trait(&self) -> &str {
        &self.special_trait
    }

    /// Get the loop count when this creature spawned.
    #[must_use]
    pub fn spawn_loop(&self) -> u32 {
        self.spawn_loop
    }

    /// Check if the creature is alive.
    #[must_use]
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    /// Check if the creature is phased (untargetable).
    #[must_use]
    pub fn is_phased(&self) -> bool {
        self.phased
    }

    /// Toggle phase state (for PhaseDeer).
    pub fn toggle_phase(&mut self) {
        if self.creature_type == PassiveType::PhaseDeer {
            self.phased = !self.phased;
        }
    }

    /// Apply damage to the creature.
    ///
    /// Returns the actual damage dealt (0 if phased).
    pub fn take_damage(&mut self, amount: u32) -> u32 {
        if self.phased {
            return 0;
        }
        let actual = amount.min(self.hp);
        self.hp = self.hp.saturating_sub(amount);
        actual
    }

    /// Attempt to catch the creature.
    ///
    /// Returns the drop item if successful (creature dies), None otherwise.
    pub fn on_catch(&mut self) -> Option<String> {
        if self.phased {
            return None;
        }
        if self.is_alive() {
            self.hp = 0;
            Some(self.drop_item.clone())
        } else {
            None
        }
    }

    /// Check if this creature has a specific trait.
    #[must_use]
    pub fn has_trait(&self, trait_name: &str) -> bool {
        self.special_trait == trait_name
    }

    /// Get bonus drop chance based on loop count.
    #[must_use]
    pub fn bonus_drop_chance(&self, current_loop: u32) -> f32 {
        let loops_since_spawn = current_loop.saturating_sub(self.spawn_loop);
        (loops_since_spawn as f32 * 0.05).min(0.25)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passive_type_memory_moth_stats() {
        assert_eq!(PassiveType::MemoryMoth.base_hp(), 8);
        assert_eq!(PassiveType::MemoryMoth.drop_item(), "temporal_dust");
        assert_eq!(PassiveType::MemoryMoth.special_trait(), "temporal_attraction");
    }

    #[test]
    fn test_passive_type_loop_fish_stats() {
        assert_eq!(PassiveType::LoopFish.base_hp(), 10);
        assert_eq!(PassiveType::LoopFish.drop_item(), "time_scale");
        assert_eq!(PassiveType::LoopFish.special_trait(), "even_loop_spawn");
    }

    #[test]
    fn test_passive_type_echo_rabbit_stats() {
        assert_eq!(PassiveType::EchoRabbit.base_hp(), 6);
        assert_eq!(PassiveType::EchoRabbit.drop_item(), "loop_fiber");
        assert_eq!(PassiveType::EchoRabbit.special_trait(), "echo_trail");
    }

    #[test]
    fn test_passive_type_phase_deer_stats() {
        assert_eq!(PassiveType::PhaseDeer.base_hp(), 15);
        assert_eq!(PassiveType::PhaseDeer.drop_item(), "phase_antler");
        assert_eq!(PassiveType::PhaseDeer.special_trait(), "phase_shift");
    }

    #[test]
    fn test_passive_type_anchor_turtle_stats() {
        assert_eq!(PassiveType::AnchorTurtle.base_hp(), 20);
        assert_eq!(PassiveType::AnchorTurtle.drop_item(), "anchor_shell");
        assert_eq!(PassiveType::AnchorTurtle.special_trait(), "time_anchor");
    }

    #[test]
    fn test_passive_type_habitats() {
        assert_eq!(PassiveType::MemoryMoth.habitat(), "temporal_rifts");
        assert_eq!(PassiveType::LoopFish.habitat(), "time_pools");
        assert_eq!(PassiveType::EchoRabbit.habitat(), "echo_fields");
        assert_eq!(PassiveType::PhaseDeer.habitat(), "phase_boundaries");
        assert_eq!(PassiveType::AnchorTurtle.habitat(), "anchor_points");
    }

    #[test]
    fn test_passive_type_display_names() {
        assert_eq!(PassiveType::MemoryMoth.display_name(), "Memory Moth");
        assert_eq!(PassiveType::LoopFish.display_name(), "Loop Fish");
        assert_eq!(PassiveType::EchoRabbit.display_name(), "Echo Rabbit");
        assert_eq!(PassiveType::PhaseDeer.display_name(), "Phase Deer");
        assert_eq!(PassiveType::AnchorTurtle.display_name(), "Anchor Turtle");
    }

    #[test]
    fn test_passive_type_all() {
        let all = PassiveType::all();
        assert_eq!(all.len(), 5);
        assert!(all.contains(&PassiveType::MemoryMoth));
        assert!(all.contains(&PassiveType::LoopFish));
        assert!(all.contains(&PassiveType::EchoRabbit));
        assert!(all.contains(&PassiveType::PhaseDeer));
        assert!(all.contains(&PassiveType::AnchorTurtle));
    }

    #[test]
    fn test_loop_fish_even_loop_spawn() {
        assert!(!PassiveType::LoopFish.can_spawn_on_loop(1));
        assert!(PassiveType::LoopFish.can_spawn_on_loop(2));
        assert!(!PassiveType::LoopFish.can_spawn_on_loop(3));
        assert!(PassiveType::LoopFish.can_spawn_on_loop(4));
    }

    #[test]
    fn test_phase_deer_min_loop() {
        assert!(!PassiveType::PhaseDeer.can_spawn_on_loop(1));
        assert!(!PassiveType::PhaseDeer.can_spawn_on_loop(2));
        assert!(PassiveType::PhaseDeer.can_spawn_on_loop(3));
        assert!(PassiveType::PhaseDeer.can_spawn_on_loop(10));
    }

    #[test]
    fn test_memory_moth_always_spawns() {
        assert!(PassiveType::MemoryMoth.can_spawn_on_loop(1));
        assert!(PassiveType::MemoryMoth.can_spawn_on_loop(2));
        assert!(PassiveType::MemoryMoth.can_spawn_on_loop(100));
    }

    #[test]
    fn test_passive_creature_new() {
        let creature = PassiveCreature::new(PassiveType::AnchorTurtle);

        assert_eq!(creature.creature_type(), PassiveType::AnchorTurtle);
        assert_eq!(creature.hp(), 20);
        assert_eq!(creature.max_hp(), 20);
        assert_eq!(creature.drop_item(), "anchor_shell");
        assert_eq!(creature.special_trait(), "time_anchor");
        assert!(creature.is_alive());
        assert!(!creature.is_phased());
    }

    #[test]
    fn test_passive_creature_new_with_loop() {
        let creature = PassiveCreature::new_with_loop(PassiveType::LoopFish, 4);

        assert_eq!(creature.spawn_loop(), 4);
        assert_eq!(creature.creature_type(), PassiveType::LoopFish);
    }

    #[test]
    fn test_passive_creature_take_damage() {
        let mut creature = PassiveCreature::new(PassiveType::EchoRabbit);
        assert_eq!(creature.hp(), 6);

        let dealt = creature.take_damage(3);
        assert_eq!(dealt, 3);
        assert_eq!(creature.hp(), 3);
        assert!(creature.is_alive());
    }

    #[test]
    fn test_passive_creature_death() {
        let mut creature = PassiveCreature::new(PassiveType::MemoryMoth);
        creature.take_damage(10);

        assert_eq!(creature.hp(), 0);
        assert!(!creature.is_alive());
    }

    #[test]
    fn test_passive_creature_overkill() {
        let mut creature = PassiveCreature::new(PassiveType::MemoryMoth);
        let dealt = creature.take_damage(100);

        assert_eq!(dealt, 8);
        assert_eq!(creature.hp(), 0);
    }

    #[test]
    fn test_passive_creature_on_catch() {
        let mut creature = PassiveCreature::new(PassiveType::EchoRabbit);
        assert!(creature.is_alive());

        let drop = creature.on_catch();
        assert!(drop.is_some());
        assert_eq!(drop.unwrap(), "loop_fiber");
        assert!(!creature.is_alive());
    }

    #[test]
    fn test_passive_creature_on_catch_when_dead() {
        let mut creature = PassiveCreature::new(PassiveType::EchoRabbit);
        creature.take_damage(100);

        let drop = creature.on_catch();
        assert!(drop.is_none());
    }

    #[test]
    fn test_passive_creature_has_trait() {
        let creature = PassiveCreature::new(PassiveType::PhaseDeer);

        assert!(creature.has_trait("phase_shift"));
        assert!(!creature.has_trait("temporal_attraction"));
    }

    #[test]
    fn test_phase_deer_phasing() {
        let mut creature = PassiveCreature::new(PassiveType::PhaseDeer);
        assert!(!creature.is_phased());

        creature.toggle_phase();
        assert!(creature.is_phased());

        creature.toggle_phase();
        assert!(!creature.is_phased());
    }

    #[test]
    fn test_phased_creature_immune_to_damage() {
        let mut creature = PassiveCreature::new(PassiveType::PhaseDeer);
        creature.toggle_phase();
        assert!(creature.is_phased());

        let dealt = creature.take_damage(100);
        assert_eq!(dealt, 0);
        assert_eq!(creature.hp(), 15);
    }

    #[test]
    fn test_phased_creature_cannot_be_caught() {
        let mut creature = PassiveCreature::new(PassiveType::PhaseDeer);
        creature.toggle_phase();

        let drop = creature.on_catch();
        assert!(drop.is_none());
        assert!(creature.is_alive());
    }

    #[test]
    fn test_non_phase_deer_cannot_phase() {
        let mut creature = PassiveCreature::new(PassiveType::MemoryMoth);
        creature.toggle_phase();
        assert!(!creature.is_phased());
    }

    #[test]
    fn test_bonus_drop_chance() {
        let creature = PassiveCreature::new_with_loop(PassiveType::MemoryMoth, 1);

        assert!((creature.bonus_drop_chance(1) - 0.0).abs() < f32::EPSILON);
        assert!((creature.bonus_drop_chance(2) - 0.05).abs() < f32::EPSILON);
        assert!((creature.bonus_drop_chance(6) - 0.25).abs() < f32::EPSILON);
        assert!((creature.bonus_drop_chance(100) - 0.25).abs() < f32::EPSILON);
    }

    #[test]
    fn test_all_passive_types_have_unique_drops() {
        let types = PassiveType::all();
        let drops: Vec<_> = types.iter().map(|t| t.drop_item()).collect();
        for (i, drop) in drops.iter().enumerate() {
            for (j, other) in drops.iter().enumerate() {
                if i != j {
                    assert_ne!(drop, other, "Duplicate drop items found");
                }
            }
        }
    }

    #[test]
    fn test_spawn_condition_memory_moth() {
        let cond = PassiveType::MemoryMoth.spawn_condition();
        assert!(!cond.even_loops_only);
        assert_eq!(cond.min_loop, 1);
    }

    #[test]
    fn test_spawn_condition_loop_fish() {
        let cond = PassiveType::LoopFish.spawn_condition();
        assert!(cond.even_loops_only);
        assert_eq!(cond.min_loop, 2);
    }

    #[test]
    fn test_spawn_condition_echo_rabbit() {
        let cond = PassiveType::EchoRabbit.spawn_condition();
        assert!(!cond.even_loops_only);
        assert_eq!(cond.min_loop, 1);
    }

    #[test]
    fn test_spawn_condition_phase_deer() {
        let cond = PassiveType::PhaseDeer.spawn_condition();
        assert!(!cond.even_loops_only);
        assert_eq!(cond.min_loop, 3);
    }

    #[test]
    fn test_spawn_condition_anchor_turtle() {
        let cond = PassiveType::AnchorTurtle.spawn_condition();
        assert!(!cond.even_loops_only);
        assert_eq!(cond.min_loop, 1);
    }

    #[test]
    fn test_loop_parity_even() {
        assert!(LoopParity::Even.matches(2));
        assert!(LoopParity::Even.matches(4));
        assert!(LoopParity::Even.matches(100));
        assert!(!LoopParity::Even.matches(1));
        assert!(!LoopParity::Even.matches(3));
    }

    #[test]
    fn test_loop_parity_odd() {
        assert!(LoopParity::Odd.matches(1));
        assert!(LoopParity::Odd.matches(3));
        assert!(LoopParity::Odd.matches(99));
        assert!(!LoopParity::Odd.matches(2));
        assert!(!LoopParity::Odd.matches(4));
    }

    #[test]
    fn test_loop_parity_any() {
        assert!(LoopParity::Any.matches(1));
        assert!(LoopParity::Any.matches(2));
        assert!(LoopParity::Any.matches(100));
    }

    #[test]
    fn test_loop_spawn_condition_memory_moth() {
        let (phase, parity) = PassiveType::MemoryMoth.loop_spawn_condition();
        assert_eq!(phase, PassiveLoopPhase::Dawn);
        assert_eq!(parity, LoopParity::Any);
    }

    #[test]
    fn test_loop_spawn_condition_loop_fish() {
        let (phase, parity) = PassiveType::LoopFish.loop_spawn_condition();
        assert_eq!(phase, PassiveLoopPhase::Day);
        assert_eq!(parity, LoopParity::Even);
    }

    #[test]
    fn test_loop_spawn_condition_echo_rabbit() {
        let (phase, parity) = PassiveType::EchoRabbit.loop_spawn_condition();
        assert_eq!(phase, PassiveLoopPhase::Day);
        assert_eq!(parity, LoopParity::Any);
    }

    #[test]
    fn test_loop_spawn_condition_phase_deer() {
        let (phase, parity) = PassiveType::PhaseDeer.loop_spawn_condition();
        assert_eq!(phase, PassiveLoopPhase::Dusk);
        assert_eq!(parity, LoopParity::Odd);
    }

    #[test]
    fn test_loop_spawn_condition_anchor_turtle() {
        let (phase, parity) = PassiveType::AnchorTurtle.loop_spawn_condition();
        assert_eq!(phase, PassiveLoopPhase::Any);
        assert_eq!(parity, LoopParity::Any);
    }

    #[test]
    fn test_can_spawn_at_memory_moth() {
        assert!(PassiveType::MemoryMoth.can_spawn_at(PassiveLoopPhase::Dawn, 1));
        assert!(PassiveType::MemoryMoth.can_spawn_at(PassiveLoopPhase::Dawn, 2));
        assert!(!PassiveType::MemoryMoth.can_spawn_at(PassiveLoopPhase::Day, 1));
    }

    #[test]
    fn test_can_spawn_at_loop_fish() {
        assert!(PassiveType::LoopFish.can_spawn_at(PassiveLoopPhase::Day, 2));
        assert!(PassiveType::LoopFish.can_spawn_at(PassiveLoopPhase::Day, 4));
        assert!(!PassiveType::LoopFish.can_spawn_at(PassiveLoopPhase::Day, 1));
        assert!(!PassiveType::LoopFish.can_spawn_at(PassiveLoopPhase::Day, 3));
        assert!(!PassiveType::LoopFish.can_spawn_at(PassiveLoopPhase::Dawn, 2));
    }

    #[test]
    fn test_can_spawn_at_phase_deer() {
        assert!(PassiveType::PhaseDeer.can_spawn_at(PassiveLoopPhase::Dusk, 1));
        assert!(PassiveType::PhaseDeer.can_spawn_at(PassiveLoopPhase::Dusk, 3));
        assert!(!PassiveType::PhaseDeer.can_spawn_at(PassiveLoopPhase::Dusk, 2));
        assert!(!PassiveType::PhaseDeer.can_spawn_at(PassiveLoopPhase::Day, 1));
    }

    #[test]
    fn test_can_spawn_at_anchor_turtle_any() {
        assert!(PassiveType::AnchorTurtle.can_spawn_at(PassiveLoopPhase::Dawn, 1));
        assert!(PassiveType::AnchorTurtle.can_spawn_at(PassiveLoopPhase::Day, 2));
        assert!(PassiveType::AnchorTurtle.can_spawn_at(PassiveLoopPhase::Dusk, 3));
        assert!(PassiveType::AnchorTurtle.can_spawn_at(PassiveLoopPhase::Night, 4));
        assert!(PassiveType::AnchorTurtle.can_spawn_at(PassiveLoopPhase::Any, 100));
    }
}
