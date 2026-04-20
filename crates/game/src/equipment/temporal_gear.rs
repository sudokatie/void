//! Temporal equipment for time-loop survival.
//!
//! Provides gear that interacts with the time loop mechanics.

use serde::{Deserialize, Serialize};

/// Types of temporal gear available.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TemporalGear {
    /// Watch that tracks loop progress and provides timing bonuses.
    LoopWatch,
    /// Lens that reveals hidden temporal anomalies.
    MemoryLens,
    /// Scanner that detects paradox buildup.
    ParadoxScanner,
    /// Anchor that reduces temporal displacement effects.
    TemporalAnchor,
    /// Boots that allow faster movement during time transitions.
    ChronoBoots,
}

impl TemporalGear {
    /// Get display name for this gear type.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            TemporalGear::LoopWatch => "Loop Watch",
            TemporalGear::MemoryLens => "Memory Lens",
            TemporalGear::ParadoxScanner => "Paradox Scanner",
            TemporalGear::TemporalAnchor => "Temporal Anchor",
            TemporalGear::ChronoBoots => "Chrono Boots",
        }
    }

    /// Get description for this gear type.
    #[must_use]
    pub fn description(&self) -> &'static str {
        match self {
            TemporalGear::LoopWatch => "Tracks loop progress and warns of impending resets",
            TemporalGear::MemoryLens => "Reveals ghost items and temporal anomalies",
            TemporalGear::ParadoxScanner => "Detects paradox buildup in the area",
            TemporalGear::TemporalAnchor => "Reduces temporal displacement and drift",
            TemporalGear::ChronoBoots => "Increases movement speed during time transitions",
        }
    }

    /// Get the loop bonus multiplier for this gear.
    #[must_use]
    pub fn loop_bonus(&self) -> f32 {
        match self {
            TemporalGear::LoopWatch => 1.1,
            TemporalGear::MemoryLens => 1.05,
            TemporalGear::ParadoxScanner => 1.0,
            TemporalGear::TemporalAnchor => 1.15,
            TemporalGear::ChronoBoots => 1.2,
        }
    }

    /// Get base durability for this gear type.
    #[must_use]
    pub fn base_durability(&self) -> u32 {
        100
    }

    /// Get the crafting cost in temporal dust.
    #[must_use]
    pub fn craft_cost(&self) -> u32 {
        match self {
            TemporalGear::LoopWatch => 10,
            TemporalGear::MemoryLens => 15,
            TemporalGear::ParadoxScanner => 20,
            TemporalGear::TemporalAnchor => 25,
            TemporalGear::ChronoBoots => 30,
        }
    }

    /// Get all temporal gear types.
    #[must_use]
    pub fn all() -> &'static [TemporalGear] {
        &[
            TemporalGear::LoopWatch,
            TemporalGear::MemoryLens,
            TemporalGear::ParadoxScanner,
            TemporalGear::TemporalAnchor,
            TemporalGear::ChronoBoots,
        ]
    }

    /// Get loop-based bonus multiplier for this gear type.
    ///
    /// Returns a bonus multiplier based on the current loop number:
    /// - ChronoBoots: 1.0 + 0.2 * min(loop, 5) (speed scaling)
    /// - LoopWatch: 1.0 (no bonus)
    /// - MemoryLens: 1.1 if loop > 3, else 1.0
    /// - ParadoxScanner: 1.0 + 0.05 * min(loop, 5) (up to 25%)
    /// - TemporalAnchor: 1.15 (flat bonus)
    #[must_use]
    pub fn loop_based_bonus(&self, current_loop: u32) -> f32 {
        match self {
            TemporalGear::ChronoBoots => {
                let loop_factor = current_loop.min(5) as f32;
                1.0 + 0.2 * loop_factor
            }
            TemporalGear::LoopWatch => 1.0,
            TemporalGear::MemoryLens => {
                if current_loop > 3 {
                    1.1
                } else {
                    1.0
                }
            }
            TemporalGear::ParadoxScanner => {
                let loop_factor = current_loop.min(5) as f32;
                1.0 + 0.05 * loop_factor
            }
            TemporalGear::TemporalAnchor => 1.15,
        }
    }

    /// Get the durability degradation rate per use.
    #[must_use]
    pub fn durability_per_use(&self) -> f32 {
        0.5
    }
}

