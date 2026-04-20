//! Resource transmutation system.
//!
//! Items can transform when crossing dimension boundaries,
//! following specific transmutation rules.

use engine_physics::dimension::Dimension;
use serde::{Deserialize, Serialize};

/// A rule defining how an item transmutes between dimensions.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransmutationRule {
    /// Source item name.
    source_item: String,
    /// Dimension the item is coming from.
    source_dimension: Dimension,
    /// Resulting item name.
    target_item: String,
    /// Dimension the item is going to.
    target_dimension: Dimension,
}

impl TransmutationRule {
    /// Create a new transmutation rule.
    #[must_use]
    pub fn new(
        source: impl Into<String>,
        source_dim: Dimension,
        target: impl Into<String>,
        target_dim: Dimension,
    ) -> Self {
        Self {
            source_item: source.into(),
            source_dimension: source_dim,
            target_item: target.into(),
            target_dimension: target_dim,
        }
    }

    /// Get the source item name.
    #[must_use]
    pub fn source_item(&self) -> &str {
        &self.source_item
    }

    /// Get the source dimension.
    #[must_use]
    pub fn source_dimension(&self) -> Dimension {
        self.source_dimension
    }

    /// Get the target item name.
    #[must_use]
    pub fn target_item(&self) -> &str {
        &self.target_item
    }

    /// Get the target dimension.
    #[must_use]
    pub fn target_dimension(&self) -> Dimension {
        self.target_dimension
    }

    /// Check if this rule applies to the given transformation.
    fn applies(&self, item: &str, from_dim: Dimension, to_dim: Dimension) -> bool {
        self.source_item == item
            && self.source_dimension == from_dim
            && self.target_dimension == to_dim
    }
}

/// Table of transmutation rules for dimension crossings.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TransmutationTable {
    /// All registered transmutation rules.
    rules: Vec<TransmutationRule>,
}

impl TransmutationTable {
    /// Create a new transmutation table with default rules.
    #[must_use]
    pub fn new() -> Self {
        let mut table = Self { rules: Vec::new() };
        table.load_default_rules();
        table
    }

    /// Create an empty transmutation table without default rules.
    #[must_use]
    pub fn empty() -> Self {
        Self { rules: Vec::new() }
    }

    /// Load the default transmutation rules.
    fn load_default_rules(&mut self) {
        // Water evaporates in the Void
        self.add_rule(TransmutationRule::new(
            "water",
            Dimension::Prime,
            "nothing",
            Dimension::Void,
        ));

        // Lava becomes obsidian in Prime
        self.add_rule(TransmutationRule::new(
            "lava",
            Dimension::Inverted,
            "obsidian",
            Dimension::Prime,
        ));

        // Wood catches fire in Inverted
        self.add_rule(TransmutationRule::new(
            "wood",
            Dimension::Prime,
            "charred_wood",
            Dimension::Inverted,
        ));

        // Crystals decay in Prime
        self.add_rule(TransmutationRule::new(
            "crystal",
            Dimension::Inverted,
            "decaying_crystal",
            Dimension::Prime,
        ));

        // Nexus essence becomes stability dust in Prime (from any dimension)
        for dim in Dimension::all() {
            if *dim != Dimension::Prime {
                self.add_rule(TransmutationRule::new(
                    "nexus_essence",
                    *dim,
                    "stability_dust",
                    Dimension::Prime,
                ));
            }
        }

        // Soil corrupts in Inverted
        self.add_rule(TransmutationRule::new(
            "soil",
            Dimension::Prime,
            "corrupted_soil",
            Dimension::Inverted,
        ));

        // Ice becomes steam in Inverted (from any dimension)
        for dim in Dimension::all() {
            if *dim != Dimension::Inverted {
                self.add_rule(TransmutationRule::new(
                    "ice",
                    *dim,
                    "steam",
                    Dimension::Inverted,
                ));
            }
        }

        // Iron becomes void-touched in the Void (from any dimension)
        for dim in Dimension::all() {
            if *dim != Dimension::Void {
                self.add_rule(TransmutationRule::new(
                    "iron",
                    *dim,
                    "void_touched_iron",
                    Dimension::Void,
                ));
            }
        }
    }

    /// Add a transmutation rule.
    pub fn add_rule(&mut self, rule: TransmutationRule) {
        self.rules.push(rule);
    }

    /// Attempt to transmute an item.
    ///
    /// Returns the resulting item name if a rule matches, or None if no transmutation occurs.
    #[must_use]
    pub fn transmute(&self, item: &str, from_dim: Dimension, to_dim: Dimension) -> Option<String> {
        // No transmutation if staying in same dimension
        if from_dim == to_dim {
            return None;
        }

        self.rules
            .iter()
            .find(|rule| rule.applies(item, from_dim, to_dim))
            .map(|rule| rule.target_item.clone())
    }

    /// Get all transmutation rules.
    #[must_use]
    pub fn list_rules(&self) -> &[TransmutationRule] {
        &self.rules
    }

    /// Get the number of rules.
    #[must_use]
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_new() {
        let rule = TransmutationRule::new("water", Dimension::Prime, "nothing", Dimension::Void);
        assert_eq!(rule.source_item(), "water");
        assert_eq!(rule.source_dimension(), Dimension::Prime);
        assert_eq!(rule.target_item(), "nothing");
        assert_eq!(rule.target_dimension(), Dimension::Void);
    }

    #[test]
    fn test_table_new_has_defaults() {
        let table = TransmutationTable::new();
        assert!(table.rule_count() > 0);
    }

    #[test]
    fn test_table_empty() {
        let table = TransmutationTable::empty();
        assert_eq!(table.rule_count(), 0);
    }

    #[test]
    fn test_transmute_water_to_void() {
        let table = TransmutationTable::new();
        let result = table.transmute("water", Dimension::Prime, Dimension::Void);
        assert_eq!(result, Some("nothing".to_string()));
    }

    #[test]
    fn test_transmute_lava_to_prime() {
        let table = TransmutationTable::new();
        let result = table.transmute("lava", Dimension::Inverted, Dimension::Prime);
        assert_eq!(result, Some("obsidian".to_string()));
    }

    #[test]
    fn test_transmute_wood_to_inverted() {
        let table = TransmutationTable::new();
        let result = table.transmute("wood", Dimension::Prime, Dimension::Inverted);
        assert_eq!(result, Some("charred_wood".to_string()));
    }

    #[test]
    fn test_transmute_no_match() {
        let table = TransmutationTable::new();
        let result = table.transmute("stone", Dimension::Prime, Dimension::Void);
        assert_eq!(result, None);
    }

    #[test]
    fn test_transmute_same_dimension() {
        let table = TransmutationTable::new();
        let result = table.transmute("water", Dimension::Prime, Dimension::Prime);
        assert_eq!(result, None);
    }

    #[test]
    fn test_transmute_iron_to_void() {
        let table = TransmutationTable::new();
        let result = table.transmute("iron", Dimension::Prime, Dimension::Void);
        assert_eq!(result, Some("void_touched_iron".to_string()));
    }

    #[test]
    fn test_custom_rule() {
        let mut table = TransmutationTable::empty();
        table.add_rule(TransmutationRule::new(
            "diamond",
            Dimension::Prime,
            "void_diamond",
            Dimension::Void,
        ));

        let result = table.transmute("diamond", Dimension::Prime, Dimension::Void);
        assert_eq!(result, Some("void_diamond".to_string()));
    }
}
