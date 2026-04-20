//! Repair bench for restoring hull integrity and fixing systems.
//!
//! Provides repair capabilities for damaged station components.

use serde::{Deserialize, Serialize};

/// Types of repairs that can be performed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RepairType {
    /// Hull plating repair.
    HullPlating,
    /// Electrical system repair.
    ElectricalSystem,
    /// Mechanical system repair.
    MechanicalSystem,
    /// Life support repair.
    LifeSupport,
    /// Pressure seal repair.
    PressureSeal,
}

impl RepairType {
    /// Get display name.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            RepairType::HullPlating => "Hull Plating",
            RepairType::ElectricalSystem => "Electrical System",
            RepairType::MechanicalSystem => "Mechanical System",
            RepairType::LifeSupport => "Life Support",
            RepairType::PressureSeal => "Pressure Seal",
        }
    }

    /// Get base repair time in seconds.
    #[must_use]
    pub fn base_repair_time(&self) -> f32 {
        match self {
            RepairType::HullPlating => 30.0,
            RepairType::ElectricalSystem => 20.0,
            RepairType::MechanicalSystem => 25.0,
            RepairType::LifeSupport => 35.0,
            RepairType::PressureSeal => 15.0,
        }
    }

    /// Get material cost for repair.
    #[must_use]
    pub fn material_cost(&self) -> u32 {
        match self {
            RepairType::HullPlating => 5,
            RepairType::ElectricalSystem => 3,
            RepairType::MechanicalSystem => 4,
            RepairType::LifeSupport => 6,
            RepairType::PressureSeal => 2,
        }
    }

    /// Get power required for repair.
    #[must_use]
    pub fn power_required(&self) -> f32 {
        match self {
            RepairType::HullPlating => 10.0,
            RepairType::ElectricalSystem => 5.0,
            RepairType::MechanicalSystem => 8.0,
            RepairType::LifeSupport => 12.0,
            RepairType::PressureSeal => 4.0,
        }
    }

    /// Get integrity restored per repair.
    #[must_use]
    pub fn integrity_restored(&self) -> f32 {
        match self {
            RepairType::HullPlating => 25.0,
            RepairType::ElectricalSystem => 30.0,
            RepairType::MechanicalSystem => 20.0,
            RepairType::LifeSupport => 35.0,
            RepairType::PressureSeal => 40.0,
        }
    }

    /// Get all repair types.
    #[must_use]
    pub fn all() -> &'static [RepairType] {
        &[
            RepairType::HullPlating,
            RepairType::ElectricalSystem,
            RepairType::MechanicalSystem,
            RepairType::LifeSupport,
            RepairType::PressureSeal,
        ]
    }
}

/// A repair job in progress.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RepairJob {
    /// Type of repair.
    repair_type: RepairType,
    /// Target system/component ID.
    target_id: usize,
    /// Time remaining.
    time_remaining: f32,
}

impl RepairJob {
    /// Create a new repair job.
    #[must_use]
    pub fn new(repair_type: RepairType, target_id: usize) -> Self {
        Self {
            repair_type,
            target_id,
            time_remaining: repair_type.base_repair_time(),
        }
    }

    /// Get repair type.
    #[must_use]
    pub fn repair_type(&self) -> RepairType {
        self.repair_type
    }

    /// Get target ID.
    #[must_use]
    pub fn target_id(&self) -> usize {
        self.target_id
    }

    /// Get time remaining.
    #[must_use]
    pub fn time_remaining(&self) -> f32 {
        self.time_remaining
    }

    /// Get progress percentage.
    #[must_use]
    pub fn progress_percent(&self) -> f32 {
        let total = self.repair_type.base_repair_time();
        ((total - self.time_remaining) / total) * 100.0
    }
}

/// Error types for repair operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RepairError {
    /// Not enough repair materials.
    InsufficientMaterials(u32),
    /// Repair bench is busy.
    BenchBusy,
    /// Repair bench is offline.
    BenchOffline,
    /// Target already at full integrity.
    AlreadyRepaired,
}

/// A repair bench station.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RepairBench {
    /// Available repair materials.
    materials: u32,
    /// Current repair job.
    current_job: Option<RepairJob>,
    /// Whether the bench is powered.
    powered: bool,
    /// Total repairs completed.
    repairs_completed: u32,
}

impl Default for RepairBench {
    fn default() -> Self {
        Self::new()
    }
}

impl RepairBench {
    /// Create a new repair bench.
    #[must_use]
    pub fn new() -> Self {
        Self {
            materials: 0,
            current_job: None,
            powered: true,
            repairs_completed: 0,
        }
    }

    /// Add repair materials.
    pub fn add_materials(&mut self, amount: u32) {
        self.materials += amount;
    }

    /// Get available materials.
    #[must_use]
    pub fn materials(&self) -> u32 {
        self.materials
    }

    /// Check if a repair can be started.
    #[must_use]
    pub fn can_repair(&self, repair_type: RepairType) -> bool {
        self.powered && self.current_job.is_none() && self.materials >= repair_type.material_cost()
    }

