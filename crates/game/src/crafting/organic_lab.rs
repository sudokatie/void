//! Organic Lab crafting station.
//!
//! Processes biological materials from the Titan into medicines,
//! consumables, and organic equipment.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::CraftResult;

/// Default recipes available in the Organic Lab.
pub const RECIPE_COAGULANT: &str = "coagulant";
pub const RECIPE_PARASITE_ANTIDOTE: &str = "parasite_antidote";
pub const RECIPE_ORGANIC_ARMOR: &str = "organic_armor";

/// A recipe for the Organic Lab.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrganicLabRecipe {
    /// Recipe identifier.
    pub id: String,
    /// Required materials and quantities.
    pub materials: HashMap<String, u32>,
    /// Resulting item.
    pub result: CraftResult,
    /// Incubation time in seconds.
    pub incubation_time: f32,
}

impl OrganicLabRecipe {
    /// Create a new organic lab recipe.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        materials: HashMap<String, u32>,
        result: CraftResult,
        incubation_time: f32,
    ) -> Self {
        Self {
            id: id.into(),
            materials,
            result,
            incubation_time,
        }
    }
}

/// The Organic Lab crafting station.
///
/// Specializes in biological processing, creating medicines, antidotes,
/// and organic equipment from Titan tissues and creature materials.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrganicLab {
    /// Whether the lab is operational.
    operational: bool,
    /// Available recipes.
    recipes: Vec<OrganicLabRecipe>,
    /// Current culture quality (affects success rate).
    culture_quality: f32,
}

impl OrganicLab {
    /// Create a new Organic Lab with default recipes.
    #[must_use]
    pub fn new() -> Self {
        let mut lab = Self {
            operational: true,
            recipes: Vec::new(),
            culture_quality: 1.0,
        };
        lab.load_default_recipes();
        lab
    }

