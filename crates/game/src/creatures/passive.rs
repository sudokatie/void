//! Passive creature types for space station survival.
//!
//! Harmless space creatures that provide useful resources.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Types of passive creatures in the void.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PassiveType {
    /// Moth that is attracted to electrical systems.
    CircuitMoth,
    /// Fish-like creature that lives in coolant systems.
    CoolantFish,
    /// Plant-like organism that releases spores.
    SporeBloom,
    /// Fluffy creature that collects dust particles.
    DustBunny,
    /// Crab-like creature with a hard exoskeleton.
    StarCrab,
}

impl PassiveType {
    /// Get base HP for this creature type.
    #[must_use]
    pub fn base_hp(&self) -> u32 {
        match self {
            PassiveType::CircuitMoth => 8,
            PassiveType::CoolantFish => 10,
            PassiveType::SporeBloom => 12,
            PassiveType::DustBunny => 6,
            PassiveType::StarCrab => 15,
        }
    }

    /// Get the drop item for this creature type.
    #[must_use]
    pub fn drop_item(&self) -> &'static str {
        match self {
            PassiveType::CircuitMoth => "conductive_dust",
            PassiveType::CoolantFish => "coolant_scale",
            PassiveType::SporeBloom => "bio_compound",
            PassiveType::DustBunny => "filter_fiber",
            PassiveType::StarCrab => "hull_chitin",
        }
    }

    /// Get display name for this creature type.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            PassiveType::CircuitMoth => "Circuit Moth",
            PassiveType::CoolantFish => "Coolant Fish",
            PassiveType::SporeBloom => "Spore Bloom",
            PassiveType::DustBunny => "Dust Bunny",
            PassiveType::StarCrab => "Star Crab",
        }
    }

    /// Get the habitat location for this creature.
    #[must_use]
    pub fn habitat(&self) -> &'static str {
        match self {
            PassiveType::CircuitMoth => "electrical_panels",
            PassiveType::CoolantFish => "coolant_pipes",
            PassiveType::SporeBloom => "hydroponics",
            PassiveType::DustBunny => "air_vents",
            PassiveType::StarCrab => "outer_hull",
        }
    }

    /// Get all passive types.
    #[must_use]
    pub fn all() -> &'static [PassiveType] {
        &[
            PassiveType::CircuitMoth,
            PassiveType::CoolantFish,
            PassiveType::SporeBloom,
            PassiveType::DustBunny,
            PassiveType::StarCrab,
        ]
    }

    /// Get the special trait for this creature type.
    #[must_use]
    pub fn special_trait(&self) -> &'static str {
        match self {
            PassiveType::CircuitMoth => "bioluminescent",
            PassiveType::CoolantFish => "temperature_resistant",
            PassiveType::SporeBloom => "regenerating",
            PassiveType::DustBunny => "camouflage",
            PassiveType::StarCrab => "armored",
        }
    }
}

