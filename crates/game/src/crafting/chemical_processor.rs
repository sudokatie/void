//! Chemical processor for creating compounds.
//!
//! Converts raw compounds into O2, coolant, sealant, and other substances.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Raw compounds used as input for chemical processing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChemicalCompound {
    /// Water for electrolysis.
    Water,
    /// Carbon dioxide for processing.
    CarbonDioxide,
    /// Raw minerals for extraction.
    RawMinerals,
    /// Organic matter for bio-processing.
    OrganicMatter,
    /// Chemical catalyst.
    Catalyst,
    /// Liquid nitrogen.
    LiquidNitrogen,
}

impl ChemicalCompound {
    /// Get display name.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            ChemicalCompound::Water => "Water",
            ChemicalCompound::CarbonDioxide => "Carbon Dioxide",
            ChemicalCompound::RawMinerals => "Raw Minerals",
            ChemicalCompound::OrganicMatter => "Organic Matter",
            ChemicalCompound::Catalyst => "Catalyst",
            ChemicalCompound::LiquidNitrogen => "Liquid Nitrogen",
        }
    }

    /// Get all compound types.
    #[must_use]
    pub fn all() -> &'static [ChemicalCompound] {
        &[
            ChemicalCompound::Water,
            ChemicalCompound::CarbonDioxide,
            ChemicalCompound::RawMinerals,
            ChemicalCompound::OrganicMatter,
            ChemicalCompound::Catalyst,
            ChemicalCompound::LiquidNitrogen,
        ]
    }
}

/// Products that can be created by the chemical processor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChemicalProduct {
    /// Oxygen for life support.
    Oxygen,
    /// Coolant for thermal regulation.
    Coolant,
    /// Sealant for hull repairs.
    Sealant,
    /// Fuel for thrusters.
    ThrusterFuel,
    /// Medical compound.
    MedicalCompound,
    /// Fire suppressant.
    FireSuppressant,
}

impl ChemicalProduct {
    /// Get display name.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            ChemicalProduct::Oxygen => "Oxygen",
            ChemicalProduct::Coolant => "Coolant",
            ChemicalProduct::Sealant => "Sealant",
            ChemicalProduct::ThrusterFuel => "Thruster Fuel",
            ChemicalProduct::MedicalCompound => "Medical Compound",
            ChemicalProduct::FireSuppressant => "Fire Suppressant",
        }
    }

    /// Get required compounds for this product.
    #[must_use]
    pub fn required_compounds(&self) -> Vec<(ChemicalCompound, u32)> {
        match self {
            ChemicalProduct::Oxygen => vec![(ChemicalCompound::Water, 2)],
            ChemicalProduct::Coolant => vec![
                (ChemicalCompound::Water, 1),
                (ChemicalCompound::LiquidNitrogen, 1),
            ],
            ChemicalProduct::Sealant => vec![
                (ChemicalCompound::RawMinerals, 2),
                (ChemicalCompound::Catalyst, 1),
            ],
            ChemicalProduct::ThrusterFuel => vec![
                (ChemicalCompound::Water, 1),
                (ChemicalCompound::RawMinerals, 1),
                (ChemicalCompound::Catalyst, 1),
            ],
            ChemicalProduct::MedicalCompound => vec![
                (ChemicalCompound::OrganicMatter, 2),
                (ChemicalCompound::Catalyst, 1),
            ],
            ChemicalProduct::FireSuppressant => vec![
                (ChemicalCompound::CarbonDioxide, 2),
                (ChemicalCompound::LiquidNitrogen, 1),
            ],
        }
    }

    /// Get processing time in seconds.
    #[must_use]
    pub fn processing_time(&self) -> f32 {
        match self {
            ChemicalProduct::Oxygen => 10.0,
            ChemicalProduct::Coolant => 15.0,
            ChemicalProduct::Sealant => 20.0,
            ChemicalProduct::ThrusterFuel => 25.0,
            ChemicalProduct::MedicalCompound => 30.0,
            ChemicalProduct::FireSuppressant => 12.0,
        }
    }

    /// Get power required for processing.
    #[must_use]
    pub fn power_required(&self) -> f32 {
        match self {
            ChemicalProduct::Oxygen => 15.0,
            ChemicalProduct::Coolant => 20.0,
            ChemicalProduct::Sealant => 10.0,
            ChemicalProduct::ThrusterFuel => 25.0,
            ChemicalProduct::MedicalCompound => 18.0,
            ChemicalProduct::FireSuppressant => 12.0,
        }
    }

    /// Get yield amount per process.
    #[must_use]
    pub fn yield_amount(&self) -> u32 {
        match self {
            ChemicalProduct::Oxygen => 5,
            ChemicalProduct::Coolant => 3,
            ChemicalProduct::Sealant => 2,
            ChemicalProduct::ThrusterFuel => 4,
            ChemicalProduct::MedicalCompound => 1,
            ChemicalProduct::FireSuppressant => 3,
        }
    }

    /// Get all product types.
    #[must_use]
    pub fn all() -> &'static [ChemicalProduct] {
        &[
            ChemicalProduct::Oxygen,
            ChemicalProduct::Coolant,
            ChemicalProduct::Sealant,
            ChemicalProduct::ThrusterFuel,
            ChemicalProduct::MedicalCompound,
            ChemicalProduct::FireSuppressant,
        ]
    }
}

