//! Fabricator for building equipment from materials.
//!
//! Converts raw materials into usable equipment and parts.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Materials that can be used in fabrication.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FabricatorMaterial {
    /// Refined metal alloy.
    MetalAlloy,
    /// Flexible polymer material.
    Polymer,
    /// Electronic components.
    Electronics,
    /// Reinforced composite material.
    Composite,
    /// Insulating material.
    Insulation,
    /// Optical glass.
    OpticalGlass,
}

impl FabricatorMaterial {
    /// Get display name.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            FabricatorMaterial::MetalAlloy => "Metal Alloy",
            FabricatorMaterial::Polymer => "Polymer",
            FabricatorMaterial::Electronics => "Electronics",
            FabricatorMaterial::Composite => "Composite",
            FabricatorMaterial::Insulation => "Insulation",
            FabricatorMaterial::OpticalGlass => "Optical Glass",
        }
    }

    /// Get all material types.
    #[must_use]
    pub fn all() -> &'static [FabricatorMaterial] {
        &[
            FabricatorMaterial::MetalAlloy,
            FabricatorMaterial::Polymer,
            FabricatorMaterial::Electronics,
            FabricatorMaterial::Composite,
            FabricatorMaterial::Insulation,
            FabricatorMaterial::OpticalGlass,
        ]
    }
}

/// Equipment types that can be fabricated.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FabricatorProduct {
    /// EVA suit component.
    EVASuitComponent,
    /// Thruster module.
    ThrusterModule,
    /// Hull patch material.
    HullPatch,
    /// Welding electrode.
    WeldingElectrode,
    /// Oxygen tank.
    OxygenTank,
    /// Multi-purpose tool head.
    ToolHead,
    /// Tether cable.
    TetherCable,
    /// Circuit board.
    CircuitBoard,
    /// Sensor array.
    SensorArray,
}

impl FabricatorProduct {
    /// Get display name.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            FabricatorProduct::EVASuitComponent => "EVA Suit Component",
            FabricatorProduct::ThrusterModule => "Thruster Module",
            FabricatorProduct::HullPatch => "Hull Patch",
            FabricatorProduct::WeldingElectrode => "Welding Electrode",
            FabricatorProduct::OxygenTank => "Oxygen Tank",
            FabricatorProduct::ToolHead => "Tool Head",
            FabricatorProduct::TetherCable => "Tether Cable",
            FabricatorProduct::CircuitBoard => "Circuit Board",
            FabricatorProduct::SensorArray => "Sensor Array",
        }
    }

    /// Get required materials for this product.
    #[must_use]
    pub fn required_materials(&self) -> Vec<(FabricatorMaterial, u32)> {
        match self {
            FabricatorProduct::EVASuitComponent => vec![
                (FabricatorMaterial::Polymer, 3),
                (FabricatorMaterial::Composite, 2),
                (FabricatorMaterial::Insulation, 1),
            ],
            FabricatorProduct::ThrusterModule => vec![
                (FabricatorMaterial::MetalAlloy, 2),
                (FabricatorMaterial::Electronics, 2),
            ],
            FabricatorProduct::HullPatch => vec![
                (FabricatorMaterial::MetalAlloy, 2),
                (FabricatorMaterial::Composite, 1),
            ],
            FabricatorProduct::WeldingElectrode => vec![(FabricatorMaterial::MetalAlloy, 1)],
            FabricatorProduct::OxygenTank => vec![
                (FabricatorMaterial::MetalAlloy, 2),
                (FabricatorMaterial::Polymer, 1),
            ],
            FabricatorProduct::ToolHead => vec![
                (FabricatorMaterial::MetalAlloy, 1),
                (FabricatorMaterial::Composite, 1),
            ],
            FabricatorProduct::TetherCable => vec![
                (FabricatorMaterial::Polymer, 2),
                (FabricatorMaterial::MetalAlloy, 1),
            ],
            FabricatorProduct::CircuitBoard => vec![
                (FabricatorMaterial::Electronics, 2),
                (FabricatorMaterial::Composite, 1),
            ],
            FabricatorProduct::SensorArray => vec![
                (FabricatorMaterial::Electronics, 3),
                (FabricatorMaterial::OpticalGlass, 2),
            ],
        }
    }

    /// Get fabrication time in seconds.
    #[must_use]
    pub fn fabrication_time(&self) -> f32 {
        match self {
            FabricatorProduct::EVASuitComponent => 30.0,
            FabricatorProduct::ThrusterModule => 25.0,
            FabricatorProduct::HullPatch => 10.0,
            FabricatorProduct::WeldingElectrode => 5.0,
            FabricatorProduct::OxygenTank => 15.0,
            FabricatorProduct::ToolHead => 8.0,
            FabricatorProduct::TetherCable => 12.0,
            FabricatorProduct::CircuitBoard => 20.0,
            FabricatorProduct::SensorArray => 35.0,
        }
    }

    /// Get power required for fabrication.
    #[must_use]
    pub fn power_required(&self) -> f32 {
        match self {
            FabricatorProduct::EVASuitComponent => 15.0,
            FabricatorProduct::ThrusterModule => 20.0,
            FabricatorProduct::HullPatch => 8.0,
            FabricatorProduct::WeldingElectrode => 5.0,
            FabricatorProduct::OxygenTank => 10.0,
            FabricatorProduct::ToolHead => 10.0,
            FabricatorProduct::TetherCable => 6.0,
            FabricatorProduct::CircuitBoard => 18.0,
            FabricatorProduct::SensorArray => 25.0,
        }
    }

    /// Get all product types.
    #[must_use]
    pub fn all() -> &'static [FabricatorProduct] {
        &[
            FabricatorProduct::EVASuitComponent,
            FabricatorProduct::ThrusterModule,
            FabricatorProduct::HullPatch,
            FabricatorProduct::WeldingElectrode,
            FabricatorProduct::OxygenTank,
            FabricatorProduct::ToolHead,
            FabricatorProduct::TetherCable,
            FabricatorProduct::CircuitBoard,
            FabricatorProduct::SensorArray,
        ]
    }
}

