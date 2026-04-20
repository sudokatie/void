//! Loop Loom crafting station for time-loop survival.
//!
//! Crafts chest upgrades and warning signs from loop_fiber.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Output types from the Loop Loom.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LoopLoomOutput {
    /// Upgrade for temporal chest capacity.
    ChestCapacityUpgrade,
    /// Upgrade for temporal chest durability.
    ChestDurabilityUpgrade,
    /// Warning sign that persists across loops.
    WarningSign,
    /// Temporal thread for advanced crafting.
    TemporalThread,
    /// Loop marker that shows position across loops.
    LoopMarker,
}

impl LoopLoomOutput {
    /// Get display name for this output type.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            LoopLoomOutput::ChestCapacityUpgrade => "Chest Capacity Upgrade",
            LoopLoomOutput::ChestDurabilityUpgrade => "Chest Durability Upgrade",
            LoopLoomOutput::WarningSign => "Warning Sign",
            LoopLoomOutput::TemporalThread => "Temporal Thread",
            LoopLoomOutput::LoopMarker => "Loop Marker",
        }
    }

    /// Get description for this output type.
    #[must_use]
    pub fn description(&self) -> &'static str {
        match self {
            LoopLoomOutput::ChestCapacityUpgrade => "Increases temporal chest storage by 4 slots",
            LoopLoomOutput::ChestDurabilityUpgrade => "Increases temporal chest durability by 25%",
            LoopLoomOutput::WarningSign => "A sign visible across all loops",
            LoopLoomOutput::TemporalThread => "Used in advanced temporal crafting",
            LoopLoomOutput::LoopMarker => "Shows your position from previous loops",
        }
    }

    /// Get all output types.
    #[must_use]
    pub fn all() -> &'static [LoopLoomOutput] {
        &[
            LoopLoomOutput::ChestCapacityUpgrade,
            LoopLoomOutput::ChestDurabilityUpgrade,
            LoopLoomOutput::WarningSign,
            LoopLoomOutput::TemporalThread,
            LoopLoomOutput::LoopMarker,
        ]
    }
}

/// Recipe for the Loop Loom.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoopLoomRecipe {
    /// Output item.
    pub output: LoopLoomOutput,
    /// Loop fiber required.
    pub loop_fiber: u32,
    /// Additional materials.
    pub materials: HashMap<String, u32>,
    /// Craft time in ticks.
    pub craft_time: u32,
}

impl LoopLoomRecipe {
    /// Create a new recipe.
    #[must_use]
    pub fn new(output: LoopLoomOutput, loop_fiber: u32) -> Self {
        Self {
            output,
            loop_fiber,
            materials: HashMap::new(),
            craft_time: 40,
        }
    }

    /// Add a material requirement.
    pub fn with_material(mut self, name: &str, count: u32) -> Self {
        self.materials.insert(name.to_string(), count);
        self
    }

    /// Set custom craft time.
    pub fn with_craft_time(mut self, ticks: u32) -> Self {
        self.craft_time = ticks;
        self
    }
}

/// State of a Loop Loom crafting operation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoomCraftState {
    /// Recipe being crafted.
    pub recipe_index: usize,
    /// Ticks remaining.
    pub ticks_remaining: u32,
}

/// Loop Loom crafting station.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoopLoom {
    /// Available recipes.
    recipes: Vec<LoopLoomRecipe>,
    /// Current crafting operation.
    current_craft: Option<LoomCraftState>,
    /// Whether the loom is active.
    active: bool,
    /// Total items crafted (persists across loops).
    items_crafted: u32,
}

impl LoopLoom {
    /// Create a new Loop Loom with default recipes.
    #[must_use]
    pub fn new() -> Self {
        let mut loom = Self {
            recipes: Vec::new(),
            current_craft: None,
            active: true,
            items_crafted: 0,
        };
        loom.init_recipes();
        loom
    }

    /// Initialize default recipes.
    fn init_recipes(&mut self) {
        self.recipes.push(
            LoopLoomRecipe::new(LoopLoomOutput::ChestCapacityUpgrade, 8)
                .with_material("anchor_shell", 1)
                .with_craft_time(60),
        );
        self.recipes.push(
            LoopLoomRecipe::new(LoopLoomOutput::ChestDurabilityUpgrade, 6)
                .with_material("time_scale", 1)
                .with_craft_time(50),
        );
        self.recipes.push(
            LoopLoomRecipe::new(LoopLoomOutput::WarningSign, 3)
                .with_craft_time(20),
        );
        self.recipes.push(
            LoopLoomRecipe::new(LoopLoomOutput::TemporalThread, 5)
                .with_material("temporal_dust", 2)
                .with_craft_time(30),
        );
        self.recipes.push(
            LoopLoomRecipe::new(LoopLoomOutput::LoopMarker, 4)
                .with_material("phase_antler", 1)
                .with_craft_time(40),
        );
    }