/// An instance of temporal equipment.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TemporalEquipment {
    /// The type of gear.
    gear: TemporalGear,
    /// Current durability (0.0-100.0).
    durability: f32,
    /// Loop bonus multiplier.
    loop_bonus: f32,
    /// Whether the equipment is currently active.
    active: bool,
    /// Uses remaining before needing recharge.
    charges: u32,
}

impl TemporalEquipment {
    /// Create new temporal equipment.
    #[must_use]
    pub fn new(gear: TemporalGear) -> Self {
        Self {
            gear,
            durability: 100.0,
            loop_bonus: gear.loop_bonus(),
            active: false,
            charges: 10,
        }
    }

    /// Get the gear type.
    #[must_use]
    pub fn gear(&self) -> TemporalGear {
        self.gear
    }

    /// Get current durability as integer (truncated).
    #[must_use]
    pub fn durability(&self) -> u32 {
        self.durability as u32
    }

    /// Get current durability as float.
    #[must_use]
    pub fn durability_f32(&self) -> f32 {
        self.durability
    }

    /// Get the loop bonus multiplier.
    #[must_use]
    pub fn loop_bonus(&self) -> f32 {
        self.loop_bonus
    }

    /// Check if equipment is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Get remaining charges.
    #[must_use]
    pub fn charges(&self) -> u32 {
        self.charges
    }

    /// Check if the equipment is broken.
    #[must_use]
    pub fn is_broken(&self) -> bool {
        self.durability <= 0.0
    }

