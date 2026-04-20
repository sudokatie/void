//! Shell Forge crafting station.
//!
//! Processes Titan shell materials into refined components for armor,
//! tools, and structural elements.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::CraftResult;

/// Default recipes available in the Shell Forge.
pub const RECIPE_SHELL_INGOT: &str = "shell_ingot";
pub const RECIPE_SCALE_PLATING: &str = "scale_plating";
pub const RECIPE_CRYSTAL_LENS: &str = "crystal_lens";

/// A recipe for the Shell Forge.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShellForgeRecipe {
    /// Recipe identifier.
    pub id: String,
    /// Required materials and quantities.
    pub materials: HashMap<String, u32>,
    /// Resulting item.
    pub result: CraftResult,
    /// Time to craft in seconds.
    pub craft_time: f32,
}

impl ShellForgeRecipe {
    /// Create a new shell forge recipe.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        materials: HashMap<String, u32>,
        result: CraftResult,
        craft_time: f32,
    ) -> Self {
        Self {
            id: id.into(),
            materials,
            result,
            craft_time,
        }
    }
}

/// The Shell Forge crafting station.
///
/// Specializes in processing raw Titan shell materials into refined
/// components used for armor, tools, and building materials.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShellForge {
    /// Whether the forge is operational.
    operational: bool,
    /// Available recipes.
    recipes: Vec<ShellForgeRecipe>,
    /// Current temperature (affects craft speed).
    temperature: f32,
}

impl ShellForge {
    /// Create a new Shell Forge with default recipes.
    #[must_use]
    pub fn new() -> Self {
        let mut forge = Self {
            operational: true,
            recipes: Vec::new(),
            temperature: 100.0,
        };
        forge.load_default_recipes();
        forge
    }

    /// Create an empty forge without default recipes.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            operational: true,
            recipes: Vec::new(),
            temperature: 100.0,
        }
    }

    /// Load default recipes for the forge.
    fn load_default_recipes(&mut self) {
        // Shell Ingot - basic refined shell material
        let mut shell_ingot_mats = HashMap::new();
        shell_ingot_mats.insert("raw_shell".to_string(), 4);
        shell_ingot_mats.insert("titan_calcium".to_string(), 2);
        self.recipes.push(ShellForgeRecipe::new(
            RECIPE_SHELL_INGOT,
            shell_ingot_mats,
            CraftResult::new("shell_ingot", 1),
            10.0,
        ));

        // Scale Plating - armor component
        let mut scale_plating_mats = HashMap::new();
        scale_plating_mats.insert("shell_ingot".to_string(), 3);
        scale_plating_mats.insert("scale_fragment".to_string(), 6);
        scale_plating_mats.insert("binding_fiber".to_string(), 2);
        self.recipes.push(ShellForgeRecipe::new(
            RECIPE_SCALE_PLATING,
            scale_plating_mats,
            CraftResult::new("scale_plating", 1),
            20.0,
        ));

        // Crystal Lens - optical component from neural crystals
        let mut crystal_lens_mats = HashMap::new();
        crystal_lens_mats.insert("neural_crystal".to_string(), 2);
        crystal_lens_mats.insert("refined_resin".to_string(), 1);
        crystal_lens_mats.insert("shell_dust".to_string(), 4);
        self.recipes.push(ShellForgeRecipe::new(
            RECIPE_CRYSTAL_LENS,
            crystal_lens_mats,
            CraftResult::new("crystal_lens", 1),
            15.0,
        ));
    }

    /// Attempt to craft an item.
    ///
    /// Returns the crafted item if successful, None if recipe not found
    /// or materials insufficient.
    #[must_use]
    pub fn craft(&self, recipe: &str, materials: &HashMap<String, u32>) -> Option<CraftResult> {
        if !self.operational {
            return None;
        }

        let forge_recipe = self.recipes.iter().find(|r| r.id == recipe)?;

        // Check if all required materials are present
        for (mat, &required) in &forge_recipe.materials {
            let available = materials.get(mat).copied().unwrap_or(0);
            if available < required {
                return None;
            }
        }

        Some(forge_recipe.result.clone())
    }

    /// Get the craft time for a recipe, adjusted for temperature.
    #[must_use]
    pub fn craft_time(&self, recipe: &str) -> Option<f32> {
        let forge_recipe = self.recipes.iter().find(|r| r.id == recipe)?;
        // Higher temperature reduces craft time (up to 50% reduction)
        let temp_modifier = 1.0 - (self.temperature / 200.0).min(0.5);
        Some(forge_recipe.craft_time * temp_modifier)
    }

    /// Add a custom recipe to the forge.
    pub fn add_recipe(&mut self, recipe: ShellForgeRecipe) {
        self.recipes.push(recipe);
    }

    /// Get all available recipes.
    #[must_use]
    pub fn recipes(&self) -> &[ShellForgeRecipe] {
        &self.recipes
    }

    /// Get a recipe by ID.
    #[must_use]
    pub fn get_recipe(&self, id: &str) -> Option<&ShellForgeRecipe> {
        self.recipes.iter().find(|r| r.id == id)
    }

    /// Check if the forge is operational.
    #[must_use]
    pub fn is_operational(&self) -> bool {
        self.operational
    }

    /// Set the operational state.
    pub fn set_operational(&mut self, operational: bool) {
        self.operational = operational;
    }

    /// Get the number of recipes.
    #[must_use]
    pub fn recipe_count(&self) -> usize {
        self.recipes.len()
    }

    /// Get current temperature.
    #[must_use]
    pub fn temperature(&self) -> f32 {
        self.temperature
    }

    /// Set the temperature.
    pub fn set_temperature(&mut self, temp: f32) {
        self.temperature = temp.max(0.0);
    }
}

