//! Parasite management and spawning.
//!
//! Handles parasite lifecycle and Titan agitation effects.

use std::fmt;

use glam::IVec3;

/// Types of parasites that infest the Titan.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParasiteType {
    /// Small tick that feeds on scales.
    ScaleTick,
    /// Borer that tunnels through shell.
    ShellBorer,
    /// Leech that drains blood.
    BloodLeech,
    /// Wasp that affects neural tissue.
    NeuralWasp,
    /// Crawler that lives in the mouth.
    MouthCrawler,
}

impl fmt::Display for ParasiteType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParasiteType::ScaleTick => write!(f, "Scale Tick"),
            ParasiteType::ShellBorer => write!(f, "Shell Borer"),
            ParasiteType::BloodLeech => write!(f, "Blood Leech"),
            ParasiteType::NeuralWasp => write!(f, "Neural Wasp"),
            ParasiteType::MouthCrawler => write!(f, "Mouth Crawler"),
        }
    }
}

impl ParasiteType {
    /// Get the base HP for this parasite type.
    #[must_use]
    fn base_hp(self) -> f32 {
        match self {
            ParasiteType::ScaleTick => 10.0,
            ParasiteType::ShellBorer => 25.0,
            ParasiteType::BloodLeech => 15.0,
            ParasiteType::NeuralWasp => 20.0,
            ParasiteType::MouthCrawler => 30.0,
        }
    }

    /// Get the agitation reduction when killed.
    #[must_use]
    fn kill_relief(self) -> f32 {
        match self {
            ParasiteType::ScaleTick => 2.0,
            ParasiteType::ShellBorer => 5.0,
            ParasiteType::BloodLeech => 3.0,
            ParasiteType::NeuralWasp => 8.0,
            ParasiteType::MouthCrawler => 10.0,
        }
    }
}

/// A single parasite entry.
#[derive(Clone, Debug)]
pub struct ParasiteEntry {
    /// Type of parasite.
    pub parasite_type: ParasiteType,
    /// Position on the Titan.
    pub position: IVec3,
    /// Current health points.
    pub hp: f32,
}

impl ParasiteEntry {
    /// Create a new parasite entry.
    #[must_use]
    pub fn new(parasite_type: ParasiteType, position: IVec3) -> Self {
        Self {
            parasite_type,
            position,
            hp: parasite_type.base_hp(),
        }
    }

    /// Check if the parasite is still alive.
    #[must_use]
    pub fn is_alive(&self) -> bool {
        self.hp > 0.0
    }
}

/// Manages all parasites on the Titan.
#[derive(Clone, Debug)]
pub struct ParasiteManager {
    /// Active parasites.
    parasites: Vec<ParasiteEntry>,
    /// Current agitation from parasites.
    titan_mood_agitation: f32,
}

impl ParasiteManager {
    /// Create a new parasite manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            parasites: Vec::new(),
            titan_mood_agitation: 0.0,
        }
    }

    /// Add a parasite and return its index.
    pub fn add_parasite(&mut self, parasite: ParasiteEntry) -> usize {
        self.parasites.push(parasite);
        // Each parasite increases agitation
        self.titan_mood_agitation = (self.titan_mood_agitation + 2.0).min(100.0);
        self.parasites.len() - 1
    }

    /// Remove a parasite by index.
    pub fn remove_parasite(&mut self, index: usize) -> Option<ParasiteEntry> {
        if index < self.parasites.len() {
            Some(self.parasites.remove(index))
        } else {
            None
        }
    }

    /// Kill a parasite and get the agitation reduction.
    ///
    /// Returns the parasite type and agitation reduction.
    pub fn kill_parasite(&mut self, index: usize) -> (ParasiteType, f32) {
        if index < self.parasites.len() {
            let parasite = self.parasites.remove(index);
            let relief = parasite.parasite_type.kill_relief();
            self.titan_mood_agitation = (self.titan_mood_agitation - relief).max(0.0);
            (parasite.parasite_type, relief)
        } else {
            (ParasiteType::ScaleTick, 0.0)
        }
    }

    /// Update parasites and potentially spawn new ones.
    ///
    /// Returns a list of newly spawned parasite types.
    pub fn tick(&mut self, _dt: f32) -> Vec<ParasiteType> {
        let mut spawned = Vec::new();

        // Higher agitation increases spawn chance
        if self.titan_mood_agitation > 50.0 && self.parasites.len() < 20 {
            // Simplified spawn logic - in real game this would be more complex
            spawned.push(ParasiteType::ScaleTick);
        }

        spawned
    }

    /// Get the total number of parasites.
    #[must_use]
    pub fn parasite_count(&self) -> usize {
        self.parasites.len()
    }

    /// Get all parasites of a specific type.
    #[must_use]
    pub fn parasites_by_type(&self, ptype: ParasiteType) -> Vec<&ParasiteEntry> {
        self.parasites
            .iter()
            .filter(|p| p.parasite_type == ptype)
            .collect()
    }

    /// Get current agitation level.
    #[must_use]
    pub fn agitation_level(&self) -> f32 {
        self.titan_mood_agitation
    }

    /// Called when tissue is harvested, increases agitation.
    pub fn harvest_tissue(&mut self) -> f32 {
        let increase = 5.0;
        self.titan_mood_agitation = (self.titan_mood_agitation + increase).min(100.0);
        increase
    }
}

