//! Reactor console for power distribution management.
//!
//! Provides control over power routing and distribution priorities.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Power priority levels for systems.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PowerPriority {
    /// Critical systems (life support, command).
    Critical = 0,
    /// High priority (sensors, communications).
    High = 1,
    /// Normal priority (lighting, non-essential).
    Normal = 2,
    /// Low priority (convenience systems).
    Low = 3,
}

impl PowerPriority {
    /// Get display name.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            PowerPriority::Critical => "Critical",
            PowerPriority::High => "High",
            PowerPriority::Normal => "Normal",
            PowerPriority::Low => "Low",
        }
    }

    /// Get all priority levels.
    #[must_use]
    pub fn all() -> &'static [PowerPriority] {
        &[
            PowerPriority::Critical,
            PowerPriority::High,
            PowerPriority::Normal,
            PowerPriority::Low,
        ]
    }
}

/// A power route configuration.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PowerRoute {
    /// Source ID.
    source_id: usize,
    /// Target ID.
    target_id: usize,
    /// Allocated power.
    allocated_power: f32,
    /// Whether the route is active.
    active: bool,
}

impl PowerRoute {
    /// Create a new power route.
    #[must_use]
    pub fn new(source_id: usize, target_id: usize, allocated_power: f32) -> Self {
        Self {
            source_id,
            target_id,
            allocated_power,
            active: true,
        }
    }

    /// Get source ID.
    #[must_use]
    pub fn source_id(&self) -> usize {
        self.source_id
    }

    /// Get target ID.
    #[must_use]
    pub fn target_id(&self) -> usize {
        self.target_id
    }

    /// Get allocated power.
    #[must_use]
    pub fn allocated_power(&self) -> f32 {
        self.allocated_power
    }

    /// Check if route is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Set allocated power.
    pub fn set_allocated_power(&mut self, power: f32) {
        self.allocated_power = power.max(0.0);
    }

    /// Enable or disable the route.
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}

/// A power distribution entry for a system.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PowerDistribution {
    /// System ID.
    system_id: usize,
    /// System name.
    name: String,
    /// Power demand.
    demand: f32,
    /// Current allocation.
    allocated: f32,
    /// Priority level.
    priority: PowerPriority,
    /// Whether powered.
    powered: bool,
}

impl PowerDistribution {
    /// Create a new power distribution entry.
    #[must_use]
    pub fn new(system_id: usize, name: String, demand: f32, priority: PowerPriority) -> Self {
        Self {
            system_id,
            name,
            demand,
            allocated: 0.0,
            priority,
            powered: false,
        }
    }

    /// Get system ID.
    #[must_use]
    pub fn system_id(&self) -> usize {
        self.system_id
    }

    /// Get system name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get power demand.
    #[must_use]
    pub fn demand(&self) -> f32 {
        self.demand
    }

    /// Get allocated power.
    #[must_use]
    pub fn allocated(&self) -> f32 {
        self.allocated
    }

    /// Get priority.
    #[must_use]
    pub fn priority(&self) -> PowerPriority {
        self.priority
    }

    /// Check if powered.
    #[must_use]
    pub fn is_powered(&self) -> bool {
        self.powered
    }

    /// Get power efficiency (allocated/demand).
    #[must_use]
    pub fn efficiency(&self) -> f32 {
        if self.demand <= 0.0 {
            return 1.0;
        }
        (self.allocated / self.demand).min(1.0)
    }
}

/// Error types for reactor console operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConsoleError {
    /// Console is offline.
    ConsoleOffline,
    /// Invalid system ID.
    InvalidSystem(usize),
    /// Insufficient power available.
    InsufficientPower,
}

/// Reactor console for power management.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReactorConsole {
    /// Power distributions by system ID.
    distributions: HashMap<usize, PowerDistribution>,
    /// Power routes.
    routes: Vec<PowerRoute>,
    /// Total available power.
    available_power: f32,
    /// Emergency power reserve.
    reserve_power: f32,
    /// Whether console is online.
    online: bool,
    /// Emergency mode active.
    emergency_mode: bool,
}

impl Default for ReactorConsole {
    fn default() -> Self {
        Self::new()
    }
}

impl ReactorConsole {
    /// Create a new reactor console.
    #[must_use]
    pub fn new() -> Self {
        Self {
            distributions: HashMap::new(),
            routes: Vec::new(),
            available_power: 100.0,
            reserve_power: 20.0,
            online: true,
            emergency_mode: false,
        }
    }

    /// Register a system for power distribution.
    pub fn register_system(
        &mut self,
        system_id: usize,
        name: String,
        demand: f32,
        priority: PowerPriority,
    ) {
        self.distributions.insert(
            system_id,
            PowerDistribution::new(system_id, name, demand, priority),
        );
    }

