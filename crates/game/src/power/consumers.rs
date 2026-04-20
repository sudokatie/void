//! Power consumers and management.
//!
//! Tracks power-consuming systems and manages load shedding.

use serde::{Deserialize, Serialize};

use super::{PowerGrid, Reactor};

/// A power-consuming system.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PowerConsumer {
    /// ID of the room containing this consumer.
    room_id: usize,
    /// Name of the system.
    system_name: String,
    /// Priority (1=critical, 2=important, 3=low).
    priority: u32,
    /// Power draw when active.
    draw: f32,
    /// Whether the consumer is active.
    active: bool,
}

impl PowerConsumer {
    /// Create a new power consumer.
    #[must_use]
    pub fn new(room_id: usize, system_name: String, priority: u32, draw: f32) -> Self {
        Self {
            room_id,
            system_name,
            priority: priority.clamp(1, 3),
            draw,
            active: true,
        }
    }

    /// Get the room ID.
    #[must_use]
    pub fn room_id(&self) -> usize {
        self.room_id
    }

    /// Get the system name.
    #[must_use]
    pub fn system_name(&self) -> &str {
        &self.system_name
    }

    /// Get the priority.
    #[must_use]
    pub fn priority(&self) -> u32 {
        self.priority
    }

    /// Get the power draw.
    #[must_use]
    pub fn draw(&self) -> f32 {
        self.draw
    }

    /// Check if the consumer is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Get the effective power draw (0 if inactive).
    #[must_use]
    pub fn effective_draw(&self) -> f32 {
        if self.active {
            self.draw
        } else {
            0.0
        }
    }

    /// Activate the consumer.
    pub fn activate(&mut self) {
        self.active = true;
    }

    /// Deactivate the consumer.
    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

/// Manages power generation and distribution.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PowerManager {
    /// Main reactor.
    reactor: Reactor,
    /// Power grid.
    grid: PowerGrid,
    /// Registered consumers.
    consumers: Vec<PowerConsumer>,
}