    /// Apply the equipment's effect.
    ///
    /// Returns a description of the effect applied.
    #[must_use]
    pub fn apply_effect(&self) -> &'static str {
        if self.is_broken() {
            return "Equipment is broken";
        }
        if !self.active {
            return "Equipment is not active";
        }
        match self.gear {
            TemporalGear::LoopWatch => "Time awareness increased, loop progress visible",
            TemporalGear::MemoryLens => "Temporal anomalies and ghost items revealed",
            TemporalGear::ParadoxScanner => "Paradox levels displayed on HUD",
            TemporalGear::TemporalAnchor => "Temporal displacement resistance active",
            TemporalGear::ChronoBoots => "Movement speed boosted during transitions",
        }
    }

    /// Use the equipment, consuming durability (0.5 per use) and a charge.
    ///
    /// Returns true if successfully used.
    pub fn use_equipment(&mut self) -> bool {
        if self.is_broken() || self.charges == 0 {
            return false;
        }
        self.durability = (self.durability - self.gear.durability_per_use()).max(0.0);
        self.charges = self.charges.saturating_sub(1);
        self.active = true;
        true
    }

    /// Activate the equipment without consuming charges.
    pub fn activate(&mut self) {
        if !self.is_broken() {
            self.active = true;
        }
    }

    /// Deactivate the equipment.
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Repair the equipment.
    pub fn repair(&mut self, amount: u32) {
        self.durability = (self.durability + amount as f32).min(100.0);
    }

    /// Recharge the equipment.
    pub fn recharge(&mut self, amount: u32) {
        self.charges = (self.charges + amount).min(10);
    }

    /// Reduce durability by a specific amount.
    pub fn degrade(&mut self, amount: u32) {
        self.durability = (self.durability - amount as f32).max(0.0);
        if self.durability <= 0.0 {
            self.active = false;
        }
    }

    /// Get effectiveness based on durability.
    #[must_use]
    pub fn effectiveness(&self) -> f32 {
        self.durability / 100.0
    }

    /// Get loop-based bonus for this equipment at the given loop.
    #[must_use]
    pub fn get_loop_bonus(&self, current_loop: u32) -> f32 {
        if self.is_broken() {
            1.0
        } else {
            self.gear.loop_based_bonus(current_loop) * self.effectiveness()
        }
    }

    /// Calculate actual loop bonus considering durability.
    #[must_use]
    pub fn effective_loop_bonus(&self) -> f32 {
        1.0 + (self.loop_bonus - 1.0) * self.effectiveness()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temporal_gear_display_names() {
        assert_eq!(TemporalGear::LoopWatch.display_name(), "Loop Watch");
        assert_eq!(TemporalGear::MemoryLens.display_name(), "Memory Lens");
        assert_eq!(TemporalGear::ParadoxScanner.display_name(), "Paradox Scanner");
        assert_eq!(TemporalGear::TemporalAnchor.display_name(), "Temporal Anchor");
        assert_eq!(TemporalGear::ChronoBoots.display_name(), "Chrono Boots");
    }

    #[test]
    fn test_temporal_gear_descriptions() {
        assert!(TemporalGear::LoopWatch.description().contains("loop"));
        assert!(TemporalGear::MemoryLens.description().contains("anomalies"));
        assert!(TemporalGear::ParadoxScanner.description().contains("paradox"));
        assert!(TemporalGear::TemporalAnchor.description().contains("displacement"));
        assert!(TemporalGear::ChronoBoots.description().contains("movement"));
    }

    #[test]
    fn test_temporal_gear_loop_bonus() {
        assert!((TemporalGear::LoopWatch.loop_bonus() - 1.1).abs() < f32::EPSILON);
        assert!((TemporalGear::MemoryLens.loop_bonus() - 1.05).abs() < f32::EPSILON);
        assert!((TemporalGear::ParadoxScanner.loop_bonus() - 1.0).abs() < f32::EPSILON);
        assert!((TemporalGear::TemporalAnchor.loop_bonus() - 1.15).abs() < f32::EPSILON);
        assert!((TemporalGear::ChronoBoots.loop_bonus() - 1.2).abs() < f32::EPSILON);
    }

    #[test]
    fn test_temporal_gear_base_durability() {
        for gear in TemporalGear::all() {
            assert_eq!(gear.base_durability(), 100);
        }
    }

    #[test]
    fn test_temporal_gear_craft_cost() {
        assert_eq!(TemporalGear::LoopWatch.craft_cost(), 10);
        assert_eq!(TemporalGear::MemoryLens.craft_cost(), 15);
        assert_eq!(TemporalGear::ParadoxScanner.craft_cost(), 20);
        assert_eq!(TemporalGear::TemporalAnchor.craft_cost(), 25);
        assert_eq!(TemporalGear::ChronoBoots.craft_cost(), 30);
    }

    #[test]
    fn test_temporal_gear_all() {
        let all = TemporalGear::all();
        assert_eq!(all.len(), 5);
        assert!(all.contains(&TemporalGear::LoopWatch));
        assert!(all.contains(&TemporalGear::MemoryLens));
        assert!(all.contains(&TemporalGear::ParadoxScanner));
        assert!(all.contains(&TemporalGear::TemporalAnchor));
        assert!(all.contains(&TemporalGear::ChronoBoots));
    }

    #[test]
    fn test_temporal_equipment_new() {
        let equip = TemporalEquipment::new(TemporalGear::LoopWatch);

        assert_eq!(equip.gear(), TemporalGear::LoopWatch);
        assert_eq!(equip.durability(), 100);
        assert!((equip.loop_bonus() - 1.1).abs() < f32::EPSILON);
        assert!(!equip.is_active());
        assert!(!equip.is_broken());
        assert_eq!(equip.charges(), 10);
    }

    #[test]
    fn test_temporal_equipment_use() {
        let mut equip = TemporalEquipment::new(TemporalGear::ChronoBoots);

        assert!(equip.use_equipment());
        assert!(equip.is_active());
        assert_eq!(equip.durability(), 99);
        assert_eq!(equip.charges(), 9);
    }

    #[test]
    fn test_temporal_equipment_apply_effect_inactive() {
        let equip = TemporalEquipment::new(TemporalGear::MemoryLens);
        assert_eq!(equip.apply_effect(), "Equipment is not active");
    }

    #[test]
    fn test_temporal_equipment_apply_effect_active() {
        let mut equip = TemporalEquipment::new(TemporalGear::MemoryLens);
        equip.activate();
        assert!(equip.apply_effect().contains("anomalies"));
    }

    #[test]
    fn test_temporal_equipment_apply_effect_broken() {
        let mut equip = TemporalEquipment::new(TemporalGear::LoopWatch);
        equip.degrade(100);
        equip.activate();
        assert_eq!(equip.apply_effect(), "Equipment is broken");
    }

    #[test]
    fn test_temporal_equipment_cannot_use_when_broken() {
        let mut equip = TemporalEquipment::new(TemporalGear::ParadoxScanner);
        equip.degrade(100);
        assert!(equip.is_broken());
        assert!(!equip.use_equipment());
    }

    #[test]
    fn test_temporal_equipment_cannot_use_when_no_charges() {
        let mut equip = TemporalEquipment::new(TemporalGear::TemporalAnchor);
        for _ in 0..10 {
            equip.use_equipment();
        }
        assert_eq!(equip.charges(), 0);
        assert!(!equip.use_equipment());
    }

    #[test]
    fn test_temporal_equipment_repair() {
        let mut equip = TemporalEquipment::new(TemporalGear::ChronoBoots);
        equip.degrade(50);
        assert_eq!(equip.durability(), 50);

        equip.repair(30);
        assert_eq!(equip.durability(), 80);

        equip.repair(100);
        assert_eq!(equip.durability(), 100);
    }

    #[test]
    fn test_temporal_equipment_recharge() {
        let mut equip = TemporalEquipment::new(TemporalGear::LoopWatch);
        for _ in 0..5 {
            equip.use_equipment();
        }
        assert_eq!(equip.charges(), 5);

        equip.recharge(3);
        assert_eq!(equip.charges(), 8);

        equip.recharge(10);
        assert_eq!(equip.charges(), 10);
    }

    #[test]
    fn test_temporal_equipment_deactivate() {
        let mut equip = TemporalEquipment::new(TemporalGear::MemoryLens);
        equip.activate();
        assert!(equip.is_active());

        equip.deactivate();
        assert!(!equip.is_active());
    }

    #[test]
    fn test_temporal_equipment_effectiveness() {
        let mut equip = TemporalEquipment::new(TemporalGear::ParadoxScanner);
        assert!((equip.effectiveness() - 1.0).abs() < f32::EPSILON);

        equip.degrade(50);
        assert!((equip.effectiveness() - 0.5).abs() < f32::EPSILON);

        equip.degrade(50);
        assert!((equip.effectiveness() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_temporal_equipment_effective_loop_bonus() {
        let mut equip = TemporalEquipment::new(TemporalGear::LoopWatch);
        // Full durability: 1.1 bonus
        assert!((equip.effective_loop_bonus() - 1.1).abs() < f32::EPSILON);

        equip.degrade(50);
        // Half durability: 1.0 + 0.1 * 0.5 = 1.05
        assert!((equip.effective_loop_bonus() - 1.05).abs() < f32::EPSILON);

        equip.degrade(50);
        // Zero durability: 1.0
        assert!((equip.effective_loop_bonus() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_broken_equipment_deactivates() {
        let mut equip = TemporalEquipment::new(TemporalGear::ChronoBoots);
        equip.activate();
        assert!(equip.is_active());

        equip.degrade(100);
        assert!(equip.is_broken());
        assert!(!equip.is_active());
    }

    #[test]
    fn test_cannot_activate_broken_equipment() {
        let mut equip = TemporalEquipment::new(TemporalGear::TemporalAnchor);
        equip.degrade(100);
        equip.activate();
        assert!(!equip.is_active());
    }

    #[test]
    fn test_loop_based_bonus_chrono_boots() {
        // ChronoBoots: 1.0 + 0.2 * min(loop, 5)
        assert!((TemporalGear::ChronoBoots.loop_based_bonus(1) - 1.2).abs() < f32::EPSILON);
        assert!((TemporalGear::ChronoBoots.loop_based_bonus(3) - 1.6).abs() < f32::EPSILON);
        assert!((TemporalGear::ChronoBoots.loop_based_bonus(5) - 2.0).abs() < f32::EPSILON);
        assert!((TemporalGear::ChronoBoots.loop_based_bonus(10) - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_loop_based_bonus_loop_watch() {
        // LoopWatch: always 1.0
        assert!((TemporalGear::LoopWatch.loop_based_bonus(1) - 1.0).abs() < f32::EPSILON);
        assert!((TemporalGear::LoopWatch.loop_based_bonus(5) - 1.0).abs() < f32::EPSILON);
        assert!((TemporalGear::LoopWatch.loop_based_bonus(100) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_loop_based_bonus_memory_lens() {
        // MemoryLens: 1.1 if loop > 3, else 1.0
        assert!((TemporalGear::MemoryLens.loop_based_bonus(1) - 1.0).abs() < f32::EPSILON);
        assert!((TemporalGear::MemoryLens.loop_based_bonus(3) - 1.0).abs() < f32::EPSILON);
        assert!((TemporalGear::MemoryLens.loop_based_bonus(4) - 1.1).abs() < f32::EPSILON);
        assert!((TemporalGear::MemoryLens.loop_based_bonus(10) - 1.1).abs() < f32::EPSILON);
    }

    #[test]
    fn test_loop_based_bonus_paradox_scanner() {
        // ParadoxScanner: 1.0 + 0.05 * min(loop, 5)
        assert!((TemporalGear::ParadoxScanner.loop_based_bonus(1) - 1.05).abs() < f32::EPSILON);
        assert!((TemporalGear::ParadoxScanner.loop_based_bonus(3) - 1.15).abs() < f32::EPSILON);
        assert!((TemporalGear::ParadoxScanner.loop_based_bonus(5) - 1.25).abs() < f32::EPSILON);
        assert!((TemporalGear::ParadoxScanner.loop_based_bonus(10) - 1.25).abs() < f32::EPSILON);
    }

    #[test]
    fn test_loop_based_bonus_temporal_anchor() {
        // TemporalAnchor: flat 1.15
        assert!((TemporalGear::TemporalAnchor.loop_based_bonus(1) - 1.15).abs() < f32::EPSILON);
        assert!((TemporalGear::TemporalAnchor.loop_based_bonus(5) - 1.15).abs() < f32::EPSILON);
        assert!((TemporalGear::TemporalAnchor.loop_based_bonus(100) - 1.15).abs() < f32::EPSILON);
    }

    #[test]
    fn test_durability_per_use() {
        for gear in TemporalGear::all() {
            assert!((gear.durability_per_use() - 0.5).abs() < f32::EPSILON);
        }
    }

    #[test]
    fn test_equipment_use_degrades_half_durability() {
        let mut equip = TemporalEquipment::new(TemporalGear::ChronoBoots);
        assert!((equip.durability_f32() - 100.0).abs() < f32::EPSILON);

        equip.use_equipment();
        assert!((equip.durability_f32() - 99.5).abs() < f32::EPSILON);

        equip.use_equipment();
        assert!((equip.durability_f32() - 99.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_equipment_durability_200_uses() {
        let mut equip = TemporalEquipment::new(TemporalGear::LoopWatch);
        // With 0.5 degradation per use, 200 uses should drain 100 durability
        for _ in 0..10 {
            equip.recharge(10);
        }
        for _ in 0..200 {
            if equip.charges() == 0 {
                equip.recharge(10);
            }
            equip.use_equipment();
        }
        assert!(equip.is_broken());
    }

    #[test]
    fn test_equipment_get_loop_bonus() {
        let equip = TemporalEquipment::new(TemporalGear::ChronoBoots);
        // Full durability, loop 5: 2.0 * 1.0 = 2.0
        assert!((equip.get_loop_bonus(5) - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_equipment_get_loop_bonus_degraded() {
        let mut equip = TemporalEquipment::new(TemporalGear::ChronoBoots);
        equip.degrade(50);
        // 50% durability, loop 5: 2.0 * 0.5 = 1.0
        assert!((equip.get_loop_bonus(5) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_equipment_get_loop_bonus_broken() {
        let mut equip = TemporalEquipment::new(TemporalGear::ChronoBoots);
        equip.degrade(100);
        // Broken equipment returns 1.0
        assert!((equip.get_loop_bonus(5) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_durability_f32_accessor() {
        let mut equip = TemporalEquipment::new(TemporalGear::MemoryLens);
        equip.use_equipment();
        assert!((equip.durability_f32() - 99.5).abs() < f32::EPSILON);
        assert_eq!(equip.durability(), 99); // truncated
    }
}