/// Error types for chemical processing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProcessingError {
    /// Missing required compounds.
    InsufficientCompounds(Vec<(ChemicalCompound, u32)>),
    /// Processor is busy.
    ProcessorBusy,
    /// Processor is offline.
    ProcessorOffline,
}

/// A chemical processing station.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChemicalProcessor {
    /// Compound inventory.
    compounds: HashMap<ChemicalCompound, u32>,
    /// Product inventory.
    products: HashMap<ChemicalProduct, u32>,
    /// Current processing job.
    current_job: Option<ChemicalProduct>,
    /// Time remaining on current job.
    job_progress: f32,
    /// Whether the processor is powered.
    powered: bool,
}

impl Default for ChemicalProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl ChemicalProcessor {
    /// Create a new chemical processor.
    #[must_use]
    pub fn new() -> Self {
        Self {
            compounds: HashMap::new(),
            products: HashMap::new(),
            current_job: None,
            job_progress: 0.0,
            powered: true,
        }
    }

    /// Add compounds to inventory.
    pub fn add_compound(&mut self, compound: ChemicalCompound, amount: u32) {
        *self.compounds.entry(compound).or_insert(0) += amount;
    }

    /// Get compound count.
    #[must_use]
    pub fn compound_count(&self, compound: ChemicalCompound) -> u32 {
        *self.compounds.get(&compound).unwrap_or(&0)
    }

    /// Get product count.
    #[must_use]
    pub fn product_count(&self, product: ChemicalProduct) -> u32 {
        *self.products.get(&product).unwrap_or(&0)
    }

    /// Check if compounds are available for a product.
    #[must_use]
    pub fn can_process(&self, product: ChemicalProduct) -> bool {
        for (compound, required) in product.required_compounds() {
            if self.compound_count(compound) < required {
                return false;
            }
        }
        true
    }

    /// Get missing compounds for a product.
    #[must_use]
    pub fn missing_compounds(&self, product: ChemicalProduct) -> Vec<(ChemicalCompound, u32)> {
        let mut missing = Vec::new();
        for (compound, required) in product.required_compounds() {
            let have = self.compound_count(compound);
            if have < required {
                missing.push((compound, required - have));
            }
        }
        missing
    }

    /// Start processing a product.
    pub fn start_process(&mut self, product: ChemicalProduct) -> Result<(), ProcessingError> {
        if !self.powered {
            return Err(ProcessingError::ProcessorOffline);
        }
        if self.current_job.is_some() {
            return Err(ProcessingError::ProcessorBusy);
        }
        if !self.can_process(product) {
            return Err(ProcessingError::InsufficientCompounds(
                self.missing_compounds(product),
            ));
        }

        // Consume compounds
        for (compound, required) in product.required_compounds() {
            *self.compounds.entry(compound).or_insert(0) -= required;
        }

        self.current_job = Some(product);
        self.job_progress = product.processing_time();
        Ok(())
    }

