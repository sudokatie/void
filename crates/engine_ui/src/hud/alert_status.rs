//! Alert status HUD display.
//!
//! Shows active alerts and their priority levels.

use serde::{Deserialize, Serialize};

/// Alert priority level.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum AlertPriority {
    /// Informational alert.
    Info,
    /// Warning alert.
    Warning,
    /// Danger alert.
    Danger,
    /// Critical alert.
    Critical,
}

impl AlertPriority {
    /// Get display name.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            AlertPriority::Info => "Info",
            AlertPriority::Warning => "Warning",
            AlertPriority::Danger => "Danger",
            AlertPriority::Critical => "Critical",
        }
    }

    /// Get color (RGBA).
    #[must_use]
    pub fn color(&self) -> [f32; 4] {
        match self {
            AlertPriority::Info => [0.0, 0.5, 1.0, 1.0],
            AlertPriority::Warning => [1.0, 1.0, 0.0, 1.0],
            AlertPriority::Danger => [1.0, 0.5, 0.0, 1.0],
            AlertPriority::Critical => [1.0, 0.0, 0.0, 1.0],
        }
    }

    /// Check if this alert should flash.
    #[must_use]
    pub fn should_flash(&self) -> bool {
        matches!(self, AlertPriority::Danger | AlertPriority::Critical)
    }
}

/// An active alert.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Alert {
    /// Alert ID.
    id: u32,
    /// Alert message.
    message: String,
    /// Priority level.
    priority: AlertPriority,
    /// Duration remaining (None = permanent).
    duration: Option<f32>,
    /// Whether the alert is acknowledged.
    acknowledged: bool,
}

impl Alert {
    /// Create a new alert.
    #[must_use]
    pub fn new(id: u32, message: String, priority: AlertPriority) -> Self {
        Self {
            id,
            message,
            priority,
            duration: None,
            acknowledged: false,
        }
    }

    /// Create a timed alert.
    #[must_use]
    pub fn with_duration(mut self, duration: f32) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Get alert ID.
    #[must_use]
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Get message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Get priority.
    #[must_use]
    pub fn priority(&self) -> AlertPriority {
        self.priority
    }

    /// Get duration remaining.
    #[must_use]
    pub fn duration(&self) -> Option<f32> {
        self.duration
    }

    /// Check if acknowledged.
    #[must_use]
    pub fn is_acknowledged(&self) -> bool {
        self.acknowledged
    }

    /// Acknowledge the alert.
    pub fn acknowledge(&mut self) {
        self.acknowledged = true;
    }

    /// Check if alert is expired.
    #[must_use]
    pub fn is_expired(&self) -> bool {
        self.duration.is_some_and(|d| d <= 0.0)
    }

    /// Update alert (tick duration).
    pub fn tick(&mut self, dt: f32) {
        if let Some(ref mut duration) = self.duration {
            *duration -= dt;
        }
    }
}

/// Alert status display state.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AlertStatusDisplay {
    /// Active alerts.
    alerts: Vec<Alert>,
    /// Next alert ID.
    next_id: u32,
    /// Maximum visible alerts.
    max_visible: usize,
    /// Whether display is visible.
    visible: bool,
    /// Flash phase for animations.
    flash_phase: f32,
}

impl Default for AlertStatusDisplay {
    fn default() -> Self {
        Self::new()
    }
}

impl AlertStatusDisplay {
    /// Create a new alert status display.
    #[must_use]
    pub fn new() -> Self {
        Self {
            alerts: Vec::new(),
            next_id: 0,
            max_visible: 5,
            visible: true,
            flash_phase: 0.0,
        }
    }

