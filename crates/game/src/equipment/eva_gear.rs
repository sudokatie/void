//! EVA equipment for space station survival.
//!
//! Provides gear for extra-vehicular activities and station repair.

use serde::{Deserialize, Serialize};

/// Types of EVA gear available.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EVAGear {
    /// Pressurized suit for vacuum survival.
    EVASuit,
    /// Propulsion system for zero-g movement.
    ThrusterPack,
    /// Emergency hull breach repair kit.
    PatchKit,
    /// High-temperature welding tool for permanent repairs.
    WeldingTorch,
    /// Spare oxygen supply canister.
    O2Canister,
    /// General purpose repair tool.
    Multitool,
    /// Safety line for EVA operations.
    TetherLine,
}

impl EVAGear {
    /// Get display name for this gear type.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            EVAGear::EVASuit => "EVA Suit",
            EVAGear::ThrusterPack => "Thruster Pack",
            EVAGear::PatchKit => "Patch Kit",
            EVAGear::WeldingTorch => "Welding Torch",
            EVAGear::O2Canister => "O2 Canister",
            EVAGear::Multitool => "Multitool",
            EVAGear::TetherLine => "Tether Line",
        }
    }

    /// Get description for this gear type.
    #[must_use]
    pub fn description(&self) -> &'static str {
        match self {
            EVAGear::EVASuit => "Pressurized suit providing protection in vacuum",
            EVAGear::ThrusterPack => "Compact propulsion system for zero-g maneuvering",
            EVAGear::PatchKit => "Emergency sealing materials for hull breaches",
            EVAGear::WeldingTorch => "High-temperature torch for permanent hull repairs",
            EVAGear::O2Canister => "Spare oxygen supply for extended EVA operations",
            EVAGear::Multitool => "Versatile tool for general repairs and maintenance",
            EVAGear::TetherLine => "Safety line to maintain station connection during EVA",
        }
    }

    /// Get base durability for this gear type.
    #[must_use]
    pub fn base_durability(&self) -> f32 {
        match self {
            EVAGear::EVASuit => 100.0,
            EVAGear::ThrusterPack => 80.0,
            EVAGear::PatchKit => 50.0,
            EVAGear::WeldingTorch => 60.0,
            EVAGear::O2Canister => 100.0,
            EVAGear::Multitool => 75.0,
            EVAGear::TetherLine => 90.0,
        }
    }

    /// Get durability cost per use.
    #[must_use]
    pub fn durability_per_use(&self) -> f32 {
        match self {
            EVAGear::EVASuit => 0.1,
            EVAGear::ThrusterPack => 2.0,
            EVAGear::PatchKit => 10.0,
            EVAGear::WeldingTorch => 5.0,
            EVAGear::O2Canister => 1.0,
            EVAGear::Multitool => 1.0,
            EVAGear::TetherLine => 0.5,
        }
    }

    /// Get the crafting cost in materials.
    #[must_use]
    pub fn craft_cost(&self) -> u32 {
        match self {
            EVAGear::EVASuit => 50,
            EVAGear::ThrusterPack => 40,
            EVAGear::PatchKit => 15,
            EVAGear::WeldingTorch => 25,
            EVAGear::O2Canister => 10,
            EVAGear::Multitool => 20,
            EVAGear::TetherLine => 12,
        }
    }

    /// Get all EVA gear types.
    #[must_use]
    pub fn all() -> &'static [EVAGear] {
        &[
            EVAGear::EVASuit,
            EVAGear::ThrusterPack,
            EVAGear::PatchKit,
            EVAGear::WeldingTorch,
            EVAGear::O2Canister,
            EVAGear::Multitool,
            EVAGear::TetherLine,
        ]
    }

    /// Check if this gear is consumable.
    #[must_use]
    pub fn is_consumable(&self) -> bool {
        matches!(self, EVAGear::PatchKit | EVAGear::O2Canister)
    }

    /// Check if this gear requires power.
    #[must_use]
    pub fn requires_power(&self) -> bool {
        matches!(self, EVAGear::ThrusterPack | EVAGear::WeldingTorch | EVAGear::Multitool)
    }
}

/// An instance of EVA equipment.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EVAEquipment {
    /// The type of gear.
    gear: EVAGear,
    /// Current durability (0.0 to max).
    durability: f32,
}