impl Default for ShellForge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_forge_new() {
        let forge = ShellForge::new();
        assert!(forge.is_operational());
        assert_eq!(forge.recipe_count(), 3);
    }

    #[test]
    fn test_shell_forge_empty() {
        let forge = ShellForge::empty();
        assert!(forge.is_operational());
        assert_eq!(forge.recipe_count(), 0);
    }

    #[test]
    fn test_shell_forge_has_default_recipes() {
        let forge = ShellForge::new();
        assert!(forge.get_recipe(RECIPE_SHELL_INGOT).is_some());
        assert!(forge.get_recipe(RECIPE_SCALE_PLATING).is_some());
        assert!(forge.get_recipe(RECIPE_CRYSTAL_LENS).is_some());
    }

    #[test]
    fn test_shell_forge_craft_shell_ingot() {
        let forge = ShellForge::new();

        let mut materials = HashMap::new();
        materials.insert("raw_shell".to_string(), 10);
        materials.insert("titan_calcium".to_string(), 5);

        let result = forge.craft(RECIPE_SHELL_INGOT, &materials);
        assert!(result.is_some());
        assert_eq!(result.unwrap().item, "shell_ingot");
    }

    #[test]
    fn test_shell_forge_craft_scale_plating() {
        let forge = ShellForge::new();

        let mut materials = HashMap::new();
        materials.insert("shell_ingot".to_string(), 5);
        materials.insert("scale_fragment".to_string(), 10);
        materials.insert("binding_fiber".to_string(), 5);

        let result = forge.craft(RECIPE_SCALE_PLATING, &materials);
        assert!(result.is_some());
        assert_eq!(result.unwrap().item, "scale_plating");
    }

    #[test]
    fn test_shell_forge_craft_crystal_lens() {
        let forge = ShellForge::new();

        let mut materials = HashMap::new();
        materials.insert("neural_crystal".to_string(), 5);
        materials.insert("refined_resin".to_string(), 2);
        materials.insert("shell_dust".to_string(), 10);

        let result = forge.craft(RECIPE_CRYSTAL_LENS, &materials);
        assert!(result.is_some());
        assert_eq!(result.unwrap().item, "crystal_lens");
    }

    #[test]
    fn test_shell_forge_craft_insufficient_materials() {
        let forge = ShellForge::new();

        let mut materials = HashMap::new();
        materials.insert("raw_shell".to_string(), 1);

        let result = forge.craft(RECIPE_SHELL_INGOT, &materials);
        assert!(result.is_none());
    }

    #[test]
    fn test_shell_forge_craft_unknown_recipe() {
        let forge = ShellForge::new();
        let materials = HashMap::new();

        let result = forge.craft("unknown", &materials);
        assert!(result.is_none());
    }

    #[test]
    fn test_shell_forge_craft_not_operational() {
        let mut forge = ShellForge::new();
        forge.set_operational(false);

        let mut materials = HashMap::new();
        materials.insert("raw_shell".to_string(), 10);
        materials.insert("titan_calcium".to_string(), 5);

        let result = forge.craft(RECIPE_SHELL_INGOT, &materials);
        assert!(result.is_none());
    }

    #[test]
    fn test_shell_forge_craft_time_base() {
        let forge = ShellForge::new();
        let time = forge.craft_time(RECIPE_SHELL_INGOT);
        assert!(time.is_some());
        // At temp 100, modifier is 0.5, so time = 10 * 0.5 = 5
        assert!((time.unwrap() - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_shell_forge_craft_time_high_temp() {
        let mut forge = ShellForge::new();
        forge.set_temperature(200.0);
        let time = forge.craft_time(RECIPE_SHELL_INGOT);
        assert!(time.is_some());
        // At temp 200, modifier is 0.5, so time = 10 * 0.5 = 5
        assert!((time.unwrap() - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_shell_forge_craft_time_unknown() {
        let forge = ShellForge::new();
        let time = forge.craft_time("unknown");
        assert!(time.is_none());
    }

    #[test]
    fn test_shell_forge_add_recipe() {
        let mut forge = ShellForge::empty();

        let mut mats = HashMap::new();
        mats.insert("test_mat".to_string(), 1);
        let recipe = ShellForgeRecipe::new("test", mats, CraftResult::new("test_item", 1), 5.0);

        forge.add_recipe(recipe);
        assert_eq!(forge.recipe_count(), 1);
        assert!(forge.get_recipe("test").is_some());
    }

    #[test]
    fn test_shell_forge_set_operational() {
        let mut forge = ShellForge::new();
        assert!(forge.is_operational());

        forge.set_operational(false);
        assert!(!forge.is_operational());

        forge.set_operational(true);
        assert!(forge.is_operational());
    }

    #[test]
    fn test_shell_forge_temperature() {
        let mut forge = ShellForge::new();
        assert!((forge.temperature() - 100.0).abs() < f32::EPSILON);

        forge.set_temperature(150.0);
        assert!((forge.temperature() - 150.0).abs() < f32::EPSILON);

        // Cannot go below 0
        forge.set_temperature(-50.0);
        assert!((forge.temperature() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_shell_forge_recipe_struct() {
        let mut mats = HashMap::new();
        mats.insert("material".to_string(), 5);

        let recipe = ShellForgeRecipe::new("test_id", mats, CraftResult::new("result", 2), 8.0);

        assert_eq!(recipe.id, "test_id");
        assert_eq!(recipe.materials.get("material"), Some(&5));
        assert_eq!(recipe.result.item, "result");
        assert_eq!(recipe.result.quantity, 2);
        assert!((recipe.craft_time - 8.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_shell_forge_default() {
        let forge = ShellForge::default();
        assert!(forge.is_operational());
        assert_eq!(forge.recipe_count(), 3);
    }

    #[test]
    fn test_shell_forge_recipes_accessor() {
        let forge = ShellForge::new();
        let recipes = forge.recipes();
        assert_eq!(recipes.len(), 3);
    }
}
