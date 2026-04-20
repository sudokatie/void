//! Anchor Workshop crafting station.
//!
//! Used to build and repair dimensional anchors from materials.

use std::collections::HashMap;

use glam::IVec3;
use serde::{Deserialize, Serialize};

use crate::dimension::{AnchorTier, DimensionalAnchor};

/// Material costs for building each anchor tier.
pub fn anchor_build_costs(tier: AnchorTier) -> HashMap<String, u32> {
    let mut costs = HashMap::new();
    match tier {
        AnchorTier::Basic => {
            costs.insert("stability_crystal".to_string(), 4);
            costs.insert("iron_ingot".to_string(), 8);
            costs.insert("void_dust".to_string(), 2);
        }
        AnchorTier::Standard => {
            costs.insert("stability_crystal".to_string(), 8);
            costs.insert("iron_ingot".to_string(), 16);
            costs.insert("void_dust".to_string(), 4);
            costs.insert("nexus_shard".to_string(), 2);
        }
        AnchorTier::Military => {
            costs.insert("stability_crystal".to_string(), 16);
            costs.insert("dimensional_alloy".to_string(), 8);
            costs.insert("void_dust".to_string(), 8);
            costs.insert("nexus_shard".to_string(), 4);
            costs.insert("nexus_core".to_string(), 1);
        }
    }
    costs
}

/// An anchor workshop for building and repairing dimensional anchors.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnchorWorkshop {
    /// Whether the workshop is operational.
    operational: bool,
}

impl AnchorWorkshop {
    /// Create a new anchor workshop.
    #[must_use]
    pub fn new() -> Self {
        Self { operational: true }
    }

    /// Attempt to build an anchor of the given tier.
    ///
    /// Returns the created anchor if materials are sufficient, None otherwise.
    #[must_use]
    pub fn build_anchor(
        &self,
        tier: &str,
        materials: &HashMap<String, u32>,
    ) -> Option<DimensionalAnchor> {
        if !self.operational {
            return None;
        }

        let anchor_tier = match tier.to_lowercase().as_str() {
            "basic" => AnchorTier::Basic,
            "standard" => AnchorTier::Standard,
            "military" => AnchorTier::Military,
            _ => return None,
        };

        let required = anchor_build_costs(anchor_tier);

        // Check if all required materials are present
        for (mat, &required_amount) in &required {
            let available = materials.get(mat).copied().unwrap_or(0);
            if available < required_amount {
                return None;
            }
        }

        // Build at origin - position should be set when placed
        Some(DimensionalAnchor::new(IVec3::ZERO, anchor_tier))
    }

    /// Attempt to build an anchor at a specific position.
    #[must_use]
    pub fn build_anchor_at(
        &self,
        tier: &str,
        pos: IVec3,
        materials: &HashMap<String, u32>,
    ) -> Option<DimensionalAnchor> {
        if !self.operational {
            return None;
        }

        let anchor_tier = match tier.to_lowercase().as_str() {
            "basic" => AnchorTier::Basic,
            "standard" => AnchorTier::Standard,
            "military" => AnchorTier::Military,
            _ => return None,
        };

        let required = anchor_build_costs(anchor_tier);

        // Check if all required materials are present
        for (mat, &required_amount) in &required {
            let available = materials.get(mat).copied().unwrap_or(0);
            if available < required_amount {
                return None;
            }
        }

        Some(DimensionalAnchor::new(pos, anchor_tier))
    }

    /// Repair an anchor, restoring fuel.
    ///
    /// Returns the amount of fuel added.
    pub fn repair_anchor(&self, anchor: &mut DimensionalAnchor, amount: f32) -> f32 {
        if !self.operational {
            return 0.0;
        }

        anchor.refuel(amount)
    }

    /// Get the material costs for building an anchor tier.
    #[must_use]
    pub fn get_build_costs(&self, tier: &str) -> Option<HashMap<String, u32>> {
        let anchor_tier = match tier.to_lowercase().as_str() {
            "basic" => AnchorTier::Basic,
            "standard" => AnchorTier::Standard,
            "military" => AnchorTier::Military,
            _ => return None,
        };

        Some(anchor_build_costs(anchor_tier))
    }

    /// Check if the workshop is operational.
    #[must_use]
    pub fn is_operational(&self) -> bool {
        self.operational
    }

    /// Set the operational state.
    pub fn set_operational(&mut self, operational: bool) {
        self.operational = operational;
    }

    /// Check if materials are sufficient for building an anchor tier.
    #[must_use]
    pub fn can_build(&self, tier: &str, materials: &HashMap<String, u32>) -> bool {
        if !self.operational {
            return false;
        }

        let Some(required) = self.get_build_costs(tier) else {
            return false;
        };

        for (mat, required_amount) in &required {
            let available = materials.get(mat).copied().unwrap_or(0);
            if available < *required_amount {
                return false;
            }
        }

        true
    }
}

impl Default for AnchorWorkshop {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anchor_build_costs() {
        let basic = anchor_build_costs(AnchorTier::Basic);
        assert!(basic.contains_key("stability_crystal"));
        assert!(basic.contains_key("iron_ingot"));

        let military = anchor_build_costs(AnchorTier::Military);
        assert!(military.contains_key("nexus_core"));
    }

    #[test]
    fn test_anchor_workshop_new() {
        let workshop = AnchorWorkshop::new();
        assert!(workshop.is_operational());
    }