    /// Get all recipes.
    #[must_use]
    pub fn recipes(&self) -> &[LoopLoomRecipe] {
        &self.recipes
    }

    /// Get a recipe by index.
    #[must_use]
    pub fn get_recipe(&self, index: usize) -> Option<&LoopLoomRecipe> {
        self.recipes.get(index)
    }

    /// Find recipe index by output type.
    #[must_use]
    pub fn find_recipe(&self, output: LoopLoomOutput) -> Option<usize> {
        self.recipes.iter().position(|r| r.output == output)
    }

    /// Check if currently crafting.
    #[must_use]
    pub fn is_crafting(&self) -> bool {
        self.current_craft.is_some()
    }

    /// Get current crafting state.
    #[must_use]
    pub fn current_craft(&self) -> Option<&LoomCraftState> {
        self.current_craft.as_ref()
    }

    /// Check if the loom is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Set active state.
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    /// Get total items crafted.
    #[must_use]
    pub fn items_crafted(&self) -> u32 {
        self.items_crafted
    }

    /// Check if a recipe can be crafted with given inventory.
    #[must_use]
    pub fn can_craft(&self, recipe_index: usize, inventory: &HashMap<String, u32>) -> bool {
        if self.is_crafting() || !self.active {
            return false;
        }

        let Some(recipe) = self.recipes.get(recipe_index) else {
            return false;
        };

        // Check loop fiber
        let fiber = inventory.get("loop_fiber").copied().unwrap_or(0);
        if fiber < recipe.loop_fiber {
            return false;
        }

        // Check other materials
        for (material, required) in &recipe.materials {
            let have = inventory.get(material).copied().unwrap_or(0);
            if have < *required {
                return false;
            }
        }

        true
    }

    /// Start crafting a recipe.
    ///
    /// Returns materials to consume, or None if cannot craft.
    pub fn start_craft(&mut self, recipe_index: usize) -> Option<HashMap<String, u32>> {
        let recipe = self.recipes.get(recipe_index)?;

        if self.is_crafting() || !self.active {
            return None;
        }

        let mut to_consume = HashMap::new();
        to_consume.insert("loop_fiber".to_string(), recipe.loop_fiber);
        for (material, count) in &recipe.materials {
            to_consume.insert(material.clone(), *count);
        }

        self.current_craft = Some(LoomCraftState {
            recipe_index,
            ticks_remaining: recipe.craft_time,
        });

        Some(to_consume)
    }

    /// Update the loom (call each tick).
    ///
    /// Returns the completed output if crafting finished.
    pub fn update(&mut self) -> Option<LoopLoomOutput> {
        if !self.active {
            return None;
        }

        let Some(ref mut craft) = self.current_craft else {
            return None;
        };

        craft.ticks_remaining = craft.ticks_remaining.saturating_sub(1);

        if craft.ticks_remaining == 0 {
            let recipe_index = craft.recipe_index;
            self.current_craft = None;
            self.items_crafted += 1;
            return self.recipes.get(recipe_index).map(|r| r.output);
        }

        None
    }

    /// Cancel current crafting operation.
    pub fn cancel_craft(&mut self) {
        self.current_craft = None;
    }

    /// Get crafting progress as a percentage (0.0 - 1.0).
    #[must_use]
    pub fn craft_progress(&self) -> f32 {
        let Some(ref craft) = self.current_craft else {
            return 0.0;
        };
        let Some(recipe) = self.recipes.get(craft.recipe_index) else {
            return 0.0;
        };
        if recipe.craft_time == 0 {
            return 1.0;
        }
        1.0 - (craft.ticks_remaining as f32 / recipe.craft_time as f32)
    }

    /// Reset for a new loop (keeps items_crafted).
    pub fn reset_for_loop(&mut self) {
        self.current_craft = None;
        self.active = true;
    }
}

impl Default for LoopLoom {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_inventory() -> HashMap<String, u32> {
        let mut inv = HashMap::new();
        inv.insert("loop_fiber".to_string(), 50);
        inv.insert("anchor_shell".to_string(), 10);
        inv.insert("time_scale".to_string(), 10);
        inv.insert("temporal_dust".to_string(), 20);
        inv.insert("phase_antler".to_string(), 10);
        inv
    }

