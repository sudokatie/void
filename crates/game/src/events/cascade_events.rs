//! Cascade event system for space station survival.
//!
//! Provides event chains where one event triggers subsequent effects.

use serde::{Deserialize, Serialize};

/// Types of cascade events that can trigger chain reactions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CascadeEventType {
    /// Hull breach triggering decompression cascade.
    HullBreach,
    /// Reactor damage causing power failures.
    ReactorDamage,
    /// Fire spreading through connected rooms.
    FireSpread,
    /// Life support failure affecting atmosphere.
    LifeSupportFailure,
    /// Electrical cascade from damaged systems.
    ElectricalCascade,
}

impl CascadeEventType {
    /// Get display name for this cascade type.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            CascadeEventType::HullBreach => "Hull Breach Cascade",
            CascadeEventType::ReactorDamage => "Reactor Damage Cascade",
            CascadeEventType::FireSpread => "Fire Spread",
            CascadeEventType::LifeSupportFailure => "Life Support Failure",
            CascadeEventType::ElectricalCascade => "Electrical Cascade",
        }
    }

    /// Get description of this cascade type.
    #[must_use]
    pub fn description(&self) -> &'static str {
        match self {
            CascadeEventType::HullBreach => {
                "Hull breach causing rapid decompression and system failures"
            }
            CascadeEventType::ReactorDamage => {
                "Reactor damage leading to power grid collapse"
            }
            CascadeEventType::FireSpread => {
                "Fire spreading to connected compartments"
            }
            CascadeEventType::LifeSupportFailure => {
                "Life support failure affecting atmosphere quality"
            }
            CascadeEventType::ElectricalCascade => {
                "Electrical damage cascading through connected systems"
            }
        }
    }

    /// Get the default effects for this cascade type.
    #[must_use]
    pub fn default_effects(&self) -> Vec<CascadeEffect> {
        match self {
            CascadeEventType::HullBreach => vec![
                CascadeEffect::Decompression,
                CascadeEffect::PowerFailure,
                CascadeEffect::StationDark,
            ],
            CascadeEventType::ReactorDamage => vec![
                CascadeEffect::PowerFailure,
                CascadeEffect::LifeSupportOffline,
                CascadeEffect::StationDark,
            ],
            CascadeEventType::FireSpread => vec![
                CascadeEffect::HullDamage,
                CascadeEffect::AtmosphereContamination,
                CascadeEffect::LifeSupportOffline,
            ],
            CascadeEventType::LifeSupportFailure => vec![
                CascadeEffect::AtmosphereContamination,
                CascadeEffect::OxygenDepletion,
            ],
            CascadeEventType::ElectricalCascade => vec![
                CascadeEffect::PowerFailure,
                CascadeEffect::SystemDamage,
                CascadeEffect::StationDark,
            ],
        }
    }

    /// Get all cascade event types.
    #[must_use]
    pub fn all() -> &'static [CascadeEventType] {
        &[
            CascadeEventType::HullBreach,
            CascadeEventType::ReactorDamage,
            CascadeEventType::FireSpread,
            CascadeEventType::LifeSupportFailure,
            CascadeEventType::ElectricalCascade,
        ]
    }
}

/// Individual effects that can occur in a cascade.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CascadeEffect {
    /// Room is decompressing.
    Decompression,
    /// Power has failed.
    PowerFailure,
    /// Station lights are out.
    StationDark,
    /// Life support is offline.
    LifeSupportOffline,
    /// Hull has taken damage.
    HullDamage,
    /// Atmosphere is contaminated.
    AtmosphereContamination,
    /// Oxygen is depleting.
    OxygenDepletion,
    /// Systems are damaged.
    SystemDamage,
}