    /// Unregister a system.
    pub fn unregister_system(&mut self, system_id: usize) -> bool {
        self.distributions.remove(&system_id).is_some()
    }

    /// Get a system's distribution info.
    #[must_use]
    pub fn get_system(&self, system_id: usize) -> Option<&PowerDistribution> {
        self.distributions.get(&system_id)
    }

    /// Set a system's priority.
    pub fn set_priority(
        &mut self,
        system_id: usize,
        priority: PowerPriority,
    ) -> Result<(), ConsoleError> {
        if !self.online {
            return Err(ConsoleError::ConsoleOffline);
        }
        if let Some(dist) = self.distributions.get_mut(&system_id) {
            dist.priority = priority;
            Ok(())
        } else {
            Err(ConsoleError::InvalidSystem(system_id))
        }
    }

    /// Set available power from reactor.
    pub fn set_available_power(&mut self, power: f32) {
        self.available_power = power.max(0.0);
    }

    /// Get available power.
    #[must_use]
    pub fn available_power(&self) -> f32 {
        self.available_power
    }

    /// Get reserve power.
    #[must_use]
    pub fn reserve_power(&self) -> f32 {
        self.reserve_power
    }

    /// Set reserve power.
    pub fn set_reserve_power(&mut self, reserve: f32) {
        self.reserve_power = reserve.max(0.0);
    }

    /// Get total power demand.
    #[must_use]
    pub fn total_demand(&self) -> f32 {
        self.distributions.values().map(|d| d.demand).sum()
    }

    /// Get total allocated power.
    #[must_use]
    pub fn total_allocated(&self) -> f32 {
        self.distributions.values().map(|d| d.allocated).sum()
    }

    /// Get usable power (available minus reserve, unless emergency).
    #[must_use]
    pub fn usable_power(&self) -> f32 {
        if self.emergency_mode {
            self.available_power
        } else {
            (self.available_power - self.reserve_power).max(0.0)
        }
    }

    /// Set online state.
    pub fn set_online(&mut self, online: bool) {
        self.online = online;
    }

    /// Check if console is online.
    #[must_use]
    pub fn is_online(&self) -> bool {
        self.online
    }

    /// Activate emergency mode (uses reserve power).
    pub fn activate_emergency_mode(&mut self) {
        self.emergency_mode = true;
    }

    /// Deactivate emergency mode.
    pub fn deactivate_emergency_mode(&mut self) {
        self.emergency_mode = false;
    }

    /// Check if emergency mode is active.
    #[must_use]
    pub fn is_emergency_mode(&self) -> bool {
        self.emergency_mode
    }

    /// Add a power route.
    pub fn add_route(&mut self, route: PowerRoute) {
        self.routes.push(route);
    }

    /// Get all routes.
    #[must_use]
    pub fn routes(&self) -> &[PowerRoute] {
        &self.routes
    }

    /// Distribute power according to priorities.
    pub fn distribute_power(&mut self) -> Vec<usize> {
        if !self.online {
            // All systems unpowered
            for dist in self.distributions.values_mut() {
                dist.allocated = 0.0;
                dist.powered = false;
            }
            return self.distributions.keys().copied().collect();
        }

        let usable = self.usable_power();
        let mut remaining = usable;
        let mut unpowered = Vec::new();

        // Reset allocations
        for dist in self.distributions.values_mut() {
            dist.allocated = 0.0;
            dist.powered = false;
        }

        // Distribute by priority
        for priority in PowerPriority::all() {
            let systems: Vec<usize> = self
                .distributions
                .values()
                .filter(|d| d.priority == *priority)
                .map(|d| d.system_id)
                .collect();

            for system_id in systems {
                if let Some(dist) = self.distributions.get_mut(&system_id) {
                    let to_allocate = dist.demand.min(remaining);
                    dist.allocated = to_allocate;
                    dist.powered = to_allocate >= dist.demand * 0.5; // 50% minimum for operation
                    remaining -= to_allocate;

                    if !dist.powered {
                        unpowered.push(system_id);
                    }
                }
            }
        }

        unpowered
    }

    /// Get systems by priority.
    #[must_use]
    pub fn systems_by_priority(&self, priority: PowerPriority) -> Vec<&PowerDistribution> {
        self.distributions
            .values()
            .filter(|d| d.priority == priority)
            .collect()
    }

    /// Get all powered systems.
    #[must_use]
    pub fn powered_systems(&self) -> Vec<&PowerDistribution> {
        self.distributions.values().filter(|d| d.powered).collect()
    }

