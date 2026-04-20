//! Crafting system with recipes and execution.
//!
//! Includes standard crafting as well as temporal crafting stations:
//! Memory Forge, Loop Loom, Paradox Engine, Time Altar, and legacy stations.

mod anchor_workshop;
mod executor;
mod furnace;
mod loop_loom;
mod memory_forge;
mod neural_interface;
mod organic_lab;
mod paradox_engine;
mod reality_forge;
mod registry;
mod shell_forge;
mod stability_infuser;
mod thermal_converter;
mod time_altar;
mod transmutation_table;

pub use anchor_workshop::{anchor_build_costs, AnchorWorkshop};
pub use executor::{check_craft, execute_craft, execute_craft_by_id, CraftError, CraftRequirements};
pub use furnace::{
    Furnace, FurnaceState, FuelEntry, DEFAULT_SMELT_TIME, FUEL_CHARCOAL, FUEL_COAL,
    FUEL_LAVA_BUCKET, FUEL_STICK, FUEL_WOOD,
};
pub use loop_loom::{LoopLoom, LoopLoomOutput, LoopLoomRecipe};
pub use memory_forge::{MemoryForge, MemoryForgeRecipe};
pub use neural_interface::{
    NeuralInterface, NeuralResult, GUIDANCE_COOLDOWN, GUIDANCE_COST, MOOD_COOLDOWN,
    MOOD_INFLUENCE_COST,
};
pub use organic_lab::{
    OrganicLab, OrganicLabRecipe, RECIPE_COAGULANT, RECIPE_ORGANIC_ARMOR, RECIPE_PARASITE_ANTIDOTE,
};
pub use paradox_engine::{ParadoxEngine, ParadoxPowerOutput};
pub use reality_forge::{CraftResult, ForgeRecipe, RealityForge};
pub use registry::{CraftingStation, Ingredient, Recipe, RecipeRegistry};
pub use shell_forge::{
    ShellForge, ShellForgeRecipe, RECIPE_CRYSTAL_LENS, RECIPE_SCALE_PLATING, RECIPE_SHELL_INGOT,
};
pub use stability_infuser::{StabilityInfuser, ENERGY_PER_LEVEL, MIN_INFUSION_ENERGY};
pub use thermal_converter::{
    PowerOutput, ThermalConverter, BASE_POWER_OUTPUT, MAX_EFFICIENCY, MIN_EFFICIENCY,
};
pub use time_altar::{StabilizationResult, TemporalKey, TimeAltar};
pub use transmutation_table::TransmutationCraftingTable;