    #[test]
    fn test_anchor_workshop_build_basic() {
        let workshop = AnchorWorkshop::new();

        let mut materials = HashMap::new();
        materials.insert("stability_crystal".to_string(), 10);
        materials.insert("iron_ingot".to_string(), 20);
        materials.insert("void_dust".to_string(), 5);

        let anchor = workshop.build_anchor("basic", &materials);
        assert!(anchor.is_some());
        assert_eq!(anchor.unwrap().tier(), AnchorTier::Basic);
    }

    #[test]
    fn test_anchor_workshop_build_standard() {
        let workshop = AnchorWorkshop::new();

        let mut materials = HashMap::new();
        materials.insert("stability_crystal".to_string(), 10);
        materials.insert("iron_ingot".to_string(), 20);
        materials.insert("void_dust".to_string(), 5);
        materials.insert("nexus_shard".to_string(), 5);

        let anchor = workshop.build_anchor("standard", &materials);
        assert!(anchor.is_some());
        assert_eq!(anchor.unwrap().tier(), AnchorTier::Standard);
    }

    #[test]
    fn test_anchor_workshop_build_military() {
        let workshop = AnchorWorkshop::new();

        let mut materials = HashMap::new();
        materials.insert("stability_crystal".to_string(), 20);
        materials.insert("dimensional_alloy".to_string(), 10);
        materials.insert("void_dust".to_string(), 10);
        materials.insert("nexus_shard".to_string(), 5);
        materials.insert("nexus_core".to_string(), 2);

        let anchor = workshop.build_anchor("military", &materials);
        assert!(anchor.is_some());
        assert_eq!(anchor.unwrap().tier(), AnchorTier::Military);
    }

    #[test]
    fn test_anchor_workshop_build_at_position() {
        let workshop = AnchorWorkshop::new();
        let pos = IVec3::new(10, 20, 30);

        let mut materials = HashMap::new();
        materials.insert("stability_crystal".to_string(), 10);
        materials.insert("iron_ingot".to_string(), 20);
        materials.insert("void_dust".to_string(), 5);

        let anchor = workshop.build_anchor_at("basic", pos, &materials);
        assert!(anchor.is_some());
        assert_eq!(anchor.unwrap().position(), pos);
    }

    #[test]
    fn test_anchor_workshop_build_insufficient_materials() {
        let workshop = AnchorWorkshop::new();

        let mut materials = HashMap::new();
        materials.insert("stability_crystal".to_string(), 1); // Need 4

        let anchor = workshop.build_anchor("basic", &materials);
        assert!(anchor.is_none());
    }

    #[test]
    fn test_anchor_workshop_build_invalid_tier() {
        let workshop = AnchorWorkshop::new();
        let materials = HashMap::new();

        let anchor = workshop.build_anchor("invalid", &materials);
        assert!(anchor.is_none());
    }

    #[test]
    fn test_anchor_workshop_build_not_operational() {
        let mut workshop = AnchorWorkshop::new();
        workshop.set_operational(false);

        let mut materials = HashMap::new();
        materials.insert("stability_crystal".to_string(), 10);
        materials.insert("iron_ingot".to_string(), 20);
        materials.insert("void_dust".to_string(), 5);

        let anchor = workshop.build_anchor("basic", &materials);
        assert!(anchor.is_none());
    }

    #[test]
    fn test_anchor_workshop_repair_anchor() {
        let workshop = AnchorWorkshop::new();
        let mut anchor = DimensionalAnchor::new(IVec3::ZERO, AnchorTier::Basic);

        // Use some fuel
        anchor.tick(100.0);
        let initial_fuel = anchor.fuel_remaining();

        let repaired = workshop.repair_anchor(&mut anchor, 50.0);
        assert!((repaired - 50.0).abs() < f32::EPSILON);
        assert!(anchor.fuel_remaining() > initial_fuel);
    }

    #[test]
    fn test_anchor_workshop_repair_not_operational() {
        let mut workshop = AnchorWorkshop::new();
        workshop.set_operational(false);

        let mut anchor = DimensionalAnchor::new(IVec3::ZERO, AnchorTier::Basic);
        anchor.tick(100.0);

        let repaired = workshop.repair_anchor(&mut anchor, 50.0);
        assert!((repaired - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_anchor_workshop_get_build_costs() {
        let workshop = AnchorWorkshop::new();

        let costs = workshop.get_build_costs("basic");
        assert!(costs.is_some());

        let costs = workshop.get_build_costs("invalid");
        assert!(costs.is_none());
    }

    #[test]
    fn test_anchor_workshop_can_build() {
        let workshop = AnchorWorkshop::new();

        let mut materials = HashMap::new();
        assert!(!workshop.can_build("basic", &materials));

        materials.insert("stability_crystal".to_string(), 10);
        materials.insert("iron_ingot".to_string(), 20);
        materials.insert("void_dust".to_string(), 5);
        assert!(workshop.can_build("basic", &materials));
    }

    #[test]
    fn test_anchor_workshop_set_operational() {
        let mut workshop = AnchorWorkshop::new();
        assert!(workshop.is_operational());

        workshop.set_operational(false);
        assert!(!workshop.is_operational());
    }

    #[test]
    fn test_anchor_workshop_case_insensitive() {
        let workshop = AnchorWorkshop::new();

        let mut materials = HashMap::new();
        materials.insert("stability_crystal".to_string(), 10);
        materials.insert("iron_ingot".to_string(), 20);
        materials.insert("void_dust".to_string(), 5);

        assert!(workshop.build_anchor("BASIC", &materials).is_some());
        assert!(workshop.build_anchor("Basic", &materials).is_some());
        assert!(workshop.build_anchor("basic", &materials).is_some());
    }
}