impl EVAEquipment {
    /// Create new EVA equipment.
    #[must_use]
    pub fn new(gear: EVAGear) -> Self {
        Self {
            gear,
            durability: gear.base_durability(),
        }
    }

    /// Create EVA equipment with custom durability.
    #[must_use]
    pub fn with_durability(gear: EVAGear, durability: f32) -> Self {
        Self {
            gear,
            durability: durability.clamp(0.0, gear.base_durability()),
        }
    }

    /// Get the gear type.
    #[must_use]
    pub fn gear(&self) -> EVAGear {
        self.gear
    }

    /// Get current durability.
    #[must_use]
    pub fn durability(&self) -> f32 {
        self.durability
    }

    /// Get durability as percentage (0-100).
    #[must_use]
    pub fn durability_percent(&self) -> f32 {
        (self.durability / self.gear.base_durability()) * 100.0
    }

    /// Check if the equipment is broken.
    #[must_use]
    pub fn is_broken(&self) -> bool {
        self.durability <= 0.0
    }

    /// Use the equipment.
    ///
    /// Returns true if successfully used, false if broken.
    pub fn use_equipment(&mut self) -> bool {
        if self.is_broken() {
            return false;
        }
        self.durability = (self.durability - self.gear.durability_per_use()).max(0.0);
        true
    }

    /// Repair the equipment.
    pub fn repair(&mut self, amount: f32) {
        self.durability = (self.durability + amount).min(self.gear.base_durability());
    }

    /// Degrade the equipment by a specific amount.
    pub fn degrade(&mut self, amount: f32) {
        self.durability = (self.durability - amount).max(0.0);
    }

    /// Get effectiveness based on durability (0.0-1.0).
    #[must_use]
    pub fn effectiveness(&self) -> f32 {
        self.durability / self.gear.base_durability()
    }

    /// Check if the equipment is functional (above 10% durability).
    #[must_use]
    pub fn is_functional(&self) -> bool {
        self.durability_percent() > 10.0
    }

