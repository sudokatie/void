//! Decompression event types and handling.
//!
//! Defines different types of hull breaches and their effects.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Type of decompression event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DecompressionType {
    /// Rapid decompression - moderate breach, fast pressure loss.
    Rapid,
    /// Slow decompression - small breach, gradual pressure loss.
    Slow,
    /// Explosive decompression - catastrophic breach, instant pressure loss.
    Explosive,
}

impl DecompressionType {
    /// Get the breach size multiplier for this decompression type.
    #[must_use]
    pub fn breach_size(&self) -> f32 {
        match self {
            DecompressionType::Slow => 0.1,
            DecompressionType::Rapid => 1.0,
            DecompressionType::Explosive => 10.0,
        }
    }

    /// Get the pressure loss rate per tick.
    #[must_use]
    pub fn loss_rate(&self) -> f32 {
        match self {
            DecompressionType::Slow => 0.5,
            DecompressionType::Rapid => 5.0,
            DecompressionType::Explosive => 50.0,
        }
    }

    /// Check if this decompression type is immediately lethal.
    #[must_use]
    pub fn is_lethal(&self) -> bool {
        matches!(self, DecompressionType::Explosive)
    }
}

impl fmt::Display for DecompressionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecompressionType::Rapid => write!(f, "Rapid"),
            DecompressionType::Slow => write!(f, "Slow"),
            DecompressionType::Explosive => write!(f, "Explosive"),
        }
    }
}

/// A decompression event in a room.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DecompressionEvent {
    /// Type of decompression.
    pub event_type: DecompressionType,
    /// ID of the affected room.
    pub room_id: usize,
    /// Current pressure loss rate.
    pub rate: f32,
}

impl DecompressionEvent {
    /// Create a new decompression event.
    #[must_use]
    pub fn new(room_id: usize, event_type: DecompressionType) -> Self {
        Self {
            event_type,
            room_id,
            rate: event_type.loss_rate(),
        }
    }

    /// Check if this event is critical (high rate or explosive).
    #[must_use]
    pub fn is_critical(&self) -> bool {
        self.rate >= 5.0 || self.event_type.is_lethal()
    }

    /// Get the severity level (0-3).
    #[must_use]
    pub fn severity(&self) -> u8 {
        match self.event_type {
            DecompressionType::Slow => 1,
            DecompressionType::Rapid => 2,
            DecompressionType::Explosive => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompression_type_slow() {
        assert!((DecompressionType::Slow.breach_size() - 0.1).abs() < f32::EPSILON);
        assert!((DecompressionType::Slow.loss_rate() - 0.5).abs() < f32::EPSILON);
        assert!(!DecompressionType::Slow.is_lethal());
    }

    #[test]
    fn test_decompression_type_rapid() {
        assert!((DecompressionType::Rapid.breach_size() - 1.0).abs() < f32::EPSILON);
        assert!((DecompressionType::Rapid.loss_rate() - 5.0).abs() < f32::EPSILON);
        assert!(!DecompressionType::Rapid.is_lethal());
    }

    #[test]
    fn test_decompression_type_explosive() {
        assert!((DecompressionType::Explosive.breach_size() - 10.0).abs() < f32::EPSILON);
        assert!((DecompressionType::Explosive.loss_rate() - 50.0).abs() < f32::EPSILON);
        assert!(DecompressionType::Explosive.is_lethal());
    }

    #[test]
    fn test_decompression_type_display() {
        assert_eq!(format!("{}", DecompressionType::Rapid), "Rapid");
        assert_eq!(format!("{}", DecompressionType::Slow), "Slow");
        assert_eq!(format!("{}", DecompressionType::Explosive), "Explosive");
    }

    #[test]
    fn test_decompression_event_new() {
        let event = DecompressionEvent::new(5, DecompressionType::Rapid);
        assert_eq!(event.room_id, 5);
        assert_eq!(event.event_type, DecompressionType::Rapid);
        assert!((event.rate - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_decompression_event_is_critical_explosive() {
        let event = DecompressionEvent::new(0, DecompressionType::Explosive);
        assert!(event.is_critical());
    }

    #[test]
    fn test_decompression_event_is_critical_rapid() {
        let event = DecompressionEvent::new(0, DecompressionType::Rapid);
        assert!(event.is_critical());
    }

    #[test]
    fn test_decompression_event_not_critical_slow() {
        let event = DecompressionEvent::new(0, DecompressionType::Slow);
        assert!(!event.is_critical());
    }

    #[test]
    fn test_decompression_event_severity() {
        assert_eq!(DecompressionEvent::new(0, DecompressionType::Slow).severity(), 1);
        assert_eq!(DecompressionEvent::new(0, DecompressionType::Rapid).severity(), 2);
        assert_eq!(DecompressionEvent::new(0, DecompressionType::Explosive).severity(), 3);
    }
}