    /// Add an alert, returns the alert ID.
    pub fn add_alert(&mut self, message: String, priority: AlertPriority) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.alerts.push(Alert::new(id, message, priority));
        // Sort by priority (highest first)
        self.alerts.sort_by(|a, b| b.priority().cmp(&a.priority()));
        id
    }

    /// Add a timed alert.
    pub fn add_timed_alert(&mut self, message: String, priority: AlertPriority, duration: f32) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.alerts.push(Alert::new(id, message, priority).with_duration(duration));
        self.alerts.sort_by(|a, b| b.priority().cmp(&a.priority()));
        id
    }

    /// Remove an alert by ID.
    pub fn remove_alert(&mut self, id: u32) -> bool {
        let len = self.alerts.len();
        self.alerts.retain(|a| a.id() != id);
        self.alerts.len() < len
    }

    /// Acknowledge an alert.
    pub fn acknowledge_alert(&mut self, id: u32) -> bool {
        if let Some(alert) = self.alerts.iter_mut().find(|a| a.id() == id) {
            alert.acknowledge();
            true
        } else {
            false
        }
    }

    /// Acknowledge all alerts.
    pub fn acknowledge_all(&mut self) {
        for alert in &mut self.alerts {
            alert.acknowledge();
        }
    }

    /// Clear all alerts.
    pub fn clear_all(&mut self) {
        self.alerts.clear();
    }

    /// Clear acknowledged alerts.
    pub fn clear_acknowledged(&mut self) {
        self.alerts.retain(|a| !a.is_acknowledged());
    }

    /// Get alert count.
    #[must_use]
    pub fn alert_count(&self) -> usize {
        self.alerts.len()
    }

    /// Get unacknowledged alert count.
    #[must_use]
    pub fn unacknowledged_count(&self) -> usize {
        self.alerts.iter().filter(|a| !a.is_acknowledged()).count()
    }

    /// Get visible alerts (up to max_visible).
    #[must_use]
    pub fn visible_alerts(&self) -> &[Alert] {
        let end = self.alerts.len().min(self.max_visible);
        &self.alerts[..end]
    }

    /// Get all alerts.
    #[must_use]
    pub fn all_alerts(&self) -> &[Alert] {
        &self.alerts
    }

    /// Get highest priority alert.
    #[must_use]
    pub fn highest_priority(&self) -> Option<&Alert> {
        self.alerts.first()
    }

    /// Check if there are critical alerts.
    #[must_use]
    pub fn has_critical(&self) -> bool {
        self.alerts.iter().any(|a| a.priority() == AlertPriority::Critical)
    }

    /// Check if visible.
    #[must_use]
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Set visibility.
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Get flash phase (0-1).
    #[must_use]
    pub fn flash_phase(&self) -> f32 {
        self.flash_phase
    }

    /// Update display state.
    pub fn update(&mut self, dt: f32) {
        // Update flash phase
        self.flash_phase = (self.flash_phase + dt * 2.0) % 1.0;

        // Update alert durations
        for alert in &mut self.alerts {
            alert.tick(dt);
        }

        // Remove expired alerts
        self.alerts.retain(|a| !a.is_expired());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_priority_display_names() {
        assert_eq!(AlertPriority::Info.display_name(), "Info");
        assert_eq!(AlertPriority::Warning.display_name(), "Warning");
        assert_eq!(AlertPriority::Danger.display_name(), "Danger");
        assert_eq!(AlertPriority::Critical.display_name(), "Critical");
    }

    #[test]
    fn test_alert_priority_ordering() {
        assert!(AlertPriority::Critical > AlertPriority::Danger);
        assert!(AlertPriority::Danger > AlertPriority::Warning);
        assert!(AlertPriority::Warning > AlertPriority::Info);
    }

    #[test]
    fn test_alert_priority_should_flash() {
        assert!(!AlertPriority::Info.should_flash());
        assert!(!AlertPriority::Warning.should_flash());
        assert!(AlertPriority::Danger.should_flash());
        assert!(AlertPriority::Critical.should_flash());
    }

    #[test]
    fn test_alert_new() {
        let alert = Alert::new(0, "Test".to_string(), AlertPriority::Warning);
        assert_eq!(alert.id(), 0);
        assert_eq!(alert.message(), "Test");
        assert_eq!(alert.priority(), AlertPriority::Warning);
        assert!(!alert.is_acknowledged());
    }

    #[test]
    fn test_alert_with_duration() {
        let alert = Alert::new(0, "Test".to_string(), AlertPriority::Info).with_duration(5.0);
        assert_eq!(alert.duration(), Some(5.0));
    }

    #[test]
    fn test_alert_acknowledge() {
        let mut alert = Alert::new(0, "Test".to_string(), AlertPriority::Warning);
        alert.acknowledge();
        assert!(alert.is_acknowledged());
    }

    #[test]
    fn test_alert_tick_expiration() {
        let mut alert = Alert::new(0, "Test".to_string(), AlertPriority::Info).with_duration(1.0);
        assert!(!alert.is_expired());

        alert.tick(1.5);
        assert!(alert.is_expired());
    }

    #[test]
    fn test_alert_status_display_new() {
        let display = AlertStatusDisplay::new();
        assert_eq!(display.alert_count(), 0);
        assert!(display.is_visible());
    }

    #[test]
    fn test_alert_status_display_add_alert() {
        let mut display = AlertStatusDisplay::new();
        let id = display.add_alert("Test".to_string(), AlertPriority::Warning);

        assert_eq!(display.alert_count(), 1);
        assert_eq!(display.all_alerts()[0].id(), id);
    }

    #[test]
    fn test_alert_status_display_priority_sorting() {
        let mut display = AlertStatusDisplay::new();
        display.add_alert("Info".to_string(), AlertPriority::Info);
        display.add_alert("Critical".to_string(), AlertPriority::Critical);
        display.add_alert("Warning".to_string(), AlertPriority::Warning);

        assert_eq!(display.highest_priority().unwrap().priority(), AlertPriority::Critical);
    }

    #[test]
    fn test_alert_status_display_add_timed_alert() {
        let mut display = AlertStatusDisplay::new();
        display.add_timed_alert("Test".to_string(), AlertPriority::Info, 5.0);

        assert_eq!(display.all_alerts()[0].duration(), Some(5.0));
    }

    #[test]
    fn test_alert_status_display_remove_alert() {
        let mut display = AlertStatusDisplay::new();
        let id = display.add_alert("Test".to_string(), AlertPriority::Warning);

        assert!(display.remove_alert(id));
        assert_eq!(display.alert_count(), 0);
    }

    #[test]
    fn test_alert_status_display_acknowledge_alert() {
        let mut display = AlertStatusDisplay::new();
        let id = display.add_alert("Test".to_string(), AlertPriority::Warning);

        assert!(display.acknowledge_alert(id));
        assert!(display.all_alerts()[0].is_acknowledged());
    }

    #[test]
    fn test_alert_status_display_acknowledge_all() {
        let mut display = AlertStatusDisplay::new();
        display.add_alert("Test1".to_string(), AlertPriority::Warning);
        display.add_alert("Test2".to_string(), AlertPriority::Info);
        display.acknowledge_all();

        assert_eq!(display.unacknowledged_count(), 0);
    }

    #[test]
    fn test_alert_status_display_clear_acknowledged() {
        let mut display = AlertStatusDisplay::new();
        let id1 = display.add_alert("Test1".to_string(), AlertPriority::Warning);
        display.add_alert("Test2".to_string(), AlertPriority::Info);
        display.acknowledge_alert(id1);
        display.clear_acknowledged();

        assert_eq!(display.alert_count(), 1);
    }

    #[test]
    fn test_alert_status_display_visible_alerts_limit() {
        let mut display = AlertStatusDisplay::new();
        for i in 0..10 {
            display.add_alert(format!("Test{}", i), AlertPriority::Info);
        }

        assert_eq!(display.visible_alerts().len(), 5); // max_visible default
    }

    #[test]
    fn test_alert_status_display_has_critical() {
        let mut display = AlertStatusDisplay::new();
        assert!(!display.has_critical());

        display.add_alert("Test".to_string(), AlertPriority::Critical);
        assert!(display.has_critical());
    }

    #[test]
    fn test_alert_status_display_update_expiration() {
        let mut display = AlertStatusDisplay::new();
        display.add_timed_alert("Test".to_string(), AlertPriority::Info, 1.0);

        display.update(1.5);
        assert_eq!(display.alert_count(), 0);
    }

    #[test]
    fn test_alert_status_display_default() {
        let display = AlertStatusDisplay::default();
        assert_eq!(display.alert_count(), 0);
    }
}
