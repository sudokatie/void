//! Resource types and harvesting results.
//!
//! Defines the various resources that can be harvested from the Titan.

use std::fmt;

use serde::{Deserialize, Serialize};

use super::tools::HarvestingToolProperties;

/// Types of resources that can be harvested.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceType {
    /// Iron extracted from the shell.
    ShellIron,
    /// Crystal formations.
    Crystal,
    /// Chitin from the shell.
    ShellChitin,
    /// Organs harvested from parasites.
    ParasiteOrgan,
    /// Steam from thermal vents.
    VentSteam,
    /// Fluid from neural tissue.
    NeuralFluid,
    /// Tissue from wounds.
    WoundTissue,
    /// Blood from the Titan.
    TitanBlood,
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResourceType::ShellIron => write!(f, "Shell Iron"),
            ResourceType::Crystal => write!(f, "Crystal"),
            ResourceType::ShellChitin => write!(f, "Shell Chitin"),
            ResourceType::ParasiteOrgan => write!(f, "Parasite Organ"),
            ResourceType::VentSteam => write!(f, "Vent Steam"),
            ResourceType::NeuralFluid => write!(f, "Neural Fluid"),
            ResourceType::WoundTissue => write!(f, "Wound Tissue"),
            ResourceType::TitanBlood => write!(f, "Titan Blood"),
        }
    }
}

impl ResourceType {
    /// Get the base harvest rate for this resource.
    #[must_use]
    fn base_harvest_rate(self) -> f32 {
        match self {
            ResourceType::ShellIron => 1.0,
            ResourceType::Crystal => 0.5,
            ResourceType::ShellChitin => 1.5,
            ResourceType::ParasiteOrgan => 1.0,
            ResourceType::VentSteam => 2.0,
            ResourceType::NeuralFluid => 0.3,
            ResourceType::WoundTissue => 1.2,
            ResourceType::TitanBlood => 0.8,
        }
    }
}

/// A resource stack.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Resource {
    /// Type of resource.
    pub resource_type: ResourceType,
    /// Quantity of the resource.
    pub quantity: u32,
}

impl Resource {
    /// Create a new resource stack.
    #[must_use]
    pub fn new(resource_type: ResourceType, quantity: u32) -> Self {
        Self {
            resource_type,
            quantity,
        }
    }

    /// Calculate the effective harvest rate with a tool.
    #[must_use]
    pub fn harvest_rate(&self, tool: &HarvestingToolProperties) -> f32 {
        self.resource_type.base_harvest_rate() * tool.harvest_rate
    }
}

/// Result of a harvesting action.
#[derive(Clone, Debug)]
pub struct HarvestResult {
    /// Type of resource harvested.
    pub resource_type: ResourceType,
    /// Quantity harvested.
    pub quantity: u32,
    /// Change in Titan agitation.
    pub agitation_change: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harvesting::tools::HarvestingTool;

    #[test]
    fn test_resource_type_display() {
        assert_eq!(format!("{}", ResourceType::ShellIron), "Shell Iron");
        assert_eq!(format!("{}", ResourceType::Crystal), "Crystal");
        assert_eq!(format!("{}", ResourceType::NeuralFluid), "Neural Fluid");
        assert_eq!(format!("{}", ResourceType::TitanBlood), "Titan Blood");
    }

    #[test]
    fn test_resource_new() {
        let resource = Resource::new(ResourceType::ShellChitin, 10);
        assert_eq!(resource.resource_type, ResourceType::ShellChitin);
        assert_eq!(resource.quantity, 10);
    }

    #[test]
    fn test_resource_harvest_rate() {
        let resource = Resource::new(ResourceType::VentSteam, 1);
        let tool = HarvestingToolProperties::new(HarvestingTool::VentTap);
        // VentSteam base: 2.0, VentTap rate: 2.0 = 4.0
        let rate = resource.harvest_rate(&tool);
        assert!((rate - 4.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_harvest_result_fields() {
        let result = HarvestResult {
            resource_type: ResourceType::ParasiteOrgan,
            quantity: 5,
            agitation_change: -5.0,
        };
        assert_eq!(result.resource_type, ResourceType::ParasiteOrgan);
        assert_eq!(result.quantity, 5);
        assert!((result.agitation_change - -5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_resource_serialization() {
        let resource = Resource::new(ResourceType::WoundTissue, 25);
        let json = serde_json::to_string(&resource).unwrap();
        let deserialized: Resource = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.resource_type, ResourceType::WoundTissue);
        assert_eq!(deserialized.quantity, 25);
    }
}
