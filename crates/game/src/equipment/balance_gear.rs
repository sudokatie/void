//! Balance equipment for Titan survival.
//!
//! Equipment that helps players maintain balance on the moving Titan.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Types of balance gear.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BalanceGear {
    /// Boots with enhanced grip for scale surfaces.
    GrippingBoots,
    /// Hook that attaches to the Titan's tail region.
    TailHook,
    /// Gloves with suction for climbing.
    SuctionGloves,
    /// Parachute for controlled falling.
    Parachute,
    /// Harness that uses wind currents.
    WindHarness,
}

impl fmt::Display for BalanceGear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BalanceGear::GrippingBoots => write!(f, "Gripping Boots"),
            BalanceGear::TailHook => write!(f, "Tail Hook"),
            BalanceGear::SuctionGloves => write!(f, "Suction Gloves"),
            BalanceGear::Parachute => write!(f, "Parachute"),
            BalanceGear::WindHarness => write!(f, "Wind Harness"),
        }
    }
}

impl BalanceGear {
    /// Get the balance bonus for this gear type.
    #[must_use]
    pub fn balance_bonus(&self) -> f32 {
        match self {
            BalanceGear::GrippingBoots => 20.0,
            BalanceGear::TailHook => 30.0,
            BalanceGear::SuctionGloves => 15.0,
            BalanceGear::Parachute => 0.0,
            BalanceGear::WindHarness => 10.0,
        }
    }

    /// Get the base durability for this gear type.
    ///
    /// All balance gear has 100.0 base durability.
    #[must_use]
    pub fn base_durability(&self) -> f32 {
        100.0
    }

    /// Calculate synergy bonus for a set of equipped gear.
    ///
    /// - GrippingBoots + TailHook = +10 bonus
    /// - SuctionGloves + Parachute = +5 bonus
    /// - Full set (all 5) = +25 bonus
    #[must_use]
    pub fn synergy_bonus(equipped: &[BalanceGear]) -> f32 {
        let has_boots = equipped.contains(&BalanceGear::GrippingBoots);
        let has_hook = equipped.contains(&BalanceGear::TailHook);
        let has_gloves = equipped.contains(&BalanceGear::SuctionGloves);
        let has_parachute = equipped.contains(&BalanceGear::Parachute);
        let has_harness = equipped.contains(&BalanceGear::WindHarness);

        let mut bonus = 0.0;

        // Full set bonus (all 5 pieces)
        if has_boots && has_hook && has_gloves && has_parachute && has_harness {
            return 25.0;
        }

        // GrippingBoots + TailHook synergy
        if has_boots && has_hook {
            bonus += 10.0;
        }

        // SuctionGloves + Parachute synergy
        if has_gloves && has_parachute {
            bonus += 5.0;
        }

        bonus
    }
}

/// Balance equipment instance.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BalanceEquipment {
    /// Type of gear.
    pub gear: BalanceGear,
    /// Balance bonus provided.
    balance_bonus: f32,
    /// Current durability.
    pub durability: f32,
    /// Maximum durability.
    max_durability: f32,
}

impl BalanceEquipment {
    /// Create new balance equipment.
    #[must_use]
    pub fn new(gear: BalanceGear) -> Self {
        let durability = gear.base_durability();
        Self {
            gear,
            balance_bonus: gear.balance_bonus(),
            durability,
            max_durability: durability,
        }
    }

    /// Apply the balance bonus.
    ///
    /// Returns the bonus amount (0 if broken).
    #[must_use]
    pub fn apply_bonus(&self) -> f32 {
        if self.is_broken() {
            0.0
        } else {
            self.balance_bonus
        }
    }

    /// Use the equipment, consuming durability (0.5 per use).
    ///
    /// Returns `false` if the equipment is broken.
    pub fn use_equipment(&mut self) -> bool {
        if self.durability <= 0.0 {
            return false;
        }
        self.durability = (self.durability - 0.5).max(0.0);
        true
    }

    /// Check if the equipment is broken.
    #[must_use]
    pub fn is_broken(&self) -> bool {
        self.durability <= 0.0
    }

    /// Get durability as a percentage.
    #[must_use]
    pub fn durability_percent(&self) -> f32 {
        self.durability / self.max_durability
    }

    /// Repair the equipment by a given amount.
    pub fn repair(&mut self, amount: f32) {
        self.durability = (self.durability + amount).min(self.max_durability);
    }

