//! Harvesting tools and their properties.
//!
//! Different tools for extracting resources from the Titan.

use std::fmt;

use crate::titan::TitanZone;

/// Types of harvesting tools.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HarvestingTool {
    /// Scrapes scales from the Titan's surface.
    ScaleScraper,
    /// Lance for killing parasites.
    ParasiteLance,
    /// Tap for collecting vent steam.
    VentTap,
    /// Probe for extracting neural fluid.
    NeuralProbe,
    /// Bandage for collecting wound tissue.
    WoundBandage,
}

impl fmt::Display for HarvestingTool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HarvestingTool::ScaleScraper => write!(f, "Scale Scraper"),
            HarvestingTool::ParasiteLance => write!(f, "Parasite Lance"),
            HarvestingTool::VentTap => write!(f, "Vent Tap"),
            HarvestingTool::NeuralProbe => write!(f, "Neural Probe"),
            HarvestingTool::WoundBandage => write!(f, "Wound Bandage"),
        }
    }
}

impl HarvestingTool {
    /// Get the effectiveness multiplier for this tool in a specific zone.
    ///
    /// Each tool has 2x effectiveness in its specialized zone:
    /// - ScaleScraper: 2x in ShellRidge
    /// - ParasiteLance: 2x in ParasiteForest
    /// - VentTap: 2x in BreathingVent
    /// - NeuralProbe: 2x in NeuralNode
    /// - WoundBandage: 2x in WoundSite
    #[must_use]
    pub fn tool_effectiveness(&self, zone: TitanZone) -> f32 {
        match (self, zone) {
            (HarvestingTool::ScaleScraper, TitanZone::ShellRidge) => 2.0,
            (HarvestingTool::ParasiteLance, TitanZone::ParasiteForest) => 2.0,
            (HarvestingTool::VentTap, TitanZone::BreathingVent) => 2.0,
            (HarvestingTool::NeuralProbe, TitanZone::NeuralNode) => 2.0,
            (HarvestingTool::WoundBandage, TitanZone::WoundSite) => 2.0,
            _ => 1.0,
        }
    }

    /// Get the optimal zone for this tool.
    #[must_use]
    pub fn optimal_zone(&self) -> TitanZone {
        match self {
            HarvestingTool::ScaleScraper => TitanZone::ShellRidge,
            HarvestingTool::ParasiteLance => TitanZone::ParasiteForest,
            HarvestingTool::VentTap => TitanZone::BreathingVent,
            HarvestingTool::NeuralProbe => TitanZone::NeuralNode,
            HarvestingTool::WoundBandage => TitanZone::WoundSite,
        }
    }
}

/// Properties of a harvesting tool.
#[derive(Clone, Debug)]
pub struct HarvestingToolProperties {
    /// The tool type.
    pub tool: HarvestingTool,
    /// Rate at which resources are harvested.
    pub harvest_rate: f32,
    /// Current durability.
    pub durability: f32,
    /// Maximum durability.
    max_durability: f32,
    /// Effect on Titan agitation when used.
    pub agitation_effect: f32,
}

impl HarvestingToolProperties {
    /// Create tool properties for a given tool type.
    #[must_use]
    pub fn new(tool: HarvestingTool) -> Self {
        let (harvest_rate, durability, agitation_effect) = match tool {
            HarvestingTool::ScaleScraper => (1.0, 100.0, 0.0),
            HarvestingTool::ParasiteLance => (1.5, 80.0, -5.0),
            HarvestingTool::VentTap => (2.0, 60.0, 0.0),
            HarvestingTool::NeuralProbe => (0.5, 50.0, 15.0),
            HarvestingTool::WoundBandage => (1.0, 120.0, 5.0),
        };
        Self {
            tool,
            harvest_rate,
            durability,
            max_durability: durability,
            agitation_effect,
        }
    }

    /// Use the tool, consuming durability.
    ///
    /// Returns `false` if the tool is broken.
    pub fn use_tool(&mut self) -> bool {
        if self.durability <= 0.0 {
            return false;
        }
        self.durability = (self.durability - 1.0).max(0.0);
        true
    }

    /// Check if the tool is broken.
    #[must_use]
    pub fn is_broken(&self) -> bool {
        self.durability <= 0.0
    }

