//! Transmutation Crafting Table.
//!
//! A crafting station that wraps the TransmutationTable from survival,
//! providing an operational toggle and enhanced crafting interface.

use engine_physics::dimension::Dimension;
use serde::{Deserialize, Serialize};

use crate::survival::{TransmutationRule, TransmutationTable};

/// A crafting station for transmutation operations.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransmutationCraftingTable {
    /// The underlying transmutation table.
    table: TransmutationTable,
    /// Whether the crafting table is operational.
    operational: bool,
}

impl TransmutationCraftingTable {
    /// Create a new transmutation crafting table with default rules.
    #[must_use]
    pub fn new() -> Self {
        Self {
            table: TransmutationTable::new(),
            operational: true,
        }
    }

    /// Create an empty transmutation crafting table.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            table: TransmutationTable::empty(),
            operational: true,
        }
    }

    /// Create from an existing transmutation table.
    #[must_use]
    pub fn from_table(table: TransmutationTable) -> Self {
        Self {
            table,
            operational: true,
        }
    }

    /// Attempt to transmute an item at this crafting table.
    ///
    /// Returns the resulting item name if a rule matches and the table is operational.
    #[must_use]
    pub fn transmute(&self, item: &str, from_dim: Dimension, to_dim: Dimension) -> Option<String> {
        if !self.operational {
            return None;
        }
        self.table.transmute(item, from_dim, to_dim)
    }

    /// Add a transmutation rule to the table.
    pub fn add_rule(&mut self, rule: TransmutationRule) {
        self.table.add_rule(rule);
    }

    /// Get all transmutation rules.
    #[must_use]
    pub fn list_rules(&self) -> &[TransmutationRule] {
        self.table.list_rules()
    }

    /// Get the number of rules.
    #[must_use]
    pub fn rule_count(&self) -> usize {
        self.table.rule_count()
    }

    /// Check if the crafting table is operational.
    #[must_use]
    pub fn is_operational(&self) -> bool {
        self.operational
    }

    /// Set the operational state.
    pub fn set_operational(&mut self, operational: bool) {
        self.operational = operational;
    }

    /// Get a reference to the underlying transmutation table.
    #[must_use]
    pub fn inner(&self) -> &TransmutationTable {
        &self.table
    }

    /// Get a mutable reference to the underlying transmutation table.
    pub fn inner_mut(&mut self) -> &mut TransmutationTable {
        &mut self.table
    }
}

impl Default for TransmutationCraftingTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transmutation_crafting_table_new() {
        let table = TransmutationCraftingTable::new();
        assert!(table.is_operational());
        assert!(table.rule_count() > 0);
    }

    #[test]
    fn test_transmutation_crafting_table_empty() {
        let table = TransmutationCraftingTable::empty();
        assert!(table.is_operational());
        assert_eq!(table.rule_count(), 0);
    }

    #[test]
    fn test_transmutation_crafting_table_transmute() {
        let table = TransmutationCraftingTable::new();

        let result = table.transmute("water", Dimension::Prime, Dimension::Void);
        assert_eq!(result, Some("nothing".to_string()));
    }

    #[test]
    fn test_transmutation_crafting_table_transmute_not_operational() {
        let mut table = TransmutationCraftingTable::new();
        table.set_operational(false);

        let result = table.transmute("water", Dimension::Prime, Dimension::Void);
        assert!(result.is_none());
    }

    #[test]
    fn test_transmutation_crafting_table_add_rule() {
        let mut table = TransmutationCraftingTable::empty();

        table.add_rule(TransmutationRule::new(
            "test_item",
            Dimension::Prime,
            "test_result",
            Dimension::Void,
        ));

        assert_eq!(table.rule_count(), 1);
        let result = table.transmute("test_item", Dimension::Prime, Dimension::Void);
        assert_eq!(result, Some("test_result".to_string()));
    }

    #[test]
    fn test_transmutation_crafting_table_set_operational() {
        let mut table = TransmutationCraftingTable::new();
        assert!(table.is_operational());

        table.set_operational(false);
        assert!(!table.is_operational());

        table.set_operational(true);
        assert!(table.is_operational());
    }

    #[test]
    fn test_transmutation_crafting_table_from_existing() {
        let mut inner = TransmutationTable::empty();
        inner.add_rule(TransmutationRule::new(
            "custom",
            Dimension::Prime,
            "custom_result",
            Dimension::Nexus,
        ));

        let table = TransmutationCraftingTable::from_table(inner);
        assert_eq!(table.rule_count(), 1);
    }

    #[test]
    fn test_transmutation_crafting_table_inner_access() {
        let table = TransmutationCraftingTable::new();
        let inner = table.inner();
        assert!(inner.rule_count() > 0);
    }
}