    /// Get the gear type.
    #[must_use]
    pub fn gear_type(&self) -> BalanceGear {
        self.gear
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balance_gear_display() {
        assert_eq!(format!("{}", BalanceGear::GrippingBoots), "Gripping Boots");
        assert_eq!(format!("{}", BalanceGear::TailHook), "Tail Hook");
        assert_eq!(format!("{}", BalanceGear::SuctionGloves), "Suction Gloves");
        assert_eq!(format!("{}", BalanceGear::Parachute), "Parachute");
        assert_eq!(format!("{}", BalanceGear::WindHarness), "Wind Harness");
    }

    #[test]
    fn test_balance_gear_bonuses() {
        assert!((BalanceGear::GrippingBoots.balance_bonus() - 20.0).abs() < f32::EPSILON);
        assert!((BalanceGear::TailHook.balance_bonus() - 30.0).abs() < f32::EPSILON);
        assert!((BalanceGear::SuctionGloves.balance_bonus() - 15.0).abs() < f32::EPSILON);
        assert!((BalanceGear::Parachute.balance_bonus() - 0.0).abs() < f32::EPSILON);
        assert!((BalanceGear::WindHarness.balance_bonus() - 10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_equipment_new_gripping_boots() {
        let equip = BalanceEquipment::new(BalanceGear::GrippingBoots);
        assert_eq!(equip.gear, BalanceGear::GrippingBoots);
        assert!((equip.durability - 100.0).abs() < f32::EPSILON);
        assert!(!equip.is_broken());
    }

    #[test]
    fn test_balance_equipment_new_tail_hook() {
        let equip = BalanceEquipment::new(BalanceGear::TailHook);
        assert_eq!(equip.gear, BalanceGear::TailHook);
        assert!((equip.durability - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_equipment_apply_bonus() {
        let equip = BalanceEquipment::new(BalanceGear::TailHook);
        assert!((equip.apply_bonus() - 30.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_equipment_apply_bonus_broken() {
        let mut equip = BalanceEquipment::new(BalanceGear::TailHook);
        equip.durability = 0.0;
        assert!((equip.apply_bonus() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_equipment_use_success() {
        let mut equip = BalanceEquipment::new(BalanceGear::SuctionGloves);
        assert!(equip.use_equipment());
        assert!((equip.durability - 99.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_equipment_use_broken() {
        let mut equip = BalanceEquipment::new(BalanceGear::Parachute);
        equip.durability = 0.0;
        assert!(!equip.use_equipment());
    }

    #[test]
    fn test_balance_equipment_is_broken() {
        let mut equip = BalanceEquipment::new(BalanceGear::WindHarness);
        assert!(!equip.is_broken());
        equip.durability = 0.0;
        assert!(equip.is_broken());
    }

    #[test]
    fn test_balance_equipment_durability_percent() {
        let mut equip = BalanceEquipment::new(BalanceGear::GrippingBoots);
        assert!((equip.durability_percent() - 1.0).abs() < f32::EPSILON);
        equip.durability = 50.0;
        assert!((equip.durability_percent() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_equipment_repair() {
        let mut equip = BalanceEquipment::new(BalanceGear::TailHook);
        equip.durability = 20.0;
        equip.repair(30.0);
        assert!((equip.durability - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_equipment_repair_capped() {
        let mut equip = BalanceEquipment::new(BalanceGear::TailHook);
        equip.durability = 70.0;
        equip.repair(50.0);
        assert!((equip.durability - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_gear_synergy_boots_and_hook() {
        let equipped = vec![BalanceGear::GrippingBoots, BalanceGear::TailHook];
        assert!((BalanceGear::synergy_bonus(&equipped) - 10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_gear_synergy_gloves_and_parachute() {
        let equipped = vec![BalanceGear::SuctionGloves, BalanceGear::Parachute];
        assert!((BalanceGear::synergy_bonus(&equipped) - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_gear_synergy_both_combos() {
        let equipped = vec![
            BalanceGear::GrippingBoots,
            BalanceGear::TailHook,
            BalanceGear::SuctionGloves,
            BalanceGear::Parachute,
        ];
        assert!((BalanceGear::synergy_bonus(&equipped) - 15.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_gear_synergy_full_set() {
        let equipped = vec![
            BalanceGear::GrippingBoots,
            BalanceGear::TailHook,
            BalanceGear::SuctionGloves,
            BalanceGear::Parachute,
            BalanceGear::WindHarness,
        ];
        assert!((BalanceGear::synergy_bonus(&equipped) - 25.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_gear_synergy_no_synergy() {
        let equipped = vec![BalanceGear::WindHarness];
        assert!((BalanceGear::synergy_bonus(&equipped) - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_gear_synergy_empty() {
        let equipped: Vec<BalanceGear> = vec![];
        assert!((BalanceGear::synergy_bonus(&equipped) - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_all_balance_gear_base_durability() {
        assert!((BalanceGear::GrippingBoots.base_durability() - 100.0).abs() < f32::EPSILON);
        assert!((BalanceGear::TailHook.base_durability() - 100.0).abs() < f32::EPSILON);
        assert!((BalanceGear::SuctionGloves.base_durability() - 100.0).abs() < f32::EPSILON);
        assert!((BalanceGear::Parachute.base_durability() - 100.0).abs() < f32::EPSILON);
        assert!((BalanceGear::WindHarness.base_durability() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_equipment_degradation_rate() {
        let mut equip = BalanceEquipment::new(BalanceGear::GrippingBoots);
        let initial = equip.durability;
        equip.use_equipment();
        assert!((equip.durability - (initial - 0.5)).abs() < f32::EPSILON);
    }
}
