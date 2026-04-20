//! Memory Forge crafting station for time-loop survival.
//!
//! Crafts temporal equipment from temporal_dust and other materials.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::equipment::TemporalGear;

/// Recipe for the Memory Forge.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryForgeRecipe {
    /// Output gear type.
    pub output: TemporalGear,
    /// Temporal dust required.
    pub temporal_dust: u32,
    /// Additional materials (item name -> count).
    pub materials: HashMap<String, u32>,
    /// Crafting time in ticks.
    pub craft_time: u32,
}

impl MemoryForgeRecipe {
    /// Create a new recipe.
    #[must_use]
    pub fn new(output: TemporalGear, temporal_dust: u32) -> Self {
        Self {
            output,
            temporal_dust,
            materials: HashMap::new(),
            craft_time: 60,
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

/// State of a crafting operation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CraftingState {
    /// Recipe being crafted.
    pub recipe_index: usize,
    /// Ticks remaining.
    pub ticks_remaining: u32,
}

/// Memory Forge crafting station.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryForge {
    /// Available recipes.
    recipes: Vec<MemoryForgeRecipe>,
    /// Current crafting operation.
    current_craft: Option<CraftingState>,
    /// Whether the forge is powered.
    powered: bool,
    /// Accumulated temporal energy.
    energy: u32,
    /// Maximum energy capacity.
    max_energy: u32,
}

impl MemoryForge {
    /// Create a new Memory Forge with default recipes.
    #[must_use]
    pub fn new() -> Self {
        let mut forge = Self {
            recipes: Vec::new(),
            current_craft: None,
            powered: false,
            energy: 0,
            max_energy: 100,
        };
        forge.init_recipes();
        forge
    }

    /// Initialize default recipes.
    fn init_recipes(&mut self) {
        self.recipes.push(
            MemoryForgeRecipe::new(TemporalGear::LoopWatch, 10)
                .with_material("anchor_shell", 1)
                .with_craft_time(60),
        );
        self.recipes.push(
            MemoryForgeRecipe::new(TemporalGear::MemoryLens, 15)
                .with_material("phase_antler", 1)
                .with_craft_time(90),
        );
        self.recipes.push(
            MemoryForgeRecipe::new(TemporalGear::ParadoxScanner, 20)
                .with_material("time_scale", 2)
                .with_craft_time(120),
        );
        self.recipes.push(
            MemoryForgeRecipe::new(TemporalGear::TemporalAnchor, 25)
                .with_material("anchor_shell", 3)
                .with_craft_time(150),
        );
        self.recipes.push(
            MemoryForgeRecipe::new(TemporalGear::ChronoBoots, 30)
                .with_material("loop_fiber", 5)
                .with_material("phase_antler", 2)
                .with_craft_time(180),
        );
    }

    /// Get all available recipes.
    #[must_use]
    pub fn recipes(&self) -> &[MemoryForgeRecipe] {
        &self.recipes
    }

    /// Get a recipe by index.
    #[must_use]
    pub fn get_recipe(&self, index: usize) -> Option<&MemoryForgeRecipe> {
        self.recipes.get(index)
    }

    /// Find recipe index by output gear type.
    #[must_use]
    pub fn find_recipe(&self, gear: TemporalGear) -> Option<usize> {
        self.recipes.iter().position(|r| r.output == gear)
    }

    /// Check if currently crafting.
    #[must_use]
    pub fn is_crafting(&self) -> bool {
        self.current_craft.is_some()
    }

    /// Get current crafting state.
    #[must_use]
    pub fn current_craft(&self) -> Option<&CraftingState> {
        self.current_craft.as_ref()
    }

    /// Check if the forge is powered.
    #[must_use]
    pub fn is_powered(&self) -> bool {
        self.powered
    }

    /// Set powered state.
    pub fn set_powered(&mut self, powered: bool) {
        self.powered = powered;
    }

    /// Get current energy level.
    #[must_use]
    pub fn energy(&self) -> u32 {
        self.energy
    }

    /// Get maximum energy capacity.
    #[must_use]
    pub fn max_energy(&self) -> u32 {
        self.max_energy
    }

    /// Add energy to the forge.
    pub fn add_energy(&mut self, amount: u32) {
        self.energy = (self.energy + amount).min(self.max_energy);
        if self.energy > 0 {
            self.powered = true;
        }
    }