    /// Start a repair job.
    pub fn start_repair(
        &mut self,
        repair_type: RepairType,
        target_id: usize,
    ) -> Result<(), RepairError> {
        if !self.powered {
            return Err(RepairError::BenchOffline);
        }
        if self.current_job.is_some() {
            return Err(RepairError::BenchBusy);
        }
        let cost = repair_type.material_cost();
        if self.materials < cost {
            return Err(RepairError::InsufficientMaterials(cost - self.materials));
        }

        self.materials -= cost;
        self.current_job = Some(RepairJob::new(repair_type, target_id));
        Ok(())
    }

    /// Check if repair is in progress.
    #[must_use]
    pub fn is_busy(&self) -> bool {
        self.current_job.is_some()
    }

    /// Get current job.
    #[must_use]
    pub fn current_job(&self) -> Option<&RepairJob> {
        self.current_job.as_ref()
    }

    /// Set powered state.
    pub fn set_powered(&mut self, powered: bool) {
        self.powered = powered;
    }

    /// Check if bench is powered.
    #[must_use]
    pub fn is_powered(&self) -> bool {
        self.powered
    }

    /// Get total repairs completed.
    #[must_use]
    pub fn repairs_completed(&self) -> u32 {
        self.repairs_completed
    }

    /// Update the repair bench, returns completed repair info if any.
    pub fn tick(&mut self, dt: f32) -> Option<(RepairType, usize, f32)> {
        if !self.powered {
            return None;
        }

        if let Some(ref mut job) = self.current_job {
            job.time_remaining -= dt;
            if job.time_remaining <= 0.0 {
                let result = (
                    job.repair_type,
                    job.target_id,
                    job.repair_type.integrity_restored(),
                );
                self.current_job = None;
                self.repairs_completed += 1;
                return Some(result);
            }
        }
        None
    }

    /// Cancel current repair job, returns materials refunded.
    pub fn cancel_repair(&mut self) -> u32 {
        if let Some(job) = self.current_job.take() {
            let refund = job.repair_type.material_cost() / 2;
            self.materials += refund;
            refund
        } else {
            0
        }
    }

    /// Get estimated time remaining on current job.
    #[must_use]
    pub fn time_remaining(&self) -> f32 {
        self.current_job
            .as_ref()
            .map(|j| j.time_remaining)
            .unwrap_or(0.0)
    }