/// Error types for fabrication operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FabricationError {
    /// Missing required materials.
    InsufficientMaterials(Vec<(FabricatorMaterial, u32)>),
    /// Fabricator is busy.
    FabricatorBusy,
    /// Fabricator is offline.
    FabricatorOffline,
    /// Not enough power.
    InsufficientPower,
}

/// A fabricator station for building equipment.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fabricator {
    /// Material inventory.
    materials: HashMap<FabricatorMaterial, u32>,
    /// Currently fabricating product.
    current_job: Option<FabricatorProduct>,
    /// Time remaining on current job.
    job_progress: f32,
    /// Whether the fabricator is powered.
    powered: bool,
    /// Completed products ready for pickup.
    output_queue: Vec<FabricatorProduct>,
}

impl Default for Fabricator {
    fn default() -> Self {
        Self::new()
    }
}

impl Fabricator {
    /// Create a new fabricator.
    #[must_use]
    pub fn new() -> Self {
        Self {
            materials: HashMap::new(),
            current_job: None,
            job_progress: 0.0,
            powered: true,
            output_queue: Vec::new(),
        }
    }

    /// Add materials to the fabricator inventory.
    pub fn add_material(&mut self, material: FabricatorMaterial, amount: u32) {
        *self.materials.entry(material).or_insert(0) += amount;
    }

    /// Get the amount of a material in inventory.
    #[must_use]
    pub fn material_count(&self, material: FabricatorMaterial) -> u32 {
        *self.materials.get(&material).unwrap_or(&0)
    }

    /// Check if materials are available for a product.
    #[must_use]
    pub fn can_build(&self, product: FabricatorProduct) -> bool {
        for (material, required) in product.required_materials() {
            if self.material_count(material) < required {
                return false;
            }
        }
        true
    }

    /// Get missing materials for a product.
    #[must_use]
    pub fn missing_materials(&self, product: FabricatorProduct) -> Vec<(FabricatorMaterial, u32)> {
        let mut missing = Vec::new();
        for (material, required) in product.required_materials() {
            let have = self.material_count(material);
            if have < required {
                missing.push((material, required - have));
            }
        }
        missing
    }

    /// Start fabricating a product.
    pub fn start_build(&mut self, product: FabricatorProduct) -> Result<(), FabricationError> {
        if !self.powered {
            return Err(FabricationError::FabricatorOffline);
        }
        if self.current_job.is_some() {
            return Err(FabricationError::FabricatorBusy);
        }
        if !self.can_build(product) {
            return Err(FabricationError::InsufficientMaterials(
                self.missing_materials(product),
            ));
        }

        // Consume materials
        for (material, required) in product.required_materials() {
            *self.materials.entry(material).or_insert(0) -= required;
        }

        self.current_job = Some(product);
        self.job_progress = product.fabrication_time();
        Ok(())
    }

    /// Check if a job is in progress.
    #[must_use]
    pub fn is_busy(&self) -> bool {
        self.current_job.is_some()
    }

    /// Get the current job.
    #[must_use]
    pub fn current_job(&self) -> Option<FabricatorProduct> {
        self.current_job
    }

    /// Get remaining time on current job.
    #[must_use]
    pub fn job_time_remaining(&self) -> f32 {
        self.job_progress
    }

    /// Get job progress as percentage (0-100).
    #[must_use]
    pub fn job_progress_percent(&self) -> f32 {
        if let Some(product) = self.current_job {
            let total = product.fabrication_time();
            ((total - self.job_progress) / total) * 100.0
        } else {
            0.0
        }
    }