    /// Check if a recipe can be crafted with given inventory.
    #[must_use]
    pub fn can_craft(&self, recipe_index: usize, inventory: &HashMap<String, u32>) -> bool {
        if self.is_crafting() || !self.powered {
            return false;
        }

        let Some(recipe) = self.recipes.get(recipe_index) else {
            return false;
        };

        // Check temporal dust
        let dust = inventory.get("temporal_dust").copied().unwrap_or(0);
        if dust < recipe.temporal_dust {
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

        if self.is_crafting() || !self.powered {
            return None;
        }

        let mut to_consume = HashMap::new();
        to_consume.insert("temporal_dust".to_string(), recipe.temporal_dust);
        for (material, count) in &recipe.materials {
            to_consume.insert(material.clone(), *count);
        }

        self.current_craft = Some(CraftingState {
            recipe_index,
            ticks_remaining: recipe.craft_time,
        });

        Some(to_consume)
    }

    /// Update the forge (call each tick).
    ///
    /// Returns the completed gear if crafting finished.
    pub fn update(&mut self) -> Option<TemporalGear> {
        if !self.powered {
            return None;
        }

        // Consume energy
        if self.energy > 0 {
            self.energy = self.energy.saturating_sub(1);
        }
        if self.energy == 0 {
            self.powered = false;
        }

        let Some(ref mut craft) = self.current_craft else {
            return None;
        };

        craft.ticks_remaining = craft.ticks_remaining.saturating_sub(1);

        if craft.ticks_remaining == 0 {
            let recipe_index = craft.recipe_index;
            self.current_craft = None;
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
}

impl Default for MemoryForge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_inventory() -> HashMap<String, u32> {
        let mut inv = HashMap::new();
        inv.insert("temporal_dust".to_string(), 100);
        inv.insert("anchor_shell".to_string(), 10);
        inv.insert("phase_antler".to_string(), 10);
        inv.insert("time_scale".to_string(), 10);
        inv.insert("loop_fiber".to_string(), 10);
        inv
    }

    #[test]
    fn test_memory_forge_new() {
        let forge = MemoryForge::new();

        assert!(!forge.is_crafting());
        assert!(!forge.is_powered());
        assert_eq!(forge.energy(), 0);
        assert_eq!(forge.recipes().len(), 5);
    }

    #[test]
    fn test_memory_forge_recipes() {
        let forge = MemoryForge::new();

        let loop_watch = forge.find_recipe(TemporalGear::LoopWatch);
        assert!(loop_watch.is_some());

        let recipe = forge.get_recipe(loop_watch.unwrap()).unwrap();
        assert_eq!(recipe.output, TemporalGear::LoopWatch);
        assert_eq!(recipe.temporal_dust, 10);
    }

    #[test]
    fn test_memory_forge_power() {
        let mut forge = MemoryForge::new();
        assert!(!forge.is_powered());

        forge.add_energy(50);
        assert!(forge.is_powered());
        assert_eq!(forge.energy(), 50);

        forge.add_energy(100);
        assert_eq!(forge.energy(), 100); // Capped at max
    }

    #[test]
    fn test_memory_forge_can_craft_not_powered() {
        let forge = MemoryForge::new();
        let inv = make_inventory();

        assert!(!forge.can_craft(0, &inv));
    }

    #[test]
    fn test_memory_forge_can_craft_success() {
        let mut forge = MemoryForge::new();
        forge.set_powered(true);
        let inv = make_inventory();

        assert!(forge.can_craft(0, &inv));
    }

    #[test]
    fn test_memory_forge_can_craft_insufficient_dust() {
        let mut forge = MemoryForge::new();
        forge.set_powered(true);
        let mut inv = HashMap::new();
        inv.insert("temporal_dust".to_string(), 5);
        inv.insert("anchor_shell".to_string(), 10);

        assert!(!forge.can_craft(0, &inv));
    }

    #[test]
    fn test_memory_forge_can_craft_missing_material() {
        let mut forge = MemoryForge::new();
        forge.set_powered(true);
        let mut inv = HashMap::new();
        inv.insert("temporal_dust".to_string(), 100);
        // Missing anchor_shell

        assert!(!forge.can_craft(0, &inv));
    }

    #[test]
    fn test_memory_forge_start_craft() {
        let mut forge = MemoryForge::new();
        forge.set_powered(true);

        let consumed = forge.start_craft(0);
        assert!(consumed.is_some());
        assert!(forge.is_crafting());

        let materials = consumed.unwrap();
        assert_eq!(materials.get("temporal_dust"), Some(&10));
        assert_eq!(materials.get("anchor_shell"), Some(&1));
    }

    #[test]
    fn test_memory_forge_start_craft_not_powered() {
        let mut forge = MemoryForge::new();

        let consumed = forge.start_craft(0);
        assert!(consumed.is_none());
        assert!(!forge.is_crafting());
    }

    #[test]
    fn test_memory_forge_start_craft_already_crafting() {
        let mut forge = MemoryForge::new();
        forge.set_powered(true);

        forge.start_craft(0);
        let second = forge.start_craft(1);
        assert!(second.is_none());
    }

    #[test]
    fn test_memory_forge_update_crafting() {
        let mut forge = MemoryForge::new();
        forge.add_energy(1000);
        forge.start_craft(0);

        // Craft time for LoopWatch is 60 ticks
        for _ in 0..59 {
            let result = forge.update();
            assert!(result.is_none());
        }

        let result = forge.update();
        assert_eq!(result, Some(TemporalGear::LoopWatch));
        assert!(!forge.is_crafting());
    }

    #[test]
    fn test_memory_forge_cancel_craft() {
        let mut forge = MemoryForge::new();
        forge.set_powered(true);
        forge.start_craft(0);
        assert!(forge.is_crafting());

        forge.cancel_craft();
        assert!(!forge.is_crafting());
    }

    #[test]
    fn test_memory_forge_craft_progress() {
        let mut forge = MemoryForge::new();
        forge.add_energy(1000);

        assert!((forge.craft_progress() - 0.0).abs() < f32::EPSILON);

        forge.start_craft(0); // 60 tick craft
        assert!((forge.craft_progress() - 0.0).abs() < f32::EPSILON);

        for _ in 0..30 {
            forge.update();
        }
        assert!((forge.craft_progress() - 0.5).abs() < 0.02);

        for _ in 0..30 {
            forge.update();
        }
        assert!((forge.craft_progress() - 0.0).abs() < f32::EPSILON); // Completed, reset
    }

    #[test]
    fn test_memory_forge_energy_consumption() {
        let mut forge = MemoryForge::new();
        forge.add_energy(5);
        assert!(forge.is_powered());

        for _ in 0..5 {
            forge.update();
        }

        assert_eq!(forge.energy(), 0);
        assert!(!forge.is_powered());
    }

    #[test]
    fn test_memory_forge_recipe_with_material() {
        let recipe = MemoryForgeRecipe::new(TemporalGear::LoopWatch, 10)
            .with_material("test_item", 5)
            .with_craft_time(100);

        assert_eq!(recipe.materials.get("test_item"), Some(&5));
        assert_eq!(recipe.craft_time, 100);
    }
}