    /// Get progress percentage of current job.
    #[must_use]
    pub fn progress_percent(&self) -> f32 {
        self.current_job
            .as_ref()
            .map(|j| j.progress_percent())
            .unwrap_or(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repair_type_display_names() {
        assert_eq!(RepairType::HullPlating.display_name(), "Hull Plating");
        assert_eq!(RepairType::ElectricalSystem.display_name(), "Electrical System");
        assert_eq!(RepairType::MechanicalSystem.display_name(), "Mechanical System");
        assert_eq!(RepairType::LifeSupport.display_name(), "Life Support");
        assert_eq!(RepairType::PressureSeal.display_name(), "Pressure Seal");
    }

    #[test]
    fn test_repair_type_base_repair_time() {
        assert!((RepairType::HullPlating.base_repair_time() - 30.0).abs() < f32::EPSILON);
        assert!((RepairType::PressureSeal.base_repair_time() - 15.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_repair_type_material_cost() {
        assert_eq!(RepairType::HullPlating.material_cost(), 5);
        assert_eq!(RepairType::PressureSeal.material_cost(), 2);
    }

    #[test]
    fn test_repair_type_integrity_restored() {
        assert!((RepairType::HullPlating.integrity_restored() - 25.0).abs() < f32::EPSILON);
        assert!((RepairType::PressureSeal.integrity_restored() - 40.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_repair_type_all() {
        let all = RepairType::all();
        assert_eq!(all.len(), 5);
    }

    #[test]
    fn test_repair_job_new() {
        let job = RepairJob::new(RepairType::HullPlating, 0);
        assert_eq!(job.repair_type(), RepairType::HullPlating);
        assert_eq!(job.target_id(), 0);
        assert!((job.time_remaining() - 30.0).abs() < f32::EPSILON);
        assert!((job.progress_percent() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_repair_bench_new() {
        let bench = RepairBench::new();
        assert_eq!(bench.materials(), 0);
        assert!(!bench.is_busy());
        assert!(bench.is_powered());
        assert_eq!(bench.repairs_completed(), 0);
    }

    #[test]
    fn test_repair_bench_add_materials() {
        let mut bench = RepairBench::new();
        bench.add_materials(10);
        assert_eq!(bench.materials(), 10);

        bench.add_materials(5);
        assert_eq!(bench.materials(), 15);
    }

    #[test]
    fn test_repair_bench_can_repair() {
        let mut bench = RepairBench::new();
        assert!(!bench.can_repair(RepairType::PressureSeal));

        bench.add_materials(2);
        assert!(bench.can_repair(RepairType::PressureSeal));
    }

    #[test]
    fn test_repair_bench_start_repair() {
        let mut bench = RepairBench::new();
        bench.add_materials(10);

        assert!(bench.start_repair(RepairType::HullPlating, 0).is_ok());
        assert!(bench.is_busy());
        assert_eq!(bench.materials(), 5); // consumed 5
    }

    #[test]
    fn test_repair_bench_start_repair_insufficient() {
        let mut bench = RepairBench::new();
        let result = bench.start_repair(RepairType::HullPlating, 0);

        assert!(matches!(result, Err(RepairError::InsufficientMaterials(5))));
    }

    #[test]
    fn test_repair_bench_start_repair_busy() {
        let mut bench = RepairBench::new();
        bench.add_materials(20);

        bench.start_repair(RepairType::HullPlating, 0).unwrap();
        let result = bench.start_repair(RepairType::HullPlating, 1);

        assert!(matches!(result, Err(RepairError::BenchBusy)));
    }

    #[test]
    fn test_repair_bench_start_repair_offline() {
        let mut bench = RepairBench::new();
        bench.add_materials(10);
        bench.set_powered(false);

        let result = bench.start_repair(RepairType::HullPlating, 0);
        assert!(matches!(result, Err(RepairError::BenchOffline)));
    }

    #[test]
    fn test_repair_bench_tick_completion() {
        let mut bench = RepairBench::new();
        bench.add_materials(5);
        bench.start_repair(RepairType::HullPlating, 0).unwrap();

        let result = bench.tick(35.0);
        assert!(result.is_some());
        let (repair_type, target_id, integrity) = result.unwrap();
        assert_eq!(repair_type, RepairType::HullPlating);
        assert_eq!(target_id, 0);
        assert!((integrity - 25.0).abs() < f32::EPSILON);
        assert!(!bench.is_busy());
        assert_eq!(bench.repairs_completed(), 1);
    }

    #[test]
    fn test_repair_bench_tick_when_offline() {
        let mut bench = RepairBench::new();
        bench.add_materials(5);
        bench.start_repair(RepairType::HullPlating, 0).unwrap();
        bench.set_powered(false);

        let time_before = bench.time_remaining();
        let result = bench.tick(1.0);

        assert!(result.is_none());
        assert!((bench.time_remaining() - time_before).abs() < f32::EPSILON);
    }

    #[test]
    fn test_repair_bench_cancel_repair() {
        let mut bench = RepairBench::new();
        bench.add_materials(10);
        bench.start_repair(RepairType::HullPlating, 0).unwrap();

        let refund = bench.cancel_repair();
        assert_eq!(refund, 2); // half of 5
        assert!(!bench.is_busy());
        assert_eq!(bench.materials(), 5 + 2);
    }

    #[test]
    fn test_repair_bench_cancel_no_repair() {
        let mut bench = RepairBench::new();
        let refund = bench.cancel_repair();
        assert_eq!(refund, 0);
    }

    #[test]
    fn test_repair_bench_time_remaining() {
        let mut bench = RepairBench::new();
        assert!((bench.time_remaining() - 0.0).abs() < f32::EPSILON);

        bench.add_materials(5);
        bench.start_repair(RepairType::HullPlating, 0).unwrap();
        assert!((bench.time_remaining() - 30.0).abs() < f32::EPSILON);

        bench.tick(10.0);
        assert!((bench.time_remaining() - 20.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_repair_bench_progress_percent() {
        let mut bench = RepairBench::new();
        assert!((bench.progress_percent() - 0.0).abs() < f32::EPSILON);

        bench.add_materials(5);
        bench.start_repair(RepairType::HullPlating, 0).unwrap();
        assert!((bench.progress_percent() - 0.0).abs() < f32::EPSILON);

        bench.tick(15.0);
        assert!((bench.progress_percent() - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_repair_bench_current_job() {
        let mut bench = RepairBench::new();
        assert!(bench.current_job().is_none());

        bench.add_materials(5);
        bench.start_repair(RepairType::HullPlating, 0).unwrap();

        let job = bench.current_job().unwrap();
        assert_eq!(job.repair_type(), RepairType::HullPlating);
        assert_eq!(job.target_id(), 0);
    }

    #[test]
    fn test_repair_bench_default() {
        let bench = RepairBench::default();
        assert!(!bench.is_busy());
        assert!(bench.is_powered());
    }

    #[test]
    fn test_repair_bench_multiple_repairs() {
        let mut bench = RepairBench::new();
        bench.add_materials(20);

        // First repair
        bench.start_repair(RepairType::PressureSeal, 0).unwrap();
        bench.tick(20.0);
        assert_eq!(bench.repairs_completed(), 1);

        // Second repair
        bench.start_repair(RepairType::PressureSeal, 1).unwrap();
        bench.tick(20.0);
        assert_eq!(bench.repairs_completed(), 2);

        // Should have used 4 materials total
        assert_eq!(bench.materials(), 16);
    }

    #[test]
    fn test_all_repair_types_work() {
        for repair_type in RepairType::all() {
            let mut bench = RepairBench::new();
            bench.add_materials(10);

            let result = bench.start_repair(*repair_type, 0);
            assert!(result.is_ok() || matches!(result, Err(RepairError::InsufficientMaterials(_))));
        }
    }
}