impl CascadeEffect {
    /// Get display name for this effect.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            CascadeEffect::Decompression => "Decompression",
            CascadeEffect::PowerFailure => "Power Failure",
            CascadeEffect::StationDark => "Station Dark",
            CascadeEffect::LifeSupportOffline => "Life Support Offline",
            CascadeEffect::HullDamage => "Hull Damage",
            CascadeEffect::AtmosphereContamination => "Atmosphere Contamination",
            CascadeEffect::OxygenDepletion => "Oxygen Depletion",
            CascadeEffect::SystemDamage => "System Damage",
        }
    }

    /// Get severity level (1-5).
    #[must_use]
    pub fn severity(&self) -> u32 {
        match self {
            CascadeEffect::Decompression => 5,
            CascadeEffect::PowerFailure => 4,
            CascadeEffect::StationDark => 2,
            CascadeEffect::LifeSupportOffline => 5,
            CascadeEffect::HullDamage => 3,
            CascadeEffect::AtmosphereContamination => 3,
            CascadeEffect::OxygenDepletion => 4,
            CascadeEffect::SystemDamage => 2,
        }
    }

    /// Check if this effect is immediately dangerous.
    #[must_use]
    pub fn is_critical(&self) -> bool {
        matches!(
            self,
            CascadeEffect::Decompression
                | CascadeEffect::LifeSupportOffline
                | CascadeEffect::OxygenDepletion
        )
    }

    /// Get delay before this effect triggers (in seconds).
    #[must_use]
    pub fn trigger_delay(&self) -> f32 {
        match self {
            CascadeEffect::Decompression => 0.0,
            CascadeEffect::PowerFailure => 2.0,
            CascadeEffect::StationDark => 3.0,
            CascadeEffect::LifeSupportOffline => 5.0,
            CascadeEffect::HullDamage => 1.0,
            CascadeEffect::AtmosphereContamination => 10.0,
            CascadeEffect::OxygenDepletion => 30.0,
            CascadeEffect::SystemDamage => 1.0,
        }
    }

    /// Get all cascade effects.
    #[must_use]
    pub fn all() -> &'static [CascadeEffect] {
        &[
            CascadeEffect::Decompression,
            CascadeEffect::PowerFailure,
            CascadeEffect::StationDark,
            CascadeEffect::LifeSupportOffline,
            CascadeEffect::HullDamage,
            CascadeEffect::AtmosphereContamination,
            CascadeEffect::OxygenDepletion,
            CascadeEffect::SystemDamage,
        ]
    }
}

/// Tracks a single effect in the cascade chain.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct PendingEffect {
    effect: CascadeEffect,
    delay_remaining: f32,
    triggered: bool,
}

/// A cascade event with multiple chained effects.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CascadeEvent {
    /// Type of cascade event.
    event_type: CascadeEventType,
    /// Room where the cascade started.
    source_room: usize,
    /// Effects in this cascade.
    effects: Vec<CascadeEffect>,
    /// Pending effects with delays.
    pending: Vec<PendingEffect>,
    /// Effects that have been triggered.
    triggered_effects: Vec<CascadeEffect>,
    /// Whether the cascade is complete.
    complete: bool,
}

impl CascadeEvent {
    /// Create a new cascade event.
    #[must_use]
    pub fn new(event_type: CascadeEventType, source_room: usize) -> Self {
        let effects = event_type.default_effects();
        let pending = effects
            .iter()
            .map(|&effect| PendingEffect {
                effect,
                delay_remaining: effect.trigger_delay(),
                triggered: false,
            })
            .collect();

        Self {
            event_type,
            source_room,
            effects,
            pending,
            triggered_effects: Vec::new(),
            complete: false,
        }
    }

    /// Create a cascade event with custom effects.
    #[must_use]
    pub fn with_effects(
        event_type: CascadeEventType,
        source_room: usize,
        effects: Vec<CascadeEffect>,
    ) -> Self {
        let pending = effects
            .iter()
            .map(|&effect| PendingEffect {
                effect,
                delay_remaining: effect.trigger_delay(),
                triggered: false,
            })
            .collect();

        Self {
            event_type,
            source_room,
            effects,
            pending,
            triggered_effects: Vec::new(),
            complete: false,
        }
    }

    /// Get the trigger (initial cause) of this cascade.
    #[must_use]
    pub fn trigger(&self) -> CascadeEventType {
        self.event_type
    }