    /// Get durability as a percentage.
    #[must_use]
    pub fn durability_percent(&self) -> f32 {
        self.durability / self.max_durability
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harvesting_tool_display() {
        assert_eq!(format!("{}", HarvestingTool::ScaleScraper), "Scale Scraper");
        assert_eq!(format!("{}", HarvestingTool::ParasiteLance), "Parasite Lance");
        assert_eq!(format!("{}", HarvestingTool::VentTap), "Vent Tap");
        assert_eq!(format!("{}", HarvestingTool::NeuralProbe), "Neural Probe");
        assert_eq!(format!("{}", HarvestingTool::WoundBandage), "Wound Bandage");
    }

    #[test]
    fn test_tool_properties_scale_scraper() {
        let props = HarvestingToolProperties::new(HarvestingTool::ScaleScraper);
        assert!((props.harvest_rate - 1.0).abs() < f32::EPSILON);
        assert!((props.durability - 100.0).abs() < f32::EPSILON);
        assert!((props.agitation_effect - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_tool_properties_parasite_lance() {
        let props = HarvestingToolProperties::new(HarvestingTool::ParasiteLance);
        assert!((props.harvest_rate - 1.5).abs() < f32::EPSILON);
        assert!((props.durability - 80.0).abs() < f32::EPSILON);
        assert!((props.agitation_effect - -5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_tool_properties_neural_probe() {
        let props = HarvestingToolProperties::new(HarvestingTool::NeuralProbe);
        assert!((props.harvest_rate - 0.5).abs() < f32::EPSILON);
        assert!((props.durability - 50.0).abs() < f32::EPSILON);
        assert!((props.agitation_effect - 15.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_tool_use_success() {
        let mut props = HarvestingToolProperties::new(HarvestingTool::VentTap);
        assert!(props.use_tool());
        assert!((props.durability - 59.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_tool_use_broken() {
        let mut props = HarvestingToolProperties::new(HarvestingTool::VentTap);
        props.durability = 0.0;
        assert!(!props.use_tool());
    }

    #[test]
    fn test_tool_is_broken() {
        let mut props = HarvestingToolProperties::new(HarvestingTool::WoundBandage);
        assert!(!props.is_broken());
        props.durability = 0.0;
        assert!(props.is_broken());
    }

    #[test]
    fn test_tool_effectiveness_scale_scraper() {
        assert!(
            (HarvestingTool::ScaleScraper.tool_effectiveness(TitanZone::ShellRidge) - 2.0).abs()
                < f32::EPSILON
        );
        assert!(
            (HarvestingTool::ScaleScraper.tool_effectiveness(TitanZone::WoundSite) - 1.0).abs()
                < f32::EPSILON
        );
    }

    #[test]
    fn test_tool_effectiveness_parasite_lance() {
        assert!(
            (HarvestingTool::ParasiteLance.tool_effectiveness(TitanZone::ParasiteForest) - 2.0)
                .abs()
                < f32::EPSILON
        );
        assert!(
            (HarvestingTool::ParasiteLance.tool_effectiveness(TitanZone::ShellRidge) - 1.0).abs()
                < f32::EPSILON
        );
    }

    #[test]
    fn test_tool_effectiveness_vent_tap() {
        assert!(
            (HarvestingTool::VentTap.tool_effectiveness(TitanZone::BreathingVent) - 2.0).abs()
                < f32::EPSILON
        );
        assert!(
            (HarvestingTool::VentTap.tool_effectiveness(TitanZone::NeuralNode) - 1.0).abs()
                < f32::EPSILON
        );
    }

    #[test]
    fn test_tool_effectiveness_neural_probe() {
        assert!(
            (HarvestingTool::NeuralProbe.tool_effectiveness(TitanZone::NeuralNode) - 2.0).abs()
                < f32::EPSILON
        );
        assert!(
            (HarvestingTool::NeuralProbe.tool_effectiveness(TitanZone::BreathingVent) - 1.0).abs()
                < f32::EPSILON
        );
    }

    #[test]
    fn test_tool_effectiveness_wound_bandage() {
        assert!(
            (HarvestingTool::WoundBandage.tool_effectiveness(TitanZone::WoundSite) - 2.0).abs()
                < f32::EPSILON
        );
        assert!(
            (HarvestingTool::WoundBandage.tool_effectiveness(TitanZone::ScaleValley) - 1.0).abs()
                < f32::EPSILON
        );
    }

    #[test]
    fn test_tool_optimal_zones() {
        assert_eq!(
            HarvestingTool::ScaleScraper.optimal_zone(),
            TitanZone::ShellRidge
        );
        assert_eq!(
            HarvestingTool::ParasiteLance.optimal_zone(),
            TitanZone::ParasiteForest
        );
        assert_eq!(
            HarvestingTool::VentTap.optimal_zone(),
            TitanZone::BreathingVent
        );
        assert_eq!(
            HarvestingTool::NeuralProbe.optimal_zone(),
            TitanZone::NeuralNode
        );
        assert_eq!(
            HarvestingTool::WoundBandage.optimal_zone(),
            TitanZone::WoundSite
        );
    }
}