impl Default for ParasiteManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parasite_type_display() {
        assert_eq!(format!("{}", ParasiteType::ScaleTick), "Scale Tick");
        assert_eq!(format!("{}", ParasiteType::ShellBorer), "Shell Borer");
        assert_eq!(format!("{}", ParasiteType::BloodLeech), "Blood Leech");
        assert_eq!(format!("{}", ParasiteType::NeuralWasp), "Neural Wasp");
        assert_eq!(format!("{}", ParasiteType::MouthCrawler), "Mouth Crawler");
    }

    #[test]
    fn test_parasite_entry_new() {
        let entry = ParasiteEntry::new(ParasiteType::ScaleTick, IVec3::ZERO);
        assert_eq!(entry.parasite_type, ParasiteType::ScaleTick);
        assert!(entry.is_alive());
    }

    #[test]
    fn test_parasite_entry_is_alive() {
        let mut entry = ParasiteEntry::new(ParasiteType::ScaleTick, IVec3::ZERO);
        assert!(entry.is_alive());
        entry.hp = 0.0;
        assert!(!entry.is_alive());
    }

    #[test]
    fn test_parasite_manager_new() {
        let manager = ParasiteManager::new();
        assert_eq!(manager.parasite_count(), 0);
        assert!((manager.agitation_level() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_parasite_manager_add() {
        let mut manager = ParasiteManager::new();
        let entry = ParasiteEntry::new(ParasiteType::ShellBorer, IVec3::new(1, 2, 3));
        let index = manager.add_parasite(entry);
        assert_eq!(index, 0);
        assert_eq!(manager.parasite_count(), 1);
    }

    #[test]
    fn test_parasite_manager_remove() {
        let mut manager = ParasiteManager::new();
        let entry = ParasiteEntry::new(ParasiteType::BloodLeech, IVec3::ZERO);
        manager.add_parasite(entry);
        let removed = manager.remove_parasite(0);
        assert!(removed.is_some());
        assert_eq!(manager.parasite_count(), 0);
    }

    #[test]
    fn test_parasite_manager_kill() {
        let mut manager = ParasiteManager::new();
        let entry = ParasiteEntry::new(ParasiteType::NeuralWasp, IVec3::ZERO);
        manager.add_parasite(entry);
        let (ptype, relief) = manager.kill_parasite(0);
        assert_eq!(ptype, ParasiteType::NeuralWasp);
        assert!(relief > 0.0);
    }

    #[test]
    fn test_parasite_manager_parasites_by_type() {
        let mut manager = ParasiteManager::new();
        manager.add_parasite(ParasiteEntry::new(ParasiteType::ScaleTick, IVec3::ZERO));
        manager.add_parasite(ParasiteEntry::new(ParasiteType::ShellBorer, IVec3::ONE));
        manager.add_parasite(ParasiteEntry::new(ParasiteType::ScaleTick, IVec3::X));
        let ticks = manager.parasites_by_type(ParasiteType::ScaleTick);
        assert_eq!(ticks.len(), 2);
    }

    #[test]
    fn test_parasite_manager_harvest_tissue() {
        let mut manager = ParasiteManager::new();
        let increase = manager.harvest_tissue();
        assert!(increase > 0.0);
        assert!(manager.agitation_level() > 0.0);
    }

    #[test]
    fn test_parasite_manager_default() {
        let manager = ParasiteManager::default();
        assert_eq!(manager.parasite_count(), 0);
    }
}