impl Default for PowerManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PowerManager {
    /// Create a new power manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            reactor: Reactor::new(),
            grid: PowerGrid::new(),
            consumers: Vec::new(),
        }
    }

    /// Get a reference to the reactor.
    #[must_use]
    pub fn reactor(&self) -> &Reactor {
        &self.reactor
    }

    /// Get a mutable reference to the reactor.
    pub fn reactor_mut(&mut self) -> &mut Reactor {
        &mut self.reactor
    }

    /// Get a reference to the grid.
    #[must_use]
    pub fn grid(&self) -> &PowerGrid {
        &self.grid
    }

    /// Get a mutable reference to the grid.
    pub fn grid_mut(&mut self) -> &mut PowerGrid {
        &mut self.grid
    }

    /// Get the number of consumers.
    #[must_use]
    pub fn consumer_count(&self) -> usize {
        self.consumers.len()
    }

    /// Register a new power consumer.
    pub fn register_consumer(&mut self, consumer: PowerConsumer) {
        self.consumers.push(consumer);
    }

    /// Get consumers for a specific room.
    #[must_use]
    pub fn consumers_in_room(&self, room_id: usize) -> Vec<&PowerConsumer> {
        self.consumers
            .iter()
            .filter(|c| c.room_id() == room_id)
            .collect()
    }

    /// Shutdown the lowest priority consumers to reduce demand.
    ///
    /// Returns the room IDs of consumers that were shut down.
    pub fn shutdown_lowest_priority(&mut self) -> Vec<usize> {
        let mut affected_rooms = Vec::new();

        // Find active consumers with lowest priority (highest number)
        let mut lowest = self
            .consumers
            .iter_mut()
            .filter(|c| c.is_active())
            .collect::<Vec<_>>();

        lowest.sort_by(|a, b| b.priority().cmp(&a.priority()));

        // Shutdown up to 3 lowest priority consumers
        for consumer in lowest.into_iter().take(3) {
            if consumer.priority() >= 2 {
                consumer.deactivate();
                affected_rooms.push(consumer.room_id());
            }
        }

        affected_rooms
    }

    /// Tick the power system.
    ///
    /// Returns room IDs that lost power due to load shedding.
    pub fn tick(&mut self, dt: f32) -> Vec<usize> {
        // Reset grid for recalculation
        self.grid.reset();

        // Add reactor output to supply
        self.grid.add_supply(self.reactor.output());

        // Calculate total demand from active consumers
        let total_demand: f32 = self.consumers.iter().map(|c| c.effective_draw()).sum();
        self.grid.set_demand(total_demand);

        // Handle power deficit
        let mut affected_rooms = Vec::new();
        if !self.grid.is_balanced() {
            // Try to use battery first
            let battery_power = self.grid.drain_battery(dt);

            // If still not enough, shed load
            if self.grid.deficit() > battery_power {
                affected_rooms = self.shutdown_lowest_priority();
            }
        } else {
            // Charge battery with surplus
            self.grid.charge_battery(dt);
        }

        affected_rooms
    }

    /// Get the total power demand.
    #[must_use]
    pub fn total_demand(&self) -> f32 {
        self.consumers.iter().map(|c| c.effective_draw()).sum()
    }

    /// Get the available power (supply minus demand).
    #[must_use]
    pub fn available_power(&self) -> f32 {
        self.reactor.output() - self.total_demand()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_consumer_new() {
        let consumer = PowerConsumer::new(0, "Life Support".to_string(), 1, 25.0);
        assert_eq!(consumer.room_id(), 0);
        assert_eq!(consumer.system_name(), "Life Support");
        assert_eq!(consumer.priority(), 1);
        assert!((consumer.draw() - 25.0).abs() < f32::EPSILON);
        assert!(consumer.is_active());
    }

    #[test]
    fn test_power_consumer_priority_clamped() {
        let low = PowerConsumer::new(0, "Test".to_string(), 0, 10.0);
        assert_eq!(low.priority(), 1);

        let high = PowerConsumer::new(0, "Test".to_string(), 5, 10.0);
        assert_eq!(high.priority(), 3);
    }

    #[test]
    fn test_power_consumer_activate_deactivate() {
        let mut consumer = PowerConsumer::new(0, "Lights".to_string(), 3, 5.0);
        consumer.deactivate();
        assert!(!consumer.is_active());
        assert!((consumer.effective_draw() - 0.0).abs() < f32::EPSILON);

        consumer.activate();
        assert!(consumer.is_active());
        assert!((consumer.effective_draw() - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_power_manager_new() {
        let manager = PowerManager::new();
        assert!(manager.reactor().is_active());
        assert_eq!(manager.consumer_count(), 0);
    }

    #[test]
    fn test_power_manager_register_consumer() {
        let mut manager = PowerManager::new();
        manager.register_consumer(PowerConsumer::new(0, "Test".to_string(), 2, 10.0));
        assert_eq!(manager.consumer_count(), 1);
    }

    #[test]
    fn test_power_manager_consumers_in_room() {
        let mut manager = PowerManager::new();
        manager.register_consumer(PowerConsumer::new(0, "A".to_string(), 1, 10.0));
        manager.register_consumer(PowerConsumer::new(1, "B".to_string(), 2, 15.0));
        manager.register_consumer(PowerConsumer::new(0, "C".to_string(), 3, 5.0));

        let room_0 = manager.consumers_in_room(0);
        assert_eq!(room_0.len(), 2);
    }

    #[test]
    fn test_power_manager_total_demand() {
        let mut manager = PowerManager::new();
        manager.register_consumer(PowerConsumer::new(0, "A".to_string(), 1, 10.0));
        manager.register_consumer(PowerConsumer::new(1, "B".to_string(), 2, 15.0));

        assert!((manager.total_demand() - 25.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_power_manager_available_power() {
        let mut manager = PowerManager::new();
        manager.register_consumer(PowerConsumer::new(0, "A".to_string(), 1, 30.0));

        assert!((manager.available_power() - 70.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_power_manager_tick_balanced() {
        let mut manager = PowerManager::new();
        manager.register_consumer(PowerConsumer::new(0, "A".to_string(), 1, 50.0));

        let affected = manager.tick(1.0);
        assert!(affected.is_empty());
    }

    #[test]
    fn test_power_manager_tick_overload() {
        let mut manager = PowerManager::new();
        // Create demand greater than supply
        manager.register_consumer(PowerConsumer::new(0, "A".to_string(), 1, 40.0));
        manager.register_consumer(PowerConsumer::new(1, "B".to_string(), 3, 80.0));

        // Drain battery first
        manager.grid_mut().set_battery_charge(0.0);

        let affected = manager.tick(1.0);
        // Low priority consumer should be shut down
        assert!(!affected.is_empty());
    }

    #[test]
    fn test_power_manager_shutdown_lowest_priority() {
        let mut manager = PowerManager::new();
        manager.register_consumer(PowerConsumer::new(0, "Critical".to_string(), 1, 50.0));
        manager.register_consumer(PowerConsumer::new(1, "Low".to_string(), 3, 30.0));
        manager.register_consumer(PowerConsumer::new(2, "Important".to_string(), 2, 20.0));

        let affected = manager.shutdown_lowest_priority();

        // Should shut down priority 3 first, then 2
        assert!(!affected.is_empty());
        assert!(affected.contains(&1)); // Low priority room
    }

    #[test]
    fn test_power_manager_default() {
        let manager = PowerManager::default();
        assert!(manager.reactor().is_active());
    }

    #[test]
    fn test_power_manager_reactor_mut() {
        let mut manager = PowerManager::new();
        manager.reactor_mut().shutdown();
        assert!(!manager.reactor().is_active());
    }
}