    /// Get the list of effects in this cascade.
    #[must_use]
    pub fn effects(&self) -> &[CascadeEffect] {
        &self.effects
    }

    /// Get the source room.
    #[must_use]
    pub fn source_room(&self) -> usize {
        self.source_room
    }

    /// Get effects that have been triggered.
    #[must_use]
    pub fn triggered_effects(&self) -> &[CascadeEffect] {
        &self.triggered_effects
    }

    /// Check if the cascade is complete.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.complete
    }

    /// Check if a specific effect has been triggered.
    #[must_use]
    pub fn has_triggered(&self, effect: CascadeEffect) -> bool {
        self.triggered_effects.contains(&effect)
    }

    /// Update the cascade, returns newly triggered effects.
    pub fn tick(&mut self, dt: f32) -> Vec<CascadeEffect> {
        if self.complete {
            return Vec::new();
        }

        let mut newly_triggered = Vec::new();

        for pending in &mut self.pending {
            if pending.triggered {
                continue;
            }

            pending.delay_remaining -= dt;
            if pending.delay_remaining <= 0.0 {
                pending.triggered = true;
                self.triggered_effects.push(pending.effect);
                newly_triggered.push(pending.effect);
            }
        }

        // Check if all effects have triggered
        if self.pending.iter().all(|p| p.triggered) {
            self.complete = true;
        }

        newly_triggered
    }

    /// Force trigger all remaining effects immediately.
    pub fn force_complete(&mut self) -> Vec<CascadeEffect> {
        let mut newly_triggered = Vec::new();

        for pending in &mut self.pending {
            if !pending.triggered {
                pending.triggered = true;
                self.triggered_effects.push(pending.effect);
                newly_triggered.push(pending.effect);
            }
        }

        self.complete = true;
        newly_triggered
    }

    /// Get the total severity of all effects.
    #[must_use]
    pub fn total_severity(&self) -> u32 {
        self.effects.iter().map(|e| e.severity()).sum()
    }

    /// Get the number of critical effects.
    #[must_use]
    pub fn critical_effect_count(&self) -> usize {
        self.effects.iter().filter(|e| e.is_critical()).count()
    }

    /// Get progress as a percentage (0-100).
    #[must_use]
    pub fn progress_percent(&self) -> f32 {
        if self.effects.is_empty() {
            return 100.0;
        }
        (self.triggered_effects.len() as f32 / self.effects.len() as f32) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // CascadeEventType tests
    #[test]
    fn test_cascade_event_type_display_names() {
        assert_eq!(
            CascadeEventType::HullBreach.display_name(),
            "Hull Breach Cascade"
        );
        assert_eq!(
            CascadeEventType::ReactorDamage.display_name(),
            "Reactor Damage Cascade"
        );
        assert_eq!(CascadeEventType::FireSpread.display_name(), "Fire Spread");
        assert_eq!(
            CascadeEventType::LifeSupportFailure.display_name(),
            "Life Support Failure"
        );
        assert_eq!(
            CascadeEventType::ElectricalCascade.display_name(),
            "Electrical Cascade"
        );
    }

    #[test]
    fn test_cascade_event_type_descriptions() {
        assert!(CascadeEventType::HullBreach
            .description()
            .contains("breach"));
        assert!(CascadeEventType::ReactorDamage
            .description()
            .contains("Reactor"));
        assert!(CascadeEventType::FireSpread.description().contains("Fire"));
        assert!(CascadeEventType::LifeSupportFailure
            .description()
            .contains("Life support"));
        assert!(CascadeEventType::ElectricalCascade
            .description()
            .contains("Electrical"));
    }

    #[test]
    fn test_cascade_event_type_default_effects() {
        let effects = CascadeEventType::HullBreach.default_effects();
        assert!(effects.contains(&CascadeEffect::Decompression));
        assert!(effects.contains(&CascadeEffect::PowerFailure));
        assert!(effects.contains(&CascadeEffect::StationDark));
    }

    #[test]
    fn test_cascade_event_type_reactor_damage_effects() {
        let effects = CascadeEventType::ReactorDamage.default_effects();
        assert!(effects.contains(&CascadeEffect::PowerFailure));
        assert!(effects.contains(&CascadeEffect::LifeSupportOffline));
        assert!(effects.contains(&CascadeEffect::StationDark));
    }

    #[test]
    fn test_cascade_event_type_all() {
        let all = CascadeEventType::all();
        assert_eq!(all.len(), 5);
        assert!(all.contains(&CascadeEventType::HullBreach));
        assert!(all.contains(&CascadeEventType::ReactorDamage));
        assert!(all.contains(&CascadeEventType::FireSpread));
        assert!(all.contains(&CascadeEventType::LifeSupportFailure));
        assert!(all.contains(&CascadeEventType::ElectricalCascade));
    }

    // CascadeEffect tests
    #[test]
    fn test_cascade_effect_display_names() {
        assert_eq!(CascadeEffect::Decompression.display_name(), "Decompression");
        assert_eq!(CascadeEffect::PowerFailure.display_name(), "Power Failure");
        assert_eq!(CascadeEffect::StationDark.display_name(), "Station Dark");
        assert_eq!(
            CascadeEffect::LifeSupportOffline.display_name(),
            "Life Support Offline"
        );
        assert_eq!(CascadeEffect::HullDamage.display_name(), "Hull Damage");
        assert_eq!(
            CascadeEffect::AtmosphereContamination.display_name(),
            "Atmosphere Contamination"
        );
        assert_eq!(
            CascadeEffect::OxygenDepletion.display_name(),
            "Oxygen Depletion"
        );
        assert_eq!(CascadeEffect::SystemDamage.display_name(), "System Damage");
    }

    #[test]
    fn test_cascade_effect_severity() {
        assert_eq!(CascadeEffect::Decompression.severity(), 5);
        assert_eq!(CascadeEffect::PowerFailure.severity(), 4);
        assert_eq!(CascadeEffect::StationDark.severity(), 2);
        assert_eq!(CascadeEffect::LifeSupportOffline.severity(), 5);
        assert_eq!(CascadeEffect::HullDamage.severity(), 3);
        assert_eq!(CascadeEffect::AtmosphereContamination.severity(), 3);
        assert_eq!(CascadeEffect::OxygenDepletion.severity(), 4);
        assert_eq!(CascadeEffect::SystemDamage.severity(), 2);
    }

    #[test]
    fn test_cascade_effect_is_critical() {
        assert!(CascadeEffect::Decompression.is_critical());
        assert!(!CascadeEffect::PowerFailure.is_critical());
        assert!(!CascadeEffect::StationDark.is_critical());
        assert!(CascadeEffect::LifeSupportOffline.is_critical());
        assert!(!CascadeEffect::HullDamage.is_critical());
        assert!(!CascadeEffect::AtmosphereContamination.is_critical());
        assert!(CascadeEffect::OxygenDepletion.is_critical());
        assert!(!CascadeEffect::SystemDamage.is_critical());
    }

    #[test]
    fn test_cascade_effect_trigger_delay() {
        assert!((CascadeEffect::Decompression.trigger_delay() - 0.0).abs() < f32::EPSILON);
        assert!((CascadeEffect::PowerFailure.trigger_delay() - 2.0).abs() < f32::EPSILON);
        assert!((CascadeEffect::StationDark.trigger_delay() - 3.0).abs() < f32::EPSILON);
        assert!((CascadeEffect::LifeSupportOffline.trigger_delay() - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_cascade_effect_all() {
        let all = CascadeEffect::all();
        assert_eq!(all.len(), 8);
    }

    // CascadeEvent tests
    #[test]
    fn test_cascade_event_new() {
        let event = CascadeEvent::new(CascadeEventType::HullBreach, 0);
        assert_eq!(event.trigger(), CascadeEventType::HullBreach);
        assert_eq!(event.source_room(), 0);
        assert!(!event.is_complete());
        assert!(event.triggered_effects().is_empty());
    }

    #[test]
    fn test_cascade_event_with_effects() {
        let effects = vec![CascadeEffect::PowerFailure, CascadeEffect::StationDark];
        let event = CascadeEvent::with_effects(CascadeEventType::ReactorDamage, 1, effects.clone());

        assert_eq!(event.effects().len(), 2);
        assert!(event.effects().contains(&CascadeEffect::PowerFailure));
        assert!(event.effects().contains(&CascadeEffect::StationDark));
    }

    #[test]
    fn test_cascade_event_tick_triggers_immediate() {
        let mut event = CascadeEvent::new(CascadeEventType::HullBreach, 0);

        // Decompression has 0 delay, should trigger immediately
        let triggered = event.tick(0.1);
        assert!(triggered.contains(&CascadeEffect::Decompression));
        assert!(event.has_triggered(CascadeEffect::Decompression));
    }

    #[test]
    fn test_cascade_event_tick_delayed_effects() {
        let effects = vec![CascadeEffect::PowerFailure]; // 2.0 second delay
        let mut event = CascadeEvent::with_effects(CascadeEventType::ReactorDamage, 0, effects);

        let triggered = event.tick(1.0);
        assert!(triggered.is_empty());
        assert!(!event.has_triggered(CascadeEffect::PowerFailure));

        let triggered = event.tick(1.5);
        assert!(triggered.contains(&CascadeEffect::PowerFailure));
        assert!(event.is_complete());
    }

    #[test]
    fn test_cascade_event_force_complete() {
        let mut event = CascadeEvent::new(CascadeEventType::HullBreach, 0);
        let triggered = event.force_complete();

        assert!(event.is_complete());
        assert_eq!(triggered.len(), event.effects().len());
    }

    #[test]
    fn test_cascade_event_total_severity() {
        let effects = vec![
            CascadeEffect::Decompression,  // 5
            CascadeEffect::PowerFailure,    // 4
        ];
        let event = CascadeEvent::with_effects(CascadeEventType::HullBreach, 0, effects);
        assert_eq!(event.total_severity(), 9);
    }

    #[test]
    fn test_cascade_event_critical_effect_count() {
        let event = CascadeEvent::new(CascadeEventType::HullBreach, 0);
        // HullBreach has: Decompression (critical), PowerFailure, StationDark
        assert_eq!(event.critical_effect_count(), 1);
    }

    #[test]
    fn test_cascade_event_progress_percent() {
        let effects = vec![CascadeEffect::PowerFailure, CascadeEffect::StationDark];
        let mut event = CascadeEvent::with_effects(CascadeEventType::ReactorDamage, 0, effects);

        assert!((event.progress_percent() - 0.0).abs() < f32::EPSILON);

        event.tick(5.0); // Trigger both
        assert!((event.progress_percent() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_cascade_event_tick_after_complete() {
        let mut event = CascadeEvent::new(CascadeEventType::HullBreach, 0);
        event.force_complete();

        let triggered = event.tick(1.0);
        assert!(triggered.is_empty());
    }

    #[test]
    fn test_cascade_event_empty_effects() {
        let event = CascadeEvent::with_effects(CascadeEventType::HullBreach, 0, Vec::new());
        assert!((event.progress_percent() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_all_cascade_types_create_events() {
        for event_type in CascadeEventType::all() {
            let event = CascadeEvent::new(*event_type, 0);
            assert_eq!(event.trigger(), *event_type);
            assert!(!event.effects().is_empty());
        }
    }

    #[test]
    fn test_cascade_event_full_sequence() {
        let mut event = CascadeEvent::new(CascadeEventType::ReactorDamage, 0);
        // ReactorDamage: PowerFailure (2s), LifeSupportOffline (5s), StationDark (3s)

        event.tick(0.5); // Nothing yet
        assert!(event.triggered_effects().is_empty());

        event.tick(2.0); // PowerFailure should trigger
        assert!(event.has_triggered(CascadeEffect::PowerFailure));

        event.tick(1.0); // StationDark should trigger (3s total)
        assert!(event.has_triggered(CascadeEffect::StationDark));

        event.tick(2.0); // LifeSupportOffline should trigger (5s total)
        assert!(event.has_triggered(CascadeEffect::LifeSupportOffline));

        assert!(event.is_complete());
    }
}
