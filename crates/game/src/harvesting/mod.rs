//! Resource harvesting systems.
//!
//! Tools and resources for harvesting from the Titan.

mod resources;
mod tools;

pub use resources::{HarvestResult, Resource, ResourceType};
pub use tools::{HarvestingTool, HarvestingToolProperties};