    /// Check if processing is in progress.
    #[must_use]
    pub fn is_busy(&self) -> bool {
        self.current_job.is_some()
    }

    /// Get current job.
    #[must_use]
    pub fn current_job(&self) -> Option<ChemicalProduct> {
        self.current_job
    }

    /// Get remaining time on current job.
    #[must_use]
    pub fn job_time_remaining(&self) -> f32 {
        self.job_progress
    }

    /// Get job progress as percentage.
    #[must_use]
    pub fn job_progress_percent(&self) -> f32 {
        if let Some(product) = self.current_job {
            let total = product.processing_time();
            ((total - self.job_progress) / total) * 100.0
        } else {
            0.0
        }
    }

    /// Set powered state.
    pub fn set_powered(&mut self, powered: bool) {
        self.powered = powered;
    }

    /// Check if processor is powered.
    #[must_use]
    pub fn is_powered(&self) -> bool {
        self.powered
    }

    /// Update the processor, returns completed product and yield if any.
    pub fn tick(&mut self, dt: f32) -> Option<(ChemicalProduct, u32)> {
        if !self.powered {
            return None;
        }

        if let Some(product) = self.current_job {
            self.job_progress -= dt;
            if self.job_progress <= 0.0 {
                let yield_amount = product.yield_amount();
                *self.products.entry(product).or_insert(0) += yield_amount;
                self.current_job = None;
                self.job_progress = 0.0;
                return Some((product, yield_amount));
            }
        }
        None
    }

    /// Take product from storage.
    pub fn take_product(&mut self, product: ChemicalProduct, amount: u32) -> u32 {
        let available = self.product_count(product);
        let to_take = amount.min(available);
        if to_take > 0 {
            *self.products.entry(product).or_insert(0) -= to_take;
        }
        to_take
    }

    /// Get all compounds in inventory.
    #[must_use]
    pub fn all_compounds(&self) -> &HashMap<ChemicalCompound, u32> {
        &self.compounds
    }