    /// Create an empty lab without default recipes.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            operational: true,
            recipes: Vec::new(),
            culture_quality: 1.0,
        }
    }

    /// Load default recipes for the lab.
    fn load_default_recipes(&mut self) {
        // Coagulant - stops bleeding, accelerates healing
        let mut coagulant_mats = HashMap::new();
        coagulant_mats.insert("titan_blood".to_string(), 3);
        coagulant_mats.insert("clotting_enzyme".to_string(), 2);
        coagulant_mats.insert("binding_agent".to_string(), 1);
        self.recipes.push(OrganicLabRecipe::new(
            RECIPE_COAGULANT,
            coagulant_mats,
            CraftResult::new("coagulant", 3),
            15.0,
        ));

        // Parasite Antidote - cures parasite infections
        let mut antidote_mats = HashMap::new();
        antidote_mats.insert("neural_fluid".to_string(), 2);
        antidote_mats.insert("immune_cells".to_string(), 4);
        antidote_mats.insert("purified_extract".to_string(), 1);
        self.recipes.push(OrganicLabRecipe::new(
            RECIPE_PARASITE_ANTIDOTE,
            antidote_mats,
            CraftResult::new("parasite_antidote", 1),
            30.0,
        ));

        // Organic Armor - living armor that regenerates
        let mut armor_mats = HashMap::new();
        armor_mats.insert("living_tissue".to_string(), 8);
        armor_mats.insert("scale_membrane".to_string(), 4);
        armor_mats.insert("growth_hormone".to_string(), 2);
        armor_mats.insert("neural_thread".to_string(), 3);
        self.recipes.push(OrganicLabRecipe::new(
            RECIPE_ORGANIC_ARMOR,
            armor_mats,
            CraftResult::new("organic_armor", 1),
            60.0,
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

        let lab_recipe = self.recipes.iter().find(|r| r.id == recipe)?;

        // Check if all required materials are present
        for (mat, &required) in &lab_recipe.materials {
            let available = materials.get(mat).copied().unwrap_or(0);
            if available < required {
                return None;
            }
        }

        Some(lab_recipe.result.clone())
    }

    /// Get the incubation time for a recipe, adjusted for culture quality.
    #[must_use]
    pub fn incubation_time(&self, recipe: &str) -> Option<f32> {
        let lab_recipe = self.recipes.iter().find(|r| r.id == recipe)?;
        // Higher quality reduces incubation time (up to 50% reduction)
        let quality_modifier = 1.0 - (self.culture_quality - 1.0).clamp(0.0, 0.5);
        Some(lab_recipe.incubation_time * quality_modifier)
    }

    /// Calculate success chance for a craft based on culture quality.
    #[must_use]
    pub fn success_chance(&self) -> f32 {
        // Base 80% chance, +20% at max quality
        (0.8 + (self.culture_quality - 1.0) * 0.2).clamp(0.5, 1.0)
    }

    /// Add a custom recipe to the lab.
    pub fn add_recipe(&mut self, recipe: OrganicLabRecipe) {
        self.recipes.push(recipe);
    }

    /// Get all available recipes.
    #[must_use]
    pub fn recipes(&self) -> &[OrganicLabRecipe] {
        &self.recipes
    }

    /// Get a recipe by ID.
    #[must_use]
    pub fn get_recipe(&self, id: &str) -> Option<&OrganicLabRecipe> {
        self.recipes.iter().find(|r| r.id == id)
    }

    /// Check if the lab is operational.
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

    /// Get current culture quality.
    #[must_use]
    pub fn culture_quality(&self) -> f32 {
        self.culture_quality
    }

    /// Set the culture quality.
    pub fn set_culture_quality(&mut self, quality: f32) {
        self.culture_quality = quality.clamp(0.5, 2.0);
    }

    /// Improve culture quality (e.g., from feeding nutrients).
    pub fn improve_culture(&mut self, amount: f32) {
        self.culture_quality = (self.culture_quality + amount).clamp(0.5, 2.0);
    }

    /// Degrade culture quality (natural decay or contamination).
    pub fn degrade_culture(&mut self, amount: f32) {
        self.culture_quality = (self.culture_quality - amount).clamp(0.5, 2.0);
    }
}

impl Default for OrganicLab {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organic_lab_new() {
        let lab = OrganicLab::new();
        assert!(lab.is_operational());
        assert_eq!(lab.recipe_count(), 3);
    }

    #[test]
    fn test_organic_lab_empty() {
        let lab = OrganicLab::empty();
        assert!(lab.is_operational());
        assert_eq!(lab.recipe_count(), 0);
    }

    #[test]
    fn test_organic_lab_has_default_recipes() {
        let lab = OrganicLab::new();
        assert!(lab.get_recipe(RECIPE_COAGULANT).is_some());
        assert!(lab.get_recipe(RECIPE_PARASITE_ANTIDOTE).is_some());
        assert!(lab.get_recipe(RECIPE_ORGANIC_ARMOR).is_some());
    }

    #[test]
    fn test_organic_lab_craft_coagulant() {
        let lab = OrganicLab::new();

        let mut materials = HashMap::new();
        materials.insert("titan_blood".to_string(), 5);
        materials.insert("clotting_enzyme".to_string(), 3);
        materials.insert("binding_agent".to_string(), 2);

        let result = lab.craft(RECIPE_COAGULANT, &materials);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.item, "coagulant");
        assert_eq!(result.quantity, 3);
    }

    #[test]
    fn test_organic_lab_craft_antidote() {
        let lab = OrganicLab::new();

        let mut materials = HashMap::new();
        materials.insert("neural_fluid".to_string(), 5);
        materials.insert("immune_cells".to_string(), 5);
        materials.insert("purified_extract".to_string(), 2);

        let result = lab.craft(RECIPE_PARASITE_ANTIDOTE, &materials);
        assert!(result.is_some());
        assert_eq!(result.unwrap().item, "parasite_antidote");
    }

    #[test]
    fn test_organic_lab_craft_organic_armor() {
        let lab = OrganicLab::new();

        let mut materials = HashMap::new();
        materials.insert("living_tissue".to_string(), 10);
        materials.insert("scale_membrane".to_string(), 5);
        materials.insert("growth_hormone".to_string(), 3);
        materials.insert("neural_thread".to_string(), 5);

        let result = lab.craft(RECIPE_ORGANIC_ARMOR, &materials);
        assert!(result.is_some());
        assert_eq!(result.unwrap().item, "organic_armor");
    }

    #[test]
    fn test_organic_lab_craft_insufficient_materials() {
        let lab = OrganicLab::new();

        let mut materials = HashMap::new();
        materials.insert("titan_blood".to_string(), 1);

        let result = lab.craft(RECIPE_COAGULANT, &materials);
        assert!(result.is_none());
    }

    #[test]
    fn test_organic_lab_craft_unknown_recipe() {
        let lab = OrganicLab::new();
        let materials = HashMap::new();

        let result = lab.craft("unknown", &materials);
        assert!(result.is_none());
    }

    #[test]
    fn test_organic_lab_craft_not_operational() {
        let mut lab = OrganicLab::new();
        lab.set_operational(false);

        let mut materials = HashMap::new();
        materials.insert("titan_blood".to_string(), 5);
        materials.insert("clotting_enzyme".to_string(), 5);
        materials.insert("binding_agent".to_string(), 5);

        let result = lab.craft(RECIPE_COAGULANT, &materials);
        assert!(result.is_none());
    }

    #[test]
    fn test_organic_lab_incubation_time_base() {
        let lab = OrganicLab::new();
        let time = lab.incubation_time(RECIPE_COAGULANT);
        assert!(time.is_some());
        // At quality 1.0, modifier is 1.0, so time = 15
        assert!((time.unwrap() - 15.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_organic_lab_incubation_time_high_quality() {
        let mut lab = OrganicLab::new();
        lab.set_culture_quality(1.5);
        let time = lab.incubation_time(RECIPE_COAGULANT);
        assert!(time.is_some());
        // At quality 1.5, modifier is 0.5, so time = 15 * 0.5 = 7.5
        assert!((time.unwrap() - 7.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_organic_lab_incubation_time_unknown() {
        let lab = OrganicLab::new();
        let time = lab.incubation_time("unknown");
        assert!(time.is_none());
    }

    #[test]
    fn test_organic_lab_success_chance_base() {
        let lab = OrganicLab::new();
        let chance = lab.success_chance();
        assert!((chance - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn test_organic_lab_success_chance_high_quality() {
        let mut lab = OrganicLab::new();
        lab.set_culture_quality(2.0);
        let chance = lab.success_chance();
        assert!((chance - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_organic_lab_success_chance_low_quality() {
        let mut lab = OrganicLab::new();
        lab.set_culture_quality(0.5);
        let chance = lab.success_chance();
        // 0.8 + (0.5 - 1.0) * 0.2 = 0.8 - 0.1 = 0.7, clamped to 0.5 minimum
        assert!((chance - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn test_organic_lab_add_recipe() {
        let mut lab = OrganicLab::empty();

        let mut mats = HashMap::new();
        mats.insert("test_mat".to_string(), 1);
        let recipe = OrganicLabRecipe::new("test", mats, CraftResult::new("test_item", 1), 5.0);

        lab.add_recipe(recipe);
        assert_eq!(lab.recipe_count(), 1);
        assert!(lab.get_recipe("test").is_some());
    }

    #[test]
    fn test_organic_lab_set_operational() {
        let mut lab = OrganicLab::new();
        assert!(lab.is_operational());

        lab.set_operational(false);
        assert!(!lab.is_operational());

        lab.set_operational(true);
        assert!(lab.is_operational());
    }

    #[test]
    fn test_organic_lab_culture_quality() {
        let mut lab = OrganicLab::new();
        assert!((lab.culture_quality() - 1.0).abs() < f32::EPSILON);

        lab.set_culture_quality(1.5);
        assert!((lab.culture_quality() - 1.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_organic_lab_culture_quality_clamp() {
        let mut lab = OrganicLab::new();

        lab.set_culture_quality(0.0);
        assert!((lab.culture_quality() - 0.5).abs() < f32::EPSILON);

        lab.set_culture_quality(5.0);
        assert!((lab.culture_quality() - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_organic_lab_improve_culture() {
        let mut lab = OrganicLab::new();
        lab.improve_culture(0.3);
        assert!((lab.culture_quality() - 1.3).abs() < f32::EPSILON);
    }

    #[test]
    fn test_organic_lab_degrade_culture() {
        let mut lab = OrganicLab::new();
        lab.degrade_culture(0.3);
        assert!((lab.culture_quality() - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn test_organic_lab_recipe_struct() {
        let mut mats = HashMap::new();
        mats.insert("material".to_string(), 5);

        let recipe = OrganicLabRecipe::new("test_id", mats, CraftResult::new("result", 2), 8.0);

        assert_eq!(recipe.id, "test_id");
        assert_eq!(recipe.materials.get("material"), Some(&5));
        assert_eq!(recipe.result.item, "result");
        assert_eq!(recipe.result.quantity, 2);
        assert!((recipe.incubation_time - 8.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_organic_lab_default() {
        let lab = OrganicLab::default();
        assert!(lab.is_operational());
        assert_eq!(lab.recipe_count(), 3);
    }

    #[test]
    fn test_organic_lab_recipes_accessor() {
        let lab = OrganicLab::new();
        let recipes = lab.recipes();
        assert_eq!(recipes.len(), 3);
    }
}