    /// Get all unpowered systems.
    #[must_use]
    pub fn unpowered_systems(&self) -> Vec<&PowerDistribution> {
        self.distributions.values().filter(|d| !d.powered).collect()
    }

    /// Get system count.
    #[must_use]
    pub fn system_count(&self) -> usize {
        self.distributions.len()
    }

    /// Get power deficit.
    #[must_use]
    pub fn power_deficit(&self) -> f32 {
        (self.total_demand() - self.usable_power()).max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_priority_display_names() {
        assert_eq!(PowerPriority::Critical.display_name(), "Critical");
        assert_eq!(PowerPriority::High.display_name(), "High");
        assert_eq!(PowerPriority::Normal.display_name(), "Normal");
        assert_eq!(PowerPriority::Low.display_name(), "Low");
    }

    #[test]
    fn test_power_priority_ordering() {
        assert!(PowerPriority::Critical < PowerPriority::High);
        assert!(PowerPriority::High < PowerPriority::Normal);
        assert!(PowerPriority::Normal < PowerPriority::Low);
    }

    #[test]
    fn test_power_priority_all() {
        let all = PowerPriority::all();
        assert_eq!(all.len(), 4);
    }

    #[test]
    fn test_power_route_new() {
        let route = PowerRoute::new(0, 1, 50.0);
        assert_eq!(route.source_id(), 0);
        assert_eq!(route.target_id(), 1);
        assert!((route.allocated_power() - 50.0).abs() < f32::EPSILON);
        assert!(route.is_active());
    }

    #[test]
    fn test_power_route_modify() {
        let mut route = PowerRoute::new(0, 1, 50.0);
        route.set_allocated_power(75.0);
        assert!((route.allocated_power() - 75.0).abs() < f32::EPSILON);

        route.set_active(false);
        assert!(!route.is_active());
    }

    #[test]
    fn test_power_distribution_new() {
        let dist = PowerDistribution::new(0, "Life Support".to_string(), 30.0, PowerPriority::Critical);
        assert_eq!(dist.system_id(), 0);
        assert_eq!(dist.name(), "Life Support");
        assert!((dist.demand() - 30.0).abs() < f32::EPSILON);
        assert_eq!(dist.priority(), PowerPriority::Critical);
        assert!(!dist.is_powered());
    }

    #[test]
    fn test_power_distribution_efficiency() {
        let mut dist = PowerDistribution::new(0, "Test".to_string(), 100.0, PowerPriority::Normal);
        assert!((dist.efficiency() - 0.0).abs() < f32::EPSILON);

        dist.allocated = 50.0;
        assert!((dist.efficiency() - 0.5).abs() < f32::EPSILON);

        dist.allocated = 100.0;
        assert!((dist.efficiency() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_reactor_console_new() {
        let console = ReactorConsole::new();
        assert!(console.is_online());
        assert!(!console.is_emergency_mode());
        assert!((console.available_power() - 100.0).abs() < f32::EPSILON);
        assert!((console.reserve_power() - 20.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_reactor_console_register_system() {
        let mut console = ReactorConsole::new();
        console.register_system(0, "Life Support".to_string(), 30.0, PowerPriority::Critical);

        let system = console.get_system(0).unwrap();
        assert_eq!(system.name(), "Life Support");
        assert_eq!(console.system_count(), 1);
    }

    #[test]
    fn test_reactor_console_unregister_system() {
        let mut console = ReactorConsole::new();
        console.register_system(0, "Test".to_string(), 10.0, PowerPriority::Normal);

        assert!(console.unregister_system(0));
        assert!(console.get_system(0).is_none());
        assert!(!console.unregister_system(0)); // Already removed
    }

    #[test]
    fn test_reactor_console_set_priority() {
        let mut console = ReactorConsole::new();
        console.register_system(0, "Test".to_string(), 10.0, PowerPriority::Normal);

        assert!(console.set_priority(0, PowerPriority::High).is_ok());
        assert_eq!(console.get_system(0).unwrap().priority(), PowerPriority::High);
    }

    #[test]
    fn test_reactor_console_set_priority_invalid() {
        let mut console = ReactorConsole::new();
        let result = console.set_priority(999, PowerPriority::High);
        assert!(matches!(result, Err(ConsoleError::InvalidSystem(999))));
    }

    #[test]
    fn test_reactor_console_set_priority_offline() {
        let mut console = ReactorConsole::new();
        console.register_system(0, "Test".to_string(), 10.0, PowerPriority::Normal);
        console.set_online(false);

        let result = console.set_priority(0, PowerPriority::High);
        assert!(matches!(result, Err(ConsoleError::ConsoleOffline)));
    }

    #[test]
    fn test_reactor_console_total_demand() {
        let mut console = ReactorConsole::new();
        console.register_system(0, "A".to_string(), 30.0, PowerPriority::Critical);
        console.register_system(1, "B".to_string(), 20.0, PowerPriority::High);

        assert!((console.total_demand() - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_reactor_console_usable_power() {
        let console = ReactorConsole::new();
        // 100 available - 20 reserve = 80 usable
        assert!((console.usable_power() - 80.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_reactor_console_usable_power_emergency() {
        let mut console = ReactorConsole::new();
        console.activate_emergency_mode();
        // All 100 available in emergency
        assert!((console.usable_power() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_reactor_console_distribute_power() {
        let mut console = ReactorConsole::new();
        console.register_system(0, "Critical".to_string(), 30.0, PowerPriority::Critical);
        console.register_system(1, "Normal".to_string(), 20.0, PowerPriority::Normal);

        let unpowered = console.distribute_power();
        assert!(unpowered.is_empty());

        let critical = console.get_system(0).unwrap();
        assert!(critical.is_powered());
        assert!((critical.allocated() - 30.0).abs() < f32::EPSILON);

        let normal = console.get_system(1).unwrap();
        assert!(normal.is_powered());
        assert!((normal.allocated() - 20.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_reactor_console_distribute_power_shortage() {
        let mut console = ReactorConsole::new();
        console.set_available_power(30.0);
        console.set_reserve_power(0.0);

        console.register_system(0, "Critical".to_string(), 30.0, PowerPriority::Critical);
        console.register_system(1, "Low".to_string(), 20.0, PowerPriority::Low);

        let unpowered = console.distribute_power();
        assert!(unpowered.contains(&1)); // Low priority unpowered

        let critical = console.get_system(0).unwrap();
        assert!(critical.is_powered());

        let low = console.get_system(1).unwrap();
        assert!(!low.is_powered());
    }

    #[test]
    fn test_reactor_console_distribute_power_offline() {
        let mut console = ReactorConsole::new();
        console.register_system(0, "Test".to_string(), 30.0, PowerPriority::Critical);
        console.set_online(false);

        let unpowered = console.distribute_power();
        assert!(unpowered.contains(&0));

        let system = console.get_system(0).unwrap();
        assert!(!system.is_powered());
    }

    #[test]
    fn test_reactor_console_systems_by_priority() {
        let mut console = ReactorConsole::new();
        console.register_system(0, "A".to_string(), 30.0, PowerPriority::Critical);
        console.register_system(1, "B".to_string(), 20.0, PowerPriority::Critical);
        console.register_system(2, "C".to_string(), 10.0, PowerPriority::Normal);

        let critical = console.systems_by_priority(PowerPriority::Critical);
        assert_eq!(critical.len(), 2);

        let normal = console.systems_by_priority(PowerPriority::Normal);
        assert_eq!(normal.len(), 1);
    }

    #[test]
    fn test_reactor_console_powered_unpowered_systems() {
        let mut console = ReactorConsole::new();
        console.set_available_power(30.0);
        console.set_reserve_power(0.0);

        console.register_system(0, "Critical".to_string(), 30.0, PowerPriority::Critical);
        console.register_system(1, "Low".to_string(), 20.0, PowerPriority::Low);

        console.distribute_power();

        let powered = console.powered_systems();
        assert_eq!(powered.len(), 1);

        let unpowered = console.unpowered_systems();
        assert_eq!(unpowered.len(), 1);
    }

    #[test]
    fn test_reactor_console_power_deficit() {
        let mut console = ReactorConsole::new();
        console.set_available_power(50.0);
        console.set_reserve_power(10.0);

        console.register_system(0, "A".to_string(), 30.0, PowerPriority::Critical);
        console.register_system(1, "B".to_string(), 30.0, PowerPriority::High);

        // Demand 60, usable 40 = deficit 20
        assert!((console.power_deficit() - 20.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_reactor_console_add_route() {
        let mut console = ReactorConsole::new();
        console.add_route(PowerRoute::new(0, 1, 50.0));

        assert_eq!(console.routes().len(), 1);
    }

    #[test]
    fn test_reactor_console_default() {
        let console = ReactorConsole::default();
        assert!(console.is_online());
        assert_eq!(console.system_count(), 0);
    }

    #[test]
    fn test_reactor_console_emergency_mode_toggle() {
        let mut console = ReactorConsole::new();
        assert!(!console.is_emergency_mode());

        console.activate_emergency_mode();
        assert!(console.is_emergency_mode());

        console.deactivate_emergency_mode();
        assert!(!console.is_emergency_mode());
    }
}
