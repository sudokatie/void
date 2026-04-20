//! Reality Forge crafting station.
//!
//! Used to craft phase suits, stability detectors, and void tethers
//! from dimensional materials.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Result of a crafting operation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CraftResult {
    /// Name of the crafted item.
    pub item: String,
    /// Quantity produced.
    pub quantity: u32,
}

impl CraftResult {
    /// Create a new craft result.
    #[must_use]
    pub fn new(item: impl Into<String>, quantity: u32) -> Self {
        Self {
            item: item.into(),
            quantity,
        }
    }
}

/// A recipe for the reality forge.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ForgeRecipe {
    /// Recipe identifier.
    pub id: String,
    /// Required materials and quantities.
    pub materials: HashMap<String, u32>,
    /// Resulting item.
    pub result: CraftResult,
}

impl ForgeRecipe {
    /// Create a new forge recipe.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        materials: HashMap<String, u32>,
        result: CraftResult,
    ) -> Self {
        Self {
            id: id.into(),
            materials,
            result,
        }
    }
}

/// The Reality Forge crafting station.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RealityForge {
    /// Whether the forge is operational.
    operational: bool,
    /// Available recipes.
    recipes: Vec<ForgeRecipe>,
}

impl RealityForge {
    /// Create a new reality forge with default recipes.
    #[must_use]
    pub fn new() -> Self {
        let mut forge = Self {
            operational: true,
            recipes: Vec::new(),
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
        }
    }

    /// Load default recipes for the forge.
    fn load_default_recipes(&mut self) {
        // Basic Phase Suit
        let mut basic_suit_mats = HashMap::new();
        basic_suit_mats.insert("void_fabric".to_string(), 4);
        basic_suit_mats.insert("stability_crystal".to_string(), 2);
        self.recipes.push(ForgeRecipe::new(
            "basic_phase_suit",
            basic_suit_mats,
            CraftResult::new("basic_phase_suit", 1),
        ));

        // Standard Phase Suit
        let mut standard_suit_mats = HashMap::new();
        standard_suit_mats.insert("void_fabric".to_string(), 8);
        standard_suit_mats.insert("stability_crystal".to_string(), 4);
        standard_suit_mats.insert("nexus_essence".to_string(), 2);
        self.recipes.push(ForgeRecipe::new(
            "standard_phase_suit",
            standard_suit_mats,
            CraftResult::new("standard_phase_suit", 1),
        ));

        // Military Phase Suit
        let mut military_suit_mats = HashMap::new();
        military_suit_mats.insert("void_fabric".to_string(), 12);
        military_suit_mats.insert("stability_crystal".to_string(), 8);
        military_suit_mats.insert("nexus_essence".to_string(), 4);
        military_suit_mats.insert("dimensional_alloy".to_string(), 2);
        self.recipes.push(ForgeRecipe::new(
            "military_phase_suit",
            military_suit_mats,
            CraftResult::new("military_phase_suit", 1),
        ));

        // Stability Detector
        let mut detector_mats = HashMap::new();
        detector_mats.insert("stability_crystal".to_string(), 3);
        detector_mats.insert("copper_wire".to_string(), 5);
        detector_mats.insert("void_glass".to_string(), 1);
        self.recipes.push(ForgeRecipe::new(
            "stability_detector",
            detector_mats,
            CraftResult::new("stability_detector", 1),
        ));

        // Void Tether
        let mut tether_mats = HashMap::new();
        tether_mats.insert("void_thread".to_string(), 10);
        tether_mats.insert("anchor_fragment".to_string(), 2);
        tether_mats.insert("stability_crystal".to_string(), 1);
        self.recipes.push(ForgeRecipe::new(
            "void_tether",
            tether_mats,
            CraftResult::new("void_tether", 1),
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

    /// Add a custom recipe to the forge.
    pub fn add_recipe(&mut self, recipe: ForgeRecipe) {
        self.recipes.push(recipe);
    }

    /// Get all available recipes.
    #[must_use]
    pub fn recipes(&self) -> &[ForgeRecipe] {
        &self.recipes
    }

    /// Get a recipe by ID.
    #[must_use]
    pub fn get_recipe(&self, id: &str) -> Option<&ForgeRecipe> {
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
}

impl Default for RealityForge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_craft_result_new() {
        let result = CraftResult::new("test_item", 5);
        assert_eq!(result.item, "test_item");
        assert_eq!(result.quantity, 5);
    }

    #[test]
    fn test_reality_forge_new() {
        let forge = RealityForge::new();
        assert!(forge.is_operational());
        assert!(forge.recipe_count() > 0);
    }

    #[test]
    fn test_reality_forge_empty() {
        let forge = RealityForge::empty();
        assert!(forge.is_operational());
        assert_eq!(forge.recipe_count(), 0);
    }

    #[test]
    fn test_reality_forge_has_default_recipes() {
        let forge = RealityForge::new();

        assert!(forge.get_recipe("basic_phase_suit").is_some());
        assert!(forge.get_recipe("stability_detector").is_some());
        assert!(forge.get_recipe("void_tether").is_some());
    }

    #[test]
    fn test_reality_forge_craft_success() {
        let forge = RealityForge::new();

        let mut materials = HashMap::new();
        materials.insert("void_fabric".to_string(), 10);
        materials.insert("stability_crystal".to_string(), 5);

        let result = forge.craft("basic_phase_suit", &materials);
        assert!(result.is_some());
        assert_eq!(result.unwrap().item, "basic_phase_suit");
    }

    #[test]
    fn test_reality_forge_craft_insufficient_materials() {
        let forge = RealityForge::new();

        let mut materials = HashMap::new();
        materials.insert("void_fabric".to_string(), 1); // Need 4

        let result = forge.craft("basic_phase_suit", &materials);
        assert!(result.is_none());
    }

    #[test]
    fn test_reality_forge_craft_missing_material() {
        let forge = RealityForge::new();

        let mut materials = HashMap::new();
        materials.insert("void_fabric".to_string(), 10);
        // Missing stability_crystal

        let result = forge.craft("basic_phase_suit", &materials);
        assert!(result.is_none());
    }

    #[test]
    fn test_reality_forge_craft_unknown_recipe() {
        let forge = RealityForge::new();
        let materials = HashMap::new();

        let result = forge.craft("unknown_recipe", &materials);
        assert!(result.is_none());
    }

    #[test]
    fn test_reality_forge_craft_not_operational() {
        let mut forge = RealityForge::new();
        forge.set_operational(false);

        let mut materials = HashMap::new();
        materials.insert("void_fabric".to_string(), 10);
        materials.insert("stability_crystal".to_string(), 5);

        let result = forge.craft("basic_phase_suit", &materials);
        assert!(result.is_none());
    }

    #[test]
    fn test_reality_forge_add_recipe() {
        let mut forge = RealityForge::empty();

        let mut mats = HashMap::new();
        mats.insert("test_mat".to_string(), 1);
        let recipe = ForgeRecipe::new("test_recipe", mats, CraftResult::new("test_item", 1));

        forge.add_recipe(recipe);
        assert_eq!(forge.recipe_count(), 1);
        assert!(forge.get_recipe("test_recipe").is_some());
    }

    #[test]
    fn test_reality_forge_set_operational() {
        let mut forge = RealityForge::new();
        assert!(forge.is_operational());

        forge.set_operational(false);
        assert!(!forge.is_operational());

        forge.set_operational(true);
        assert!(forge.is_operational());
    }
}