    /// Get all products in storage.
    #[must_use]
    pub fn all_products(&self) -> &HashMap<ChemicalProduct, u32> {
        &self.products
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chemical_compound_display_names() {
        assert_eq!(ChemicalCompound::Water.display_name(), "Water");
        assert_eq!(ChemicalCompound::CarbonDioxide.display_name(), "Carbon Dioxide");
        assert_eq!(ChemicalCompound::RawMinerals.display_name(), "Raw Minerals");
        assert_eq!(ChemicalCompound::OrganicMatter.display_name(), "Organic Matter");
        assert_eq!(ChemicalCompound::Catalyst.display_name(), "Catalyst");
        assert_eq!(ChemicalCompound::LiquidNitrogen.display_name(), "Liquid Nitrogen");
    }

    #[test]
    fn test_chemical_compound_all() {
        let all = ChemicalCompound::all();
        assert_eq!(all.len(), 6);
    }

    #[test]
    fn test_chemical_product_display_names() {
        assert_eq!(ChemicalProduct::Oxygen.display_name(), "Oxygen");
        assert_eq!(ChemicalProduct::Coolant.display_name(), "Coolant");
        assert_eq!(ChemicalProduct::Sealant.display_name(), "Sealant");
        assert_eq!(ChemicalProduct::ThrusterFuel.display_name(), "Thruster Fuel");
        assert_eq!(ChemicalProduct::MedicalCompound.display_name(), "Medical Compound");
        assert_eq!(ChemicalProduct::FireSuppressant.display_name(), "Fire Suppressant");
    }

    #[test]
    fn test_chemical_product_required_compounds() {
        let compounds = ChemicalProduct::Oxygen.required_compounds();
        assert_eq!(compounds.len(), 1);
        assert!(compounds.contains(&(ChemicalCompound::Water, 2)));
    }

    #[test]
    fn test_chemical_product_processing_time() {
        assert!((ChemicalProduct::Oxygen.processing_time() - 10.0).abs() < f32::EPSILON);
        assert!((ChemicalProduct::MedicalCompound.processing_time() - 30.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_chemical_product_yield_amount() {
        assert_eq!(ChemicalProduct::Oxygen.yield_amount(), 5);
        assert_eq!(ChemicalProduct::MedicalCompound.yield_amount(), 1);
    }

    #[test]
    fn test_chemical_product_all() {
        let all = ChemicalProduct::all();
        assert_eq!(all.len(), 6);
    }

    #[test]
    fn test_chemical_processor_new() {
        let proc = ChemicalProcessor::new();
        assert!(!proc.is_busy());
        assert!(proc.is_powered());
    }

    #[test]
    fn test_chemical_processor_add_compound() {
        let mut proc = ChemicalProcessor::new();
        proc.add_compound(ChemicalCompound::Water, 5);
        assert_eq!(proc.compound_count(ChemicalCompound::Water), 5);

        proc.add_compound(ChemicalCompound::Water, 3);
        assert_eq!(proc.compound_count(ChemicalCompound::Water), 8);
    }

    #[test]
    fn test_chemical_processor_compound_count_empty() {
        let proc = ChemicalProcessor::new();
        assert_eq!(proc.compound_count(ChemicalCompound::Catalyst), 0);
    }

    #[test]
    fn test_chemical_processor_can_process() {
        let mut proc = ChemicalProcessor::new();
        assert!(!proc.can_process(ChemicalProduct::Oxygen));

        proc.add_compound(ChemicalCompound::Water, 2);
        assert!(proc.can_process(ChemicalProduct::Oxygen));
    }

    #[test]
    fn test_chemical_processor_missing_compounds() {
        let proc = ChemicalProcessor::new();
        let missing = proc.missing_compounds(ChemicalProduct::Coolant);

        assert!(missing.contains(&(ChemicalCompound::Water, 1)));
        assert!(missing.contains(&(ChemicalCompound::LiquidNitrogen, 1)));
    }

    #[test]
    fn test_chemical_processor_start_process() {
        let mut proc = ChemicalProcessor::new();
        proc.add_compound(ChemicalCompound::Water, 5);

        assert!(proc.start_process(ChemicalProduct::Oxygen).is_ok());
        assert!(proc.is_busy());
        assert_eq!(proc.current_job(), Some(ChemicalProduct::Oxygen));
        assert_eq!(proc.compound_count(ChemicalCompound::Water), 3); // consumed 2
    }

    #[test]
    fn test_chemical_processor_start_process_insufficient() {
        let mut proc = ChemicalProcessor::new();
        let result = proc.start_process(ChemicalProduct::Oxygen);

        assert!(matches!(result, Err(ProcessingError::InsufficientCompounds(_))));
    }

    #[test]
    fn test_chemical_processor_start_process_busy() {
        let mut proc = ChemicalProcessor::new();
        proc.add_compound(ChemicalCompound::Water, 5);

        proc.start_process(ChemicalProduct::Oxygen).unwrap();
        let result = proc.start_process(ChemicalProduct::Oxygen);

        assert!(matches!(result, Err(ProcessingError::ProcessorBusy)));
    }

    #[test]
    fn test_chemical_processor_start_process_offline() {
        let mut proc = ChemicalProcessor::new();
        proc.add_compound(ChemicalCompound::Water, 5);
        proc.set_powered(false);

        let result = proc.start_process(ChemicalProduct::Oxygen);
        assert!(matches!(result, Err(ProcessingError::ProcessorOffline)));
    }

    #[test]
    fn test_chemical_processor_job_progress() {
        let mut proc = ChemicalProcessor::new();
        proc.add_compound(ChemicalCompound::Water, 2);
        proc.start_process(ChemicalProduct::Oxygen).unwrap();

        assert!((proc.job_time_remaining() - 10.0).abs() < f32::EPSILON);
        assert!((proc.job_progress_percent() - 0.0).abs() < f32::EPSILON);

        proc.tick(5.0);
        assert!((proc.job_progress_percent() - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_chemical_processor_tick_completion() {
        let mut proc = ChemicalProcessor::new();
        proc.add_compound(ChemicalCompound::Water, 2);
        proc.start_process(ChemicalProduct::Oxygen).unwrap();

        let result = proc.tick(15.0);
        assert_eq!(result, Some((ChemicalProduct::Oxygen, 5)));
        assert!(!proc.is_busy());
        assert_eq!(proc.product_count(ChemicalProduct::Oxygen), 5);
    }

    #[test]
    fn test_chemical_processor_tick_when_offline() {
        let mut proc = ChemicalProcessor::new();
        proc.add_compound(ChemicalCompound::Water, 2);
        proc.start_process(ChemicalProduct::Oxygen).unwrap();
        proc.set_powered(false);

        let progress_before = proc.job_time_remaining();
        let result = proc.tick(1.0);

        assert!(result.is_none());
        assert!((proc.job_time_remaining() - progress_before).abs() < f32::EPSILON);
    }

    #[test]
    fn test_chemical_processor_take_product() {
        let mut proc = ChemicalProcessor::new();
        proc.add_compound(ChemicalCompound::Water, 2);
        proc.start_process(ChemicalProduct::Oxygen).unwrap();
        proc.tick(15.0);

        let taken = proc.take_product(ChemicalProduct::Oxygen, 3);
        assert_eq!(taken, 3);
        assert_eq!(proc.product_count(ChemicalProduct::Oxygen), 2);
    }

    #[test]
    fn test_chemical_processor_take_product_more_than_available() {
        let mut proc = ChemicalProcessor::new();
        proc.add_compound(ChemicalCompound::Water, 2);
        proc.start_process(ChemicalProduct::Oxygen).unwrap();
        proc.tick(15.0);

        let taken = proc.take_product(ChemicalProduct::Oxygen, 10);
        assert_eq!(taken, 5); // Only 5 available
        assert_eq!(proc.product_count(ChemicalProduct::Oxygen), 0);
    }

    #[test]
    fn test_chemical_processor_take_product_none_available() {
        let mut proc = ChemicalProcessor::new();
        let taken = proc.take_product(ChemicalProduct::Oxygen, 5);
        assert_eq!(taken, 0);
    }

    #[test]
    fn test_chemical_processor_default() {
        let proc = ChemicalProcessor::default();
        assert!(!proc.is_busy());
        assert!(proc.is_powered());
    }

    #[test]
    fn test_chemical_processor_all_compounds() {
        let mut proc = ChemicalProcessor::new();
        proc.add_compound(ChemicalCompound::Water, 5);
        proc.add_compound(ChemicalCompound::Catalyst, 3);

        let compounds = proc.all_compounds();
        assert_eq!(compounds.len(), 2);
    }

    #[test]
    fn test_chemical_processor_all_products() {
        let mut proc = ChemicalProcessor::new();
        proc.add_compound(ChemicalCompound::Water, 2);
        proc.start_process(ChemicalProduct::Oxygen).unwrap();
        proc.tick(15.0);

        let products = proc.all_products();
        assert_eq!(products.len(), 1);
        assert_eq!(*products.get(&ChemicalProduct::Oxygen).unwrap(), 5);
    }

    #[test]
    fn test_chemical_processor_complex_product() {
        let mut proc = ChemicalProcessor::new();
        proc.add_compound(ChemicalCompound::Water, 1);
        proc.add_compound(ChemicalCompound::RawMinerals, 1);
        proc.add_compound(ChemicalCompound::Catalyst, 1);

        assert!(proc.can_process(ChemicalProduct::ThrusterFuel));
        proc.start_process(ChemicalProduct::ThrusterFuel).unwrap();

        // All compounds should be consumed
        assert_eq!(proc.compound_count(ChemicalCompound::Water), 0);
        assert_eq!(proc.compound_count(ChemicalCompound::RawMinerals), 0);
        assert_eq!(proc.compound_count(ChemicalCompound::Catalyst), 0);

        proc.tick(30.0);
        assert_eq!(proc.product_count(ChemicalProduct::ThrusterFuel), 4);
    }
}