    /// Get the maximum durability for this equipment.
    #[must_use]
    pub fn max_durability(&self) -> f32 {
        self.gear.base_durability()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // EVAGear enum tests
    #[test]
    fn test_eva_gear_display_names() {
        assert_eq!(EVAGear::EVASuit.display_name(), "EVA Suit");
        assert_eq!(EVAGear::ThrusterPack.display_name(), "Thruster Pack");
        assert_eq!(EVAGear::PatchKit.display_name(), "Patch Kit");
        assert_eq!(EVAGear::WeldingTorch.display_name(), "Welding Torch");
        assert_eq!(EVAGear::O2Canister.display_name(), "O2 Canister");
        assert_eq!(EVAGear::Multitool.display_name(), "Multitool");
        assert_eq!(EVAGear::TetherLine.display_name(), "Tether Line");
    }

    #[test]
    fn test_eva_gear_descriptions() {
        assert!(EVAGear::EVASuit.description().contains("vacuum"));
        assert!(EVAGear::ThrusterPack.description().contains("zero-g"));
        assert!(EVAGear::PatchKit.description().contains("breach"));
        assert!(EVAGear::WeldingTorch.description().contains("hull"));
        assert!(EVAGear::O2Canister.description().contains("oxygen"));
        assert!(EVAGear::Multitool.description().contains("repair"));
        assert!(EVAGear::TetherLine.description().contains("safety"));
    }

    #[test]
    fn test_eva_gear_base_durability() {
        assert!((EVAGear::EVASuit.base_durability() - 100.0).abs() < f32::EPSILON);
        assert!((EVAGear::ThrusterPack.base_durability() - 80.0).abs() < f32::EPSILON);
        assert!((EVAGear::PatchKit.base_durability() - 50.0).abs() < f32::EPSILON);
        assert!((EVAGear::WeldingTorch.base_durability() - 60.0).abs() < f32::EPSILON);
        assert!((EVAGear::O2Canister.base_durability() - 100.0).abs() < f32::EPSILON);
        assert!((EVAGear::Multitool.base_durability() - 75.0).abs() < f32::EPSILON);
        assert!((EVAGear::TetherLine.base_durability() - 90.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_eva_gear_durability_per_use() {
        assert!((EVAGear::EVASuit.durability_per_use() - 0.1).abs() < f32::EPSILON);
        assert!((EVAGear::ThrusterPack.durability_per_use() - 2.0).abs() < f32::EPSILON);
        assert!((EVAGear::PatchKit.durability_per_use() - 10.0).abs() < f32::EPSILON);
        assert!((EVAGear::WeldingTorch.durability_per_use() - 5.0).abs() < f32::EPSILON);
        assert!((EVAGear::O2Canister.durability_per_use() - 1.0).abs() < f32::EPSILON);
        assert!((EVAGear::Multitool.durability_per_use() - 1.0).abs() < f32::EPSILON);
        assert!((EVAGear::TetherLine.durability_per_use() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_eva_gear_craft_cost() {
        assert_eq!(EVAGear::EVASuit.craft_cost(), 50);
        assert_eq!(EVAGear::ThrusterPack.craft_cost(), 40);
        assert_eq!(EVAGear::PatchKit.craft_cost(), 15);
        assert_eq!(EVAGear::WeldingTorch.craft_cost(), 25);
        assert_eq!(EVAGear::O2Canister.craft_cost(), 10);
        assert_eq!(EVAGear::Multitool.craft_cost(), 20);
        assert_eq!(EVAGear::TetherLine.craft_cost(), 12);
    }

    #[test]
    fn test_eva_gear_all() {
        let all = EVAGear::all();
        assert_eq!(all.len(), 7);
        assert!(all.contains(&EVAGear::EVASuit));
        assert!(all.contains(&EVAGear::ThrusterPack));
        assert!(all.contains(&EVAGear::PatchKit));
        assert!(all.contains(&EVAGear::WeldingTorch));
        assert!(all.contains(&EVAGear::O2Canister));
        assert!(all.contains(&EVAGear::Multitool));
        assert!(all.contains(&EVAGear::TetherLine));
    }

    #[test]
    fn test_eva_gear_is_consumable() {
        assert!(!EVAGear::EVASuit.is_consumable());
        assert!(!EVAGear::ThrusterPack.is_consumable());
        assert!(EVAGear::PatchKit.is_consumable());
        assert!(!EVAGear::WeldingTorch.is_consumable());
        assert!(EVAGear::O2Canister.is_consumable());
        assert!(!EVAGear::Multitool.is_consumable());
        assert!(!EVAGear::TetherLine.is_consumable());
    }

    #[test]
    fn test_eva_gear_requires_power() {
        assert!(!EVAGear::EVASuit.requires_power());
        assert!(EVAGear::ThrusterPack.requires_power());
        assert!(!EVAGear::PatchKit.requires_power());
        assert!(EVAGear::WeldingTorch.requires_power());
        assert!(!EVAGear::O2Canister.requires_power());
        assert!(EVAGear::Multitool.requires_power());
        assert!(!EVAGear::TetherLine.requires_power());
    }

    // EVAEquipment struct tests
    #[test]
    fn test_eva_equipment_new() {
        let equip = EVAEquipment::new(EVAGear::EVASuit);
        assert_eq!(equip.gear(), EVAGear::EVASuit);
        assert!((equip.durability() - 100.0).abs() < f32::EPSILON);
        assert!(!equip.is_broken());
    }

    #[test]
    fn test_eva_equipment_with_durability() {
        let equip = EVAEquipment::with_durability(EVAGear::ThrusterPack, 50.0);
        assert_eq!(equip.gear(), EVAGear::ThrusterPack);
        assert!((equip.durability() - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_eva_equipment_with_durability_clamped() {
        let equip = EVAEquipment::with_durability(EVAGear::ThrusterPack, 200.0);
        assert!((equip.durability() - 80.0).abs() < f32::EPSILON);

        let equip2 = EVAEquipment::with_durability(EVAGear::ThrusterPack, -10.0);
        assert!((equip2.durability() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_eva_equipment_durability_percent() {
        let equip = EVAEquipment::with_durability(EVAGear::EVASuit, 50.0);
        assert!((equip.durability_percent() - 50.0).abs() < f32::EPSILON);

        let equip2 = EVAEquipment::new(EVAGear::EVASuit);
        assert!((equip2.durability_percent() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_eva_equipment_is_broken() {
        let mut equip = EVAEquipment::new(EVAGear::PatchKit);
        assert!(!equip.is_broken());

        equip.degrade(50.0);
        assert!(equip.is_broken());
    }

    #[test]
    fn test_eva_equipment_use() {
        let mut equip = EVAEquipment::new(EVAGear::WeldingTorch);
        let initial = equip.durability();

        assert!(equip.use_equipment());
        assert!(equip.durability() < initial);
        assert!((equip.durability() - (60.0 - 5.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn test_eva_equipment_use_when_broken() {
        let mut equip = EVAEquipment::with_durability(EVAGear::Multitool, 0.0);
        assert!(equip.is_broken());
        assert!(!equip.use_equipment());
    }

    #[test]
    fn test_eva_equipment_repair() {
        let mut equip = EVAEquipment::with_durability(EVAGear::TetherLine, 50.0);
        equip.repair(20.0);
        assert!((equip.durability() - 70.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_eva_equipment_repair_capped() {
        let mut equip = EVAEquipment::with_durability(EVAGear::TetherLine, 80.0);
        equip.repair(50.0);
        assert!((equip.durability() - 90.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_eva_equipment_degrade() {
        let mut equip = EVAEquipment::new(EVAGear::O2Canister);
        equip.degrade(30.0);
        assert!((equip.durability() - 70.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_eva_equipment_degrade_min() {
        let mut equip = EVAEquipment::with_durability(EVAGear::O2Canister, 20.0);
        equip.degrade(50.0);
        assert!((equip.durability() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_eva_equipment_effectiveness() {
        let equip = EVAEquipment::new(EVAGear::EVASuit);
        assert!((equip.effectiveness() - 1.0).abs() < f32::EPSILON);

        let equip2 = EVAEquipment::with_durability(EVAGear::EVASuit, 50.0);
        assert!((equip2.effectiveness() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_eva_equipment_is_functional() {
        let equip = EVAEquipment::new(EVAGear::Multitool);
        assert!(equip.is_functional());

        let equip2 = EVAEquipment::with_durability(EVAGear::Multitool, 7.5);
        assert!(!equip2.is_functional());

        let equip3 = EVAEquipment::with_durability(EVAGear::Multitool, 8.0);
        assert!(equip3.is_functional());
    }

    #[test]
    fn test_eva_equipment_max_durability() {
        let equip = EVAEquipment::new(EVAGear::ThrusterPack);
        assert!((equip.max_durability() - 80.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_eva_equipment_multiple_uses() {
        let mut equip = EVAEquipment::new(EVAGear::PatchKit);
        // PatchKit: 50 durability, 10 per use = 5 uses
        for _ in 0..5 {
            assert!(equip.use_equipment());
        }
        assert!(equip.is_broken());
        assert!(!equip.use_equipment());
    }

    #[test]
    fn test_eva_equipment_use_and_repair_cycle() {
        let mut equip = EVAEquipment::new(EVAGear::Multitool);

        // Use 10 times (10 durability used)
        for _ in 0..10 {
            equip.use_equipment();
        }
        assert!((equip.durability() - 65.0).abs() < f32::EPSILON);

        // Repair 5
        equip.repair(5.0);
        assert!((equip.durability() - 70.0).abs() < f32::EPSILON);

        // Use 5 more times
        for _ in 0..5 {
            equip.use_equipment();
        }
        assert!((equip.durability() - 65.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_all_gear_types_create_equipment() {
        for gear in EVAGear::all() {
            let equip = EVAEquipment::new(*gear);
            assert!(!equip.is_broken());
            assert!(equip.use_equipment());
        }
    }

    #[test]
    fn test_eva_suit_long_duration() {
        let mut equip = EVAEquipment::new(EVAGear::EVASuit);
        // EVASuit: 100 durability, 0.1 per use = 1000 uses
        for _ in 0..1000 {
            equip.use_equipment();
        }
        assert!(equip.is_broken());
    }

    #[test]
    fn test_thruster_pack_limited_uses() {
        let mut equip = EVAEquipment::new(EVAGear::ThrusterPack);
        // ThrusterPack: 80 durability, 2.0 per use = 40 uses
        for _ in 0..40 {
            assert!(equip.use_equipment());
        }
        assert!(equip.is_broken());
    }
}