    #[test]
    fn test_loop_loom_output_display_names() {
        assert_eq!(
            LoopLoomOutput::ChestCapacityUpgrade.display_name(),
            "Chest Capacity Upgrade"
        );
        assert_eq!(LoopLoomOutput::WarningSign.display_name(), "Warning Sign");
        assert_eq!(LoopLoomOutput::TemporalThread.display_name(), "Temporal Thread");
    }

    #[test]
    fn test_loop_loom_output_descriptions() {
        assert!(LoopLoomOutput::ChestCapacityUpgrade.description().contains("storage"));
        assert!(LoopLoomOutput::WarningSign.description().contains("loops"));
    }

    #[test]
    fn test_loop_loom_output_all() {
        assert_eq!(LoopLoomOutput::all().len(), 5);
    }

    #[test]
    fn test_loop_loom_new() {
        let loom = LoopLoom::new();

        assert!(!loom.is_crafting());
        assert!(loom.is_active());
        assert_eq!(loom.items_crafted(), 0);
        assert_eq!(loom.recipes().len(), 5);
    }

    #[test]
    fn test_loop_loom_recipes() {
        let loom = LoopLoom::new();

        let warning_sign = loom.find_recipe(LoopLoomOutput::WarningSign);
        assert!(warning_sign.is_some());

        let recipe = loom.get_recipe(warning_sign.unwrap()).unwrap();
        assert_eq!(recipe.output, LoopLoomOutput::WarningSign);
        assert_eq!(recipe.loop_fiber, 3);
    }

    #[test]
    fn test_loop_loom_can_craft() {
        let loom = LoopLoom::new();
        let inv = make_inventory();

        assert!(loom.can_craft(0, &inv));
        assert!(loom.can_craft(2, &inv)); // Warning sign
    }

    #[test]
    fn test_loop_loom_can_craft_inactive() {
        let mut loom = LoopLoom::new();
        loom.set_active(false);
        let inv = make_inventory();

        assert!(!loom.can_craft(0, &inv));
    }

    #[test]
    fn test_loop_loom_can_craft_insufficient_fiber() {
        let loom = LoopLoom::new();
        let mut inv = HashMap::new();
        inv.insert("loop_fiber".to_string(), 1);

        assert!(!loom.can_craft(0, &inv)); // Needs 8 fiber
    }

    #[test]
    fn test_loop_loom_start_craft() {
        let mut loom = LoopLoom::new();

        let consumed = loom.start_craft(2); // Warning sign
        assert!(consumed.is_some());
        assert!(loom.is_crafting());

        let materials = consumed.unwrap();
        assert_eq!(materials.get("loop_fiber"), Some(&3));
    }

    #[test]
    fn test_loop_loom_update_crafting() {
        let mut loom = LoopLoom::new();
        loom.start_craft(2); // Warning sign, 20 tick craft

        for _ in 0..19 {
            let result = loom.update();
            assert!(result.is_none());
        }

        let result = loom.update();
        assert_eq!(result, Some(LoopLoomOutput::WarningSign));
        assert!(!loom.is_crafting());
        assert_eq!(loom.items_crafted(), 1);
    }

    #[test]
    fn test_loop_loom_cancel_craft() {
        let mut loom = LoopLoom::new();
        loom.start_craft(0);
        assert!(loom.is_crafting());

        loom.cancel_craft();
        assert!(!loom.is_crafting());
    }

    #[test]
    fn test_loop_loom_craft_progress() {
        let mut loom = LoopLoom::new();
        loom.start_craft(2); // 20 tick craft

        for _ in 0..10 {
            loom.update();
        }

        assert!((loom.craft_progress() - 0.5).abs() < 0.1);
    }

    #[test]
    fn test_loop_loom_reset_for_loop() {
        let mut loom = LoopLoom::new();
        loom.start_craft(0);
        loom.set_active(false);

        // Complete some crafts
        for _ in 0..100 {
            loom.update();
        }

        loom.reset_for_loop();

        assert!(!loom.is_crafting());
        assert!(loom.is_active());
        // items_crafted persists
    }

    #[test]
    fn test_loop_loom_recipe_builder() {
        let recipe = LoopLoomRecipe::new(LoopLoomOutput::WarningSign, 5)
            .with_material("test", 3)
            .with_craft_time(100);

        assert_eq!(recipe.loop_fiber, 5);
        assert_eq!(recipe.materials.get("test"), Some(&3));
        assert_eq!(recipe.craft_time, 100);
    }
}