impl fmt::Display for PassiveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
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
    /// Whether the creature is fleeing.
    fleeing: bool,
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
            fleeing: false,
        }
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

    /// Check if the creature is alive.
    #[must_use]
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    /// Check if the creature is fleeing.
    #[must_use]
    pub fn is_fleeing(&self) -> bool {
        self.fleeing
    }

    /// Start fleeing.
    pub fn start_fleeing(&mut self) {
        self.fleeing = true;
    }

    /// Stop fleeing.
    pub fn stop_fleeing(&mut self) {
        self.fleeing = false;
    }

    /// Apply damage to the creature.
    ///
    /// Returns the actual damage dealt.
    pub fn take_damage(&mut self, amount: u32) -> u32 {
        let actual = amount.min(self.hp);
        self.hp = self.hp.saturating_sub(amount);
        if self.hp > 0 {
            self.fleeing = true;
        }
        actual
    }

    /// Attempt to catch the creature.
    ///
    /// Returns the drop item if successful (creature dies), None otherwise.
    pub fn on_catch(&mut self) -> Option<String> {
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

    /// Tick the creature for regeneration (SporeBloom).
    pub fn tick(&mut self, dt: f32) {
        if self.creature_type == PassiveType::SporeBloom && self.hp > 0 && self.hp < self.max_hp {
            let regen = (dt * 0.5) as u32;
            self.hp = (self.hp + regen).min(self.max_hp);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passive_type_circuit_moth_stats() {
        assert_eq!(PassiveType::CircuitMoth.base_hp(), 8);
        assert_eq!(PassiveType::CircuitMoth.drop_item(), "conductive_dust");
        assert_eq!(PassiveType::CircuitMoth.special_trait(), "bioluminescent");
    }

    #[test]
    fn test_passive_type_coolant_fish_stats() {
        assert_eq!(PassiveType::CoolantFish.base_hp(), 10);
        assert_eq!(PassiveType::CoolantFish.drop_item(), "coolant_scale");
        assert_eq!(PassiveType::CoolantFish.special_trait(), "temperature_resistant");
    }

    #[test]
    fn test_passive_type_spore_bloom_stats() {
        assert_eq!(PassiveType::SporeBloom.base_hp(), 12);
        assert_eq!(PassiveType::SporeBloom.drop_item(), "bio_compound");
        assert_eq!(PassiveType::SporeBloom.special_trait(), "regenerating");
    }

    #[test]
    fn test_passive_type_dust_bunny_stats() {
        assert_eq!(PassiveType::DustBunny.base_hp(), 6);
        assert_eq!(PassiveType::DustBunny.drop_item(), "filter_fiber");
        assert_eq!(PassiveType::DustBunny.special_trait(), "camouflage");
    }

    #[test]
    fn test_passive_type_star_crab_stats() {
        assert_eq!(PassiveType::StarCrab.base_hp(), 15);
        assert_eq!(PassiveType::StarCrab.drop_item(), "hull_chitin");
        assert_eq!(PassiveType::StarCrab.special_trait(), "armored");
    }

    #[test]
    fn test_passive_type_display_names() {
        assert_eq!(PassiveType::CircuitMoth.display_name(), "Circuit Moth");
        assert_eq!(PassiveType::CoolantFish.display_name(), "Coolant Fish");
        assert_eq!(PassiveType::SporeBloom.display_name(), "Spore Bloom");
        assert_eq!(PassiveType::DustBunny.display_name(), "Dust Bunny");
        assert_eq!(PassiveType::StarCrab.display_name(), "Star Crab");
    }

    #[test]
    fn test_passive_type_display_trait() {
        assert_eq!(format!("{}", PassiveType::CircuitMoth), "Circuit Moth");
    }

    #[test]
    fn test_passive_type_habitats() {
        assert_eq!(PassiveType::CircuitMoth.habitat(), "electrical_panels");
        assert_eq!(PassiveType::CoolantFish.habitat(), "coolant_pipes");
        assert_eq!(PassiveType::SporeBloom.habitat(), "hydroponics");
        assert_eq!(PassiveType::DustBunny.habitat(), "air_vents");
        assert_eq!(PassiveType::StarCrab.habitat(), "outer_hull");
    }

    #[test]
    fn test_passive_type_all() {
        let all = PassiveType::all();
        assert_eq!(all.len(), 5);
        assert!(all.contains(&PassiveType::CircuitMoth));
        assert!(all.contains(&PassiveType::CoolantFish));
        assert!(all.contains(&PassiveType::SporeBloom));
        assert!(all.contains(&PassiveType::DustBunny));
        assert!(all.contains(&PassiveType::StarCrab));
    }

    #[test]
    fn test_passive_creature_new() {
        let creature = PassiveCreature::new(PassiveType::StarCrab);
        assert_eq!(creature.creature_type(), PassiveType::StarCrab);
        assert_eq!(creature.hp(), 15);
        assert_eq!(creature.max_hp(), 15);
        assert_eq!(creature.drop_item(), "hull_chitin");
        assert_eq!(creature.special_trait(), "armored");
        assert!(creature.is_alive());
        assert!(!creature.is_fleeing());
    }

    #[test]
    fn test_passive_creature_take_damage() {
        let mut creature = PassiveCreature::new(PassiveType::DustBunny);
        assert_eq!(creature.hp(), 6);

        let dealt = creature.take_damage(3);
        assert_eq!(dealt, 3);
        assert_eq!(creature.hp(), 3);
        assert!(creature.is_alive());
        assert!(creature.is_fleeing());
    }

    #[test]
    fn test_passive_creature_death() {
        let mut creature = PassiveCreature::new(PassiveType::CircuitMoth);
        creature.take_damage(10);
        assert_eq!(creature.hp(), 0);
        assert!(!creature.is_alive());
    }

    #[test]
    fn test_passive_creature_overkill() {
        let mut creature = PassiveCreature::new(PassiveType::CircuitMoth);
        let dealt = creature.take_damage(100);
        assert_eq!(dealt, 8);
        assert_eq!(creature.hp(), 0);
    }

    #[test]
    fn test_passive_creature_on_catch() {
        let mut creature = PassiveCreature::new(PassiveType::CoolantFish);
        assert!(creature.is_alive());

        let drop = creature.on_catch();
        assert!(drop.is_some());
        assert_eq!(drop.unwrap(), "coolant_scale");
        assert!(!creature.is_alive());
    }

    #[test]
    fn test_passive_creature_on_catch_when_dead() {
        let mut creature = PassiveCreature::new(PassiveType::CoolantFish);
        creature.take_damage(100);

        let drop = creature.on_catch();
        assert!(drop.is_none());
    }

    #[test]
    fn test_passive_creature_has_trait() {
        let creature = PassiveCreature::new(PassiveType::StarCrab);
        assert!(creature.has_trait("armored"));
        assert!(!creature.has_trait("bioluminescent"));
    }

    #[test]
    fn test_passive_creature_fleeing() {
        let mut creature = PassiveCreature::new(PassiveType::DustBunny);
        assert!(!creature.is_fleeing());

        creature.start_fleeing();
        assert!(creature.is_fleeing());

        creature.stop_fleeing();
        assert!(!creature.is_fleeing());
    }

    #[test]
    fn test_spore_bloom_regeneration() {
        let mut creature = PassiveCreature::new(PassiveType::SporeBloom);
        creature.hp = 5;

        creature.tick(2.0);
        assert!(creature.hp() > 5);
    }

    #[test]
    fn test_non_spore_bloom_no_regeneration() {
        let mut creature = PassiveCreature::new(PassiveType::CircuitMoth);
        creature.hp = 5;
        let initial_hp = creature.hp();

        creature.tick(2.0);
        assert_eq!(creature.hp(), initial_hp);
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
}