    /// Set powered state.
    pub fn set_powered(&mut self, powered: bool) {
        self.powered = powered;
    }

    /// Check if fabricator is powered.
    #[must_use]
    pub fn is_powered(&self) -> bool {
        self.powered
    }

    /// Update the fabricator, returns completed product if any.
    pub fn tick(&mut self, dt: f32) -> Option<FabricatorProduct> {
        if !self.powered {
            return None;
        }

        if let Some(product) = self.current_job {
            self.job_progress -= dt;
            if self.job_progress <= 0.0 {
                self.current_job = None;
                self.job_progress = 0.0;
                self.output_queue.push(product);
                return Some(product);
            }
        }
        None
    }

    /// Take a completed product from the output queue.
    pub fn take_output(&mut self) -> Option<FabricatorProduct> {
        self.output_queue.pop()
    }

    /// Get number of completed products in output queue.
    #[must_use]
    pub fn output_count(&self) -> usize {
        self.output_queue.len()
    }

    /// Get all materials in inventory.
    #[must_use]
    pub fn all_materials(&self) -> &HashMap<FabricatorMaterial, u32> {
        &self.materials
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fabricator_material_display_names() {
        assert_eq!(FabricatorMaterial::MetalAlloy.display_name(), "Metal Alloy");
        assert_eq!(FabricatorMaterial::Polymer.display_name(), "Polymer");
        assert_eq!(FabricatorMaterial::Electronics.display_name(), "Electronics");
        assert_eq!(FabricatorMaterial::Composite.display_name(), "Composite");
        assert_eq!(FabricatorMaterial::Insulation.display_name(), "Insulation");
        assert_eq!(FabricatorMaterial::OpticalGlass.display_name(), "Optical Glass");
    }

    #[test]
    fn test_fabricator_material_all() {
        let all = FabricatorMaterial::all();
        assert_eq!(all.len(), 6);
    }

    #[test]
    fn test_fabricator_product_display_names() {
        assert_eq!(FabricatorProduct::EVASuitComponent.display_name(), "EVA Suit Component");
        assert_eq!(FabricatorProduct::ThrusterModule.display_name(), "Thruster Module");
        assert_eq!(FabricatorProduct::HullPatch.display_name(), "Hull Patch");
    }

    #[test]
    fn test_fabricator_product_required_materials() {
        let materials = FabricatorProduct::HullPatch.required_materials();
        assert!(materials.contains(&(FabricatorMaterial::MetalAlloy, 2)));
        assert!(materials.contains(&(FabricatorMaterial::Composite, 1)));
    }

    #[test]
    fn test_fabricator_product_fabrication_time() {
        assert!((FabricatorProduct::WeldingElectrode.fabrication_time() - 5.0).abs() < f32::EPSILON);
        assert!((FabricatorProduct::SensorArray.fabrication_time() - 35.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_fabricator_product_power_required() {
        assert!((FabricatorProduct::WeldingElectrode.power_required() - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_fabricator_product_all() {
        let all = FabricatorProduct::all();
        assert_eq!(all.len(), 9);
    }

    #[test]
    fn test_fabricator_new() {
        let fab = Fabricator::new();
        assert!(!fab.is_busy());
        assert!(fab.is_powered());
        assert_eq!(fab.output_count(), 0);
    }

    #[test]
    fn test_fabricator_add_material() {
        let mut fab = Fabricator::new();
        fab.add_material(FabricatorMaterial::MetalAlloy, 5);
        assert_eq!(fab.material_count(FabricatorMaterial::MetalAlloy), 5);

        fab.add_material(FabricatorMaterial::MetalAlloy, 3);
        assert_eq!(fab.material_count(FabricatorMaterial::MetalAlloy), 8);
    }

    #[test]
    fn test_fabricator_material_count_empty() {
        let fab = Fabricator::new();
        assert_eq!(fab.material_count(FabricatorMaterial::Polymer), 0);
    }

    #[test]
    fn test_fabricator_can_build() {
        let mut fab = Fabricator::new();
        assert!(!fab.can_build(FabricatorProduct::WeldingElectrode));

        fab.add_material(FabricatorMaterial::MetalAlloy, 1);
        assert!(fab.can_build(FabricatorProduct::WeldingElectrode));
    }

    #[test]
    fn test_fabricator_missing_materials() {
        let fab = Fabricator::new();
        let missing = fab.missing_materials(FabricatorProduct::HullPatch);

        assert!(missing.contains(&(FabricatorMaterial::MetalAlloy, 2)));
        assert!(missing.contains(&(FabricatorMaterial::Composite, 1)));
    }

    #[test]
    fn test_fabricator_start_build() {
        let mut fab = Fabricator::new();
        fab.add_material(FabricatorMaterial::MetalAlloy, 5);

        assert!(fab.start_build(FabricatorProduct::WeldingElectrode).is_ok());
        assert!(fab.is_busy());
        assert_eq!(fab.current_job(), Some(FabricatorProduct::WeldingElectrode));
        assert_eq!(fab.material_count(FabricatorMaterial::MetalAlloy), 4); // consumed 1
    }

    #[test]
    fn test_fabricator_start_build_insufficient_materials() {
        let mut fab = Fabricator::new();
        let result = fab.start_build(FabricatorProduct::WeldingElectrode);

        assert!(matches!(result, Err(FabricationError::InsufficientMaterials(_))));
    }

    #[test]
    fn test_fabricator_start_build_when_busy() {
        let mut fab = Fabricator::new();
        fab.add_material(FabricatorMaterial::MetalAlloy, 5);

        fab.start_build(FabricatorProduct::WeldingElectrode).unwrap();
        let result = fab.start_build(FabricatorProduct::WeldingElectrode);

        assert!(matches!(result, Err(FabricationError::FabricatorBusy)));
    }

    #[test]
    fn test_fabricator_start_build_when_offline() {
        let mut fab = Fabricator::new();
        fab.add_material(FabricatorMaterial::MetalAlloy, 5);
        fab.set_powered(false);

        let result = fab.start_build(FabricatorProduct::WeldingElectrode);
        assert!(matches!(result, Err(FabricationError::FabricatorOffline)));
    }

    #[test]
    fn test_fabricator_job_progress() {
        let mut fab = Fabricator::new();
        fab.add_material(FabricatorMaterial::MetalAlloy, 1);
        fab.start_build(FabricatorProduct::WeldingElectrode).unwrap();

        assert!((fab.job_time_remaining() - 5.0).abs() < f32::EPSILON);
        assert!((fab.job_progress_percent() - 0.0).abs() < f32::EPSILON);

        fab.tick(2.5);
        assert!((fab.job_progress_percent() - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_fabricator_tick_completion() {
        let mut fab = Fabricator::new();
        fab.add_material(FabricatorMaterial::MetalAlloy, 1);
        fab.start_build(FabricatorProduct::WeldingElectrode).unwrap();

        let result = fab.tick(10.0);
        assert_eq!(result, Some(FabricatorProduct::WeldingElectrode));
        assert!(!fab.is_busy());
        assert_eq!(fab.output_count(), 1);
    }

    #[test]
    fn test_fabricator_tick_when_offline() {
        let mut fab = Fabricator::new();
        fab.add_material(FabricatorMaterial::MetalAlloy, 1);
        fab.start_build(FabricatorProduct::WeldingElectrode).unwrap();
        fab.set_powered(false);

        let progress_before = fab.job_time_remaining();
        let result = fab.tick(1.0);

        assert!(result.is_none());
        assert!((fab.job_time_remaining() - progress_before).abs() < f32::EPSILON);
    }

    #[test]
    fn test_fabricator_take_output() {
        let mut fab = Fabricator::new();
        fab.add_material(FabricatorMaterial::MetalAlloy, 1);
        fab.start_build(FabricatorProduct::WeldingElectrode).unwrap();
        fab.tick(10.0);

        let output = fab.take_output();
        assert_eq!(output, Some(FabricatorProduct::WeldingElectrode));
        assert_eq!(fab.output_count(), 0);
    }

    #[test]
    fn test_fabricator_take_output_empty() {
        let mut fab = Fabricator::new();
        assert!(fab.take_output().is_none());
    }

    #[test]
    fn test_fabricator_default() {
        let fab = Fabricator::default();
        assert!(!fab.is_busy());
        assert!(fab.is_powered());
    }

    #[test]
    fn test_fabricator_all_materials() {
        let mut fab = Fabricator::new();
        fab.add_material(FabricatorMaterial::MetalAlloy, 5);
        fab.add_material(FabricatorMaterial::Polymer, 3);

        let materials = fab.all_materials();
        assert_eq!(materials.len(), 2);
        assert_eq!(*materials.get(&FabricatorMaterial::MetalAlloy).unwrap(), 5);
    }

    #[test]
    fn test_fabricator_complex_build() {
        let mut fab = Fabricator::new();
        fab.add_material(FabricatorMaterial::Polymer, 3);
        fab.add_material(FabricatorMaterial::Composite, 2);
        fab.add_material(FabricatorMaterial::Insulation, 1);

        assert!(fab.can_build(FabricatorProduct::EVASuitComponent));
        fab.start_build(FabricatorProduct::EVASuitComponent).unwrap();

        // All materials should be consumed
        assert_eq!(fab.material_count(FabricatorMaterial::Polymer), 0);
        assert_eq!(fab.material_count(FabricatorMaterial::Composite), 0);
        assert_eq!(fab.material_count(FabricatorMaterial::Insulation), 0);
    }
}
